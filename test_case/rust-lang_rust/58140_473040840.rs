plain
[01:02:44] 
[01:02:44] ---- [ui] ui/type_length_limit.rs stdout ----
[01:02:44] diff of stderr:
[01:02:44] 
[01:02:44] 1 error: reached the type-length limit while instantiating `std::mem::drop::<std::option::Op... G), (G, G, G), (G, G, G))))))>>`
[01:02:44] -   --> $SRC_DIR/libcore/mem.rs:LL:COL
[01:02:44] -    |
[01:02:44] - LL | pub fn drop<T>(_x: T) { }
[01:02:44] 6    |
[01:02:44] 6    |
[01:02:44] 7    = note: consider adding a `#![type_length_limit="1094"]` attribute to your crate
[01:02:44] 
[01:02:44] 
[01:02:44] The actual stderr differed from the expected stderr.
[01:02:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/type_length_limit.stderr
[01:02:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/type_length_limit.stderr
[01:02:44] To update references, rerun the tests and pass the `--bless` flag
[01:02:44] To only update this specific test, also pass `--test-args type_length_limit.rs`
[01:02:44] error: 1 errors occurred comparing output.
[01:02:44] status: exit code: 1
[01:02:44] status: exit code: 1
[01:02:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type_length_limit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-x86_64/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/auxiliary" "-A" "unused"
[01:02:44] ------------------------------------------
[01:02:44] 
[01:02:44] ------------------------------------------
[01:02:44] stderr:
[01:02:44] stderr:
[01:02:44] ------------------------------------------
[01:02:44] {"message":"reached the type-length limit while instantiating `std::mem::drop::<std::option::Op... G), (G, G, G), (G, G, G))))))>>`","code":null,"level":"error","spans":[{"file_name":"/rustc/71d6e76d687f70e7c7b05f80a330dc555b3e1b47/src/libcore/mem.rs","byte_start":24455,"byte_end":24480,"line_start":777,"line_end":777,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider adding a `#![type_length_limit=\"1094\"]` attribute to your crate","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: reached the type-length limit while instantiating `std::mem::drop::<std::option::Op... G), (G, G, G), (G, G, G))))))>>`\n   |\n   = note: consider adding a `#![type_length_limit=\"1094\"]` attribute to your crate\n\n"}
[01:02:44] 
[01:02:44] ------------------------------------------
[01:02:44] 
[01:02:44] thread '[ui] ui/type_length_limit.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
---
[01:02:44] 
[01:02:44] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:02:44] 
[01:02:44] 
[01:02:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-musl" "--mode" "ui" "--target" "x86_64-unknown-linux-musl" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/musl-x86_64/bin/musl-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:44] 
[01:02:44] 
[01:02:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target x86_64-unknown-linux-musl
[01:02:44] Build completed unsuccessfully in 0:59:23
---
travis_time:end:04094a12:start=1552594174521738601,finish=1552594174542570367,duration=20831766
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01a63e46
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04072e78
travis_time:start:04072e78
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1191ea5a
$ dmesg | grep -i kill
