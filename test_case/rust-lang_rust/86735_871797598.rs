
error[E0659]: `Default` is ambiguous (built-in attribute vs any other name)
  --> library/core/src/time.rs:63:61
   |
63 | #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
   |                                                             ^^^^^^^ ambiguous name
   |
   = note: `Default` could refer to a built-in attribute
note: `Default` could also refer to the derive macro imported here
  --> library/core/src/prelude/v1.rs:32:9
   |
32 | pub use crate::default::Default;
   |         ^^^^^^^^^^^^^^^^^^^^^^^
