plain
travis_time:end:2c7089ac:start=1541719375895988623,finish=1541719378297689044,duration=2401700421
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:04:06] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:06] tidy error: /checkout/src/librustc_resolve/lib.rs:140: line longer than 100 chars
[00:04:07] tidy error: /checkout/src/librustc_mir/hair/pattern/mod.rs:44: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/librustdoc/core.rs:235: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/librustdoc/html/render.rs:1572: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/librustdoc/clean/mod.rs:1264: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/librustdoc/clean/mod.rs:2562: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/librustdoc/clean/mod.rs:2812: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/libsyntax/parse/parser.rs:5187: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/libsyntax/parse/parser.rs:5210: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/libsyntax/parse/parser.rs:5325: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/libsyntax/parse/parser.rs:5343: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/librustc/ty/context.rs:2459: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/librustc/ty/sty.rs:128: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/librustc/ty/subst.rs:656: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/librustc/ty/relate.rs:482: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/librustc/ty/relate.rs:614: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/librustc/infer/mod.rs:667: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/librustc/infer/mod.rs:990: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/librustc_typeck/check/mod.rs:5324: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/librustc_typeck/diagnostics.rs:1043: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/librustc_typeck/diagnostics.rs:1071: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/librustc_typeck/astconv.rs:1521: TODO is deprecated; use FIXME
[00:04:07] tidy error: /checkout/src/librustc_typeck/collect.rs:1311: TODO is deprecated; use FIXME
[00:04:07] tidy error: duplicate error code: 86
[00:04:07] tidy error: /checkout/src/librustc_typeck/diagnostics.rs:1060: E0086: r##"
[00:04:07] tidy error: /checkout/src/librustc_typeck/diagnostics.rs:4903: //  E0086,
[00:04:07] tidy error: duplicate error code: 668
[00:04:07] tidy error: /checkout/src/librustc_resolve/diagnostics.rs:1655: E0668: r##"
[00:04:07] tidy error: /checkout/src/librustc_codegen_llvm/diagnostics.rs:50: E0668: r##"
[00:04:07] tidy error: duplicate error code: 107
[00:04:07] tidy error: /checkout/src/librustc_typeck/diagnostics.rs:1043: // TODO(const_generics:docs): these should be unified with E0107 (i.e. these can just be
[00:04:07] tidy error: /checkout/src/librustc_typeck/diagnostics.rs:1299: E0107: r##"
[00:04:07] tidy error: duplicate error code: 85
[00:04:07] tidy error: /checkout/src/librustc_typeck/diagnostics.rs:1045: E0085: r##"
[00:04:07] tidy error: /checkout/src/librustc_typeck/diagnostics.rs:4902: //  E0085,
[00:04:07] Expected a gate test for the feature 'const_generics'.
[00:04:07] Hint: create a failing test file named 'feature-gate-const_generics.rs'
[00:04:07] tidy error: Found 1 features without a gate test.
[00:04:07]       in the 'ui' test suite, with its failures due to
[00:04:07]       missing usage of #![feature(const_generics)].
[00:04:07] Hint: If you already have such a test and don't want to rename it,
[00:04:07]       you can also add a // gate-test-const_generics line to the test file.
[00:04:08] some tidy checks failed
[00:04:08] 
[00:04:08] 
[00:04:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:08] 
[00:04:08] 
[00:04:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:08] Build completed unsuccessfully in 0:00:47
[00:04:08] Build completed unsuccessfully in 0:00:47
[00:04:08] Makefile:79: recipe for target 'tidy' failed
[00:04:08] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00ba2a48
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Nov  8 23:27:16 UTC 2018
---
travis_time:end:197b6188:start=1541719637102028585,finish=1541719637106512066,duration=4483481
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00a958b7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3383a995
travis_time:start:3383a995
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:046045ce
$ dmesg | grep -i kill
