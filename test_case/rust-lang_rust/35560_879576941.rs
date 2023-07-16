Rust
fn main() {
    macro_rules! my_macro {
        () => {
            println!("hello 1");
            println!("hello 2");
        }
    }

    my_macro!();
}
