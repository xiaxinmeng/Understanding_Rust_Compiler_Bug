rust
macro_rules! def_struct {
    () => {
        // The import here is important cannot inline scroll::SizeWith
        use scroll::SizeWith;
        #[derive(Copy, Clone, SizeWith)]
        pub struct Test {
            pub a: u32,
            pub b: u32,
        }
    };
}

pub struct Test {
    pub a: u64,
    pub b: u64,
}

macro_rules! def_test {
    () => {
        #[cfg(test)]
        mod tests {
            use super::*;
            #[test]
            fn size_of() {
                assert_eq!(::std::mem::size_of::<Test>(), 8);
            }
        }
    };
}

pub mod dyn32 {
    pub use super::*;

    def_struct!();
    def_test!();
}
