\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/resolve_self_super_hint.rs","byte_start":602,"byte_end":607,"line_start":20,"line_end":20,"column_start":21,"column_end":26,"is_primary":true,"text":[{"text":"                use alloc::HashMap;","highlight_start":21,"highlight_end":26}],"label":"unresolved import","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"a similar path exists","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/resolve_self_super_hint.rs","byte_start":602,"byte_end":607,"line_start":20,"line_end":20,"column_start":21,"column_end":26,"is_primary":true,"text":[{"text":"                use alloc::HashMap;","highlight_start":21,"highlight_end":26}],"label":null,"suggested_replacement":"a::alloc","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0432]: unresolved import `alloc`\n  --> /checkout/src/test/ui/resolve_self_super_hint.rs:20:21\n   |\nLL |                 use alloc::HashMap;\n   |                     ^^^^^\n   |                     |\n   |                     unresolved import\n   |                     help: a similar path exists: `a::alloc`\n\n"}
[01:15:23] {"message":"For more information about this error, try `rustc --explain E0432`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0432`.\n"}
[01:15:23] 
[01:15:23] ------------------------------------------
[01:15:23] 
---
[01:15:23] 7 note: lint level defined here
[01:15:23] -   --> $DIR/remove-extern-crate.rs:8:9
[01:15:23] +   --> $DIR/remove-extern-crate.rs:7:9
[01:15:23] 9    |
[01:15:23] 10 LL | #![warn(rust_2018_idioms)]
[01:15:23] 
[01:15:23] 
[01:15:23] 12    = note: #[warn(unused_extern_crates)] implied by #[warn(rust_2018_idioms)]
[01:15:23] 14 warning: `extern crate` is not idiomatic in the new edition
[01:15:23] -   --> $DIR/remove-extern-crate.rs:11:1
[01:15:23] +   --> $DIR/remove-extern-crate.rs:10:1
[01:15:23] 16    |
---
[01:15:23] 24    |     ^^^^^^^^^^^^^^^^^^ help: convert it to a `use`
[01:15:23] 
[01:15:23] 
[01:15:23] The actual stderr differed from the expected stderr.
[01:15:23] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/remove-extern-crate/remove-extern-crate.stderr
[01:15:23] To update references, rerun the tests and pass the `--bless` flag
[01:15:23] To only update this specific test, also pass `--test-args rust-2018/remove-extern-crate.rs`
[01:15:23] error: 1 errors occurred comparing output.
[01:15:23] status: exit code: 0
[01:15:23] status: exit code: 0
[01:15:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/remove-extern-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/remove-extern-crate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--extern" "remove_extern_crate" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/remove-extern-crate/auxiliary" "-A" "unused"
[01:15:23] ------------------------------------------
[01:15:23] 
[01:15:23] ------------------------------------------
[01:15:23] stderr:
[01:15:23] stderr:
[01:15:23] ------------------------------------------
[01:15:23] {"message":"unused extern crate","code":{"code":"unused_extern_crates","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/remove-extern-crate.rs","byte_start":158,"byte_end":176,"line_start":9,"line_end":9,"column_start":1,"column_end":19,"is_primary":true,"text":[{"text":"extern crate core;","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/remove-extern-crate.rs","byte_start":138,"byte_end":154,"line_start":7,"line_end":7,"column_start":9,"column_end":25,"is_primary":true,"text":[{"text":"#![warn(rust_2018_idioms)]","highlight_start":9,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"#[warn(unused_extern_crates)] implied by #[warn(rust_2018_idioms)]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove it","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/remove-extern-crate.rs","byte_start":158,"byte_end":176,"line_start":9,"line_end":9,"column_start":1,"column_end":19,"is_primary":true,"text":[{"text":"extern crate core;","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused extern crate\n  --> /checkout/src/test/ui/rust-2018/remove-extern-crate.rs:9:1\n   |\nLL | extern crate core;\n   | ^^^^^^^^^^^^^^^^^^ help: remove it\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/rust-2018/remove-extern-crate.rs:7:9\n   |\nLL | #![warn(rust_2018_idioms)]\n   |         ^^^^^^^^^^^^^^^^\n   = note: #[warn(unused_extern_crates)] implied by #[warn(rust_2018_idioms)]\n\n"}
[01:15:23] {"message":"`extern crate` is not idiomatic in the new edition","code":{"code":"unused_extern_crates","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/remove-extern-crate.rs","byte_start":177,"byte_end":211,"line_start":10,"line_end":10,"column_start":1,"column_end":35,"is_primary":true,"text":[{"text":"extern crate core as another_name;","highlight_start":1,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"convert it to a `use`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/remove-extern-crate.rs","byte_start":177,"byte_end":211,"line_start":10,"line_end":10,"column_start":1,"column_end":35,"is_primary":true,"text":[{"text":"extern crate core as another_name;","highlight_start":1,"highlight_end":35}],"label":null,"suggested_replacement":"use core as another_name;","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: `extern crate` is not idiomatic in the new edition\n  --> /checkout/src/test/ui/rust-2018/remove-extern-crate.rs:10:1\n   |\nLL | extern crate core as another_name;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: convert it to a `use`\n\n"}
[01:15:23] {"message":"`extern crate` is not idiomatic in the new edition","code":{"code":"unused_extern_crates","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/remove-extern-crate.rs","byte_start":581,"byte_end":599,"line_start":28,"line_end":28,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    extern crate core;","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"convert it to a `use`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/remove-extern-crate.rs","byte_start":581,"byte_end":599,"line_start":28,"line_end":28,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    extern crate core;","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":"use core;","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: `extern crate` is not idiomatic in the new edition\n  --> /checkout/src/test/ui/rust-2018/remove-extern-crate.rs:28:5\n   |\nLL |     extern crate core;\n   |     ^^^^^^^^^^^^^^^^^^ help: convert it to a `use`\n\n"}
[01:15:23] ------------------------------------------
[01:15:23] 
[01:15:23] thread '[ui] ui/rust-2018/remove-extern-crate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:15:23] 
---
[01:15:23] 
[01:15:23] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:15:23] 
[01:15:23] 
[01:15:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:23] 
[01:15:23] 
[01:15:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:23] Build completed unsuccessfully in 0:04:24
[01:15:23] Build completed unsuccessfully in 0:04:24
[01:15:23] make: *** [check] Error 1
[01:15:23] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17ea4658
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr  3 19:33:57 UTC 2019
---
travis_time:end:1012d79c:start=1554320038771387391,finish=1554320038776551106,duration=5163715
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:018bc910
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05b3c280
travis_time:start:05b3c280
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0daf452f
$ dmesg | grep -i kill
