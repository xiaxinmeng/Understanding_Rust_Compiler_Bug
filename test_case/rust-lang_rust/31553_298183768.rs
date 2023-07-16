rust
macro_rules! a {
    () => {
        macro_rules! b {
            () => {
                0
            };
        }
        b!()
    }
}

fn main() {
    a!();
}
