plain
[00:29:56]      |
[00:29:56] 110  |         return ()
[00:29:56]      |                ^^ help: remove these parentheses
[00:29:56] ...
[00:29:56] 1001 |         let cmt = ignore_err!(self.with_mc(|mc| mc.cat_expr(base)));
[00:29:56]      |
[00:29:56]      = note: `-D unused-parens` implied by `-D warnings`
[00:29:56] 
[00:29:56] error: unnecessary parentheses
[00:29:56] error: unnecessary parentheses
[00:29:56]     --> librustc_typeck/check/regionck.rs:110:16
[00:29:56]      |
[00:29:56] 110  |         return ()
[00:29:56]      |                ^^ help: remove these parentheses
[00:29:56] ...
[00:29:56] 1017 |         let discr_cmt = Rc::new(ignore_err!(self.with_mc(|mc| mc.cat_expr(init_expr))));
[00:29:56] 
[00:29:56] error: unnecessary parentheses
[00:29:56]     --> librustc_typeck/check/regionck.rs:110:16
[00:29:56]      |
[00:29:56]      |
[00:29:56] 110  |         return ()
[00:29:56]      |                ^^ help: remove these parentheses
[00:29:56] ...
[00:29:56] 1026 |         let discr_cmt = Rc::new(ignore_err!(self.with_mc(|mc| mc.cat_expr(discr))));
[00:29:56] 
[00:29:56] error: unnecessary parentheses
[00:29:56]     --> librustc_typeck/check/regionck.rs:110:16
[00:29:56]      |
[00:29:56]      |
[00:29:56] 110  |           return ()
[00:29:56]      |                  ^^ help: remove these parentheses
[00:29:56] ...
[00:29:56] 1060 | /         ignore_err!(self.with_mc(|mc| {
[00:29:56] 1061 | |             mc.cat_pattern(discr_cmt, root_pat, |sub_cmt, sub_pat| {
[00:29:56] 1062 | |                 match sub_pat.node {
[00:29:56] 1063 | |                     // `ref x` pattern
[00:29:56] 1076 | |             })
[00:29:56] 1077 | |         }));
[00:29:56]      | |____________- in this macro invocation
[00:29:56] 
[00:29:56] 
[00:30:03] error: aborting due to 4 previous errors
[00:30:03] 
[00:30:03] error: Could not compile `rustc_typeck`.
[00:30:03] warning: build failed, waiting for other jobs to finish...
[00:32:42] error: build failed
[00:32:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:32:42] expected success, got: exit code: 101
[00:32:42] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1135:9
[00:32:42] travis_fold:end:stage1-rustc

[00:32:42] travis_time:end:stage1-rustc:start=1537549501952959121,finish=1537550175139458214,duration=673186499093


[00:32:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:32:42] Build completed unsuccessfully in 0:28:06
[00:32:42] make: *** [all] Error 1
[00:32:42] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:282cb854
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
