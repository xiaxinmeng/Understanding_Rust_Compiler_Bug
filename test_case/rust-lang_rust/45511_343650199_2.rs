
pub static I: u32 = 1;

pub fn foo(i: u32) -> u32 {
    bar::Bar::baz(i)
}

mod bar {
    pub struct Bar;

    impl Bar {
        #[inline]
        pub fn baz(i: u32) -> u32 {
            Self::qux(i)
        }

        pub fn qux(i: u32) -> u32 {
            i * i
        }
    }
}
