rust
use std::cell::Cell;

struct Foo {
    inner: Cell<u8>
}

impl Foo {
    fn sneaky_len(&self) -> u8 {
        self.inner.replace(0)
    }
}

const FOO: Foo = Foo { inner: Cell::new(25) };

fn main() {
    assert_eq!(FOO.sneaky_len(), 25);
    assert_eq!(FOO.sneaky_len(), 25);
    
    let foo = FOO;
    assert_eq!(foo.sneaky_len(), 25);
    assert_eq!(foo.sneaky_len(), 0);
}
