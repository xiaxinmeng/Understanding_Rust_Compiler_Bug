
pub type BoxError = Box<dyn Error + Send + Sync + 'static>;

    pub async fn run_profile(
        &self,
        app_config: &AppiConfig,
        app_args: ArgMatches<'static>,
    ) -> Result<(), BoxError> {
