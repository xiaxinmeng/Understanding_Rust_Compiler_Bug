rust
trait Handler { }

impl<F> Handler for F where F: Fn(&str) -> &str { }

fn is_handler<H: Handler>(_: H) {  }

fn dyn_handler(_: &str) -> &str { "hi" }

is_handler(dyn_handler);
