plain
[00:54:42] ....................................................................................................
[00:54:45] .......................................................i............................................
[00:54:48] ....................................................................................................
[00:54:51] ....................................................................................................
[00:54:54] ....iiiiiiiii.......................................................................................
[00:55:00] ....................................................................................................
[00:55:03] ........................................................................................i...........
[00:55:06] ....................................................................................................
[00:55:08] ................................................i.i..ii.............................................
---
[01:21:46]    Compiling rustc_driver v0.0.0 (file:///checkout/src/librustc_driver)
[01:21:48] error[E0061]: this function takes 4 parameters but 3 parameters were supplied
[01:21:48]    --> librustc_driver/test.rs:180:19
[01:21:48]     |
[01:21:48] 180 |             infcx.resolve_regions_and_report_errors(def_id, &region_scope_tree, &outlives_env);
[01:21:48] 
[01:21:48] error: aborting due to previous error
[01:21:48] 
[01:21:48] For more information about this error, try `rustc --explain E0061`.
[01:21:48] For more information about this error, try `rustc --explain E0061`.
[01:21:48] error: Could not compile `rustc_driver`.
[01:21:48] 
[01:21:48] To learn more, run the command again with --verbose.
[01:21:48] 
[01:21:48] 
[01:21:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:21:48] 
[01:21:48] 
[01:21:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:21:48] Build completed unsuccessfully in 0:35:44
[01:21:48] Build completed unsuccessfully in 0:35:44
[01:21:48] Makefile:58: recipe for target 'check' failed
[01:21:48] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a35de7e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0d641e74:start=1537895535073488418,finish=1537895535078975910,duration=5487492
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00d8137f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f5e8030
travis_time:start:0f5e8030
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a86e5b0
$ dmesg | grep -i kill
