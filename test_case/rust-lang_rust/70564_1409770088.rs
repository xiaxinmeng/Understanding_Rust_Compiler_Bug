rust
// Present
let mut config = Config::default();
config.a = 123;
use(config);

// Future
use(Config { a: 123, ..Default::default());
