console
$ cargo check -p spinoso-time
    Checking spinoso-time v0.7.1 (/Users/lopopolo/dev/artichoke/artichoke/spinoso-time)
warning: unreachable `pub` item
  --> spinoso-time/src/time/tzrs/offset.rs:12:24
   |
12 | pub use super::error::{TimeError, TzOutOfRangeError, TzStringError};
   | ---                    ^^^^^^^^^
   | |
   | help: consider restricting its visibility: `pub(crate)`
   |
   = help: or consider exporting it for use by other crates
note: the lint level is defined here
  --> spinoso-time/src/lib.rs:13:9
   |
13 | #![warn(unreachable_pub)]
   |         ^^^^^^^^^^^^^^^

warning: unreachable `pub` item
  --> spinoso-time/src/time/tzrs/offset.rs:12:35
   |
12 | pub use super::error::{TimeError, TzOutOfRangeError, TzStringError};
   | ---                               ^^^^^^^^^^^^^^^^^
   | |
   | help: consider restricting its visibility: `pub(crate)`
   |
   = help: or consider exporting it for use by other crates

warning: unreachable `pub` item
  --> spinoso-time/src/time/tzrs/offset.rs:12:54
   |
12 | pub use super::error::{TimeError, TzOutOfRangeError, TzStringError};
   | ---                                                  ^^^^^^^^^^^^^
   | |
   | help: consider restricting its visibility: `pub(crate)`
   |
   = help: or consider exporting it for use by other crates

warning: `spinoso-time` (lib) generated 3 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.38s
