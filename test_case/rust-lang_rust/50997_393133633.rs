plain
[00:41:21] ......................................................................i.............................
[00:41:25] ....................................................................................................
[00:41:30] ....................................................................................................
[00:41:36] ...................................................................................................i
[00:41:39] .................iiiiiiiii...................................................
[00:41:39] 
[00:41:39] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:42:24] ......................................................................i.............................
[00:42:28] ....................................................................................................
[00:42:33] ....................................................................................................
[00:42:38] ...................................................................................................i
[00:42:41] .................iiiiiiiii...................................................
[00:42:41] 
[00:42:41]  finished in 62.248
[00:42:41] travis_fold:end:test_ui_nll

---
[01:14:53] running 15 tests
[01:14:53] ............F..
[01:14:53] failures:
[01:14:53] 
[01:14:53] ---- analyze_filemap::output_offset_all stdout ----
[01:14:53] thread 'analyze_filemap::output_offset_all' panicked at 'assertion failed: `(left == right)`
[01:14:53]   left: `[BytePos(1000), BytePos(1007), BytePos(1027)]`,
[01:14:53]  right: `[BytePos(1000), BytePos(1006), BytePos(1026)]`', libsyntax_pos/analyze_filemap.rs:427:1
[01:14:53] 
[01:14:53] 
[01:14:53] failures:
[01:14:53] failures:
[01:14:53]     analyze_filemap::output_offset_all
[01:14:53] test result: FAILED. 14 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:14:53] 
[01:14:53] error: test failed, to rerun pass '--lib'
[01:14:53] 
[01:14:53] 
[01:14:53] 
[01:14:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax_pos" "--" "--quiet"
[01:14:53] 
[01:14:53] 
[01:14:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:53] Build completed unsuccessfully in 0:35:34
[01:14:53] Build completed unsuccessfully in 0:35:34
[01:14:53] Makefile:58: recipe for target 'check' failed
[01:14:53] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2c5882f8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
