 rust
trait T {}
fn main() { let _: T = (|| loop {})(); }
