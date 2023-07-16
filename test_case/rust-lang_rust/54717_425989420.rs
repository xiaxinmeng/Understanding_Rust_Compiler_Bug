plain
[00:04:59]     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:00]     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:05:14]     Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:05:15]     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:06:28] error[E0507]: cannot move out of borrowed content
[00:06:28]    --> librustc/ty/query/plumbing.rs:714:84
[00:06:28]     |
[00:06:28] 714 |                           self.$name.try_lock().unwrap().active.values().filter_map(|&v|
[00:06:28]     |                                                                                      ^-
[00:06:28]     |                                                                                      ||
[00:06:28]     |                                                                                      |data moved here
[00:06:28]     |                                                                                      cannot move out of borrowed content
[00:06:28]     |                                                                                      help: consider removing the `&`: `v`
[00:06:28]     | 
[00:06:28]    ::: librustc/ty/query/mod.rs:102:1
[00:06:28]     |
[00:06:28] 102 | / define_queries! { <'tcx>
[00:06:28] 103 | |     Other {
[00:06:28] 104 | |         /// Records the type of every item.
[00:06:28] 105 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:06:28] 658 | |     },
[00:06:28] 659 | | }
[00:06:28]     | |_- in this macro invocation
[00:06:28]     |
[00:06:28]     |
[00:06:28] note: move occurs because `v` has type `ty::query::job::QueryResult<'_>`, which does not implement the `Copy` trait
[00:06:28]    --> librustc/ty/query/plumbing.rs:714:85
[00:06:28]     |
[00:06:28] 714 |                           self.$name.try_lock().unwrap().active.values().filter_map(|&v|
[00:06:28]     | 
[00:06:28]     | 
[00:06:28]    ::: librustc/ty/query/mod.rs:102:1
[00:06:28]     |
[00:06:28] 102 | / define_queries! { <'tcx>
[00:06:28] 103 | |     Other {
[00:06:28] 104 | |         /// Records the type of every item.
[00:06:28] 105 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:06:28] 658 | |     },
[00:06:28] 659 | | }
[00:06:28]     | |_- in this macro invocation
[00:06:28] 
[00:06:28] 
[00:06:32] error: aborting due to previous error
[00:06:32] 
[00:06:32] For more information about this error, try `rustc --explain E0507`.
[00:06:32] error: Could not compile `rustc`.
[00:06:32] 
[00:06:32] To learn more, run the command again with --verbose.
[00:06:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:32] expected success, got: exit code: 101
[00:06:32] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:06:32] travis_fold:end:stage0-rustc

[00:06:32] travis_time:end:stage0-rustc:start=1538409089454165904,finish=1538409205968674273,duration=116514508369

---
travis_time:end:0907de68:start=1538409206577798463,finish=1538409206582644137,duration=4845674
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d047510
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1c030b80
travis_time:start:1c030b80
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1648a29a
$ dmesg | grep -i kill
