rust
pub mod prelude {
    pub mod v1 {
        // Prelude
        pub use __core::prelude::v1::*;
        #[cfg(all(feature = "alloc", feature = "unstable"))]
        pub use __alloc::prelude::v1::*;
        #[cfg(all(feature = "alloc", not(feature = "unstable")))]
        pub use __alloc::{
            borrow::ToOwned,
            boxed::Box,
            // UNSTABLE: slice::SliceConcatExt,
            string::String,
            string::ToString,
            vec::Vec,
        };

        // Other imports
        #[cfg(feature = "alloc")]
        pub use __alloc::{format, vec};
        #[cfg(feature = "compat_macros")]
        pub use crate::{print, println, eprint, eprintln, dbg};
    }
}
