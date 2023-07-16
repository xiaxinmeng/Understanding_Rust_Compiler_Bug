plain
[00:03:23]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:03:25] warning: method is never used: `map`
[00:03:25]   --> libcore/task.rs:37:5
[00:03:25]    |
[00:03:25] 37 | /     fn map<U, F>(self, f: F) -> Poll<U>
[00:03:25] 38 | |         where F: FnOnce(T) -> U
[00:03:25] 39 | |     {
[00:03:25] 40 | |         match self {
[00:03:25] 43 | |         }
[00:03:25] 44 | |     }
[00:03:25]    | |_____^
[00:03:25]    |
[00:03:25]    |
[00:03:25]    = note: #[warn(dead_code)] on by default
[00:03:25] 
[00:03:25] warning: method is never used: `is_ready`
[00:03:25]   --> libcore/task.rs:47:5
[00:03:25]    |
[00:03:25] 47 |     fn is_ready(&self) -> bool {
[00:03:25] 
[00:03:25] 
[00:03:25] warning: method is never used: `is_pending`
[00:03:25]   --> libcore/task.rs:55:5
[00:03:25]    |
[00:03:25] 55 |     fn is_pending(&self) -> bool {
[00:03:25] 
[00:03:25] 
[00:03:25] warning: method is never used: `map_ok`
[00:03:25]   --> libcore/task.rs:62:5
[00:03:25]    |
[00:03:25] 62 | /     fn map_ok<U, F>(self, f: F) -> Poll<Result<U, E>>
[00:03:25] 63 | |         where F: FnOnce(T) -> U
[00:03:25] 64 | |     {
[00:03:25] 65 | |         match self {
[00:03:25] 69 | |         }
[00:03:25] 70 | |     }
[00:03:25]    | |_____^
[00:03:25] 
[00:03:25] 
[00:03:25] warning: method is never used: `map_err`
[00:03:25]   --> libcore/task.rs:73:5
[00:03:25]    |
[00:03:25] 73 | /     fn map_err<U, F>(self, f: F) -> Poll<Result<T, U>>
[00:03:25] 74 | |         where F: FnOnce(E) -> U
[00:03:25] 75 | |     {
[00:03:25] 76 | |         match self {
[00:03:25] 80 | |         }
[00:03:25] 81 | |     }
[00:03:25]    | |_____^
[00:03:25] 
---

[00:04:47] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:48] tidy error: /checkout/src/libstd/panic.rs:330: trailing whitespace
[00:04:49] some tidy checks failed
[00:04:49] 
[00:04:49] 
[00:04:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:49] 
[00:04:49] 
[00:04:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:49] Build completed unsuccessfully in 0:01:43
[00:04:49] Build completed unsuccessfully in 0:01:43
[00:04:49] make: *** [tidy] Error 1
[00:04:49] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:062d1aa4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
