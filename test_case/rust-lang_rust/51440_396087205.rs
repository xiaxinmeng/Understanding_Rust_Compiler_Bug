plain
[00:03:15] DirectMap2M:     3084288 kB
[00:03:15] DirectMap1G:    14680064 kB
[00:03:15] + python2.7 /checkout/x.py test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/parse-fail src/test/mir-opt src/test/codegen-units src/libcore
[00:03:16]     Finished dev [unoptimized] target(s) in 0.25s
[00:03:17] thread 'main' panicked at 'Error: no rules matched src/libcore.', bootstrap/builder.rs:233:21
[00:03:17] travis_fold:end:log-system-info
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/parse-fail src/test/mir-opt src/test/codegen-units src/libcore
[00:03:17] Build completed unsuccessfully in 0:00:01
travis_time:end:0fe416f9:start=1528669591745171509,finish=1528669789205040074,duration=197459868565
---
travis_time:end:0069ea66:start=1528669789452072125,finish=1528669789458695478,duration=6623353
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05372a11
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00b1a051
$ dmesg | grep -i kill
