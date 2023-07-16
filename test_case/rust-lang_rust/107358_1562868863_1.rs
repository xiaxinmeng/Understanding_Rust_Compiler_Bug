rust
    #[tracing::instrument(skip_all)] // HERE
    pub async fn build_patch(
        &self,
        incoming_message: IncomingMessage,
    ) -> Result<ProcessResultItem> {
    ...
