 rust
fn test(_x: &mut String) {}
fn test2(_x: &mut i32) {}

fn main() {
    let x: usize = String::new();
    let x: &str = String::new();
    let y = String::new();
    test(&y);
    test2(&y);
}
