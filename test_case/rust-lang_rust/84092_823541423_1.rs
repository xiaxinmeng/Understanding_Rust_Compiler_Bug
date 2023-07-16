rust
fn do_something() -> ControlFlow<()> { ... }

fn main() {
    do_something()?;
    do_something()?;
    do_something()?;
}
