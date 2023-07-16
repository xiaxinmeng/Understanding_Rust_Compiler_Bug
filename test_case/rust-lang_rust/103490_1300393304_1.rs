rust
fn f(_: impl X<'a>(&'a str)) {}
use FnOnce as X;
