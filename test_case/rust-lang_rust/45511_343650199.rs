
pub struct Bar;

impl Bar {
    #[inline]
    pub fn baz(i: u32) {
        Self::qux(i);
    }

    pub fn qux(i: u32) {
        println!("{}", i);
    }
}
