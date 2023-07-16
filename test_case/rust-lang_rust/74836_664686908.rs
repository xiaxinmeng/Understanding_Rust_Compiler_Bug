rust
struct A;

impl Drop for A {
    fn drop(&mut self) {
        println!("Dropping A");
    }
}

fn foo() -> [A; 0] {
    let a = A;
    [a; 0]
}

fn main() {
    foo();
}
