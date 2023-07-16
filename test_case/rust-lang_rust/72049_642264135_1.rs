rust
// main.rs

#[link(name = "foo")]
extern {
    fn foo();
}

fn main() {
    unsafe { foo() }
}
