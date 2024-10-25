mod cli;
mod event;
// mod figterm;
mod auth_watcher;
mod file_watcher;
mod install;
mod local_ipc;
pub mod notification_bus;
mod platform;
pub mod protocol;
mod remote_ipc;
mod request;
mod tray;
mod update;
mod utils;
mod webview;

use std::path::Path;
use std::process::exit;
use std::sync::Arc;

use clap::Parser;
use event::Event;
use fig_log::{
    LogArgs,
    initialize_logging,
};
use fig_os_shim::Context;
use fig_util::consts::{
    APP_PROCESS_NAME,
    PRODUCT_NAME,
};
use fig_util::{
    URL_SCHEMA,
    directories,
};
use parking_lot::RwLock;
use platform::PlatformState;
use sysinfo::{
    ProcessRefreshKind,
    RefreshKind,
    System,
    get_current_pid,
};
use tao::event_loop::{
    EventLoop as WryEventLoop,
    EventLoopProxy as WryEventLoopProxy,
    EventLoopWindowTarget as WryEventLoopWindowTarget,
};
use tracing::{
    error,
    warn,
};
use url::Url;
use webview::notification::WebviewNotificationsState;
pub use webview::{
    AUTOCOMPLETE_ID,
    AUTOCOMPLETE_WINDOW_TITLE,
    DASHBOARD_ID,
};
use webview::{
    AutocompleteOptions,
    DashboardOptions,
    WebviewManager,
    autocomplete,
    build_autocomplete,
    build_dashboard,
    dashboard,
};

// #[global_allocator]
// static GLOBAL: Jemalloc = Jemalloc;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[derive(Debug, Default)]
pub struct DebugState {
    pub debug_lines: RwLock<Vec<String>>,
    pub color: RwLock<Option<String>>,
}

#[derive(Debug, Default)]
pub struct InterceptState {
    pub intercept_bound_keystrokes: RwLock<bool>,
    pub intercept_global_keystrokes: RwLock<bool>,
}

pub type EventLoop = WryEventLoop<Event>;
pub type EventLoopProxy = WryEventLoopProxy<Event>;
pub type EventLoopWindowTarget = WryEventLoopWindowTarget<Event>;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();

    let _log_guard = initialize_logging(LogArgs {
        log_level: None,
        log_to_stdout: true,
        log_file_path: Some(
            directories::logs_dir()
                .expect("home dir must be set")
                .join("fig_desktop.log"),
        ),
        delete_old_log_file: false,
    })
    .expect("Failed to init logging");

    fig_telemetry::init_global_telemetry_emitter();

    #[cfg(target_os = "macos")]
    install::migrate_data_dir().await;

    if let Err(err) = fig_settings::settings::init_global() {
        error!(%err, "failed to init global settings");
    }

    if cli.is_startup && !fig_settings::settings::get_bool_or("app.launchOnStartup", true) {
        std::process::exit(0);
    }

    let page = cli
        .url_link
        .as_ref()
        .and_then(|url| match Url::parse(url) {
            Ok(url) => Some(url),
            Err(err) => {
                error!(%err, %url, "Failed to parse url");
                exit(1)
            },
        })
        .and_then(|url| {
            if url.scheme() != URL_SCHEMA {
                error!(scheme = %url.scheme(), %url, "Invalid scheme");
                exit(1)
            }

            url.host_str().and_then(|s| match s {
                "dashboard" => Some(url.path().to_owned()),
                _ => {
                    error!("Invalid deep link");
                    None
                },
            })
        });

    if !cli.allow_multiple {
        match get_current_pid() {
            Ok(current_pid) => {
                allow_multiple_running_check(current_pid, cli.kill_old, page.clone()).await;
            },
            Err(err) => warn!(%err, "Failed to get pid"),
        }
    }

    #[cfg(target_os = "macos")]
    if let Ok(current_exe) = fig_util::current_exe_origin() {
        if let Ok(statvfs) = nix::sys::statvfs::statvfs(&current_exe) {
            if statvfs.flags().contains(nix::sys::statvfs::FsFlags::ST_RDONLY) {
                rfd::MessageDialog::new()
                    .set_title("Error")
                    .set_description(
                        format!("Cannot execute {PRODUCT_NAME} from within a readonly volume. Please move {PRODUCT_NAME} to your applications folder and try again.")
                    )
                    .show();

                return;
            }
        }
    }

    // tokio::spawn(async {
    //     fig_telemetry::emit_track(fig_telemetry::TrackEvent::new(
    //         fig_telemetry::TrackEventType::LaunchedApp,
    //         fig_telemetry::TrackSource::Desktop,
    //         env!("CARGO_PKG_VERSION").into(),
    //         empty::<(&str, &str)>(),
    //     ))
    //     .await
    //     .ok();
    // });

    let ctx = Context::new();
    install::run_install(Arc::clone(&ctx), cli.ignore_immediate_update).await;

    #[cfg(target_os = "linux")]
    {
        match std::env::var("Q_BACKEND").ok().as_deref() {
            Some("default") => {},
            // SAFETY: we are calling set_var in a single-threaded context.
            Some(backend) => unsafe { std::env::set_var("GDK_BACKEND", backend) },
            None => unsafe { std::env::set_var("GDK_BACKEND", "x11") },
        }

        platform::gtk::init().expect("Failed initializing GTK");
    }

    let is_logged_in = fig_auth::is_logged_in().await;

    if !is_logged_in {
        tracing::info!("Showing onboarding");
    }

    let accessibility_enabled = PlatformState::accessibility_is_enabled().unwrap_or(true);
    let visible = !cli.no_dashboard;

    let autocomplete_enabled =
        !fig_settings::settings::get_bool_or("autocomplete.disable", false) && is_logged_in && accessibility_enabled;

    let mut webview_manager = WebviewManager::new(ctx, visible);
    webview_manager
        .build_webview(
            DASHBOARD_ID,
            build_dashboard,
            DashboardOptions {
                show_onboarding: !is_logged_in,
                visible,
                page,
            },
            true,
            dashboard::url,
        )
        .unwrap();
    webview_manager
        .build_webview(
            AUTOCOMPLETE_ID,
            build_autocomplete,
            AutocompleteOptions,
            autocomplete_enabled,
            autocomplete::url,
        )
        .unwrap();

    webview_manager.run().await.unwrap();
    fig_telemetry::finish_telemetry().await;
}

#[cfg(target_os = "linux")]
async fn allow_multiple_running_check(current_pid: sysinfo::Pid, kill_old: bool, page: Option<String>) {
    use tracing::debug;

    if kill_old {
        eprintln!("Option kill-old is not supported on Linux.");
        #[allow(clippy::exit)]
        exit(0);
    }

    let system = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::new().with_user(sysinfo::UpdateKind::Always)),
    );
    let processes = system.processes_by_exact_name(APP_PROCESS_NAME);

    let processes = processes.collect::<Vec<_>>();
    debug!("Checking for already running desktop instance: {:?}", processes);

    let current_user_id = nix::unistd::getuid().as_raw();
    for process in processes {
        let pid = process.pid();
        let uid = process.user_id().map(|uid| uid as &u32);
        match (process.parent(), uid) {
            // The Linux desktop app returns multiple processes with the same name for some reason.
            (Some(parent_pid), Some(uid))
                if pid != current_pid && parent_pid != current_pid && *uid == current_user_id =>
            {
                let exe = process.exe().unwrap_or(Path::new("")).display();
                eprintln!("{PRODUCT_NAME} is already running: {exe} (pid={pid}, uid={uid})");

                match &page {
                    Some(page) => {
                        eprintln!("Opening /{page}...");
                        Some(page)
                    },
                    None => {
                        eprintln!("Opening {PRODUCT_NAME} Window...");
                        None
                    },
                };

                if let Err(err) =
                    fig_ipc::local::open_ui_element(fig_proto::local::UiElement::MissionControl, page).await
                {
                    eprintln!("Failed to open Fig: {err}");
                }

                #[allow(clippy::exit)]
                exit(0);
            },
            _ => (),
        }
    }
}

#[cfg(target_os = "macos")]
async fn allow_multiple_running_check(current_pid: sysinfo::Pid, kill_old: bool, page: Option<String>) {
    let system = System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::new()));
    let processes = system.processes_by_name(APP_PROCESS_NAME);

    cfg_if::cfg_if! {
        if #[cfg(unix)] {
            let current_user_id = Some(nix::unistd::getuid().as_raw());
        } else {
            let current_user_id = None;
        }
    };

    for process in processes {
        let pid = process.pid();
        if current_pid != pid {
            if kill_old {
                process.kill();
                let exe = process.exe().unwrap_or(Path::new("")).display();
                eprintln!("Killing instance: {exe} ({pid})");
            } else {
                let page = page.clone();
                let on_match = async {
                    let exe = process.exe().unwrap_or(Path::new("")).display();

                    let mut extra = vec![format!("pid={pid}")];

                    if let Some(user_id) = process.user_id() {
                        extra.push(format!("uid={}", **user_id));
                    }

                    if let Some(group_id) = process.group_id() {
                        extra.push(format!("gid={}", *group_id));
                    }

                    eprintln!("{PRODUCT_NAME} is already running: {exe} ({})", extra.join(" "),);
                    match &page {
                        Some(page) => {
                            eprintln!("Opening /{page}...");
                            Some(page)
                        },
                        None => {
                            eprintln!("Opening {PRODUCT_NAME} Window...");
                            None
                        },
                    };

                    if let Err(err) =
                        fig_ipc::local::open_ui_element(fig_proto::local::UiElement::MissionControl, page).await
                    {
                        eprintln!("Failed to open Fig: {err}");
                    }

                    #[allow(clippy::exit)]
                    exit(0);
                };

                match (process.user_id().map(|uid| uid as &u32), current_user_id.as_ref()) {
                    (Some(uid), Some(current_uid)) if uid == current_uid => {
                        on_match.await;
                    },
                    (_, None) => {
                        on_match.await;
                    },
                    _ => {},
                }
            }
        }
    }
}