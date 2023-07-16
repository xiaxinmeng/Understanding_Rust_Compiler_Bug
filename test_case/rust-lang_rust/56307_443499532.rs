plain
[00:04:36]     Checking rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:04:36]     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
[00:04:38]     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
[00:04:38]     Checking rustc_plugin v0.0.0 (/checkout/src/librustc_plugin)
[00:04:42] error[E0063]: missing field `two_phase` in initializer of `rustc::mir::StatementKind<'_>`
[00:04:42]     |
[00:04:42]     |
[00:04:42] 232 |                 kind: StatementKind::Retag { fn_entry: true, place: dropee_ptr.clone() },
[00:04:42]     |                       ^^^^^^^^^^^^^^^^^^^^ missing `two_phase`
[00:04:46] error: aborting due to previous error
[00:04:46] 
[00:04:46] For more information about this error, try `rustc --explain E0063`.
[00:04:46] error: Could not compile `rustc_mir`.
[00:04:46] error: Could not compile `rustc_mir`.
[00:04:46] warning: build failed, waiting for other jobs to finish...
[00:04:53] error: build failed
[00:04:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:04:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
[00:04:53] Build completed unsuccessfully in 0:04:23
travis_time:end:2628d813:start=1543748871691302623,finish=1543749165398302845,duration=293707000222
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:0f206560:start=1543749165872531215,finish=1543749165878860272,duration=6329057
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0133d0fa
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:21362f9f
travis_time:start:21362f9f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0cc78100
$ dmesg | grep -i kill
