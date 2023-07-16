
     Running `build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo-miri.exe miri test --manifest-path D:\a\rust\rust\src/tools/miri/test-cargo-miri/Cargo.toml --target i686-pc-windows-msvc --tests --`
   Compiling byteorder v1.4.3
   Compiling exported_symbol_dep v0.1.0 (D:\a\rust\rust\src\tools\miri\test-cargo-miri\exported-symbol-dep)
   Compiling issue_rust_86261 v0.1.0 (D:\a\rust\rust\src\tools\miri\test-cargo-miri\issue-rust-86261)
   Compiling cargo-miri-test v0.1.0 (D:\a\rust\rust\src\tools\miri\test-cargo-miri)
   Compiling issue_1691 v0.1.0 (D:\a\rust\rust\src\tools\miri\test-cargo-miri\issue-1691)
   Compiling byteorder v0.5.3
   Compiling exported_symbol v0.1.0 (D:\a\rust\rust\src\tools\miri\test-cargo-miri\exported-symbol)
   Compiling issue_1567 v0.1.0 (D:\a\rust\rust\src\tools\miri\test-cargo-miri\issue-1567)
   Compiling cdylib v0.1.0 (D:\a\rust\rust\src\tools\miri\test-cargo-miri\cdylib)
   Compiling issue_1705 v0.1.0 (D:\a\rust\rust\src\tools\miri\test-cargo-miri\issue-1705)
    Finished test [unoptimized + debuginfo] target(s) in 1.30s
     Running unittests src\main.rs (build\x86_64-pc-windows-msvc\stage2-tools\miri\i686-pc-windows-msvc\debug\deps\cargo_miri_test-0f976196cd517b80.exe)

running 2 tests
test test::dev_dependency ... ok
test test::exported_symbol ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running tests\main.rs (build\x86_64-pc-windows-msvc\stage2-tools\miri\i686-pc-windows-msvc\debug\deps\main-991d0e7eee076530.exe)
imported main
     Running tests\test.rs (build\x86_64-pc-windows-msvc\stage2-tools\miri\i686-pc-windows-msvc\debug\deps\test-5a629d2bcf30ccfb.exe)

running 6 tests
test cargo_env ... ok
test deps ... ok
test do_panic - should panic ... ok
test does_not_work_on_miri ... ignored
test fail_index_check - should panic ... ok
test simple ... ok

test result: ok. 5 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out

[TIMING] test::Miri { stage: 2, host: x86_64-pc-windows-msvc, target: i686-pc-windows-msvc } -- 101.188
Build completed successfully in 0:01:46
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.12s
warning: the following packages contain code that will be rejected by a future version of Rust: ntapi v0.3.7
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 8`
thread 'main' panicked at '

couldn't find required command: "cc"

', sanity.rs:59:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Build completed unsuccessfully in 0:00:00
