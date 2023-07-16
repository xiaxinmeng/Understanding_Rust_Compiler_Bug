plain
travis_time:end:2731df7c:start=1549430160354441515,finish=1549430163473658323,duration=3119216808
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:58:33]     Checking rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:58:33]     Checking rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:58:33]     Checking rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:58:33]  Documenting std v0.0.0 (/checkout/src/libstd)
[00:58:37] error: internal compiler error: src/librustc/hir/def.rs:258: attempted .def_id() on invalid def: NonMacroAttr(Builtin)
[00:58:37] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:605:9
[00:58:37] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:58:38] 
[00:58:38] error: Unrecognized option: 'markdown-css'
[00:58:38] error: Unrecognized option: 'markdown-css'
[00:58:38] 
[00:58:38] error: Could not document `std`.
[00:58:38] 
[00:58:38] Caused by:
[00:58:38]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-name std src/libstd/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="backtrace"' --cfg 'feature="backtrace-sys"' --cfg 'feature="compiler_builtins"' --cfg 'feature="compiler_builtins_c"' --cfg 'feature="default"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' --markdown-css rust.css --markdown-no-toc --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-9e6c0311b71511c6.rmeta --extern backtrace_sys=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libbacktrace_sys-f13b165ed9b4dd57.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-d54fe968dea87029.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-84e5b9599b1b7754.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-a69fda92b07aedd5.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-349a3e5cce9f18ee.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-d7c48504ff1056b6.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-20814bca47e9a554.rmeta --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-8bc4c4bf1d4ec8f0.rmeta --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-0814373332ff833c.rmeta --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-655667bc8a1522d0.rmeta --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-5756fbda854daecf.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-5e31c590860d0940.rmeta` (exit code: 1)
[00:58:38] 
[00:58:38] 
[00:58:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--index-page" "/checkout/src/doc/index.md"
[00:58:38] 
[00:58:38] 
[00:58:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:58:38] Build completed unsuccessfully in 0:06:04
[00:58:38] Build completed unsuccessfully in 0:06:04
[00:58:38] Makefile:18: recipe for target 'all' failed
[00:58:38] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:32958cb5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb  6 06:14:52 UTC 2019
