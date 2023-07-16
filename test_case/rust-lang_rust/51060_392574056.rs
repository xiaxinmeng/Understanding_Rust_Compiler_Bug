plain
[00:50:20] ................................................................i...................................
[00:50:25] ....................................................................................................
[00:50:31] ....................................................................................................
[00:50:38] .............................................................................................i......
[00:50:41] ...........iiiiiiiii...................................................
[00:50:41] 
[00:50:41] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:51:30] ................................................................i...................................
[00:51:34] ....................................................................................................
[00:51:40] ....................................................................................................
[00:51:46] .............................................................................................i......
[00:51:48] ...........iiiiiiiii...................................................
[00:51:48] 
[00:51:48]  finished in 67.856
[00:51:48] travis_fold:end:test_ui_nll

---
[01:27:22]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[01:27:25] error[E0308]: mismatched types
[01:27:25]    --> librustc_data_structures/tiny_list.rs:177:36
[01:27:25]     |
[01:27:25] 177 |             assert!(!list.contains(i));
[01:27:25]     |                                    |
[01:27:25]     |                                    expected &u32, found u32
[01:27:25]     |                                    help: consider borrowing here: `&i`
[01:27:25]     |
---
[01:27:25] 
[01:27:25] To learn more, run the command again with --verbose.
[01:27:25] 
[01:27:25] 
[01:27:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_data_structures" "--" "--quiet"
[01:27:25] 
[01:27:25] 
[01:27:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:27:25] Build completed unsuccessfully in 0:39:32
[01:27:25] Build completed unsuccessfully in 0:39:32
[01:27:25] make: *** [check] Error 1
[01:27:25] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06d52276
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
