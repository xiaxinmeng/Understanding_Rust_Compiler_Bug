 nocode
Building stage2 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.18 secs
Building stage2 tool error_index_generator (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.0 secs
Testing error-index stage2
thread 'main' panicked at 'entity not found', /buildslave/rust-buildbot/slave/auto-linux-64-opt-rustbuild/build/src/tools/error_index_generator/main.rs:209
note: Run with `RUST_BACKTRACE=1` for a backtrace.

command did not execute successfully: "/buildslave/rust-buildbot/slave/auto-linux-64-opt-rustbuild/build/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/error_index_generator" "markdown" "/buildslave/rust-buildbot/slave/auto-linux-64-opt-rustbuild/build/obj/build/x86_64-unknown-linux-gnu/test/error-index.md"
expected success, got: exit code: 101
