plain
[00:03:35]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:03:36] error[E0658]: use of unstable library feature 'slice_index_methods'
[00:03:36]    --> liballoc/str.rs:371:47
[00:03:36]     |
[00:03:36] 371 |                         s.push_str((lower..i).get_unchecked(self));
[00:03:36]     |
[00:03:36]     |
[00:03:36]     = help: add #![feature(slice_index_methods)] to the crate attributes to enable
[00:03:36] error[E0658]: use of unstable library feature 'slice_index_methods'
[00:03:36]    --> liballoc/str.rs:390:44
[00:03:36]     |
[00:03:36]     |
[00:03:36] 390 |             s.push_str((lower..self.len()).get_unchecked(self));
[00:03:36]     |
[00:03:36]     |
[00:03:36]     = help: add #![feature(slice_index_methods)] to the crate attributes to enable
[00:03:36] error[E0658]: use of unstable library feature 'slice_index_methods'
[00:03:36]    --> liballoc/str.rs:451:47
[00:03:36]     |
[00:03:36]     |
[00:03:36] 451 |                         s.push_str((lower..i).get_unchecked(self));
[00:03:36]     |
[00:03:36]     |
[00:03:36]     = help: add #![feature(slice_index_methods)] to the crate attributes to enable
[00:03:36] error[E0658]: use of unstable library feature 'slice_index_methods'
[00:03:36]    --> liballoc/str.rs:461:44
[00:03:36]     |
[00:03:36]     |
[00:03:36] 461 |             s.push_str((lower..self.len()).get_unchecked(self));
[00:03:36]     |
[00:03:36]     |
[00:03:36]     = help: add #![feature(slice_index_methods)] to the crate attributes to enable
[00:03:37] error: aborting due to 4 previous errors
[00:03:37] 
[00:03:37] For more information about this error, try `rustc --explain E0658`.
[00:03:37] error: Could not compile `alloc`.
[00:03:37] error: Could not compile `alloc`.
[00:03:37] 
[00:03:37] Caused by:
[00:03:37]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc liballoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=dab08365760adda8 -C extra-filename=-dab08365760adda8 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-8c383f2d2bbbcb1d.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-c8f7a210a5d40dd6.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-c2bb2884be27c6a0/out` (exit code: 101)
[00:03:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:37] expected success, got: exit code: 101
[00:03:37] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1114:9
[00:03:37] travis_fold:end:stage0-std

[00:03:37] travis_time:end:stage0-std:start=1530734845838641966,finish=1530734877917793046,duration=32079151080


[00:03:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:37] Build completed unsuccessfully in 0:00:33
[00:03:37] Makefile:79: recipe for target 'tidy' failed
[00:03:37] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2c69bc28
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:103b1194:start=1530734878481067013,finish=1530734878490020612,duration=8953599
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:138573e4
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:21845564
$ dmesg | grep -i kill
