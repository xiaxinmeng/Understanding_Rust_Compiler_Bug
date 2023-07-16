plain
    Checking hashbrown v0.12.3
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking object v0.26.2
    Checking addr2line v0.16.0
error[E0428]: the name `PathLike` is defined multiple times
     |
     |
1125 | pub trait PathLike {
     | ------------------ previous definition of the trait `PathLike` here
...
3298 | pub trait PathLike {
     | ^^^^^^^^^^^^^^^^^^ `PathLike` redefined here
     |
     = note: `PathLike` must be defined only once in the type namespace of this module

error[E0252]: the name `PathLike` is defined multiple times
   |
   |
17 | use crate::path::{Path, PathBuf, PathLike};
   |                                  -------- previous import of the trait `PathLike` here
21 | use crate::path::PathLike;
21 | use crate::path::PathLike;
   |     ^^^^^^^^^^^^^^^^^^^^^ `PathLike` reimported here
   |
   = note: `PathLike` must be defined only once in the type namespace of this module
error: unused import: `crate::path::PathLike`
  --> library/std/src/fs.rs:21:5
   |
21 | use crate::path::PathLike;
21 | use crate::path::PathLike;
   |     ^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`

error[E0119]: conflicting implementations of trait `path::PathLike`
     |
     |
1130 | impl<P: AsRef<Path>> PathLike for P {
...
...
3303 | impl<P: AsRef<Path>> PathLike for P {

Some errors have detailed explanations: E0119, E0252, E0428.
For more information about an error, try `rustc --explain E0119`.
error: could not compile `std` due to 4 previous errors
