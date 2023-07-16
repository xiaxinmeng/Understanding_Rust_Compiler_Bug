rust
fn borked() {
    loop {
        struct NonCopy;
        let value: NonCopy;
        let used = value;
    }
}
