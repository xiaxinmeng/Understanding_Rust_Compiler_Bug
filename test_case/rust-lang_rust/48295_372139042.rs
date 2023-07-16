
[01:29:46] failures:
[01:29:46] 
[01:29:46] ---- [run-make] run-make/reproducible-build stdout ----
[01:29:46] 	
[01:29:46] error: make failed
[01:29:46] status: exit code: 2
[01:29:46] command: "make"
[01:29:46] stdout:
[01:29:46] ------------------------------------------
[01:29:46] rm -rf /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/reproducible-build.stage2-i686-apple-darwin && mkdir /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/reproducible-build.stage2-i686-apple-darwin
[01:29:46] rm -rf /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/reproducible-build.stage2-i686-apple-darwin && mkdir /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/reproducible-build.stage2-i686-apple-darwin
[01:29:46] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/reproducible-build.stage2-i686-apple-darwin:/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib:" '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustc' --out-dir /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/reproducible-build.stage2-i686-apple-darwin -L /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/reproducible-build.stage2-i686-apple-darwin  linker.rs -O
[01:29:46] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/reproducible-build.stage2-i686-apple-darwin:/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib:" '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustc' --out-dir /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/reproducible-build.stage2-i686-apple-darwin -L /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/reproducible-build.stage2-i686-apple-darwin  reproducible-build-aux.rs
[01:29:46] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/reproducible-build.stage2-i686-apple-darwin:/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib:" '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustc' --out-dir /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/reproducible-build.stage2-i686-apple-darwin -L /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/reproducible-build.stage2-i686-apple-darwin  reproducible-build.rs -C linker=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/reproducible-build.stage2-i686-apple-darwin/linker
[01:29:46] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/reproducible-build.stage2-i686-apple-darwin:/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib:" '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustc' --out-dir /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/reproducible-build.stage2-i686-apple-darwin -L /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/reproducible-build.stage2-i686-apple-darwin  reproducible-build.rs -C linker=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/reproducible-build.stage2-i686-apple-darwin/linker
[01:29:46] diff -u "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/reproducible-build.stage2-i686-apple-darwin/linker-arguments1" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/reproducible-build.stage2-i686-apple-darwin/linker-arguments2"
[01:29:46] 
[01:29:46] ------------------------------------------
[01:29:46] stderr:
[01:29:46] ------------------------------------------
[01:29:46] mkdir: /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make/reproducible-build.stage2-i686-apple-darwin: File exists
[01:29:46] make[1]: *** [debug] Error 1
[01:29:46] make[1]: *** Waiting for unfinished jobs....
[01:29:46] 
[01:29:46] ------------------------------------------
[01:29:46] 
[01:29:46] thread '[run-make] run-make/reproducible-build' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2893:9
[01:29:46] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:29:46] 
[01:29:46] 
[01:29:46] failures:
[01:29:46]     [run-make] run-make/reproducible-build
[01:29:46] 
[01:29:46] test result: [31mFAILED(B[m. 176 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
