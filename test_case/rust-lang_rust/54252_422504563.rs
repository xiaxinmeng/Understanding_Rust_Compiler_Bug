plain
✓ uploaded script
travis_fold:end:step_upload_script
travis_fold:start:worker_info
Worker information
hostname: 1779a2a6-e81a-4e44-ad23-cd082c5254fe@1.production-2-worker-org-gce-zfk4
instance: travis-job-e1fec006-941d-4f3f-8aa2-c82089310c43 travis-ci-connie-trusty-1512502258-986baf0 (via amqp)
startup: 6.405881021s
travis_fold:end:worker_info
travis_fold:start:system_info
---
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0b58bca6
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:31:01] [ 86%] Building CXX object ELF/CMakeFiles/lldELF.dir/InputFiles.cpp.o
[01:31:02] [ 87%] Building CXX object ELF/CMakeFiles/lldELF.dir/InputSection.cpp.o
[01:31:06] [ 88%] Building CXX object ELF/CMakeFiles/lldELF.dir/LTO.cpp.o
[01:31:06] [ 89%] Building CXX object ELF/CMakeFiles/lldELF.dir/LinkerScript.cpp.o
[01:31:07] /checkout/src/tools/lld/ELF/InputFiles.cpp:132:43: error: non-const lvalue reference to type 'unique_ptr<llvm::DWARFCompileUnit, default_delete<llvm::DWARFCompileUnit>>' cannot bind to a value of unrelated type 'unique_ptr<llvm::DWARFUnit, default_delete<llvm::DWARFUnit>>'
[01:31:07]   for (std::unique_ptr<DWARFCompileUnit> &CU : Dwarf->compile_units()) {
[01:31:07]                                           ^  ~
[01:31:07] /checkout/src/llvm/include/llvm/ADT/iterator_range.h:46:13: note: selected 'begin' function with iterator type 'std::unique_ptr<llvm::DWARFUnit, std::default_delete<llvm::DWARFUnit> > *'
[01:31:07]   IteratorT begin() const { return begin_iterator; }
[01:31:07] 1 error generated.
[01:31:07] 1 error generated.
[01:31:07] gmake[2]: *** [ELF/CMakeFiles/lldELF.dir/InputFiles.cpp.o] Error 1
[01:31:07] gmake[2]: *** Waiting for unfinished jobs....
[01:31:12] gmake[1]: *** [ELF/CMakeFiles/lldELF.dir/all] Error 2
[01:31:12] gmake: *** [all] Error 2
[01:31:12] command did not execute successfully, got: exit code: 2
[01:31:12] 
[01:31:12] 
[01:31:12] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.33/src/lib.rs:773:5
[01:31:12]  finished in 81.639
[01:31:12] travis_fold:end:lld

[01:31:12] travis_time:end:lld:start=1537296249950359339,finish=1537296331590334580,duration=81639975241
---
travis_time:end:02a8e84a:start=1537296333158766697,finish=1537296333163518306,duration=4751609
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2615d430
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11dcb0e8
travis_time:start:11dcb0e8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a56536e
$ dmesg | grep -i kill
