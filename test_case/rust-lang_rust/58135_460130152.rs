rust
% rustc -Vv
rustc 1.34.0-nightly (852701ad6 2019-02-01)
binary: rustc
commit-hash: 852701ad6df90f4e4cdb11d487373f026f38e5b3
commit-date: 2019-02-01
host: x86_64-unknown-linux-gnu
release: 1.34.0-nightly
LLVM version: 8.0
% rustc a.rs 
error: lifetime parameters must be declared prior to type parameters
 --> a.rs:1:19
  |
1 | pub fn crashes<T, 'a, 'b>(start: T, sep: &'a str, strings: &'b str) -> String
  |                   ^^  ^^
help: move the lifetime parameter prior to the first type parameter
  |
1 | pub fn crashes<'a, 'b, T>(start: T, sep: &'a str, strings: &'b str) -> String
  |                ^^^ ^^^ --

error[E0601]: `main` function not found in crate `a`
  |
  = note: consider adding a `main` function to `a.rs`

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0601`.
