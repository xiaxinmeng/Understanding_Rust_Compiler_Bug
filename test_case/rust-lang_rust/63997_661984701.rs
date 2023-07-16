rust
unsafe fn foo() { }
fn bar() { }

fn main() {
    let x = if true { foo } else { bar };
}
