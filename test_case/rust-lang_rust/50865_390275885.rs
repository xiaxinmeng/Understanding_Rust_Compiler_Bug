rust
pub fn run(config: Arc<Config>) -> impl Future<Item = (), Error = io::Error> + Send
