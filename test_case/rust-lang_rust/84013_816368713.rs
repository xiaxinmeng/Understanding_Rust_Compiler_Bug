rust
use std::fmt;

struct A;
struct B;

impl fmt::Debug for A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("X").finish()
    }
}

impl fmt::Debug for B {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("X")
    }
}

fn main() {
    assert_eq!(format!("<{:3?}>", A), "<X>");
    assert_eq!(format!("<{:3?}>", B), "<X  >");
}
