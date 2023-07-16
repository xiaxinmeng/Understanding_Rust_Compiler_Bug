plain
Resolving deltas: 100% (611417/611417), completed with 4858 local objects.
---
[00:00:55] configure: rust.quiet-tests     := True
---
[00:05:25] error[E0425]: cannot find value `Await` in module `keywords`
[00:05:25]   --> libsyntax/edition.rs:62:66
[00:05:25]    |
[00:05:25] 62 |             if sym == keywords::Async.name() || sym == keywords::Await.name() {
[00:05:25]    |                                                                  ^^^^^ not found in `keywords`
[00:05:25]
-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f27fded04dd31022.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-7eebd272275b441b.rlib --extern rustc_cratesio_shim=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_cratesio_shim-ea44e45cdc9f4609.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-e01ce88b04783514.rlib` (exit code: 101)
[00:05:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:05:32] expected success, got: exit code: 101
[00:05:32] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1064:9
[00:05:32] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:05:32] travis_fold:end:stage0-rustc
[00:05:32] travis_time:end:stage0-rustc:start=1522858929934602757,finish=1522858983087959890,duration=53153357133
[00:05:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:05:32] Build completed unsuccessfully in 0:01:06
[00:05:32] make: *** [all] Error 1
[00:05:32] Makefile:28: recipe for target 'all' failed
---
113676 ./obj/build/bootstrap/debug/incremental/bootstrap-171j8p2wafl95/s-ezt3nltf6k-zosvln-gd1ykehqltr2
