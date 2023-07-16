
$ cargo +nightly-2020-08-08 doc
 Documenting const-generics v0.1.0 (/home/joshua/test-rustdoc/const-generics)
error[E0658]: const generics are unstable
 --> src/lib.rs:4:26
  |
4 | pub type CellIndex<const D: usize> = [i64; D];
  |                          ^
  |
  = note: see issue #44580 <https://github.com/rust-lang/rust/issues/44580> for more information
  = help: add `#![feature(const_generics)]` to the crate attributes to enable

error: Compilation failed, aborting rustdoc

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
error: Could not document `const-generics`.

Caused by:
  process didn't exit successfully: `rustdoc --edition=2018 --crate-type lib --crate-name const_generics src/lib.rs -o /home/joshua/.local/lib/cargo/target/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/home/joshua/.local/lib/cargo/target/debug/deps --crate-version 0.1.0` (exit code: 1)
$ cargo +nightly-2020-08-09 doc
 Documenting const-generics v0.1.0 (/home/joshua/test-rustdoc/const-generics)
error: internal compiler error: src/librustc_middle/ty/subst.rs:569:17: const parameter `D/#0` (Const { ty: usize, val: Param(D/#0) }/0) out of range when substituting substs=[]

thread 'rustc' panicked at 'Box<Any>', /rustc/ceedf1d5febd65b012b8bcd513d70a0a6a091210/src/librustc_errors/lib.rs:870:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: Unrecognized option: 'crate-version'

error: aborting due to previous error

error: Could not document `const-generics`.

Caused by:
  process didn't exit successfully: `rustdoc --edition=2018 --crate-type lib --crate-name const_generics src/lib.rs -o /home/joshua/.local/lib/cargo/target/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/home/joshua/.local/lib/cargo/target/debug/deps --crate-version 0.1.0` (exit code: 1)
