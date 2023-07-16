plain
[00:03:22] DirectMap2M:     3076096 kB
[00:03:22] DirectMap1G:    14680064 kB
[00:03:22] + python2.7 /checkout/x.py test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/parse-fail src/test/mir-opt src/test/codegen-units src/libcore
[00:03:22]     Finished dev [unoptimized] target(s) in 0.24s
[00:03:23] thread 'main' panicked at 'Error: no rules matched src/libcore.', bootstrap/builder.rs:233:21
[00:03:23] travis_fold:end:log-system-info
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/parse-fail src/test/mir-opt src/test/codegen-units src/libcore
[00:03:23] Build completed unsuccessfully in 0:00:01
travis_time:end:2395b105:start=1528653363441270143,finish=1528653567684126023,duration=204242855880
---
travis_time:end:253876fe:start=1528653567953164292,finish=1528653567959884245,duration=6719953
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:130f5f08
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:19fc7941
$ dmesg | grep -i kill
