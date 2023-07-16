rust
trait Foo<const N: usize> {
    fn do_x(&self) -> [u8; N];
}

struct Bar;

impl Foo<42> for Bar {
    fn do_x(&self) -> [u8; 42] {
        [0u8; 42]
    }
}
