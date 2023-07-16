plain
[00:51:30] ....................................................................................................
[00:51:33] .....................................................i..............................................
[00:51:35] ....................................................................................................
[00:51:38] ....................................................................................................
[00:51:41] .iiiiiiiii..........................................................................................
[00:51:46] ....................................................................................................
[00:51:49] ...................................................................................i................
[00:51:52] ....................................................................................................
[00:51:55] ......................................i.i..ii.......................................................
---
[00:56:39] .....................................................................................i..............
[00:56:42] ....................................................................................................
[00:56:45] ....................................................................................................
[00:56:48] ....................................................................................................
[00:56:50] i.ii.ii.ii.............................i............................................................
[00:56:50] test result: ok. 6712 passed; 0 failed; 88 ignored; 0 measured; 0 filtered out
[00:56:50] 
[00:56:50]  finished in 377.625
[00:56:50] travis_fold:end:test_ui_nll
---
[01:18:15] travis_fold:start:test_stage2-rustdoc
travis_time:start:test_stage2-rustdoc
Testing rustdoc stage2 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:16]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[01:18:20] error[E0063]: missing field `edition` in initializer of `html::markdown::LangString`
[01:18:20]     |
[01:18:20]     |
[01:18:20] 973 |             assert_eq!(LangString::parse(s, ErrorCodes::Yes), LangString {
[01:18:20]     |                                                               ^^^^^^^^^^ missing `edition`
[01:18:21] error: aborting due to previous error
[01:18:21] 
[01:18:21] For more information about this error, try `rustc --explain E0063`.
[01:18:21] error: Could not compile `rustdoc`.
[01:18:21] error: Could not compile `rustdoc`.
[01:18:21] 
[01:18:21] To learn more, run the command again with --verbose.
[01:18:21] 
[01:18:21] 
[01:18:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"
[01:18:21] 
[01:18:21] 
[01:18:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:18:21] Build completed unsuccessfully in 0:35:03
[01:18:21] Build completed unsuccessfully in 0:35:03
[01:18:21] make: *** [check] Error 1
[01:18:21] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:31c52d3c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:10fa6e43:start=1537354260286287500,finish=1537354260290635104,duration=4347604
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:007c29ba
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then
