plain
travis_time:end:067d7890:start=1543844795225871120,finish=1543844851356965071,duration=56131093951
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:45:40] running 4991 tests
[00:45:42] .................................................................................................... 100/4991
[00:45:45] .................................................................................................... 200/4991
[00:45:48] ............................................................................................ii...... 300/4991
[00:45:50] .........................................................................................iii.......i 400/4991
[00:45:53] iiiiiii.iii............................iii...........................................i...........i.. 500/4991
[00:46:00] .................................................................................................... 700/4991
[00:46:05] .................................................................i...........i...................... 800/4991
[00:46:08] ...................................................................................iiiii............ 900/4991
[00:46:11] .................................................................................................... 1000/4991
---
[00:46:45] .................................................................................................... 2200/4991
[00:46:49] .................................................................................................... 2300/4991
[00:46:53] .................................................................................................... 2400/4991
[00:46:56] .................................................................................................... 2500/4991
[00:46:59] ...........................................................................iiiiiiiii................ 2600/4991
[00:47:06] .........................ii......................................................................... 2800/4991
[00:47:08] .................................................................................................... 2900/4991
[00:47:12] .................................................................................................... 3000/4991
[00:47:15] ..................i................................................................................. 3100/4991
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:59:15] 
[00:59:15] running 113 tests
[00:59:18] i..ii...iii.......i...i.........i..iii.............i......i....ii...i.i.ii..............i...ii..ii.i 100/113
[00:59:18] test result: ok. 83 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[00:59:18] 
[00:59:18]  finished in 3.279
[00:59:18] travis_fold:end:test_codegen
---
[01:24:54] travis_fold:end:stage0-linkchecker

[01:24:54] travis_time:end:stage0-linkchecker:start=1543849951873430685,finish=1543849953965023976,duration=2091593291

[01:26:20] edition-guide/rust-2018/the-compiler/index.html:138: broken link - edition-guide/improved-error-messages.md
[01:26:20] edition-guide/rust-2018/macros/index.html:138: broken link - edition-guide/custom-derive.md
[01:26:20] edition-guide/rust-2018/cargo-and-crates-io/index.html:138: broken link - edition-guide/cargo-check-for-faster-checking.md
[01:26:20] edition-guide/rust-2018/documentation/index.html:138: broken link - edition-guide/new-editions-of-the-book.md
[01:26:20] edition-guide/rust-2018/control-flow/index.html:138: broken link - edition-guide/async-await-for-easier-concurrency.md
[01:26:20] edition-guide/rust-2018/module-system/path-clarity.html:214: broken link - macros/macro-changes.html
[01:26:20] edition-guide/rust-2018/module-system/index.html:138: broken link - edition-guide/path-clarity.md
[01:26:20] edition-guide/rust-2018/trait-system/index.html:138: broken link - edition-guide/impl-trait-for-returning-complex-types-with-ease.md
[01:26:20] edition-guide/rust-2018/trait-system/dyn-trait-for-trait-objects.html:169: broken link - edition-guide/impl-trait-for-returning-complex-types-with-ease.html
[01:26:20] edition-guide/rust-2018/platform-and-target-support/index.html:138: broken link - edition-guide/libcore-for-low-level-rust.md
[01:26:20] edition-guide/rust-2018/rustdoc/index.html:138: broken link - edition-guide/documentation-tests-can-now-compile-fail.md
[01:26:20] edition-guide/rust-2018/ownership-and-lifetimes/lifetime-elision-in-impl.html:179: broken link - edition-guide/the-anonymous-lifetime.html
[01:26:20] edition-guide/rust-2018/ownership-and-lifetimes/index.html:138: broken link - edition-guide/default-match-bindings.md
[01:26:20] edition-guide/rust-2018/data-types/index.html:138: broken link - edition-guide/field-init-shorthand.md
[01:26:20] edition-guide/rust-2018/error-handling-and-panics/index.html:138: broken link - edition-guide/the-question-mark-operator-for-easier-error-handling.md
[01:26:20] edition-guide/print.html:254: broken link - rust-2018/trait-system/no-anon-params.html
[01:26:20] edition-guide/print.html:306: broken link - rust-2018/trait-system/dyn-trait-for-trait-objects.html
[01:26:20] edition-guide/print.html:371: broken link - edition-guide/path-clarity.md
[01:26:20] edition-guide/print.html:494: broken link - macros/macro-changes.html
[01:26:20] edition-guide/print.html:787: broken link - edition-guide/the-question-mark-operator-for-easier-error-handling.md
[01:26:20] edition-guide/print.html:1087: broken link - edition-guide/async-await-for-easier-concurrency.md
[01:26:20] edition-guide/print.html:1118: broken link - edition-guide/impl-trait-for-returning-complex-types-with-ease.md
[01:26:20] edition-guide/print.html:1287: broken link - edition-guide/impl-trait-for-returning-complex-types-with-ease.html
[01:26:20] edition-guide/print.html:1499: broken link - edition-guide/default-match-bindings.md
[01:26:20] edition-guide/print.html:1749: broken link - edition-guide/the-anonymous-lifetime.html
[01:26:20] edition-guide/print.html:1864: broken link - edition-guide/field-init-shorthand.md
[01:26:20] edition-guide/print.html:2211: broken link - edition-guide/custom-derive.md
[01:26:20] edition-guide/print.html:2490: broken link - edition-guide/improved-error-messages.md
[01:26:20] edition-guide/print.html:2703: broken link - edition-guide/cargo-check-for-faster-checking.md
[01:26:20] edition-guide/print.html:2912: broken link - edition-guide/new-editions-of-the-book.md
[01:26:20] edition-guide/print.html:2981: broken link - edition-guide/documentation-tests-can-now-compile-fail.md
[01:26:20] edition-guide/print.html:3010: broken link - edition-guide/libcore-for-low-level-rust.md
[01:26:20] edition-guide/editions/transitioning-an-existing-project-to-a-new-edition.html:157: broken link - rust-2018/trait-system/no-anon-params.html
[01:26:20] edition-guide/editions/transitioning-an-existing-project-to-a-new-edition.html:209: broken link - rust-2018/trait-system/dyn-trait-for-trait-objects.html
[01:26:51] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:26:51] 
[01:26:51] 
[01:26:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:26:51] expected success, got: exit code: 101
[01:26:51] expected success, got: exit code: 101
[01:26:51] 
[01:26:51] 
[01:26:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:26:51] Build completed unsuccessfully in 0:44:41
[01:26:51] Makefile:58: recipe for target 'check' failed
[01:26:51] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b37ea3c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec  3 15:14:31 UTC 2018
