rust
#![warn(unused_extern_crates)]

use time as time_crate;

pub mod m {
    pub use time_crate::Duration;  // Error, time_crate not found.
}
