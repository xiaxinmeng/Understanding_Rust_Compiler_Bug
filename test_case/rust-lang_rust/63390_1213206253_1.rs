rust
fn diverge() -> ! {
    loop {
        break loop { };
    }
}
