rust
fn foo() {}

pub fn main() {
    while let Some(foo) = Some(1) {
        // break; // EDIT: the break is not neccessary
    }
    foo();
}
