plain
[00:20:37]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:20:37]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:20:38]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:20:38]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:21:03] error: internal compiler error: librustc/middle/reachable.rs:324: found unexpected thingy in worklist: root_crate
[00:21:03] thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:586:9
[00:21:03] note: Run with `RUST_BACKTRACE=1` for a backtrace.
inux-gnu
285392 ./src/tools
---
151200 ./src/tools/clang
149116 ./src/llvm-emscripten/test
148696 ./obj/build/bootstrap/debug/incremental
134264 ./obj/build/bootstrap/debug/incremental/bootstrap-1fo02lb1azu6i
134260 ./obj/build/bootstrap/debug/incremental/bootstrap-1fo02lb1azu6i/s-f473byonm0-ymn7ph-heneaay3bnr5
103868 ./src/tools/lldb
98952 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
93756 ./src/tools/clang/test
90416 ./obj/build/x86_64-unknown-linux-gnu/stage1
