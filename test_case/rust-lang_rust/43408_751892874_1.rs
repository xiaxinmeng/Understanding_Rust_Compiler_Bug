
warning: cannot use constants which depend on generic parameters in types
 --> src\main.rs:8:30
  |
8 | pub fn to_byte_array<T>() -> [u8; sof::<T>()] {
  |                              ^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(const_evaluatable_unchecked)]` on by default
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!        
  = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>
