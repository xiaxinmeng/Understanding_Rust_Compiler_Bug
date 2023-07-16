rust
impl<'a, A: Authenticator + Sized> NodeTcpConfigBuilder<'a, A> {
   const DEFAULT_MAX_SIZE: u32 = 10;
   const DEFAULT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);
