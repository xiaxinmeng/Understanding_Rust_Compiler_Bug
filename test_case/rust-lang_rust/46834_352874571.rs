rust
pub fn bar<F: Fn()>(_f: F) {}

pub fn foo() {
    let mut x = 0;
    bar(move || x = 1); // Removing `move` produces a different, unhelpful error message
}
