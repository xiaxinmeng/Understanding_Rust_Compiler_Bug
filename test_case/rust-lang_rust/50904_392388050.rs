plain
[00:45:16] ...............................................................i....................................
[00:45:20] ....................................................................................................
[00:45:25] ....................................................................................................
[00:45:31] ............................................................................................i.......
[00:45:34] ..........iiiiiiiii...................................................
[00:45:34] 
[00:45:34] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:46:21] ...............................................................i....................................
[00:46:25] ....................................................................................................
[00:46:30] ....................................................................................................
[00:46:36] ............................................................................................i.......
[00:46:38] ..........iiiiiiiii...................................................
[00:46:38] 
[00:46:38]  finished in 64.166
[00:46:38] travis_fold:end:test_ui_nll

---
[01:21:37]    Compiling rand v0.4.2
[01:21:38]    Compiling aho-corasick v0.6.4
[01:21:44]    Compiling tempdir v0.3.7
[01:22:16]    Compiling minifier v0.0.11
[01:22:19] error[E0523]: found two different crates with name `rand` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
[01:22:19]    |
[01:22:19] 48 | extern crate tempdir;
[01:22:19]    | ^^^^^^^^^^^^^^^^^^^^^
[01:22:19] 
[01:22:19] 
[01:22:19] error[E0523]: found two different crates with name `rand` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
[01:22:19]    |
[01:22:19] 48 | extern crate tempdir;
[01:22:19]    | ^^^^^^^^^^^^^^^^^^^^^
[01:22:19] 
---
[01:22:19] 
[01:22:19] To learn more, run the command again with --verbose.
[01:22:19] 
[01:22:19] 
[01:22:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"
[01:22:19] 
[01:22:19] 
[01:22:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:22:19] Build completed unsuccessfully in 0:39:15
[01:22:19] Build completed unsuccessfully in 0:39:15
[01:22:19] Makefile:58: recipe for target 'check' failed
[01:22:19] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e26c3ee
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
