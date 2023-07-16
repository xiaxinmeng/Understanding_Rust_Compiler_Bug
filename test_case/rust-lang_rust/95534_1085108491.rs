
// in the prelude

mod secret {
    pub trait CopyExt: Copy {
        fn copy(&self) -> Self { *self }
    }
    impl<T: Copy> CopyExt for T {}
}

pub use secret::{CopyExt as _};
