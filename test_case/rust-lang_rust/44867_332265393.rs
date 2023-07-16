
...

---- $DIR/test/error-index.md - Rust_Compiler_Error_Index::E0626 (line 11733) stdout ----
	error[E0514]: found crate `std` compiled by an incompatible version of rustc
  |
  = help: please recompile that crate using this compiler (rustc 1.22.0-dev)
  = note: crate `std` path #1: $DIR/stage0-sysroot/lib/rustlib/x86_64-apple-darwin/lib/libstd_unicode-0dc809f4018b643d.rlib compiled by "rustc 1.21.0-beta.1 (198109911 2017-08-29)"
  = note: crate `std` path #2: $DIR/stage0-sysroot/lib/rustlib/x86_64-apple-darwin/lib/libstd-202599072911ceaa.rlib compiled by "rustc 1.21.0-beta.1 (198109911 2017-08-29)"
  = note: crate `std` path #3: $DIR/stage0-sysroot/lib/rustlib/x86_64-apple-darwin/lib/libstd-202599072911ceaa.dylib compiled by "rustc 1.21.0-beta.1 (198109911 2017-08-29)"

error: aborting due to previous error

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:525:8
thread 'rustc' panicked at 'couldn't compile the test', src/librustdoc/test.rs:281:12


failures:
    $DIR/test/error-index.md - Rust_Compiler_Error_Index::E0001::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 15)
    $DIR/test/error-index.md - Rust_Compiler_Error_Index::E0002::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 42)
    $DIR/test/error-index.md - Rust_Compiler_Error_Index::E0003::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 86)
    $DIR/test/error-index.md - Rust_Compiler_Error_Index::E0004 (line 103)
    $DIR/test/error-index.md - Rust_Compiler_Error_Index::E0004 (line 123)
    $DIR/test/error-index.md - Rust_Compiler_Error_Index::E0005 (line 150)
    $DIR/test/error-index.md - Rust_Compiler_Error_Index::E0005 (line 160)

...
