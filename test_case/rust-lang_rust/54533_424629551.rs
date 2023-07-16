plain
[00:14:46]    Compiling rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)
[00:14:48] error[E0308]: mismatched types
[00:14:48]     --> librustc_typeck/check/mod.rs:2074:9
[00:14:48]      |
[00:14:48] 2073 |       pub fn local_ty(&self, span: Span, nid: ast::NodeId) -> LocalTy<'tcx> {
[00:14:48]      |                                                               ------------- expected `check::LocalTy<'tcx>` because of return type
[00:14:48] 2074 | /         self.locals.borrow().get(&nid).unwrap_or_else(||
[00:14:48] 2075 | |             span_bug!(span, "no type for local variable {}",
[00:14:48] 2076 | |                       self.tcx.hir.node_to_string(nid))
[00:14:48] 2077 | |         )
[00:14:48]      | |_________^ expected struct `check::LocalTy`, found reference
[00:14:48]      = note: expected type `check::LocalTy<'tcx>`
[00:14:48]      = note: expected type `check::LocalTy<'tcx>`
[00:14:48]                 found type `&check::LocalTy<'_>`
[00:14:48] help: consider dereferencing the borrow
[00:14:48]      |
[00:14:48] 2074 |         *self.locals.borrow().get(&nid).unwrap_or_else(||
[00:14:48] 2075 |             span_bug!(span, "no type for local variable {}",
[00:14:48] 2076 |                       self.tcx.hir.node_to_string(nid))
[00:14:48]      |
[00:14:48] 
[00:14:53] error: aborting due to previous error
[00:14:53] 
[00:14:53] 
[00:14:53] For more information about this error, try `rustc --explain E0308`.
[00:14:53] error: Could not compile `rustc_typeck`.
[00:14:53] warning: build failed, waiting for other jobs to finish...
[00:17:49] error: build failed
[00:17:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:17:49] expected success, got: exit code: 101
[00:17:49] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:17:49] travis_fold:end:stage0-rustc

[00:17:49] travis_time:end:stage0-rustc:start=1537949553916863486,finish=1537950273091912507,duration=719175049021


[00:17:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:17:49] Build completed unsuccessfully in 0:13:02
[00:17:49] make: *** [all] Error 1
[00:17:49] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
u/stage0/lib/rustlib
326024 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu
---
151412 ./src/tools/clang
149112 ./src/llvm-emscripten/test
148656 ./obj/build/bootstrap/debug/incremental
134216 ./obj/build/bootstrap/debug/incremental/bootstrap-2zc4gzhr0d54q
134212 ./obj/build/bootstrap/debug/incremental/bootstrap-2zc4gzhr0d54q/s-f55o6x1mpj-cthxrq-abll1u66rzrn
111072 ./src/llvm/test/CodeGen
104700 ./src/tools/lldb
98940 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
93748 ./src/tools/clang/test
