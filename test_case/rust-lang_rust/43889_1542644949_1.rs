rust
let http_fut: Box<dyn Future<Item=(),Error=()> + Send> = get().boxed();
