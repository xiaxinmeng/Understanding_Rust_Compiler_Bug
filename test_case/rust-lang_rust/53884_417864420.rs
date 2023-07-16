plain
[01:29:43]    --> tools/miri/src/lib.rs:554:5
[01:29:43]     |
[01:29:43] 554 | /     fn global_item_with_linkage<'a>(
[01:29:43] 555 | |         _ecx: &mut EvalContext<'a, 'mir, 'tcx, Self>,
[01:29:43] 556 | |         _instance: ty::Instance<'tcx>m|         mem.check_locks(ptr, size.bytes(), access)
[01:29:43]     | |_____^ not a member of trait `Machine`
[01:29:43] 
[01:29:43] error[E0407]: method `add_lock` is not a member of trait `Machine`
[01:29:43]    --> tools/miri/src/lib.rs:571:5
---
[01:29:43] 
[01:29:43] error[E0423]: expected value, found struct variant `StackPopCleanup::None`
[01:29:43]    --> tools/miri/src/fn_call.rs:163:38
[01:29:43]     |
[01:29:43] 163 |             None => (Place::[0m^^^^^^^^^^^ did you mean `$crate::mir::interpret::EvalErrorKind::Panic { /* fields */ }`?
[01:29:43]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:29:43] 
[01:29:43] error[E0423]: expected value, found struct variant `$crate::mir::interpret::EvalErrorKind::Panic`
[01:29:43]    --> tools/miri/src/fn_call.rs:736:50
---
[01:29:43] 
[01:29:43] warning: unused imports: `EvalErrorKind`, `EvalError`
[01:29:43]   --> tools/miri/src/validation.rs:16:58
[01:29:43]    |
[01:29:43] 16 | use rustc::mi[1m^^^^^^^^^^^^^^^^^^
[01:29:43] warning: unused import: `tls::EvalContextExt as TlsEvalContextExt`
[01:29:43]   --> tools/miri/src/lib.rs:54:5
[01:29:43]    |
[01:29:43] 54 | use tls::EvalContextExt as TlsEvalContextExt;
---
[01:29:44] Verifying status of rust-by-example...
[01:29:44] Verifying status of rls...
[01:29:44] This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
[01:29:44] 
[01:29:44] ⚠️ We detected that this PR updated 'rls', but its tests failed.
[01:29:44] 
[01:29:44] If you do intend to update 'rls', please check the error messages above and
[01:29:44] commit another update.
[01:29:44] If youllvm-emscripten
219712 ./obj/build/x86_64-unknown-linux-gnu/stage0-codegen
209616 ./obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu
209612 ./obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release
---
145112 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
145108 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
142912 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
134532 ./obj/build/bootstrap/debug/incremental/bootstrap-11nz4fw202v9g
134528 ./obj/build/bootstrap/debug/incremental/bootstrap-11nz4fw202v9g/s-f4ecfhgr0r-17p59mv-29k09wkatlb53
131912 ./86_64-unknown-linux-gnu/openssl/openssl-1.0.2n/test
55548 ./obj/build/x86_64-unknown-linux-gnu/stage0/bin
51036 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin
50132 ./src/llvm/test/CodeGen/X86
48928 ./obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/AArch64
---
travis_time:end:1f8710d5:start=1535813184920740575,finish=1535813184927831449,duration=7090874
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2de0e860
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07ac1a26
travis_time:start:07ac1a26
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:16877448
$ dmesg | grep -i kill
