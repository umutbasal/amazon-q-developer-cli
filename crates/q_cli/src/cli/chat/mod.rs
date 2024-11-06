mod api;
mod parse;
mod prompt;

use std::io::{
    Stderr,
    Write,
    stderr,
};
use std::process::ExitCode;
use std::time::Duration;

use color_eyre::owo_colors::OwoColorize;
use crossterm::style::{
    Attribute,
    Color,
    Print,
};
use crossterm::{
    ExecutableCommand,
    QueueableCommand,
    cursor,
    style,
    terminal,
};
use eyre::{
    Result,
    bail,
    eyre,
};
use fig_api_client::StreamingClient;
use fig_util::CLI_BINARY_NAME;
use prompt::{
    PROMPT,
    rl,
};
use rustyline::error::ReadlineError;
use spinners::{
    Spinner,
    Spinners,
};
use winnow::Partial;
use winnow::stream::Offset;

use self::api::send_message;
use self::parse::{
    ParseState,
    interpret_markdown,
};
use crate::util::region_check;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ApiResponse {
    Text(String),
    ConversationId(String),
    MessageId(String),
    Error(Option<String>),
    End,
}

pub async fn chat(input: String) -> Result<ExitCode> {
    if !fig_util::system_info::in_cloudshell() && !fig_auth::is_logged_in().await {
        bail!(
            "You are not logged in, please log in with {}",
            format!("{CLI_BINARY_NAME} login",).bold()
        );
    }

    region_check("chat")?;

    let mut stderr = stderr();
    let result = try_chat(&mut stderr, input).await;

    stderr
        .queue(style::SetAttribute(Attribute::Reset))?
        .queue(style::ResetColor)?
        .flush()
        .ok();

    result.map(|_| ExitCode::SUCCESS)
}

async fn try_chat(stderr: &mut Stderr, mut input: String) -> Result<()> {
    let mut rl = rl()?;
    let client = StreamingClient::new().await?;
    let mut rx = None;
    let mut conversation_id: Option<String> = None;
    let mut message_id = None;

    loop {
        // Make request with input
        match input.trim() {
            "exit" | "quit" => {
                if let Some(conversation_id) = conversation_id {
                    fig_telemetry::send_end_chat(conversation_id.clone()).await;
                }
                return Ok(());
            },
            _ => (),
        }

        if !input.is_empty() {
            stderr.queue(style::SetForegroundColor(Color::Magenta))?;
            if input.contains("@history") {
                stderr.queue(style::Print("Using shell history\n"))?;
            }

            if input.contains("@git") {
                stderr.queue(style::Print("Using git context\n"))?;
            }

            if input.contains("@env") {
                stderr.queue(style::Print("Using environment\n"))?;
            }

            rx = Some(send_message(client.clone(), input, conversation_id.clone()).await?);
            stderr
                .queue(style::SetForegroundColor(Color::Reset))?
                .execute(style::Print("\n"))?;
        } else if fig_settings::settings::get_bool_or("chat.greeting.enabled", true) {
            stderr.execute(style::Print(format!(
                "
Hi, I'm Amazon Q. I can answer questions about your shell and CLI tools!
You can include additional context by adding the following to your prompt:

{} to pass your shell history
{} to pass information about your current git repository
{} to pass your shell environment

",
                "@history".bold(),
                "@git".bold(),
                "@env".bold()
            )))?;
        }

        // Print response as we receive it
        if let Some(rx) = &mut rx {
            stderr.queue(cursor::Hide)?;
            let mut spinner = Some(Spinner::new(Spinners::Dots, "Generating your answer...".to_owned()));

            let mut buf = String::new();
            let mut offset = 0;
            let mut ended = false;

            let columns = crossterm::terminal::window_size()?.columns.into();
            let mut state = ParseState::new(columns);

            loop {
                if let Some(response) = rx.recv().await {
                    match response {
                        ApiResponse::Text(content) => match buf.is_empty() {
                            true => buf.push_str(content.trim_start()),
                            false => buf.push_str(&content),
                        },
                        ApiResponse::ConversationId(id) => {
                            if conversation_id.is_none() {
                                fig_telemetry::send_start_chat(id.clone()).await;

                                tokio::task::spawn(async move {
                                    tokio::signal::ctrl_c().await.unwrap();
                                    if let Some(conversation_id) = &conversation_id {
                                        fig_telemetry::send_end_chat(conversation_id.clone()).await;
                                        fig_telemetry::finish_telemetry().await;
                                        #[allow(clippy::exit)]
                                        std::process::exit(0);
                                    }
                                });
                            }

                            conversation_id = Some(id);
                        },
                        ApiResponse::MessageId(id) => message_id = Some(id),
                        ApiResponse::End => {
                            ended = true;
                        },
                        ApiResponse::Error(error) => {
                            drop(spinner.take());
                            stderr.queue(cursor::MoveToColumn(0))?;

                            match error {
                                Some(error) => stderr
                                    .queue(style::SetForegroundColor(Color::Red))?
                                    .queue(style::SetAttribute(Attribute::Bold))?
                                    .queue(style::Print("error"))?
                                    .queue(style::SetForegroundColor(Color::Reset))?
                                    .queue(style::SetAttribute(Attribute::Reset))?
                                    .queue(style::Print(format!(": {error}\n")))?,
                                None => stderr.queue(style::Print(
                                    "Amazon Q is having trouble responding right now. Try again later.",
                                ))?,
                            };

                            stderr.flush()?;
                            ended = true;
                        },
                    }
                }

                // this is a hack since otherwise the parser might report Incomplete with useful data
                // still left in the buffer. I'm not sure how this is intended to be handled.
                if ended {
                    buf.push('\n');
                }

                if !buf.is_empty() && spinner.take().is_some() {
                    stderr
                        .queue(terminal::Clear(terminal::ClearType::CurrentLine))?
                        .queue(cursor::MoveToColumn(0))?
                        .queue(cursor::Show)?;
                }

                loop {
                    let input = Partial::new(&buf[offset..]);
                    match interpret_markdown(input, stderr as &mut Stderr, &mut state) {
                        Ok(parsed) => {
                            offset += parsed.offset_from(&input);
                            stderr.lock().flush()?;
                            state.newline = state.set_newline;
                            state.set_newline = false;
                        },
                        Err(err) => match err.into_inner() {
                            Some(err) => return Err(eyre!(err.to_string())),
                            None => break, // Data was incomplete
                        },
                    }

                    tokio::time::sleep(Duration::from_millis(2)).await;
                }

                if ended {
                    stderr
                        .queue(style::ResetColor)?
                        .queue(style::SetAttribute(Attribute::Reset))?
                        .queue(Print("\n"))?;

                    for (i, citation) in &state.citations {
                        stderr
                            .queue(style::SetForegroundColor(Color::Blue))?
                            .queue(style::Print(format!("{i} ")))?
                            .queue(style::SetForegroundColor(Color::DarkGrey))?
                            .queue(style::Print(format!("{citation}\n")))?
                            .queue(style::SetForegroundColor(Color::Reset))?;
                    }

                    if !state.citations.is_empty() {
                        stderr.execute(Print("\n"))?;
                    }

                    if let (Some(conversation_id), Some(message_id)) = (&conversation_id, &message_id) {
                        fig_telemetry::send_chat_added_message(conversation_id.to_owned(), message_id.to_owned()).await;
                    }

                    break;
                }
            }
        }

        loop {
            let readline = rl.readline(PROMPT);
            match readline {
                Ok(line) => {
                    if line.trim().is_empty() {
                        continue;
                    }
                    let _ = rl.add_history_entry(line.as_str());
                    input = line;
                    break;
                },
                Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                    return Ok(());
                },
                Err(err) => {
                    return Err(err.into());
                },
            }
        }
    }
}