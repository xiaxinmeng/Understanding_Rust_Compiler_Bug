 rust
macro_rules! def {
    ($v:ident, #[$d:meta] $t:ty, $e:expr) => {
        /// Returns an
        #[$d]
        pub fn $v() -> $t { $e }
    }
}

def!(a, #[doc="i32"] i32, 42);
