plain
travis_time:end:1c13ede8:start=1551032458755014020,finish=1551032460897623872,duration=2142609852
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
[01:13:24] 
[01:13:24] running 119 tests
[01:13:49] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:13:53] i......iii.i.....ii
[01:13:53] 
[01:13:53]  finished in 29.533
[01:13:53] travis_fold:end:test_debuginfo

---
[01:39:49] travis_fold:end:stage0-linkchecker

[01:39:49] travis_time:end:stage0-linkchecker:start=1551038459193836386,finish=1551038461376226718,duration=2182390332

[01:39:50] book/second-edition/ch07-00-modules.html:31: broken link - book/ch07-02-modules-and-use-to-control-scope-and-privacy.html
[01:39:50] book/second-edition/ch07-03-importing-names-with-use.html:31: broken link - book/ch07-02-modules-and-use-to-control-scope-and-privacy.html
[01:39:50] book/second-edition/print.html:206: broken link - book/ch07-02-modules-and-use-to-control-scope-and-privacy.html
[01:39:50] book/second-edition/print.html:213: broken link - book/ch07-02-modules-and-use-to-control-scope-and-privacy.html
[01:39:50] book/second-edition/print.html:220: broken link - book/ch07-02-modules-and-use-to-control-scope-and-privacy.html
[01:39:50] book/second-edition/print.html:227: broken link - book/ch07-02-modules-and-use-to-control-scope-and-privacy.html
[01:39:50] book/second-edition/ch07-01-mod-and-the-filesystem.html:31: broken link - book/ch07-02-modules-and-use-to-control-scope-and-privacy.html
[01:39:50] book/second-edition/ch07-02-controlling-visibility-with-pub.html:31: broken link - book/ch07-02-modules-and-use-to-control-scope-and-privacy.html
[01:39:50] book/crates-and-modules.html:47: broken link - book/ch07-00-packages-crates-and-modules.html
[01:39:50] book/print.html:2919: broken link - book/ch07-02-modules-and-use-to-control-scope-and-privacy.html
[01:39:50] book/print.html:9846: broken link - book/ch07-02-modules-and-use-to-control-scope-and-privacy.html
[01:39:50] book/print.html:10736: broken link - book/ch07-02-modules-and-use-to-control-scope-and-privacy.html
[01:39:50] book/first-edition/type-aliases.html:60: broken link fragment `#type-aliases-create-type-synonyms` pointing to `book/ch19-04-advanced-types.html`
[01:39:50] book/first-edition/crates-and-modules.html:60: broken link - book/ch07-00-packages-crates-and-modules.html
[01:39:50] book/first-edition/print.html:256: broken link - book/ch07-00-packages-crates-and-modules.html
[01:39:50] book/first-edition/print.html:277: broken link fragment `#type-aliases-create-type-synonyms` pointing to `book/ch19-04-advanced-types.html`
[01:39:50] book/ch11-03-test-organization.html:316: broken link - book/ch07-02-modules-and-use-to-control-scope-and-privacy.html
[01:39:50] book/ch04-01-what-is-ownership.html:290: broken link - book/ch07-02-modules-and-use-to-control-scope-and-privacy.html
[01:39:50] book/ch11-01-writing-tests.html:365: broken link - book/ch07-02-modules-and-use-to-control-scope-and-privacy.html
[01:39:52] std/index.html:9: broken link - book/ch07-02-modules-and-use-to-control-scope-and-privacy.html
[01:39:57] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:39:57] 
[01:39:57] 
[01:39:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:39:57] expected success, got: exit code: 101
[01:39:57] expected success, got: exit code: 101
[01:39:57] 
[01:39:57] 
[01:39:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:39:57] Build completed unsuccessfully in 0:38:12
[01:39:57] make: *** [check] Error 1
[01:39:57] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3ad4d921
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb 24 20:01:09 UTC 2019
