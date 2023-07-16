
[01:08:49] failures:
[01:08:49] 
[01:08:49] ---- [ui] ui/enum-size-variance.rs stdout ----
[01:08:49] 	normalized stderr:
[01:08:49] warning: enum variant is more than three times larger (32 bytes) than the next largest
[01:08:49]   --> $DIR/enum-size-variance.rs:28:5
[01:08:49]    |
[01:08:49] 28 |     L(i64, i64, i64, i64), //~ WARNING three times larger
[01:08:49]    |     ^^^^^^^^^^^^^^^^^^^^^
[01:08:49]    |
[01:08:49] note: lint level defined here
[01:08:49]   --> $DIR/enum-size-variance.rs:13:9
[01:08:49]    |
[01:08:49] 13 | #![warn(variant_size_differences)]
[01:08:49]    |         ^^^^^^^^^^^^^^^^^^^^^^^^
[01:08:49] 
[01:08:49] warning: looks like the linker segfaulted when we tried to call it, automatically retrying again
[01:08:49]   |
[01:08:49]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui/enum-size-variance.enum_size_variance0.rust-cgu.o" "-o" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui/enum-size-variance.stage2-x86_64-apple-darwin" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui/enum-size-variance.crate.allocator.rust-cgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui/enum-size-variance.stage2-x86_64-apple-darwin.ui.libaux" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-l" "std-0e08cf54369b7bb9" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-959a484cb4105bfc.rlib" "-l" "System" "-l" "resolv" "-l" "pthread" "-l" "c" "-l" "m" "-Wl,-rpath,@loader_path/../../stage2/lib/rustlib/x86_64-apple-darwin/lib" "-Wl,-rpath,/Users/travis/build/rust-lang/rust/lib/rustlib/x86_64-apple-darwin/lib"
[01:08:49]   = note: ld: warning: directory not found for option '-L/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui/enum-size-variance.stage2-x86_64-apple-darwin.ui.libaux'
[01:08:49]           clang: error: unable to execute command: Segmentation fault: 11
[01:08:49]           clang: error: linker command failed due to signal (use -v to see invocation)
[01:08:49]           
[01:08:49] 
[01:08:49] 
[01:08:49] 
[01:08:49] expected stderr:
[01:08:49] warning: enum variant is more than three times larger (32 bytes) than the next largest
[01:08:49]   --> $DIR/enum-size-variance.rs:28:5
[01:08:49]    |
[01:08:49] 28 |     L(i64, i64, i64, i64), //~ WARNING three times larger
[01:08:49]    |     ^^^^^^^^^^^^^^^^^^^^^
[01:08:49]    |
[01:08:49] note: lint level defined here
[01:08:49]   --> $DIR/enum-size-variance.rs:13:9
[01:08:49]    |
[01:08:49] 13 | #![warn(variant_size_differences)]
[01:08:49]    |         ^^^^^^^^^^^^^^^^^^^^^^^^
[01:08:49] 
[01:08:49] 
[01:08:49] 
[01:08:49] diff of stderr:
[01:08:49] 
[01:08:49]  warning: enum variant is more than three times larger (32 bytes) than the next largest
[01:08:49]    --> $DIR/enum-size-variance.rs:28:5
[01:08:49]     |
[01:08:49]  28 |     L(i64, i64, i64, i64), //~ WARNING three times larger
[01:08:49]     |     ^^^^^^^^^^^^^^^^^^^^^
[01:08:49]     |
[01:08:49]  note: lint level defined here
[01:08:49]    --> $DIR/enum-size-variance.rs:13:9
[01:08:49]     |
[01:08:49]  13 | #![warn(variant_size_differences)]
[01:08:49]     |         ^^^^^^^^^^^^^^^^^^^^^^^^
[01:08:49]  
[01:08:49] +warning: looks like the linker segfaulted when we tried to call it, automatically retrying again
[01:08:49] +  |
[01:08:49] +  = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui/enum-size-variance.enum_size_variance0.rust-cgu.o" "-o" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui/enum-size-variance.stage2-x86_64-apple-darwin" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui/enum-size-variance.crate.allocator.rust-cgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui/enum-size-variance.stage2-x86_64-apple-darwin.ui.libaux" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-l" "std-0e08cf54369b7bb9" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-959a484cb4105bfc.rlib" "-l" "System" "-l" "resolv" "-l" "pthread" "-l" "c" "-l" "m" "-Wl,-rpath,@loader_path/../../stage2/lib/rustlib/x86_64-apple-darwin/lib" "-Wl,-rpath,/Users/travis/build/rust-lang/rust/lib/rustlib/x86_64-apple-darwin/lib"
[01:08:49] +  = note: ld: warning: directory not found for option '-L/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui/enum-size-variance.stage2-x86_64-apple-darwin.ui.libaux'
[01:08:49] +          clang: error: unable to execute command: Segmentation fault: 11
[01:08:49] +          clang: error: linker command failed due to signal (use -v to see invocation)
[01:08:49] +          
[01:08:49] +
[01:08:49] 
[01:08:49] The actual stderr differed from the expected stderr.
[01:08:49] Actual stderr saved to /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui/enum-size-variance.stderr
[01:08:49] To update references, run this command from build directory:
[01:08:49] /Users/travis/build/rust-lang/rust/src/test/ui/update-references.sh '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui' 'enum-size-variance.rs'
[01:08:49] 
[01:08:49] error: 1 errors occurred comparing output.
[01:08:49] status: exit code: 0
[01:08:49] command: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/travis/build/rust-lang/rust/src/test/ui/enum-size-variance.rs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui" "--target=x86_64-apple-darwin" "-C" "prefer-dynamic" "-o" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui/enum-size-variance.stage2-x86_64-apple-darwin" "-Crpath" "-O" "-Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui/enum-size-variance.stage2-x86_64-apple-darwin.ui.libaux" "-A" "unused"
[01:08:49] stdout:
[01:08:49] ------------------------------------------
[01:08:49] 
[01:08:49] ------------------------------------------
[01:08:49] stderr:
[01:08:49] ------------------------------------------
[01:08:49] warning: enum variant is more than three times larger (32 bytes) than the next largest
[01:08:49]   --> /Users/travis/build/rust-lang/rust/src/test/ui/enum-size-variance.rs:28:5
[01:08:49]    |
[01:08:49] 28 |     L(i64, i64, i64, i64), //~ WARNING three times larger
[01:08:49]    |     ^^^^^^^^^^^^^^^^^^^^^
[01:08:49]    |
[01:08:49] note: lint level defined here
[01:08:49]   --> /Users/travis/build/rust-lang/rust/src/test/ui/enum-size-variance.rs:13:9
[01:08:49]    |
[01:08:49] 13 | #![warn(variant_size_differences)]
[01:08:49]    |         ^^^^^^^^^^^^^^^^^^^^^^^^
[01:08:49] 
[01:08:49] warning: looks like the linker segfaulted when we tried to call it, automatically retrying again
[01:08:49]   |
[01:08:49]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui/enum-size-variance.enum_size_variance0.rust-cgu.o" "-o" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui/enum-size-variance.stage2-x86_64-apple-darwin" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui/enum-size-variance.crate.allocator.rust-cgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui/enum-size-variance.stage2-x86_64-apple-darwin.ui.libaux" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-l" "std-0e08cf54369b7bb9" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-959a484cb4105bfc.rlib" "-l" "System" "-l" "resolv" "-l" "pthread" "-l" "c" "-l" "m" "-Wl,-rpath,@loader_path/../../stage2/lib/rustlib/x86_64-apple-darwin/lib" "-Wl,-rpath,/Users/travis/build/rust-lang/rust/lib/rustlib/x86_64-apple-darwin/lib"
[01:08:49]   = note: ld: warning: directory not found for option '-L/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui/enum-size-variance.stage2-x86_64-apple-darwin.ui.libaux'
[01:08:49]           clang: error: unable to execute command: Segmentation fault: 11
[01:08:49]           clang: error: linker command failed due to signal (use -v to see invocation)
[01:08:49]           
[01:08:49] 
[01:08:49] 
[01:08:49] ------------------------------------------
[01:08:49] 
[01:08:49] thread '[ui] ui/enum-size-variance.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2485:8
[01:08:49] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:08:49] 
[01:08:49] 
[01:08:49] failures:
[01:08:49]     [ui] ui/enum-size-variance.rs
[01:08:49] 
[01:08:49] test result: FAILED. 434 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out
