plain
[00:53:50] ....................................................................................................
[00:53:52] .....................................................i..............................................
[00:53:55] ....................................................................................................
[00:53:58] ....................................................................................................
[00:54:01] .iiiiiiiii..........................................................................................
[00:54:07] ....................................................................................................
[00:54:10] ..................................................................................i.................
[00:54:13] ....................................................................................................
[00:54:16] ....................................i.i..ii.........................................................
---
[01:29:26] 17 
[01:29:26] - note: rustc 1.30.0-dev running on x86_64-apple-darwin
[01:29:26] + note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[01:29:26] 19 
[01:29:26] 20 note: compiler flags: -Z ui-testing -Z unstable-options -Z treat-err-as-bug
[01:29:26] 
[01:29:26] 
[01:29:26] The actual stderr differed from the expected stderr.
[01:29:26] The actual stderr differed from the expected stderr.
[01:29:26] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/treat-err-as-bug/treat-err-as-bug.stderr
[01:29:26] To update references, rerun the tests and pass the `--bless` flag
[01:29:26] To only update this specific test, also pass `--test-args treat-err-as-bug.rs`
[01:29:26] error: 1 errors occurred comparing output.
[01:29:26] status: exit code: 1
[01:29:26] status: exit code: 1
[01:29:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/treat-err-as-bug.rs" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/treat-err-as-bug/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ztreat-err-as-bud completed unsuccessfully in 0:44:19
[01:29:26] Makefile:58: recipe for target 'check' failed
[01:29:26] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1764ff5c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
