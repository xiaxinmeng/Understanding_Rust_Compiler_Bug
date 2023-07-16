plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---
test client_init_duplicated_and_unknown_settings ... FAILED
test client_invalid_member_toml_manifest ... ok
test client_invalid_toml_manifest ... ok
test client_dependency_typo_and_fix ... FAILED
thread panicked while panicking. aborting.
[2021-04-21T21:38:59Z ERROR rls::server] Can't read message
thread '[2021-04-21T21:38:59Z ERROR rls::server] Can't read message
main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
error: test failed, to rerun pass '--test client'

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/client-c22ae4f720a9c514` (signal: 4, SIGILL: illegal instruction)

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rls/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--"
expected success, got: exit code: 101



Building stage2 tool rustfmt (x86_64-unknown-linux-gnu)
    Blocking waiting for file lock on package cache
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
thread '<unnamed>' panicked at 'child threads shouldn't panic: Any', src/tools/cargo/src/cargo/core/compiler/job_queue.rs:465:10
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'child threads shouldn't panic: Any', src/tools/cargo/src/cargo/core/compiler/job_queue.rs:465:10
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
    Blocking waiting for file lock on package cache
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
    Blocking waiting for file lock on package cache
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread 'note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread 'note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rsthread ':190progress-notifier:' panicked at '38called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }
', note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
   Compiling cc v1.0.60
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "PoisonError { inner: .. }"', src/tools/rls/rls/src/build/cargo.rs:427:63
thread '<unnamed>' panicked at 'child threads shouldn't panic: Any', src/tools/cargo/src/cargo/core/compiler/job_queue.rs:465:10
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'child threads shouldn't panic: Any', src/tools/cargo/src/cargo/core/compiler/job_queue.rs:465:10
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'child threads shouldn't panic: Any', src/tools/cargo/src/cargo/core/compiler/job_queue.rs:465:10
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'child threads shouldn't panic: Any', src/tools/cargo/src/cargo/core/compiler/job_queue.rs:465:10
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'child threads shouldn't panic: Any', src/tools/cargo/src/cargo/core/compiler/job_queue.rs:465:10
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'child threads shouldn't panic: Any', src/tools/cargo/src/cargo/core/compiler/job_queue.rs:465:10
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
   Compiling psm v0.1.11
   Compiling stacker v0.1.12
   Compiling openssl-sys v0.9.58
   Compiling libz-sys v1.1.2
---
Verifying status of edition-guide...
Verifying status of rls...
This PR updated 'src/tools/rls', verifying if status is 'test-pass'...

We detected that this PR updated 'rls', but its tests failed.

If you do intend to update 'rls', please check the error messages above and
commit another update.

If you do NOT intend to update 'rls', please ensure you did not accidentally
change the submodule at 'src/tools/rls'. You may ask your reviewer for the
proper steps.
{"rust-by-example":"test-pass","rls":"test-fail","cargo-miri":"test-fail","reference":"test-pass","nomicon":"test-pass","edition-guide":"test-pass","book":"test-pass","rustfmt":"test-pass","rustbook":"test-fail","embedded-book":"test-pass","miri":"test-pass"}failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 check-tools
