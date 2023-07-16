rust
struct A;

impl Drop for A {
    fn drop(&mut self) {
        println!("Dropping A");
    }
}

fn main() {
    let a = A;
    let _ = [a; 0];
}
