plain
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:59:51] 
[00:59:51] running 48 tests
[01:00:01] ERROR 2018-07-21T07:16:32Z: compiletest::runtest: None
01:00:10]                                          | Live variables on entry to bb0[1]: []
[01:00:10]         _1 = [const 1usize, const 2usize, const 3usize];
[01:00:10]                                          | Live variables on entry to bb0[2]: []
[01:00:10]         StorageLive(_2);
[01:00:10]                                          | Live variables on entry to bb0[3]: []
[01:00:10]         StorageLive(_3);
[01:00:10]                                          | Live variables on entry to bb0[4]: []
[01:00:10]         _3 = const 0usize;
[01:00:10]                                          | Live variables on entry to bb0[5]: []
[01:00:10]         _4 = Len(_1);
[01:00:10]                                          | Live variables on entry to bb0[6]: []
[01:00:10]         _5 = Lt(_3, _4);
[01:00:10]                                          | Live variables on entry to bb0[7]: []
[01:00:10]         assert(move _5, "index out of bounds: the len is move _4 but the index is _3") -> [success: bb2, unwind: bb1];
[01:00:10]     | Live variables on exit from bb0: []
[01:00:10]     bb1: {
[01:00:10]     bb1: {
[01:00:10]                                          | Live variables on entry to bb1[0]: []
[01:00:10]         resume;
[01:00:10]     | Live variables on exit from bb1: []
[01:00:10]     }
[01:00:10]     bb2: {                              
[01:00:10]                                          | Live variables on entry to bb2[0]: []
[01:00:10]         _2 = &'_#2r _1[_3];
[01:00:10]                                          | Live variables on entry to bb2[1]: [0]
[01:00:10]         switchInt(const true) -> [false: bb4, otherwise: bb3];
[01:00:10]     | Live variables on exit from bb2: [0]
[01:00:10]     }
[01:00:10]     bb3: {                              
[01:00:10]                                          | Live variables on entry to bb3[0]: [0]
[01:00:10]         StorageLive(_7);
[01:00:10]                                          | Live variables on entry to bb3[1]: [0]
[01:00:10]         _7 = (*_2);
[01:00:10]                                          | Live variables on entry to bb3[2]: []
[01:00:10]         _6 = const use_x(move _7) -> [return: bb5, unwind: bb1];
[01:00:10]     | Live variables on exit from bb3: []
[01:00:10]     }
[01:00:10]     bb4: {                              
[01:00:10]                                          | Live variables on entry to bb4[0]: []
[01:00:10]         _8 = const use_x(const 22usize) -> [return: bb6, unwind: bb1];
[01:00:10]     | Live variables on exit from bb4: []
[01:00:10]     }
[01:00:10]     bb5: {                              
[01:00:10]                                          | Live variables on entry to bb5[0]: []
[01:00:10]         StorageDead(_7);
[01:00:10]                                          | Live variables on entry to bb5[1]: []
[01:00:10]         _0 = ();
[01:00:10]                                          | Live variables on entry to bb5[2]: []
[01:00:10]         goto -> bb7;
[01:00:10]     | Live variables on exit from bb5: []
[01:00:10]     }
[01:00:10]     bb6: {                              
[01:00:10]                                          | Live variables on entry to bb6[0]: []
[01:00:10]         _0 = ();
travis_time:start:158c7974
---
travis_time:end:3ab7c9e0:start=1532157402585879831,finish=1532157402593163890,duration=7284059
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00a29e00
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03d56606
travis_time:start:03d56606
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:30d4542a
$ dmesg | grep -i kill
