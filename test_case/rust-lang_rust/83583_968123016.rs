rust
use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(module)]
struct Foo;

/// 