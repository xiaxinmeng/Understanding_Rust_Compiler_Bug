rust
// crate bar
use foo::foo;

pub fn bar() {
    assert_eq!(foo(), 42);
}
