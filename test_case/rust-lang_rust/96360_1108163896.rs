rust
#[async_trait(?Send)]
pub trait ListenerDriver {
    async fn accept(&mut self) -> Result<Connection, RecvError>;
}

#[async_trait]
impl ListenerDriver for Server {
    async fn accept(&mut self) -> Result<Connection, RecvError> {...}
}
