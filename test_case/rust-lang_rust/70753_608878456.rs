rust
macro_rules! n {
    ($t:ty) => {
        std::mem::size_of::<$t>()
    }
}

type B = Foo<n!(u8)>;
