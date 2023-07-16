rust
error[E0365]: `foo` is private, and cannot be re-exported
 --> inline.rs:6:9
  |
6 | pub use foo as bar;
  |         ^^^^^^^^^^ re-export of private `foo`
  |
  = note: consider declaring type or module `foo` with `pub`

error: extern crate `std` is private, and cannot be re-exported (error E0365), consider declaring with `pub`
 --> inline.rs:9:9
  |
9 | pub use std as _std;
  |         ^^^^^^^^^^^
  |
  = note: `#[deny(pub_use_of_private_extern_crate)]` on by default
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
  = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
