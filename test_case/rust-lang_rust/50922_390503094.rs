plain
[00:03:06]    Compiling cc v1.0.15
[00:03:06]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:03:06]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:03:06]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:03:06] error: incorrect close delimiter: `)`
[00:03:06]    --> libcore/char/methods.rs:782:57
[00:03:06]     |
[00:03:06] 782 |         ToLowercase(conversions::Lowercase.lookup(self)))
[00:03:06]     |
[00:03:06] note: unclosed delimiter
[00:03:06]    --> libcore/char/methods.rs:781:46
[00:03:06]     |
[00:03:06]     |
[00:03:06] 781 |     pub fn to_lowercase(self) -> ToLowercase {
[00:03:06] 
[00:03:06] 
[00:03:06] error: incorrect close delimiter: `)`
[00:03:06]    --> libcore/char/methods.rs:868:57
[00:03:06]     |
[00:03:06] 868 |         ToUppercase(conversions::Uppercase.lookup(self)))
[00:03:06]     |
[00:03:06] note: unclosed delimiter
[00:03:06]    --> libcore/char/methods.rs:867:46
[00:03:06]     |
[00:03:06]     |
[00:03:06] 867 |     pub fn to_uppercase(self) -> ToUppercase {
[00:03:06] 
[00:03:06] 
[00:03:06] error: unexpected close delimiter: `}`
[00:03:06]    --> libcore/char/methods.rs:869:5
[00:03:06] 869 |     }
[00:03:06]     |     ^
[00:03:06] 
[00:03:06] error: aborting due to 3 previous errors
[00:03:06] error: aborting due to 3 previous errors
[00:03:06] 
[00:03:06] error: Could not compile `core`.
[00:03:06] 
[00:03:06] Caused by:
[00:03:06]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=0d1ebef792b1d9ca -C extra-filename=-0d1ebef792b1d9ca --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 101)
[00:03:06] warning: build failed, waiting for other jobs to finish...
[00:03:12] error: build failed
[00:03:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:12] expected success, got: exit code: 101
[00:03:12] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9

[00:03:12] travis_time:end:stage0-std:start=1526842376752063012,finish=1526842384083215718,duration=7331152706

[00:03:12] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:03:12] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:03:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:12] Build completed unsuccessfully in 0:00:08
[00:03:12] Makefile:79: recipe for target 'tidy' failed
[00:03:12] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08c4f195
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
