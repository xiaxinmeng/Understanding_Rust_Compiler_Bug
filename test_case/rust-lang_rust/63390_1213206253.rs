rust
fn diverge() -> ! {
    loop {
        break 3;
    }
}
