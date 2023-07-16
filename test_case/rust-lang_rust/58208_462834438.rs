plain
travis_time:end:0ee0b13e:start=1549986021201989431,finish=1549986024525614630,duration=3323625199
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:59:13]     Checking term v0.0.0 (/checkout/src/libterm)
[00:59:13]     Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:59:13]     Checking getopts v0.2.17
[00:59:17]  Documenting test v0.0.0 (/checkout/src/libtest)
[00:59:17] error: internal compiler error: src/librustc/hir/def.rs:259: attempted .def_id() on invalid def: NonMacroAttr(Builtin)
[00:59:17] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:596:9
[00:59:17] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:59:17] 
[00:59:17] error: Unrecognized option: 'crate-version'
[00:59:17] error: Unrecognized option: 'crate-version'
[00:59:17] 
[00:59:17] error: Could not document `test`.
[00:59:17] 
[00:59:17] Caused by:
[00:59:17]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-name test src/libtest/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/release/deps --extern getopts=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/libgetopts-2cd31f015e79e3a8.rmeta --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/libproc_macro-9b2503eb72968b11.rmeta --extern term=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/libterm-0bd89f567a02c34d.rmeta` (exit code: 1)
[00:59:17] 
[00:59:17] 
[00:59:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--no-deps" "-p" "test"
[00:59:17] 
[00:59:17] 
[00:59:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:59:17] Build completed unsuccessfully in 0:06:26
[00:59:17] Build completed unsuccessfully in 0:06:26
[00:59:17] Makefile:18: recipe for target 'all' failed
[00:59:17] make: *** [all] Error 1
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:08ad7515
---
travis_time:end:020de3f8:start=1549989594107516937,finish=1549989594122592532,duration=15075595
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:009622d1
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06974c0d
$ dmesg | grep -i kill
