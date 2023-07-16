rust
macro_rules! a {
    () => {
        macro_rules! b {
            () => {}
        }
    }
}

a!();
b!();
