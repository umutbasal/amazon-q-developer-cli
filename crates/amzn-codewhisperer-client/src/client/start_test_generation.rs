// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the
    /// [`StartTestGeneration`](crate::operation::start_test_generation::builders::StartTestGenerationFluentBuilder)
    /// operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`upload_id(impl Into<String>)`](crate::operation::start_test_generation::builders::StartTestGenerationFluentBuilder::upload_id) / [`set_upload_id(Option<String>)`](crate::operation::start_test_generation::builders::StartTestGenerationFluentBuilder::set_upload_id):<br>required: **true**<br>Upload ID returned by CreateUploadUrl API<br>
    ///   - [`target_code_list(TargetCode)`](crate::operation::start_test_generation::builders::StartTestGenerationFluentBuilder::target_code_list) / [`set_target_code_list(Option<Vec::<TargetCode>>)`](crate::operation::start_test_generation::builders::StartTestGenerationFluentBuilder::set_target_code_list):<br>required: **true**<br>(undocumented)<br>
    ///   - [`user_input(impl Into<String>)`](crate::operation::start_test_generation::builders::StartTestGenerationFluentBuilder::user_input) / [`set_user_input(Option<String>)`](crate::operation::start_test_generation::builders::StartTestGenerationFluentBuilder::set_user_input):<br>required: **true**<br>The content of user input.<br>
    ///   - [`test_generation_job_group_name(impl Into<String>)`](crate::operation::start_test_generation::builders::StartTestGenerationFluentBuilder::test_generation_job_group_name) / [`set_test_generation_job_group_name(Option<String>)`](crate::operation::start_test_generation::builders::StartTestGenerationFluentBuilder::set_test_generation_job_group_name):<br>required: **false**<br>Test generation job group name<br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::start_test_generation::builders::StartTestGenerationFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::start_test_generation::builders::StartTestGenerationFluentBuilder::set_client_token):<br>required: **false**<br>(undocumented)<br>
    /// - On success, responds with
    ///   [`StartTestGenerationOutput`](crate::operation::start_test_generation::StartTestGenerationOutput)
    ///   with field(s):
    ///   - [`test_generation_job(Option<TestGenerationJob>)`](crate::operation::start_test_generation::StartTestGenerationOutput::test_generation_job): Represents a test generation job
    /// - On failure, responds with
    ///   [`SdkError<StartTestGenerationError>`](crate::operation::start_test_generation::StartTestGenerationError)
    pub fn start_test_generation(
        &self,
    ) -> crate::operation::start_test_generation::builders::StartTestGenerationFluentBuilder {
        crate::operation::start_test_generation::builders::StartTestGenerationFluentBuilder::new(self.handle.clone())
    }
}