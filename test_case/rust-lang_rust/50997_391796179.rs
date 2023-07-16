plain
[00:59:47] .....................................................i..............................................
[00:59:52] .........................................................................ii.........................
[01:00:00] ....................................................................................................
[01:00:07] ...................................................................................i................
[01:00:10] .iiiiiiiii...................................................
[01:00:10] 
[01:00:10] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[01:01:07] .....................................................i..............................................
[01:01:13] .........................................................................ii.........................
[01:01:19] ....................................................................................................
[01:01:27] ...................................................................................i................
[01:01:29] .iiiiiiiii...................................................
[01:01:29] 
[01:01:29]  finished in 79.239
[01:01:29] travis_fold:end:test_ui_nll

---
[01:49:21] travis_fold:start:test_stage1-syntax
travis_time:start:test_stage1-syntax
Testing syntax stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:49:21]    Compiling syntax v0.0.0 (file:///checkout/src/libsyntax)
0m^^^ help: consider using `_fm3` instead
[01:49:37] error: unused variable: `fm1`
[01:49:37]     --> libsyntax/codemap.rs:1077:13
[01:49:37]      |
[01:49:37] 1077 |         let fm1 =
[01:49:37] 1077 |         let fm1 =
[01:49:37]      |             ^^^ help: consider using `_fm1` instead
[01:49:37] error: unused variable: `fm2`
[01:49:37]     --> libsyntax/codemap.rs:1080:13
[01:49:37]      |
[01:49:37]      |
[01:49:37] 1080 |         let fm2 = cm.new_filemap(PathBuf::from("blork2.rs").into(),
[01:49:37]      |             ^^^ help: consider using `_fm2` instead
[01:49:38] error: aborting due to 5 previous errors
[01:49:38] 
[01:49:38] error: Could not compile `syntax`.
[01:49:38] 
[01:49:38] 
[01:49:38] To learn more, run the command again with --verbose.
[01:49:38] 
[01:49:38] 
[01:49:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:49:38] 
[01:49:38] 
[01:49:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:49:38] Build completed unsuccessfully in 0:52:42
[01:49:38] Build completed unsuccessfully in 0:52:42
[01:49:38] make: *** [check] Error 1
[01:49:38] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2fdadc87
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
