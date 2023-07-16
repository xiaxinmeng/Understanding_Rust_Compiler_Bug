
[01:09:13] ---- bad_git_dependency stdout ----
[01:09:13] 	running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build -v`
[01:09:13] thread 'bad_git_dependency' panicked at '
[01:09:13] Expected: execs
[01:09:13]     but: differences:
[01:09:13]  10 - |  [[..]] 'file:///' is not a valid local file URI|
[01:09:13]     + |  'file:///' is not a valid local file URI; class=Config (7)|
[01:09:13] 
[01:09:13] 
[01:09:13] other output:
[01:09:13] ``', /cargo/registry/src/github.com-1ecc6299db9ec823/hamcrest-0.1.1/src/core.rs:31:12
[01:09:13] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:09:13] 
[01:09:13] 
[01:09:13] failures:
[01:09:13]     bad_git_dependency
[01:09:13] 
[01:09:13] test result: FAILED. 36 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
