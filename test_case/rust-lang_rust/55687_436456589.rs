plain
travis_time:end:0271dd2b:start=1541548622018031204,finish=1541548625113226008,duration=3095194804
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:07:26]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:13:21]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:13:21]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:14:44]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:14:44] error: incorrect close delimiter: `)`
[00:14:44]    --> librustc_typeck/astconv.rs:842:51
[00:14:44]     |
[00:14:44] 107 | impl<'o, 'gcx: 'tcx, 'tcx> dyn AstConv<'gcx, 'tcx>+'o {
[00:14:44]     |                                                       - un-closed delimiter
[00:14:44] ...
[00:14:44] 842 |         dup_bindings: &mut FxHashMap<DefId, Span>))
[00:14:44]     |                                                   ^ incorrect close delimiter
[00:14:44] error: unexpected close delimiter: `}`
[00:14:44]     --> librustc_typeck/astconv.rs:1717:1
[00:14:44]      |
[00:14:44] 1717 | }
[00:14:44] 1717 | }
[00:14:44]      | ^ unexpected close delimiter
[00:14:44] error: aborting due to 2 previous errors
[00:14:44] 
[00:14:44] error: Could not compile `rustc_typeck`.
[00:14:44] warning: build failed, waiting for other jobs to finish...
---
150256 ./obj/build/bootstrap/debug/incremental
149128 ./src/llvm-emscripten/test
136632 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
134668 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u
134664 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u/s-f6flqasdiz-a06xrd-22tmsi8iacpi9
107892 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
104700 ./src/tools/lldb
94684 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
94680 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
