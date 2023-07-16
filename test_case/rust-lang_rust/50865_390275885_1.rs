rust
pub fn run(config: Arc<Config>) -> Box<Future<Item = (), Error = io::Error> + Send>
