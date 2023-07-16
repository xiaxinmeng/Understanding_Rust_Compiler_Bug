plain
[00:06:56]     Checking rustc_plugin v0.0.0 (/checkout/src/librustc_plugin)
[00:06:56] error[E0599]: no method named `get_parent_node_by_hir_id` found for type `&rustc::hir::map::Map<'_>` in the current scope
[00:06:56]     --> src/librustc_typeck/check/mod.rs:3964:40
[00:06:56]      |
[00:06:56] 3964 |         let item_id = self.tcx().hir().get_parent_node_by_hir_id(self.body_id);
[00:06:56] 
[00:06:56]     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:07:00] error: aborting due to previous error
[00:07:00] 
---
travis_time:end:15014657:start=1561639104220077617,finish=1561639104228336625,duration=8259008
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04e66800
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:250fbe97
travis_time:start:250fbe97
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:035b50cf
$ dmesg | grep -i kill
