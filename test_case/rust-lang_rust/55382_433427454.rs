plain
[00:46:09] 
[00:46:09] ---- [ui] ui/consts/dangling-alloc-id-ice.rs stdout ----
[00:46:09] diff of stderr:
[00:46:09] 
[00:46:09] - error: this constant cannot be used
[00:46:09] + error: any use of this value will cause an error
[00:46:09] 2   --> $DIR/dangling-alloc-id-ice.rs:10:1
[00:46:09] 3    |
[00:46:09] 4 LL | / const FOO: &() = { //~ ERROR this constant cannot be used
[00:46:09] 
[00:46:09] The actual stderr differed from the expected stderr.
[00:46:09] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/dangling-alloc-id-ice/dangling-alloc-id-ice.stderr
[00:46:09] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/dangling-alloc-id-ice/dangling-alloc-id-ice.stderr
[00:46:09] To update references, rerun the tests and pass the `--bless` flag
[00:46:09] To only update this specific test, also pass `--test-args consts/dangling-alloc-id-ice.rs`
[00:46:09] error: 1 errors occurred comparing output.
[00:46:09] status: exit code: 1
[00:46:09] status: exit code: 1
[00:46:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/dangling-alloc-id-ice.rs" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/dangling-alloc-id-ice/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/dangling-alloc-id-ice/auxiliary" "-A" "unused"
[00:46:09] ------------------------------------------
[00:46:09] 
[00:46:09] ------------------------------------------
[00:46:09] stderr:
[00:46:09] stderr:
[00:46:09] ------------------------------------------
[00:46:09] {"message":"any use of this value will cause an error","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/dangling-alloc-id-ice.rs","byte_start":146,"byte_end":270,"line_start":10,"line_end":13,"column_start":1,"column_end":3,"is_primary":true,"text":[{"text":"const FOO: &() = { //~ ERROR this constant cannot be used","highlight_start":1,"highlight_end":58},{"text":"    let y = ();","highlight_start":1,"highlight_end":16},{"text":"    unsafe { Foo { y: &y }.long_live_the_unit }","highlight_start":1,"highlight_end":48},{"text":"};","highlight_start":1,"highlight_end":3}],"label":"type validation failed: encountered dangling pointer in final constant","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[deny(const_err)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: any use of this value will cause an error\n  --> /checkout/src/test/ui/consts/dangling-alloc-id-ice.rs:10:1\n   |\nLL | / const FOO: &() = { //~ ERROR this constant cannot be used\nLL | |     let y = ();\nLL | |     unsafe { Foo { y: &y }.long_live_the_unit }\nLL | | };\n   | |__^ type validation failed: encountered dangling pointer in final constant\n   |\n   = note: #[deny(const_err)] on by default\n\n"}
[00:46:09] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:46:09] ------------------------------------------
[00:46:09] 
[00:46:09] thread '[ui] ui/consts/dangling-alloc-id-ice.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:46:09] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:46:09] 
[00:46:09] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:46:09] 
[00:46:09] 
[00:46:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:09] 
[00:46:09] 
[00:46:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:46:09] Build completed unsuccessfully in 0:43:11
---
travis_time:end:2a55f625:start=1540564201251380452,finish=1540564201265037890,duration=13657438
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07698a6a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:028d62f4
travis_time:start:028d62f4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09da39c0
$ dmesg | grep -i kill
