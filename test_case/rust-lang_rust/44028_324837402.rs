
[01:13:30]      Running build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/build_auth-a9dcf589dab07a7e
[01:13:30] 
[01:13:30] running 3 tests
[01:13:30] test https_something_happens ... ok
[01:13:30] test ssh_something_happens ... ok
[01:13:30] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:13:30]   left: `{"Accept: */*", "GET /foo/bar/info/refs?service=git-upload-pack HTTP/1.1", "User-Agent: git/2.0 (libgit2 0.26.0)"}`,
[01:13:30]  right: `{"Accept: */*", "User-Agent: git/2.0 (libgit2 0.25.0)", "GET /foo/bar/info/refs?service=git-upload-pack HTTP/1.1"}`', /checkout/src/tools/cargo/tests/build-auth.rs:46:8
[01:13:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:13:30] test http_auth_offered ... FAILED
[01:13:30] 
[01:13:30] failures:
[01:13:30] 
[01:13:30] ---- http_auth_offered stdout ----
[01:13:30] 	running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build -v`
[01:13:30] running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build`
[01:13:30] thread 'http_auth_offered' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/src/libcore/option.rs:335:20
[01:13:30] 
[01:13:30] 
[01:13:30] failures:
[01:13:30]     http_auth_offered
[01:13:30] 
[01:13:30] test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:13:30] 
[01:13:30] error: test failed, to rerun pass '--test build-auth'
[01:13:30] 
[01:13:30] 
[01:13:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "-j" "4" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/cargo/Cargo.toml"
[01:13:30] expected success, got: exit code: 101
[01:13:30] 
[01:13:30] 
[01:13:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/cargotest src/tools/cargo src/tools/rls src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty
[01:13:30] Build completed unsuccessfully in 0:25:57
[01:13:30] Makefile:56: recipe for target 'check-aux' failed
[01:13:30] make: *** [check-aux] Error 1
