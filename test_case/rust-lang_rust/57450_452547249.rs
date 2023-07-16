plain
travis_time:end:39612e90:start=1546994803717246026,finish=1546994879909973627,duration=76192727601
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:05] 
[01:09:05] running 118 tests
[01:09:29] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:09:34] ......iii.i.....ii
[01:09:34] 
[01:09:34]  finished in 28.539
[01:09:34] travis_fold:end:test_debuginfo

---
[01:29:08] .................................................................................................... 400/991
[01:29:16] ..........................i.i.....................................iiii........ii.................... 500/991
[01:29:23] .................................................................................................... 600/991
[01:29:30] .................................................................................................... 700/991
[01:29:38] .............iiii............F...................................................................... 800/991
[01:29:56] .......................................iiii................................................
[01:29:56] failures:
[01:29:56] 
[01:29:56] ---- primitive_docs.rs - prim_slice (line 573) stdout ----
[01:29:56] ---- primitive_docs.rs - prim_slice (line 573) stdout ----
[01:29:56] error[E0596]: cannot borrow immutable local variable `x` as mutable
[01:29:56]   |
[01:29:56]   |
[01:29:56] 4 | let x = [1, 2, 3];
[01:29:56]   |     - help: make this binding mutable: `mut x`
[01:29:56] 5 | let x = &mut x[..]; // take a full slice of `x`
[01:29:56]   |              ^ cannot borrow mutably
[01:29:56] thread 'primitive_docs.rs - prim_slice (line 573)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:321:13
[01:29:56] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:29:56] 
[01:29:56] 
---
[01:29:56] 
[01:29:56] error: test failed, to rerun pass '--doc'
[01:29:56] 
[01:29:56] 
[01:29:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:29:56] 
[01:29:56] 
[01:29:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:29:56] Build completed unsuccessfully in 0:32:05
[01:29:56] Build completed unsuccessfully in 0:32:05
[01:29:56] make: *** [check] Error 1
[01:29:56] Makefile:48: recipe for target 'check' failed
3092524 ./obj
3077260 ./obj/build
2417924 ./obj/build/x86_64-unknown-linux-gnu
1135364 ./src
---
175216 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
160416 ./obj/build/bootstrap/debug/incremental
153276 ./src/tools/clang
144308 ./obj/build/bootstrap/debug/incremental/bootstrap-2zp0dzg596kc2
144304 ./obj/build/bootstrap/debug/incremental/bootstrap-2zp0dzg596kc2/s-f8d3oyo3n2-11vkl9s-2vbxibotdym97
138624 ./obj/build/x86_64-unknown-linux-gnu/stage0-rus_64-unknown-linux-gnu
61668 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release
61364 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
61036 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps
---
32632 ./obj/build/x86_64-unknown-linux-gnu/doc/src
32040 ./src/llvm/test/Transforms
30696 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build
28344 ./obj/build/x86_64-unknown-linux-gnu/doc/book
27052 ./obj/build/x86_64-unknown-linux-gnu/stackout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
