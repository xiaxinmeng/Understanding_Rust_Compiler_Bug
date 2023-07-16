plain

[00:04:28] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:28] tidy error: /checkout/src/librustc_resolve/lib.rs:138: line longer than 100 chars
[00:04:28] tidy error: /checkout/src/librustdoc/core.rs:225: TODO is deprecated; use FIXME
[00:04:28] tidy error: /checkout/src/librustdoc/html/render.rs:1470: TODO is deprecated; use FIXME
[00:04:28] tidy error: /checkout/src/librustdoc/clean/mod.rs:1258: TODO is deprecated; use FIXME
[00:04:28] tidy error: /checkout/src/librustdoc/clean/mod.rs:2547: TODO is deprecated; use FIXME
[00:04:28] tidy error: /checkout/src/librustdoc/clean/mod.rs:2798: TODO is deprecated; use FIXME
[00:04:28] tidy error: /checkout/src/librustdoc/clean/mod.rs:3759: TODO is deprecated; use FIXME
[00:04:28] tidy error: /checkout/src/libsyntax/parse/parser.rs:5047: TODO is deprecated; use FIXME
[00:04:28] tidy error: /checkout/src/libsyntax/parse/parser.rs:5187: TODO is deprecated; use FIXME
[00:04:28] tidy error: /checkout/src/librustc/ty/context.rs:2279: TODO is deprecated; use FIXME
[00:04:28] tidy error: /checkout/src/librustc/ty/sty.rs:117: TODO is deprecated; use FIXME
[00:04:28] tidy error: /checkout/src/librustc/ty/subst.rs:508: TODO is deprecated; use FIXME
[00:04:28] tidy error: /checkout/src/librustc/ty/relate.rs:481: TODO is deprecated; use FIXME
[00:04:28] tidy error: /checkout/src/librustc/ty/relate.rs:614: TODO is deprecated; use FIXME
[00:04:28] tidy error: /checkout/src/librustc/ty/relate.rs:620: TODO is deprecated; use FIXME
[00:04:28] tidy error: /checkout/src/librustc/infer/mod.rs:592: TODO is deprecated; use FIXME
[00:04:28] tidy error: /checkout/src/librustc/infer/mod.rs:917: TODO is deprecated; use FIXME
[00:04:28] tidy error: /checkout/src/librustc_typeck/check/mod.rs:5190: TODO is deprecated; use FIXME
[00:04:28] tidy error: /checkout/src/librustc_typeck/diagnostics.rs:1043: TODO is deprecated; use FIXME
[00:04:28] tidy error: /checkout/src/librustc_typeck/diagnostics.rs:1071: TODO is deprecated; use FIXME
[00:04:28] tidy error: /checkout/src/librustc_typeck/astconv.rs:1534: TODO is deprecated; use FIXME
[00:04:28] tidy error: /checkout/src/librustc_typeck/collect.rs:1191: TODO is deprecated; use FIXME
[00:04:28] tidy error: duplicate error code: 85
[00:04:28] tidy error: /checkout/src/librustc_typeck/diagnostics.rs:1045: E0085: r##"
[00:04:28] tidy error: /checkout/src/librustc_typeck/diagnostics.rs:4798: //  E0085,
[00:04:28] tidy error: duplicate error code: 107
[00:04:28] tidy error: /checkout/src/librustc_typeck/diagnostics.rs:1043: // TODO(const_generics:docs): these should be unified with E0107 (i.e. these can just be
[00:04:28] tidy error: /checkout/src/librustc_typeck/diagnostics.rs:1291: E0107: r##"
[00:04:28] tidy error: duplicate error code: 86
[00:04:28] tidy error: /checkout/src/librustc_typeck/diagnostics.rs:1060: E0086: r##"
[00:04:28] tidy error: /checkout/src/librustc_typeck/diagnostics.rs:4799: //  E0086,
[00:04:29] Expected a gate test for the feature 'const_generics'.
[00:04:29] Hint: create a failing test file named 'feature-gate-const_generics.rs'
[00:04:29] tidy error: Found 1 features without a gate test.
[00:04:29]       in the 'ui' test suite, with its failures due to
[00:04:29]       missing usage of #![feature(const_generics)].
[00:04:29] Hint: If you already have such a test and don't want to rename it,
[00:04:29]       you can also add a // gate-test-const_generics line to the test file.
[00:04:29] some tidy checks failed
[00:04:29] 
[00:04:29] 
[00:04:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:29] 
[00:04:29] 
[00:04:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:29] Build completed unsuccessfully in 0:00:50
[00:04:29] Build completed unsuccessfully in 0:00:50
[00:04:29] make: *** [tidy] Error 1
[00:04:29] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d7ce8e7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:213c91e3:start=1535050856368938027,finish=1535050856377184018,duration=8245991
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0abcf093
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0080c712
travis_time:start:0080c712
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a56e2c7
$ dmesg | grep -i kill
