plain
[00:06:50]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:06:54]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:08:41]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:09:02]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:09:07] error: unused import: `GenericParamDefKind`
[00:09:07]   --> librustc/middle/reachable.rs:23:24
[00:09:07]    |
[00:09:07] 23 | use ty::{self, TyCtxt, GenericParamDefKind};
[00:09:07]    |
[00:09:07]    = note: `-D unused-imports` implied by `-D warnings`
[00:09:07] 
nux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-62d80197b9ec531e/out` (exit code: 101)
nux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-62d80197b9ec531e/out` (exit code: 101)
[00:09:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:09:41] expected success, got: exit code: 101
[00:09:41] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:09:41] travis_fold:end:stage0-rustc

[00:09:41] travis_time:end:stage0-rustc:start=1530221892755854520,finish=1530222126722609673,duration=233966755153


[00:09:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:09:41] Build completed unsuccessfully in 0:04:08
[00:09:41] make: *** [all] Error 1
[00:09:41] Makefile:28: recipe for target 'all' failed
399444 ./.git/objects
399404 ./.git/objects/pack
315012 ./src/llvm
249872 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib
---
153516 ./.git/modules/src
149132 ./src/llvm-emscripten/test
138748 ./obj/build/bootstrap/debug/incremental
124176 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj
124172 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj/s-f2f24xs85m-1vegpck-38adk87dv7782
92708 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
89804 ./src/llvm/test/CodeGen
72384 ./.git/modules/src/tools
70500 ./obj/build/x86_64-unknown-linux-gnu/native
