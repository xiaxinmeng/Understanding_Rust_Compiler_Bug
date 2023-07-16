plain
[00:56:21] 
[00:56:21] ---- [ui] ui/wasm-custom-section-relocations.rs stdout ----
[00:56:21] diff of stderr:
[00:56:21] 
[00:56:21] 1 error: statics with a custom `#[link_section]` must be a simple list of bytes on the wasm target with no extra levels of indirection such as references
[00:56:21] +   --> $DIR/wasm-custom-section-relocations.rs:4:1
[00:56:21] 3    |
[00:56:21] 3    |
[00:56:21] 4 LL | pub static A: &[u8] = &[1]; //~ ERROR: no extra levels of indirection
[00:56:21] 
[00:56:21] 6 
[00:56:21] 6 
[00:56:21] 7 error: statics with a custom `#[link_section]` must be a simple list of bytes on the wasm target with no extra levels of indirection such as references
[00:56:21] +   --> $DIR/wasm-custom-section-relocations.rs:13:1
[00:56:21] 9    |
[00:56:21] 9    |
[00:56:21] 10 LL | pub static D: &usize = &C; //~ ERROR: no extra levels of indirection
[00:56:21] 
[00:56:21] 
[00:56:21] The actual stderr differed from the expected stderr.
[00:56:21] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wasm-custom-section-relocations/wasm-custom-section-relocations.stderr
[00:56:21] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wasm-custom-section-relocations/wasm-custom-section-relocations.stderr
[00:56:21] To update references, rerun the tests and pass the `--bless` flag
[00:56:21] To only update this specific test, also pass `--test-args wasm-custom-section-relocations.rs`
[00:56:21] error: 1 errors occurred comparing output.
[00:56:21] status: exit code: 1
[00:56:21] status: exit code: 1
[00:56:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/wasm-custom-section-relocations.rs" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wasm-custom-section-relocations/a.wasm" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wasm-custom-section-relocations/auxiliary" "-A" "unused"
[00:56:21] ------------------------------------------
[00:56:21] 
[00:56:21] ------------------------------------------
[00:56:21] stderr:
[00:56:21] stderr:
[00:56:21] ------------------------------------------
[00:56:21] {"message":"statics with a custom `#[link_section]` must be a simple list of bytes on the wasm target with no extra levels of indirection such as references","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/wasm-custom-section-relocations.rs","byte_start":41,"byte_end":68,"line_start":4,"line_end":4,"column_start":1,"column_end":28,"is_primary":true,"text":[{"text":"pub static A: &[u8] = &[1]; //~ ERROR: no extra levels of indirection","highlight_start":1,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statics with a custom `#[link_section]` must be a simple list of bytes on the wasm target with no extra levels of indirection such as references\n  --> /checkout/src/test/ui/wasm-custom-section-relocations.rs:4:1\n   |\nLL | pub static A: &[u8] = &[1]; //~ ERROR: no extra levels of indirection\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:56:21] {"message":"statics with a custom `#[link_section]` must be a simple list of bytes on the wasm target with no extra levels of indirection such as references","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/wasm-custom-section-relocations.rs","byte_start":249,"byte_end":275,"line_start":13,"line_end":13,"column_start":1,"column_end":27,"is_primary":true,"text":[{"text":"pub static D: &usize = &C; //~ ERROR: no extra levels of indirection","highlight_start":1,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: statics with a custom `#[link_section]` must be a simple list of bytes on the wasm target with no extra levels of indirection such as references\n  --> /checkout/src/test/ui/wasm-custom-section-relocations.rs:13:1\n   |\nLL | pub static D: &usize = &C; //~ ERROR: no extra levels of indirection\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:56:21] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:56:21] ------------------------------------------
[00:56:21] 
[00:56:21] thread '[ui] ui/wasm-custom-section-relocations.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[00:56:21] 
---
[00:56:21] 
[00:56:21] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[00:56:22] 
[00:56:22] 
[00:56:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:22] 
[00:56:22] 
[00:56:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
[00:56:22] Build completed unsuccessfully in 0:54:19
---
travis_time:end:0732eaee:start=1545706948747789240,finish=1545706948755893214,duration=8103974
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:25296dec
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0708d88d
travis_time:start:0708d88d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0997e0fc
$ dmesg | grep -i kill
