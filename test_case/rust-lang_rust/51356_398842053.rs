plain
[01:18:39] travis_fold:start:test_stage1-serialize
travis_time:start:test_stage1-serialize
Testing serialize stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:39]    Compiling serialize v0.0.0 (file:///checkout/src/libserialize)
[01:18:40] error[E0425]: cannot find value `cursor` in this scope
[01:18:40]    --> libserialize/opaque.rs:367:45
[01:18:40]     |
[01:18:40] 367 |         let mut encoder = Encoder::new(&mut cursor);
[01:18:40] 
[01:18:40] 
[01:18:40] error: unused import: `std::io::Cursor`
[01:18:40]    --> libserialize/opaque.rs:338:9
[01:18:40]     |
[01:18:40] 338 |     use std::io::Cursor;
[01:18:40]     |
[01:18:40]     = note: `-D unused-imports` implied by `-D warnings`
[01:18:40] 
[01:18:42] error: aborting due to 2 previous errors
[01:18:42] error: aborting due to 2 previous errors
[01:18:42] 
[01:18:42] For more information about this error, try `rustc --explain E0425`.
[01:18:42] error: Could not compile `serialize`.
[01:18:42] To learn more, run the command again with --verbose.
[01:18:42] 
[01:18:42] 
[01:18:42] 
[01:18:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "serialize" "--" "--quiet"
[01:18:42] 
[01:18:42] 
[01:18:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:18:42] Build completed unsuccessfully in 0:37:30
[01:18:42] Build completed unsuccessfully in 0:37:30
[01:18:42] make: *** [check] Error 1
[01:18:42] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07339b58
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0128ea7e:start=1529517610753572213,finish=1529517610760821759,duration=7249546
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:248746f4
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:20464e03
$ dmesg | grep -i kill
