plain
[00:17:43]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:17:43]    Compiling rustc_codegen_llvm v0.0.0 (file:///checkout/src/librustc_codegen_llvm)
[00:17:43]    Compiling cc v1.0.18
[00:17:44]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:17:54] error[E0609]: no field `param_env` on type `&mut mir::FunctionCx<'a, 'll, 'tcx>`
[00:17:54]    --> librustc_codegen_llvm/mir/block.rs:527:70
[00:17:54]     |
[00:17:54] 527 |                                     let c = bx.tcx().const_eval(self.param_env.and(cid));
[00:17:54] 
rc/tools/lld
13740 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
13736 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
---
travis_time:end:06d8cce0:start=1533289284626934833,finish=1533289284636001434,duration=9066601
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05179827
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01a148d3
travis_time:start:01a148d3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00ceb33e
$ dmesg | grep -i kill
