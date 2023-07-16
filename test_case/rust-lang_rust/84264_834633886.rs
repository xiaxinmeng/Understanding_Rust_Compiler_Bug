
Z:\native> @'
>> #![crate_type="rlib"]
>> #[link(name="upstream")] extern {}
>> '@ > lib.rs
Z:\native> @'
>> #[link(name="library")] extern {}
>> fn main() {}
>> '@ > main.rs
Z:\native> rustc +nightly lib.rs
Z:\native> rustc +nightly main.rs --extern lib=liblib.rlib -l library:msvcrt -l upstream:msvcrt --edition 2018
error: renaming of the library `upstream` was specified, however this crate contains no `#[link(...)]` attributes referencing this library.

error: aborting due to previous error
