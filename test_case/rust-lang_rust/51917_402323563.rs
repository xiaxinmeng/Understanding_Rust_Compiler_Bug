plain
[00:03:27] DirectMap2M:     3080192 kB
[00:03:27] DirectMap1G:    14680064 kB
[00:03:27] + python2.7 ../x.py test --target arm-unknown-linux-gnueabihf
[00:03:28]     Finished dev [unoptimized] target(s) in 0.34s
[00:03:29] thread 'main' panicked at 'assertion failed: !use_snapshot || stage == 0', bootstrap/builder.rs:778:9
[00:03:29] travis_fold:end:log-system-info
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-unknown-linux-gnueabihf
[00:03:29] Build completed unsuccessfully in 0:00:01
travis_time:end:08b4084c:start=1530661814870250059,finish=1530662024814049484,duration=209943799425
---
travis_time:end:0ba98b61:start=1530662025135009316,finish=1530662025146358794,duration=11349478
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07a7f5fb
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ���./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:151106d0
$ dmesg | grep -i kill
