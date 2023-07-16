rust
macro_rules! m {
    () => {
        let_x_transparent!();
        let y = x;
    }
}

m!();
