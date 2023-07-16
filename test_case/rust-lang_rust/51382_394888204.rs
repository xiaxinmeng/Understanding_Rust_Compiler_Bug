plain
[00:36:08]    Compiling aho-corasick v0.6.4
[00:36:15]    Compiling tempdir v0.3.7
[00:36:52]    Compiling minifier v0.0.11
[00:36:55]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:37:01] error[E0609]: no field `current_item_id` on type `&core::DocContext<'_, '_, '_>`
[00:37:01]     --> librustdoc/clean/mod.rs:1256:52
[00:37:01]      |
[00:37:01] 1256 |                                                 cx.current_item_id.borrow().unwrap(),
[00:37:01] 
[00:37:11] error: aborting due to previous error
[00:37:11] 
[00:37:11] For more information about this error, try `rustc --explain E0609`.
---
[00:37:11] 
[00:37:11] 
[00:37:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:37:11] Build completed unsuccessfully in 0:31:57
[00:37:11] make: *** [all] Error 1
[00:37:11] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12f21d65
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
