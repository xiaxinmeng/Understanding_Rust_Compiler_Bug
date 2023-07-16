
[01:32:42] failures:
[01:32:42] 
[01:32:42] ---- [run-make] run-make/pgo-gen stdout ----
[01:32:42]  
[01:32:42] error: make failed
[01:32:42] status: exit code: 2
[01:32:42] command: "make"
[01:32:42] stdout:
[01:32:42] ------------------------------------------
[01:32:42] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/pgo-gen.stage2-x86_64-apple-darwin:/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/pgo-gen.stage2-x86_64-apple-darwin -L /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/pgo-gen.stage2-x86_64-apple-darwin  -g -Z pgo-gen=test.profraw test.rs
[01:32:42] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/pgo-gen.stage2-x86_64-apple-darwin:/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib:" /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/pgo-gen.stage2-x86_64-apple-darwin/test || exit 1
[01:32:42] [ -e "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/pgo-gen.stage2-x86_64-apple-darwin/test.profraw" ] || (echo "No .profraw file"; exit 1)
[01:32:42] No .profraw file
[01:32:42] 
[01:32:42] ------------------------------------------
[01:32:42] stderr:
[01:32:42] ------------------------------------------
[01:32:42] make[1]: *** [all] Error 1
[01:32:42] 
[01:32:42] ------------------------------------------
[01:32:42] 
[01:32:42] thread '[run-make] run-make/pgo-gen' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2903:9
[01:32:42] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:32:42] 
[01:32:42] ---- [run-make] run-make/pgo-gen-lto stdout ----
[01:32:42]  
[01:32:42] error: make failed
[01:32:42] status: exit code: 2
[01:32:42] command: "make"
[01:32:42] stdout:
[01:32:42] ------------------------------------------
[01:32:42] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/pgo-gen-lto.stage2-x86_64-apple-darwin:/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/pgo-gen-lto.stage2-x86_64-apple-darwin -L /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make/pgo-gen-lto.stage2-x86_64-apple-darwin  -Copt-level=3 -Clto=fat -Z pgo-gen=test.profraw test.rs
[01:32:42] 
[01:32:42] ------------------------------------------
[01:32:42] stderr:
[01:32:42] ------------------------------------------
[01:32:42] error: couldn't read "test.rs": No such file or directory (os error 2)
[01:32:42] 
[01:32:42] make[1]: *** [all] Error 101
[01:32:42] 
[01:32:42] ------------------------------------------
[01:32:42] 
[01:32:42] thread '[run-make] run-make/pgo-gen-lto' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2903:9
[01:32:42] 
[01:32:42] 
[01:32:42] failures:
[01:32:42]     [run-make] run-make/pgo-gen
[01:32:42]     [run-make] run-make/pgo-gen-lto
[01:32:42] 
[01:32:42] test result: [31mFAILED(B[m. 177 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:32:42] 
[01:32:42] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:478:22
[01:32:42] 
[01:32:42] 
