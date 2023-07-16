rust
fn may_error() -> Result<(), ()> { Err(()) }

fn consume_closure<T, F: Fn() -> T>(_: F) {}

fn f() {
    consume_closure(|| {
        may_error()?;
        Ok(())
    });
}
