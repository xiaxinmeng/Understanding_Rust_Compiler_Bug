plain
tests/pass\stacked-borrows\stack-printing.rs ... ok
tests/pass\stacked-borrows\zst-field-retagging-terminates.rs ... ok
tests/pass\weak_memory\extra_cpp.rs ... ok
tests/pass\stacked-borrows\stacked-borrows.rs ... ok
tests/pass\stacked-borrows\unknown-bottom-gc.rs ... ok
tests/pass\weak_memory\extra_cpp_unsafe.rs ... ok
tests/pass\shims\env\current_exe.rs ... ignored (in-test comment)
tests/pass\shims\env\home.rs ... ignored (in-test comment)
tests/pass\shims\env\args.rs ... ok
---
  Downloaded byteorder v0.5.3
  Downloaded byteorder v1.4.3
   Compiling byteorder v1.4.3
   Compiling proc-macro2 v1.0.44
   Compiling issue_1760 v0.1.0 (D:\a\rust\rust\src\tools\miri\test-cargo-miri\issue-1760)
   Compiling exported_symbol_dep v0.1.0 (D:\a\rust\rust\src\tools\miri\test-cargo-miri\exported-symbol-dep)
   Compiling cargo-miri-test v0.1.0 (D:\a\rust\rust\src\tools\miri\test-cargo-miri)
   Compiling serde_derive v1.0.145
   Compiling serde_derive v1.0.145
   Compiling issue_rust_86261 v0.1.0 (D:\a\rust\rust\src\tools\miri\test-cargo-miri\issue-rust-86261)
   Compiling exported_symbol v0.1.0 (D:\a\rust\rust\src\tools\miri\test-cargo-miri\exported-symbol)
   Compiling issue_1691 v0.1.0 (D:\a\rust\rust\src\tools\miri\test-cargo-miri\issue-1691)
   Compiling byteorder v0.5.3
   Compiling issue_1567 v0.1.0 (D:\a\rust\rust\src\tools\miri\test-cargo-miri\issue-1567)
   Compiling cdylib v0.1.0 (D:\a\rust\rust\src\tools\miri\test-cargo-miri\cdylib)
   Compiling issue_1705 v0.1.0 (D:\a\rust\rust\src\tools\miri\test-cargo-miri\issue-1705)
    Finished test [unoptimized + debuginfo] target(s) in 14.89s
    Finished test [unoptimized + debuginfo] target(s) in 14.89s
     Running unittests src\main.rs (build\x86_64-pc-windows-msvc\stage2-tools\miri\x86_64-pc-windows-msvc\debug\deps\cargo_miri_test-73fefbf7e9974aea.exe)
running 2 tests
test test::dev_dependency ... ok
test test::exported_symbol ... ok


test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running tests\main.rs (build\x86_64-pc-windows-msvc\stage2-tools\miri\x86_64-pc-windows-msvc\debug\deps\main-6fc89803aebbe269.exe)
imported main
     Running tests\test.rs (build\x86_64-pc-windows-msvc\stage2-tools\miri\x86_64-pc-windows-msvc\debug\deps\test-c554a5ce2e5aeebc.exe)
running 6 tests
test cargo_env ... ok
test deps ... ok
test do_panic - should panic ... ok
---
tests/pass\stacked-borrows\non_scalar_field_retagging.rs ... ok
tests/pass\stacked-borrows\zst-field-retagging-terminates.rs ... ok
tests/pass\weak_memory\extra_cpp.rs ... ok
tests/pass\stacked-borrows\stacked-borrows.rs ... ok
tests/pass\stacked-borrows\unknown-bottom-gc.rs ... ok
tests/pass\panic\catch_panic.rs ... ok
tests/pass\shims\env\current_exe.rs ... ignored (in-test comment)
tests/pass\shims\env\home.rs ... ignored (in-test comment)
tests/pass\backtrace\backtrace-global-alloc.rs ... ok
---
[RUSTC-TIMING] cargo_miri test:false 3.302
    Finished release [optimized] target(s) in 0.55s
     Running `build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo-miri.exe miri test --manifest-path D:\a\rust\rust\src/tools/miri/test-cargo-miri/Cargo.toml --target i686-pc-windows-msvc --tests --`
   Compiling byteorder v1.4.3
   Compiling exported_symbol_dep v0.1.0 (D:\a\rust\rust\src\tools\miri\test-cargo-miri\exported-symbol-dep)
   Compiling issue_rust_86261 v0.1.0 (D:\a\rust\rust\src\tools\miri\test-cargo-miri\issue-rust-86261)
   Compiling cargo-miri-test v0.1.0 (D:\a\rust\rust\src\tools\miri\test-cargo-miri)
   Compiling issue_1691 v0.1.0 (D:\a\rust\rust\src\tools\miri\test-cargo-miri\issue-1691)
   Compiling exported_symbol v0.1.0 (D:\a\rust\rust\src\tools\miri\test-cargo-miri\exported-symbol)
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
