plain
travis_time:end:03dc3740:start=1543845053928890582,finish=1543845055170562885,duration=1241672303
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:46] 
[00:57:46] running 118 tests
[00:58:10] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:58:14] ......iii.i.....ii
[00:58:14] 
[00:58:14]  finished in 28.702
[00:58:14] travis_fold:end:test_debuginfo

---
[01:23:50] travis_fold:end:stage0-linkchecker

[01:23:50] travis_time:end:stage0-linkchecker:start=1543850093256846671,finish=1543850095637889614,duration=2381042943

[01:23:52] edition-guide/rust-2018/the-compiler/index.html:138: broken link - edition-guide/improved-error-messages.md
[01:23:52] edition-guide/rust-2018/macros/index.html:138: broken link - edition-guide/custom-derive.md
[01:23:52] edition-guide/rust-2018/cargo-and-crates-io/index.html:138: broken link - edition-guide/cargo-check-for-faster-checking.md
[01:23:52] edition-guide/rust-2018/documentation/index.html:138: broken link - edition-guide/new-editions-of-the-book.md
[01:23:52] edition-guide/rust-2018/control-flow/index.html:138: broken link - edition-guide/async-await-for-easier-concurrency.md
[01:23:52] edition-guide/rust-2018/module-system/path-clarity.html:214: broken link - macros/macro-changes.html
[01:23:52] edition-guide/rust-2018/module-system/index.html:138: broken link - edition-guide/path-clarity.md
[01:23:52] edition-guide/rust-2018/trait-system/index.html:138: broken link - edition-guide/impl-trait-for-returning-complex-types-with-ease.md
[01:23:52] edition-guide/rust-2018/trait-system/dyn-trait-for-trait-objects.html:169: broken link - edition-guide/impl-trait-for-returning-complex-types-with-ease.html
[01:23:52] edition-guide/rust-2018/platform-and-target-support/index.html:138: broken link - edition-guide/libcore-for-low-level-rust.md
[01:23:52] edition-guide/rust-2018/rustdoc/index.html:138: broken link - edition-guide/documentation-tests-can-now-compile-fail.md
[01:23:52] edition-guide/rust-2018/ownership-and-lifetimes/lifetime-elision-in-impl.html:179: broken link - edition-guide/the-anonymous-lifetime.html
[01:23:52] edition-guide/rust-2018/ownership-and-lifetimes/index.html:138: broken link - edition-guide/default-match-bindings.md
[01:23:52] edition-guide/rust-2018/data-types/index.html:138: broken link - edition-guide/field-init-shorthand.md
[01:23:52] edition-guide/rust-2018/error-handling-and-panics/index.html:138: broken link - edition-guide/the-question-mark-operator-for-easier-error-handling.md
[01:23:52] edition-guide/print.html:254: broken link - rust-2018/trait-system/no-anon-params.html
[01:23:52] edition-guide/print.html:306: broken link - rust-2018/trait-system/dyn-trait-for-trait-objects.html
[01:23:52] edition-guide/print.html:371: broken link - edition-guide/path-clarity.md
[01:23:52] edition-guide/print.html:494: broken link - macros/macro-changes.html
[01:23:52] edition-guide/print.html:787: broken link - edition-guide/the-question-mark-operator-for-easier-error-handling.md
[01:23:52] edition-guide/print.html:1087: broken link - edition-guide/async-await-for-easier-concurrency.md
[01:23:52] edition-guide/print.html:1118: broken link - edition-guide/impl-trait-for-returning-complex-types-with-ease.md
[01:23:52] edition-guide/print.html:1287: broken link - edition-guide/impl-trait-for-returning-complex-types-with-ease.html
[01:23:52] edition-guide/print.html:1499: broken link - edition-guide/default-match-bindings.md
[01:23:52] edition-guide/print.html:1749: broken link - edition-guide/the-anonymous-lifetime.html
[01:23:52] edition-guide/print.html:1864: broken link - edition-guide/field-init-shorthand.md
[01:23:52] edition-guide/print.html:2211: broken link - edition-guide/custom-derive.md
[01:23:52] edition-guide/print.html:2490: broken link - edition-guide/improved-error-messages.md
[01:23:52] edition-guide/print.html:2703: broken link - edition-guide/cargo-check-for-faster-checking.md
[01:23:52] edition-guide/print.html:2912: broken link - edition-guide/new-editions-of-the-book.md
[01:23:52] edition-guide/print.html:2981: broken link - edition-guide/documentation-tests-can-now-compile-fail.md
[01:23:52] edition-guide/print.html:3010: broken link - edition-guide/libcore-for-low-level-rust.md
[01:23:52] edition-guide/editions/transitioning-an-existing-project-to-a-new-edition.html:157: broken link - rust-2018/trait-system/no-anon-params.html
[01:23:52] edition-guide/editions/transitioning-an-existing-project-to-a-new-edition.html:209: broken link - rust-2018/trait-system/dyn-trait-for-trait-objects.html
[01:23:56] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:49:9
[01:23:56] 
[01:23:56] 
[01:23:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:23:56] expected success, got: exit code: 101
[01:23:56] expected success, got: exit code: 101
[01:23:56] 
[01:23:56] 
[01:23:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:23:56] Build completed unsuccessfully in 0:37:19
[01:23:56] Makefile:58: recipe for target 'check' failed
[01:23:56] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01e3b6d5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec  3 15:15:01 UTC 2018
---
travis_time:end:04a6d453:start=1543850104407931655,finish=1543850104414779998,duration=6848343
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1cd442a8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:29571198
travis_time:start:29571198
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:150135c0
$ dmesg | grep -i kill
