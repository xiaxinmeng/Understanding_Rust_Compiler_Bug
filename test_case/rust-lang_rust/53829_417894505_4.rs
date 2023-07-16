\n"},"level":"error","spans":[{"file_name":"libcore/hash/mod.rs","byte_start":4730,"byte_end":4731,"line_start":185,"line_end":185,"column_start":13,"column_end":14,"is_primary":false,"text":[{"text":"","highlight_start":13,"highlight_end":14}],"label":"declaration in trait here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs","byte_start":967,"byte_end":978,"line_start":38,"line_end":38,"column_start":33,"column_end":44,"is_primary":true,"text":[{"text":"    fn hash(&self, hasher: &mut impl Hasher) {}","highlight_start":33,"highlight_end":44}],"label":"expected generic parameter, found `impl Trait`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0643]: method `hash` has incompatible signature for trait\n  --> /checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs:38:33\n   |\nLL |     fn hash(&self, hasher: &mut impl Hasher) {}\n   |                                 ^^^^^^^^^^^ expected generic parameter, found `impl Trait`\n\n"}
[00:46:20] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:46:20] {"message":"For more information about this error, try `rustc --explain E0643`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0643`.\n"}
[00:46:20] ------------------------------------------
[00:46:20] 
[00:46:20] thread '[ui] ui/impl-trait/impl-generic-mismatch.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:46:20] 
---
[00:46:20] 
[00:46:20] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:46:20] 
[00:46:20] 
[00:46:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-musl" "--mode" "ui" "--target" "x86_64-unknown-linux-musl" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/musl-x86_64/bin/musl-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:20] 
[00:46:20] 
[00:46:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target x86_64-unknown-linux-musl
[00:46:20] Build completed unsuccessfully in 0:42:56
---
travis_time:end:1d8a573d:start=1535844752093607353,finish=1535844752100785664,duration=7178311
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c5e395c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00dcb720
travis_time:start:00dcb720
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:279550fb
$ dmesg | grep -i kill
