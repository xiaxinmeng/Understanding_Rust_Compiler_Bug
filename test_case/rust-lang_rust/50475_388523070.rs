plain
[00:05:39]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:05:47]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:07:15]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:31]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:07:36] warning: `#[must_use]` on methods is experimental (see issue #43302)
[00:07:36]     --> librustc/ty/sty.rs:1233:5
[00:07:36] 1233 |     #[must_use]
[00:07:36]      |     ^^^^^^^^^^^
[00:07:36]      |
[00:07:36]      |
[00:07:36]      = help: add #![feature(fn_must_use)] to the crate attributes to enable
[00:07:36] 
[00:07:36] warning: `#[must_use]` on methods is experimental (see issue #43302)
[00:07:36]     --> librustc/ty/sty.rs:1246:5
[00:07:36] 1246 |     #[must_use]
[00:07:36]      |     ^^^^^^^^^^^
[00:07:36]      |
[00:07:36]      |
[00:07:36]      = help: add #![feature(fn_must_use)] to the crate attributes to enable
[00:13:25]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:13:25]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:16:23]    Compiling rustc_resolve v0.0.0 (file:///checkout/src/librustc_resolve)
[00:16:24]    Compiling rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)
---
[00:42:09] ....................................................................................................
[00:42:13] ....................................................................................................
[00:42:16] ....................................................................................................
[00:42:22] ....................................................................................................
[00:42:27] ......................F.............................................................................
[00:42:38] .....................................i..............................................................
[00:42:43] .............i......................................................................................
[00:42:48] ................................ii..................................................................
[00:42:55] ....................................................................................................
[00:42:55] ....................................................................................................
[00:43:02] ...............................i....................................................................
[00:43:02] 
[00:43:02] failures:
[00:43:02] 
[00:43:02] ---- [ui] ui/issue-26638.rs stdout ----
[00:43:02]  
[00:43:02] error: /checkout/src/test/ui/issue-26638.rs:14: expected error not found: missing lifetime specifier [E0106]
[00:43:02] 
[00:43:02] error: 0 unexpected errors found, 1 expected errors not found
[00:43:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc"4018296 .
2744076 ./obj
2744044 ./obj/build
1968988 ./obj/build/x86_64-unknown-linux-gnu
---
149772 ./.git/modules
149768 ./.git/modules/src
149112 ./src/llvm-emscripten/test
124332 ./obj/build/bootstrap/debug/incremental/bootstrap-182x3aewwy26b
124328 ./obj/build/bootstrap/debug/incremental/bootstrap-182x3aewwy26b/s-f0ybg62yrc-1wjl4li-15rj8twogke02
