
[01:51:45] failures:
[01:51:45] 
[01:51:45] ---- [run-make] run-make/extern-fn-struct-passing-abi stdout ----
[01:51:45] 	
[01:51:45] error: make failed
[01:51:45] status: exit code: 2
[01:51:45] command: "make"
[01:51:45] stdout:
[01:51:45] ------------------------------------------
[01:51:45] cc -ffunction-sections -fdata-sections -fPIC -m32 -stdlib=libc++ -c -o /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/extern-fn-struct-passing-abi.stage2-i686-apple-darwin/libtest.o test.c
[01:51:45] ar crus /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/extern-fn-struct-passing-abi.stage2-i686-apple-darwin/libtest.a /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/extern-fn-struct-passing-abi.stage2-i686-apple-darwin/libtest.o
[01:51:45] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/extern-fn-struct-passing-abi.stage2-i686-apple-darwin:/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib:" '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustc' --out-dir /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/extern-fn-struct-passing-abi.stage2-i686-apple-darwin -L /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/extern-fn-struct-passing-abi.stage2-i686-apple-darwin  test.rs
[01:51:45] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/extern-fn-struct-passing-abi.stage2-i686-apple-darwin:/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib:" /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/extern-fn-struct-passing-abi.stage2-i686-apple-darwin/test || exit 1
[01:51:45] rm /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/extern-fn-struct-passing-abi.stage2-i686-apple-darwin/libtest.o
[01:51:45] 
[01:51:45] ------------------------------------------
[01:51:45] stderr:
[01:51:45] ------------------------------------------
[01:51:45] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:51:45]   left: `FloatOne { x: -1.673584938129909 }`,
[01:51:45]  right: `FloatOne { x: 7 }`', test.rs:142:8
[01:51:45] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:51:45] make[1]: *** [all] Error 1
[01:51:45] 
[01:51:45] ------------------------------------------
[01:51:45] 
[01:51:45] thread '[run-make] run-make/extern-fn-struct-passing-abi' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2435:8
[01:51:45] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:51:45] 
[01:51:45] 
[01:51:45] failures:
[01:51:45]     [run-make] run-make/extern-fn-struct-passing-abi
[01:51:45] 
[01:51:45] test result: [31mFAILED(B[m. 158 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
