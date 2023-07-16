rust
macro_rules! m {
    () => {
        cost X: &str = "hi";
        const Y: &str = "hello";
    }
}

m!();

fn main() {
    println!("{} {}", X, Y);
}
