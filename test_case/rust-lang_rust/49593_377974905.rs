rust
use std::error::Error;
fn foo(x: !) -> Box<Error> {
    Box::new(x)
}
