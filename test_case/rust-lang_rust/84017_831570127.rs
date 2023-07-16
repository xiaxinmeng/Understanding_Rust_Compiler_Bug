plain
.................................................................................................... 9400/11819
.................................................................................................... 9500/11819
.................................................................................................... 9600/11819
........................................i......i.................................................... 9700/11819
......................................................................................iiiiiii...iiii 9800/11819
.................................................................................................... 10000/11819
.................................................................................................... 10100/11819
.................................................................................................... 10200/11819
.................................................................................................... 10300/11819
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.139 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 1.88s

 finished in 1.948 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
    Finished release [optimized] target(s) in 19.22s
     Running unittests (build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/rustdoc-873a6d222f95b19d)

running 63 tests
..FF.F.........................................................

---- clean::utils::tests::int_format_octal stdout ----
---- clean::utils::tests::int_format_octal stdout ----
thread 'clean::utils::tests::int_format_octal' panicked at 'assertion failed: `(left == right)`
  left: `"-0b_101"`,
 right: `"-0b101"`', src/librustdoc/clean/utils/tests.rs:38:5
---- clean::utils::tests::int_format_binary stdout ----
---- clean::utils::tests::int_format_binary stdout ----
thread 'clean::utils::tests::int_format_binary' panicked at 'assertion failed: `(left == right)`
  left: `"-0_o12_345_671"`,
 right: `"-0o12_345_671"`', src/librustdoc/clean/utils/tests.rs:28:5
---- clean::utils::tests::int_format_hex stdout ----
---- clean::utils::tests::int_format_hex stdout ----
thread 'clean::utils::tests::int_format_hex' panicked at 'assertion failed: `(left == right)`
  left: `"0x_ab3"`,
 right: `"0xab3"`', src/librustdoc/clean/utils/tests.rs:15:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun pass '-p rustdoc --lib'

failures:
    clean::utils::tests::int_format_binary
    clean::utils::tests::int_format_hex
    clean::utils::tests::int_format_hex
    clean::utils::tests::int_format_octal

test result: FAILED. 60 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:19:35
