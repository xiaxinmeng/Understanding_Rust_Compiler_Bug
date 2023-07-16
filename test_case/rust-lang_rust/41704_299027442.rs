
> rustup update nightly
> rustup default nightly
info: using existing install for 'nightly-x86_64-apple-darwin'
info: default toolchain set to 'nightly-x86_64-apple-darwin'

  nightly-x86_64-apple-darwin unchanged - rustc 1.19.0-nightly (6a5fc9eec 2017-05-02)
> git clone https://github.com/nagisa/rust_libloading --depth 1
Cloning into 'rust_libloading'...
remote: Counting objects: 28, done.
remote: Compressing objects: 100% (24/24), done.
remote: Total 28 (delta 1), reused 11 (delta 1), pack-reused 0
Unpacking objects: 100% (28/28), done.
> cd rust_libloading/
> cargo test
    Updating registry `https://github.com/rust-lang/crates.io-index`
   Compiling lazy_static v0.2.8
   Compiling libloading v0.4.0 (file:///Users/imperio/rust/rust_libloading)
    Finished dev [unoptimized + debuginfo] target(s) in 2.13 secs
     Running target/debug/deps/functions-c411e914ee9872ed

running 8 tests
test wrong_name_fails ... ok
test interior_null_fails ... ok
test test_incompatible_type ... ok
test test_incompatible_type_named_fn ... ok
test test_id_struct ... ok
test test_id_u32 ... ok
test test_0_no_0 ... ok
test missing_symbol_fails ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/deps/libloading-6ad1c66ce6d2a21c

running 1 test
test os::unix::this ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/deps/markers-d67de58f175b0e1b

running 8 tests
test check_library_sync ... ok
test check_library_send ... ok
test check_symbol_send ... ok
test check_symbol_sync ... ok
test check_unix_library_send ... ok
test check_unix_library_sync ... ok
test check_unix_symbol_send ... ok
test check_unix_symbol_sync ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/deps/statics-2434806dff1504cd

running 2 tests
test test_static_u32 ... ok
test test_static_ptr ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/deps/windows-e7698f1af943e30a

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests libloading

running 8 tests
test src/os/mod.rs - os (line 11) ... ignored
test src/lib.rs -  (line 25) ... ok
test src/lib.rs - Library::get (line 136) ... ok
test src/lib.rs - Symbol<'lib, T>::from_raw (line 240) ... ok
test src/lib.rs - Library::new (line 102) ... ok
test src/lib.rs - Symbol<'lib, T>::into_raw (line 216) ... ok
test src/lib.rs - Library::get (line 143) ... ok
test src/lib.rs - Library::get (line 155) ... ok

test result: ok. 7 passed; 0 failed; 1 ignored; 0 measured
