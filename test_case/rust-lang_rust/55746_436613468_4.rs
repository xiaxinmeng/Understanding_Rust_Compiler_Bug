\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/extern/extern-const.rs","byte_start":490,"byte_end":495,"line_start":15,"line_end":15,"column_start":38,"column_end":43,"is_primary":true,"text":[{"text":"    const rust_dbg_static_mut: libc::c_int; //~ ERROR extern items cannot be `const`","highlight_start":38,"highlight_end":43}],"label":"not found in `libc`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"possible candidate is found in another module, you can import it into scope","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/extern/extern-const.rs","byte_start":387,"byte_end":387,"line_start":13,"line_end":13,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"#[link(name = \"rust_test_helpers\", kind = \"static\")]","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use std::os::raw::c_int;\n\n","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0412]: cannot find type `c_int` in module `libc`\n  --> /checkout/src/test/ui/extern/extern-const.rs:15:38\n   |\nLL |     const rust_dbg_static_mut: libc::c_int; //~ ERROR extern items cannot be `const`\n   |                                      ^^^^^ not found in `libc`\nhelp: possible candidate is found in another module, you can import it into scope\n   |\nLL | use std::os::raw::c_int;\n   |\n\n"}
[00:54:47] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:54:47] {"message":"For more information about this error, try `rustc --explain E0412`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0412`.\n"}
[00:54:47] ------------------------------------------
[00:54:47] 
[00:54:47] thread '[ui] ui/extern/extern-const.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:54:47] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:54:47] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:54:47] 
[00:54:47] 
[00:54:47] 
[00:54:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:54:47] 
[00:54:47] 
[00:54:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
[00:54:47] Build completed unsuccessfully in 0:52:03
---
travis_time:end:169d428e:start=1541595122978017233,finish=1541595122985108328,duration=7091095
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2bc2e868
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:22f8fee0
travis_time:start:22f8fee0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08dfbb2a
$ dmesg | grep -i kill
