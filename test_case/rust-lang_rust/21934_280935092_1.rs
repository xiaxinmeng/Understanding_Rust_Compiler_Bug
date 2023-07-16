rust
extern crate foo;

use foo::foo;

fn main() {
    foo::<()>(); // Afoo is not implemented for `()`
}
