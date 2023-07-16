plain
[00:57:26] - error[E0723]: function pointers in const fn are unstable (see issue #57563)
[00:57:26] + error[E0723]: function pointers in const fn are unstable
[00:57:26] 2   --> $DIR/allow_const_fn_ptr.rs:4:16
[00:57:26] 3    |
[00:57:26] 4 LL | const fn error(_: fn()) {}
[00:57:26] 5    |                ^
[00:57:26] 6    |
[00:57:26] +    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[00:57:26] 7    = help: add #![feature(const_fn)] to the crate attributes to enable
[00:57:26] 7    = help: add #![feature(const_fn)] to the crate attributes to enable
[00:57:26] 8 
[00:57:26] 9 error: aborting due to previous error
[00:57:26] 
[00:57:26] 
[00:57:26] The actual stderr differed from the expected stderr.
[00:57:26] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/allow_const_fn_ptr/allow_const_fn_ptr.stderr
[00:57:26] To update references, rerun the tests and pass the `--bless` flag
[00:57:26] To only update this specific test, also pass `--test-args consts/min_const_fn/allow_const_fn_ptr.rs`
[00:57:26] error: 1 errors occurred comparing output.
[00:57:26] status: exit code: 1
[00:57:26] status: exit code: 1
[00:57:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/allow_const_fn_ptr.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/allow_const_fn_ptr/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/allow_const_fn_ptr/auxiliary" "-A" "unused"
[00:57:26] ------------------------------------------
[00:57:26] 
[00:57:26] ------------------------------------------
[00:57:26] stderr:
[00:57:26] stderr:
[00:57:26] ------------------------------------------
[00:57:26] error[E0723]: function pointers in const fn are unstable
[00:57:26]   --> /checkout/src/test/ui/consts/min_const_fn/allow_const_fn_ptr.rs:4:16
[00:57:26]    |
[00:57:26] LL | const fn error(_: fn()) {} //~ ERROR function pointers in const fn are unstable
[00:57:26]    |
[00:57:26]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[00:57:26]    = help: add #![feature(const_fn)] to the crate attributes to enable
[00:57:26] 
---
[00:57:26] test result: FAILED. 5429 passed; 1 failed; 33 ignored; 0 measured; 0 filtered out
[00:57:26] 
[00:57:26] 
[00:57:26] 
[00:57:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.36.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:26] 
[00:57:26] 
[00:57:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:57:26] Build completed unsuccessfully in 0:53:51
---
travis_time:end:0e287610:start=1556048950108511603,finish=1556048950127415901,duration=18904298
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:021988a0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ec012ce
travis_time:start:0ec012ce
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:009eb96a
$ dmesg | grep -i kill
