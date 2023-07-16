plain
[00:21:34]    Compiling owning_ref v0.3.3
[00:21:34]    Compiling rustc-hash v1.0.1
[00:21:34]    Compiling num_cpus v1.8.0
[00:21:35]    Compiling rand v0.4.2
[00:21:36] thread 'main' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:21:36] 
[00:21:36] error: internal compiler error: unexpected panic
[00:21:36] 
[00:21:36] 
[00:21:36] note: the compiler unexpectedly panicked. this is a bug.
[00:21:36] 
[00:21:36] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:21:36] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[00:21:36] 
[00:21:36] 
[00:21:36] note: compiler flags: -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib
[00:21:36] 
[00:21:36] note: some of the compiler flags provided by cargo are hidden
[00:21:36] error: Could not compile `rustc_platform_intrinsics`.
[00:21:36] 
[00:21:36] Caused by:
[00:21:36] Caused by:
[00:21:36]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_platform_intrinsics librustc_platform_intrinsics/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=5b99d5cb81aaadf0 -C extra-filename=-5b99d5cb81aaadf0 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps` (exit code: 101)
2568972 .
1680632 ./obj
1680600 ./obj/build
1078852 ./obj/build/x86_64-unknown-linux-gnu
---
146452 ./.git/modules/src
137836 ./obj/build/bootstrap/debug/incremental
127044 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
123264 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj
123260 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj/s-f1qmw5j8u3-8rx311-t4oe0he9btpl
91160 ./obj/build/x86_64-unknown-linux-gnu/stage1
91136 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
89804 ./src/llvm/test/CodeGen
79540 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu
