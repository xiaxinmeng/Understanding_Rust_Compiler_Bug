plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
........................................................................................ 264/1506
........................................................................................ 352/1506
........................................................................................ 440/1506
........................................................................................ 528/1506
.............F..........................F..........................F.................... 616/1506
.......F...........................F.................................................... 704/1506
................................................i.i..................................... 880/1506
........................................................................................ 968/1506
........................................................................................ 1056/1506
........................................................................................ 1144/1506
........................................................................................ 1144/1506
........................................................................................ 1232/1506
........................................................................................ 1320/1506
........................................................................................ 1408/1506
........................................................................................ 1496/1506
..........
failures:

---- num::i128::tests::test_rem_euclid stdout ----
thread 'num::i128::tests::test_rem_euclid' panicked at 'attempt to negate with overflow', /checkout/library/core/src/num/mod.rs:261:5
error: test failed, to rerun pass `-p core --test coretests`
---- num::i16::tests::test_rem_euclid stdout ----
---- num::i16::tests::test_rem_euclid stdout ----
thread 'num::i16::tests::test_rem_euclid' panicked at 'attempt to negate with overflow', /checkout/library/core/src/num/mod.rs:243:5
---- num::i32::tests::test_rem_euclid stdout ----
---- num::i32::tests::test_rem_euclid stdout ----
thread 'num::i32::tests::test_rem_euclid' panicked at 'attempt to negate with overflow', /checkout/library/core/src/num/mod.rs:248:5
---- num::i64::tests::test_rem_euclid stdout ----
---- num::i64::tests::test_rem_euclid stdout ----
thread 'num::i64::tests::test_rem_euclid' panicked at 'attempt to negate with overflow', /checkout/library/core/src/num/mod.rs:254:5
---- num::i8::tests::test_rem_euclid stdout ----
---- num::i8::tests::test_rem_euclid stdout ----
thread 'num::i8::tests::test_rem_euclid' panicked at 'attempt to negate with overflow', /checkout/library/core/src/num/mod.rs:238:5

failures:
    num::i128::tests::test_rem_euclid
    num::i16::tests::test_rem_euclid
