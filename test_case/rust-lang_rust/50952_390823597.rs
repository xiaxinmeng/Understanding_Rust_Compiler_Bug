plain
[01:52:07] travis_fold:end:stage0-linkchecker

[01:52:07] travis_time:end:stage0-linkchecker:start=1526948181861338750,finish=1526948185094104534,duration=3232765784

[01:52:18] book/type-aliases.html:42: broken link fragment `#type-aliases-create-type-synonyms` pointing to `book/second-edition/ch19-04-advanced-types.html`
[01:52:18] book/unsized-types.html:46: broken link fragment `#dynamically-sized-types-and-sized` pointing to `book/second-edition/ch19-04-advanced-types.html`
[01:52:18] book/associated-types.html:45: broken link fragment `#associated-types-specify-placeholder-types-in-trait-definitions` pointing to `book/second-edition/ch19-03-advanced-traits.html`
[01:52:26] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:52:26] 
[01:52:26] 
[01:52:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:52:26] expected success, got: exit code: 101
[01:52:26] expected success, got: exit code: 101
[01:52:26] 
[01:52:26] 
[01:52:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:52:26] Build completed unsuccessfully in 0:54:44
[01:52:26] Makefile:58: recipe for target 'check' failed
[01:52:26] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01a130ac
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
