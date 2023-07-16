rust
macro_rules! m { () => () }

#[path] // error: malformed `path` attribute input
m!();

fn main() {}
