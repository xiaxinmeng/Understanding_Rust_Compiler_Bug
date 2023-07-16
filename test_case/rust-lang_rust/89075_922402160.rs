
error: changes to closure capture in Rust 2021 will affect drop order
    --> src/librustdoc/doctest.rs:966:46
     |
966  |             testfn: test::DynTestFn(Box::new(move || {
     |                                              ^^^^^^^
...
975  |                     config.should_panic,
     |                     ------------------- in Rust 2018, this closure captures all of `config`, but in Rust 2021, it will only capture `config.should_panic`
976  |                     no_run,
977  |                     config.test_harness,
     |                     ------------------- in Rust 2018, this closure captures all of `config`, but in Rust 2021, it will only capture `config.test_harness`
...
981  |                     config.compile_fail,
     |                     ------------------- in Rust 2018, this closure captures all of `config`, but in Rust 2021, it will only capture `config.compile_fail`
982  |                     config.error_codes,
     |                     ------------------ in Rust 2018, this closure captures all of `config`, but in Rust 2021, it will only capture `config.error_codes`
...
1050 |     }
     |     -
     |     |
     |     in Rust 2018, `config` is dropped here, but in Rust 2021, only `config.should_panic` will be dropped here as part of the closure
     |     in Rust 2018, `config` is dropped here, but in Rust 2021, only `config.test_harness` will be dropped here as part of the closure
     |     in Rust 2018, `config` is dropped here, but in Rust 2021, only `config.compile_fail` will be dropped here as part of the closure
     |     in Rust 2018, `config` is dropped here, but in Rust 2021, only `config.error_codes` will be dropped here as part of the closure
     |
     = note: `-D rust-2021-incompatible-closure-captures` implied by `-D rust-2021-compatibility`
     = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `config` to be fully captured
     |
966  ~             testfn: test::DynTestFn(Box::new(move || {
967  +                 let _ = &config;
     |
