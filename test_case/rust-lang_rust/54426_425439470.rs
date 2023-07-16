plain
[00:07:33]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:07:56] error[E0308]: mismatched types
[00:07:56]    --> librustc/mir/tcx.rs:287:24
[00:07:56]     |
[00:07:56] 287 |                 elems: &self.elems[..elem_index],
[00:07:56]     |                        ^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `ty::List`, found slice
[00:07:56]     |
[00:07:56]     = note: expected type `&'tcx ty::List<mir::ProjectionElem<'tcx, mir::Local, &'tcx ty::TyS<'tcx>>>`
[00:07:56]                found type `&[mir::ProjectionElem<'tcx, mir::Local, &'tcx ty::TyS<'tcx>>]`
[00:08:03] error: aborting due to previous error
[00:08:03] 
[00:08:03] For more information about this error, try `rustc --explain E0308`.
[00:08:03] error: Could not compile `rustc`.
---

[00:08:03] expected success, got: exit code: 101
[00:08:03] travis_time:end:stage0-rustc:start=1538141978951131582,finish=1538142149658357029,duration=170707225447

[00:08:03] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:08:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:08:03] Build completed unsuccessfully in 0:03:46
[00:08:03] Build completed unsuccessfully in 0:03:46
[00:08:03] make: *** [all] Error 1
[00:08:03] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:16550118
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:03b58dfe:start=1538142150342760860,finish=1538142150347639287,duration=4878427
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0805d608
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0283b0c8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/
