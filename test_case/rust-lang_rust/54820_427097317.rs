plain
[00:16:29]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:16:31] error[E0308]: match arms have incompatible types
[00:16:31]    --> librustc_lint/unused.rs:381:53
[00:16:31]     |
[00:16:31] 381 |           let (value, msg, struct_lit_needs_parens) = match p.node {
[00:16:31]     |  _____________________________________________________^
[00:16:31] 382 | |             Ident(.., Some(ref pat)) => (pat, "optional subpattern", false),
[00:16:31] 383 | |             Ref(ref pat, _) => (pat, "reference pattern", false),
[00:16:31] 384 | |             Slice(_, Some(ref pat), _) => (pat, "optional position pattern", false),
[00:16:31] 385 | |             Paren(_) => (p, "pattern", false),
[00:16:31] 386 | |             _ => return,
[00:16:31] 387 | |         };
[00:16:31] 387 | |         };
[00:16:31]     | |_________^ expected struct `syntax::ptr::P`, found struct `syntax::ast::Pat`
[00:16:31]     |
[00:16:31]     = note: expected type `(&syntax::ptr::P<syntax::ast::Pat>, &str, bool)`
[00:16:31]                found type `(&syntax::ast::Pat, &'static str, bool)`
[00:16:31] error: aborting due to previous error
[00:16:31] 
[00:16:31] For more information about this error, try `rustc --explain E0308`.
[00:16:31] error: Could not compile `rustc_lint`.
[00:16:31] error: Could not compile `rustc_lint`.
[00:16:31] warning: build failed, waiting for other jobs to finish...
[00:17:11] error: build failed
[00:17:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:17:11] expected success, got: exit code: 101
[00:17:11] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:17:11] travis_fold:end:stage0-rustc

[00:17:11] travis_time:end:stage0-rustc:start=1538672263605734725,finish=1538672986283279588,duration=722677544863


[00:17:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:17:11] Build completed unsuccessfully in 0:12:57
[00:17:11] Makefile:28: recipe for target 'all' failed
[00:17:11] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1341f380
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
151412 ./src/tools/clang
149112 ./src/llvm-emscripten/test
144852 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
135996 ./obj/build/bootstrap/debug/incremental/bootstrap-3ivyub3ic2113
135992 ./obj/build/bootstrap/debug/incremental/bootstrap-3ivyub3ic2113/s-f5ew7hrzv0-1v9mkjx-1akhp35ixzhyj
107652 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
104700 ./src/tools/lldb
103088 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
103084 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
