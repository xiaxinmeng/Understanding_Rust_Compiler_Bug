rust
use humantime as _;
fn main() {
    let run: Result<(), ()> = Ok(());
    run.context("foo");
}
