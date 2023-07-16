plain
[00:25:36]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:25:42]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:25:42]    Compiling cmake v0.1.33
[00:25:42]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:25:43] thread 'main' panicked at 'librustc/hir/map/collector.rs:217: inconsistent DepNode for `PathSegment(PathSegment { ident: prelude#0, id: Some(NodeId(103)), def: Some(Mod(DefId(0/0:547))), args: None, infer_types: false })`: current_dep_node_owner=::{{?}}[0], hir_id.owner=', librustc/util/bug.rs:47:26
[00:25:43] 
[00:25:43] error: internal compiler error: unexpected panic
[00:25:43] 
[00:25:43] note: the compiler unexpectedly panicked. this is a bug.
[00:25:43] note: the compiler unexpectedly panicked. this is a bug.
[00:25:43] 
[00:25:43] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:25:43] 
[00:25:43] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:25:43] 
[00:25:43] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:25:43] note: some of the compiler flags provided by cargo are hidden
[00:25:43] 
[00:25:43] error: Could not compile `core`.
[00:25:43] warning: build failed, waiting for other jobs to finish...
---
travis_time:end:003c4446:start=1537345035028561735,finish=1537345035032539718,duration=3977983
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:3cb70ed3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07f79280
travis_time:start:07f79280
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09537984
$ dmesg | grep -i kill
