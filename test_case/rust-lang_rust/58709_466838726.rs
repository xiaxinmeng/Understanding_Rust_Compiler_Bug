plain
travis_time:end:1048a0bb:start=1551050439465941504,finish=1551050442031584920,duration=2565643416
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
    86% |███████████████████████████▊    | 102kB 50.7MB/s eta 0:00:01
    95% |██████████████████████████████▌ | 112kB 59.8MB/s eta 0:00:01
    100% |████████████████████████████████| 122kB 38.9MB/s 
Collecting jmespath<1.0.0,>=0.7.1 (from botocore==1.12.101->awscli)
  Downloading https://files.pythonhosted.org/packages/83/94/7179c3832a6d45b266ddb2aac329e101367fbdb11f425f13771d27f225bb/jmespath-0.9.4-py2.py3-none-any.whl
  Downloading https://files.pythonhosted.org/packages/41/17/c62faccbfbd163c7f57f3844689e3a78bae1f403648a6afb1d0866d87fbb/python_dateutil-2.8.0-py2.py3-none-any.whl (226kB)
    4% |█▌                              | 10kB 25.7MB/s eta 0:00:01
    9% |███                             | 20kB 32.2MB/s eta 0:00:01
    13% |████▍                           | 30kB 39.6MB/s eta 0:00:01
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:37] 
[01:17:37] running 119 tests
[01:18:03] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:18:08] i......iii.i.....ii
[01:18:08] 
[01:18:08]  finished in 30.413
[01:18:08] travis_fold:end:test_debuginfo

---
[01:44:50] travis_fold:end:stage0-linkchecker

[01:44:50] travis_time:end:stage0-linkchecker:start=1551056740978954463,finish=1551056743080794043,duration=2101839580

[01:44:50] book/second-edition/ch07-03-importing-names-with-use.html:31: broken link fragment `#the-use-keyword-to-bring-paths-into-a-scope.html` pointing to `book/ch07-02-defining-modules-to-control-scope-and-privacy.html`
[01:44:50] book/second-edition/print.html:220: broken link fragment `#using-the-pub-keyword-to-make-items-public.html` pointing to `book/ch07-02-defining-modules-to-control-scope-and-privacy.html`
[01:44:50] book/second-edition/print.html:227: broken link fragment `#the-use-keyword-to-bring-paths-into-a-scope.html` pointing to `book/ch07-02-defining-modules-to-control-scope-and-privacy.html`
[01:44:50] book/second-edition/ch07-02-controlling-visibility-with-pub.html:31: broken link fragment `#using-the-pub-keyword-to-make-items-public.html` pointing to `book/ch07-02-defining-modules-to-control-scope-and-privacy.html`
[01:44:50] book/crates-and-modules.html:47: broken link - book/ch07-00-packages-crates-and-modules.html
[01:44:50] book/print.html:2919: broken link fragment `#paths-for-referring-to-an-item-in-the-module-tree` pointing to `book/ch07-02-defining-modules-to-control-scope-and-privacy.html`
[01:44:50] book/print.html:9846: broken link fragment `#modules-as-the-privacy-boundary` pointing to `book/ch07-02-defining-modules-to-control-scope-and-privacy.html`
[01:44:50] book/print.html:10736: broken link fragment `#separating-modules-into-different-files` pointing to `book/ch07-02-defining-modules-to-control-scope-and-privacy.html`
[01:44:50] book/first-edition/type-aliases.html:60: broken link fragment `#type-aliases-create-type-synonyms` pointing to `book/ch19-04-advanced-types.html`
[01:44:50] book/first-edition/crates-and-modules.html:60: broken link - book/ch07-00-packages-crates-and-modules.html
[01:44:50] book/first-edition/print.html:256: broken link - book/ch07-00-packages-crates-and-modules.html
[01:44:50] book/first-edition/print.html:277: broken link fragment `#type-aliases-create-type-synonyms` pointing to `book/ch19-04-advanced-types.html`
[01:44:50] book/ch11-03-test-organization.html:316: broken link fragment `#separating-modules-into-different-files` pointing to `book/ch07-02-defining-modules-to-control-scope-and-privacy.html`
[01:44:50] book/ch04-01-what-is-ownership.html:290: broken link fragment `#paths-for-referring-to-an-item-in-the-module-tree` pointing to `book/ch07-02-defining-modules-to-control-scope-and-privacy.html`
[01:44:50] book/ch11-01-writing-tests.html:365: broken link fragment `#modules-as-the-privacy-boundary` pointing to `book/ch07-02-defining-modules-to-control-scope-and-privacy.html`
[01:44:52] std/index.html:9: broken link fragment `#the-use-keyword-to-bring-paths-into-a-scope` pointing to `book/ch07-02-defining-modules-to-control-scope-and-privacy.html`
[01:44:57] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:44:57] 
[01:44:57] 
[01:44:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:44:57] expected success, got: exit code: 101
[01:44:57] expected success, got: exit code: 101
[01:44:57] 
[01:44:57] 
[01:44:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:44:57] Build completed unsuccessfully in 0:39:33
[01:44:57] make: *** [check] Error 1
[01:44:57] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:002eb3f0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb 25 01:05:51 UTC 2019
---
travis_time:end:0171fabc:start=1551056753021834281,finish=1551056753027321611,duration=5487330
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a908986
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
