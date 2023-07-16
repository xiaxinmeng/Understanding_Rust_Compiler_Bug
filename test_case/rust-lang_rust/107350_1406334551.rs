rust
pub mod oops {
    pub use crate::oops::OhNo;

    mod inner {
        pub enum OhNo {
            Item = 1,
        }
    }

    pub use self::inner::*;
}
