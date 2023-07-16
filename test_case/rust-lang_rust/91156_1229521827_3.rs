
   Compiling my_crate v0.1.0 (…path…)
warning: the feature `adt_const_params` is incomplete and may not be safe to use and/or cause compiler crashes
 --> src/lib.rs:1:12
  |
1 | #![feature(adt_const_params)]
  |            ^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #44580 <https://github.com/rust-lang/rust/issues/44580> for more information

warning: `my_crate` (lib) generated 1 warning
    Finished test [unoptimized + debuginfo] target(s) in 0.15s
   Doc-tests my_crate
warning: the feature `adt_const_params` is incomplete and may not be safe to use and/or cause compiler crashes
 --> …path…/src/lib.rs:1:12
  |
1 | #![feature(adt_const_params)]
  |            ^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #44580 <https://github.com/rust-lang/rust/issues/44580> for more information

warning: 1 warning emitted


running 1 test
test src/lib.rs - Documented (line 23) ... FAILED

failures:

---- src/lib.rs - Documented (line 23) stdout ----
thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 0', /rustc/936f2600b6c903b04387f74ed5cbce88bb06d243/compiler/rustc_middle/src/mir/interpret/mod.rs:282:35
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (936f2600b 2021-11-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C embed-bitcode=no --crate-type bin

query stack during panic:
#0 [exported_symbols] exported_symbols
#1 [upstream_monomorphizations] collecting available upstream monomorphizations
end of query stack
Couldn't compile the test.

failures:
    src/lib.rs - Documented (line 23)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass '--doc'
