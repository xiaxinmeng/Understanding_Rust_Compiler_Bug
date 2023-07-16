\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/issue-14021.rs","byte_start":647,"byte_end":681,"line_start":16,"line_end":16,"column_start":35,"column_end":69,"is_primary":true,"text":[{"text":"extern crate rustc_ezilaires; use rustc_ezilaires as rustc_serialize;","highlight_start":35,"highlight_end":69}],"label":"`rustc_serialize` reimported here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/run-pass-fulldeps/issue-14021.rs","byte_start":577,"byte_end":611,"line_start":15,"line_end":15,"column_start":35,"column_end":69,"is_primary":false,"text":[{"text":"extern crate rustc_ezilaires; use rustc_ezilaires as rustc_serialize;","highlight_start":35,"highlight_end":69}],"label":"previous import of the module `rustc_serialize` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`rustc_serialize` must be defined only once in the type namespace of this module","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use `as` to change the binding name of the import","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/issue-14021.rs","byte_start":647,"byte_end":681,"line_start":16,"line_end":16,"column_start":35,"column_end":69,"is_primary":true,"text":[{"text":"extern crate rustc_ezilaires; use rustc_ezilaires as rustc_serialize;","highlight_start":35,"highlight_end":69}],"label":null,"suggested_replacement":"rustc_ezilaires as other_rustc_serialize","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0252]: the name `rustc_serialize` is defined multiple times\n  --> /checkout/src/test/run-pass-fulldeps/issue-14021.rs:16:35\n   |\nLL | extern crate rustc_ezilaires; use rustc_ezilaires as rustc_serialize;\n   |                                   ---------------------------------- previous import of the module `rustc_serialize` here\nLL | extern crate rustc_ezilaires; use rustc_ezilaires as rustc_serialize;\n   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `rustc_serialize` reimported here\n   |\n   = note: `rustc_serialize` must be defined only once in the type namespace of this module\nhelp: you can use `as` to change the binding name of the import\n   |\nLL | extern crate rustc_ezilaires; use rustc_ezilaires as other_rustc_serialize;\n   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:14:20] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:14:20] {"message":"Some errors occurred: E0252, E0259.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0252, E0259.\n"}
[01:14:20] {"message":"For more information about an error, try `rustc --explain E0252`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0252`.\n"}
[01:14:20] ------------------------------------------
[01:14:20] 
[01:14:20] thread '[run-pass] run-pass-fulldeps/issue-14021.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:14:20] 
[01:14:20] 
[01:14:20] ---- [run-pass] run-pass-fulldeps/issue-24972.rs stdout ----
[01:14:20] normalized stderr:
[01:14:20] warning: unused import: `rustc_ezilaires as rustc_serialize`
[01:14:20]    |
[01:14:20]    |
[01:14:20] LL | extern crate rustc_ezilaires; use rustc_ezilaires as rustc_serialize;
[01:14:20]    |
[01:14:20]    = note: #[warn(unused_imports)] on by default
[01:14:20] 
[01:14:20] 
[01:14:20] 
[01:14:20] 
[01:14:20] 
[01:14:20] The actual stderr differed from the expected stderr.
[01:14:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-24972/issue-24972.stderr
[01:14:20] To update references, rerun the tests and pass the `--bless` flag
[01:14:20] To only update this specific test, also pass `--test-args issue-24972.rs`
[01:14:20] error: 1 errors occurred comparing output.
[01:14:20] status: exit code: 0
[01:14:20] status: exit code: 0
[01:14:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/issue-24972.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-24972/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-24972/auxiliary"
[01:14:20] ------------------------------------------
[01:14:20] 
[01:14:20] ------------------------------------------
[01:14:20] stderr:
[01:14:20] stderr:
[01:14:20] ------------------------------------------
[01:14:20] {"message":"unused import: `rustc_ezilaires as rustc_serialize`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/issue-24972.rs","byte_start":550,"byte_end":584,"line_start":14,"line_end":14,"column_start":35,"column_end":69,"is_primary":true,"text":[{"text":"extern crate rustc_ezilaires; use rustc_ezilaires as rustc_serialize;","highlight_start":35,"highlight_end":69}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `rustc_ezilaires as rustc_serialize`\n  --> /checkout/src/test/run-pass-fulldeps/issue-24972.rs:14:35\n   |\nLL | extern crate rustc_ezilaires; use rustc_ezilaires as rustc_serialize;\n   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:14:20] ------------------------------------------
[01:14:20] 
[01:14:20] thread '[run-pass] run-pass-fulldeps/issue-24972.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:14:20] 
[01:14:20] 
[01:14:20] ---- [run-pass] run-pass-fulldeps/issue-2804.rs stdout ----
[01:14:20] normalized stderr:
[01:14:20] warning: unused import: `rustc_ezilaires as rustc_serialize`
[01:14:20]    |
[01:14:20]    |
[01:14:20] LL | extern crate rustc_ezilaires; use rustc_ezilaires as rustc_serialize;
[01:14:20]    |
[01:14:20]    = note: #[warn(unused_imports)] on by default
[01:14:20] 
[01:14:20] 
[01:14:20] 
[01:14:20] 
[01:14:20] 
[01:14:20] The actual stderr differed from the expected stderr.
[01:14:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-2804/issue-2804.stderr
[01:14:20] To update references, rerun the tests and pass the `--bless` flag
[01:14:20] To only update this specific test, also pass `--test-args issue-2804.rs`
[01:14:20] error: 1 errors occurred comparing output.
[01:14:20] status: exit code: 0
[01:14:20] status: exit code: 0
[01:14:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/issue-2804.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-2804/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-2804/auxiliary"
[01:14:20] ------------------------------------------
[01:14:20] 
[01:14:20] ------------------------------------------
[01:14:20] stderr:
[01:14:20] stderr:
[01:14:20] ------------------------------------------
[01:14:20] {"message":"unused import: `rustc_ezilaires as rustc_serialize`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/issue-2804.rs","byte_start":587,"byte_end":621,"line_start":15,"line_end":15,"column_start":35,"column_end":69,"is_primary":true,"text":[{"text":"extern crate rustc_ezilaires; use rustc_ezilaires as rustc_serialize;","highlight_start":35,"highlight_end":69}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `rustc_ezilaires as rustc_serialize`\n  --> /checkout/src/test/run-pass-fulldeps/issue-2804.rs:15:35\n   |\nLL | extern crate rustc_ezilaires; use rustc_ezilaires as rustc_serialize;\n   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:14:20] ------------------------------------------
[01:14:20] 
[01:14:20] thread '[run-pass] run-pass-fulldeps/issue-2804.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:14:20] 
[01:14:20] 
[01:14:20] ---- [run-pass] run-pass-fulldeps/issue-4016.rs stdout ----
[01:14:20] normalized stderr:
[01:14:20] warning: unused import: `rustc_ezilaires as rustc_serialize`
[01:14:20]    |
[01:14:20]    |
[01:14:20] LL | extern crate rustc_ezilaires; use rustc_ezilaires as rustc_serialize;
[01:14:20]    |
[01:14:20]    = note: #[warn(unused_imports)] on by default
[01:14:20] 
[01:14:20] 
[01:14:20] 
[01:14:20] 
[01:14:20] 
[01:14:20] The actual stderr differed from the expected stderr.
[01:14:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-4016/issue-4016.stderr
[01:14:20] To update references, rerun the tests and pass the `--bless` flag
[01:14:20] To only update this specific test, also pass `--test-args issue-4016.rs`
[01:14:20] error: 1 errors occurred comparing output.
[01:14:20] status: exit code: 0
[01:14:20] status: exit code: 0
[01:14:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/issue-4016.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-4016/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-4016/auxiliary"
[01:14:20] ------------------------------------------
[01:14:20] 
[01:14:20] ------------------------------------------
[01:14:20] stderr:
[01:14:20] stderr:
[01:14:20] ------------------------------------------
[01:14:20] {"message":"unused import: `rustc_ezilaires as rustc_serialize`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/issue-4016.rs","byte_start":556,"byte_end":590,"line_start":15,"line_end":15,"column_start":35,"column_end":69,"is_primary":true,"text":[{"text":"extern crate rustc_ezilaires; use rustc_ezilaires as rustc_serialize;","highlight_start":35,"highlight_end":69}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `rustc_ezilaires as rustc_serialize`\n  --> /checkout/src/test/run-pass-fulldeps/issue-4016.rs:15:35\n   |\nLL | extern crate rustc_ezilaires; use rustc_ezilaires as rustc_serialize;\n   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:14:20] ------------------------------------------
[01:14:20] 
[01:14:20] thread '[run-pass] run-pass-fulldeps/issue-4016.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:14:20] 
[01:14:20] 
[01:14:20] ---- [run-pass] run-pass-fulldeps/issue-4036.rs stdout ----
[01:14:20] normalized stderr:
[01:14:20] warning: unused import: `rustc_ezilaires as rustc_serialize`
[01:14:20]    |
[01:14:20]    |
[01:14:20] LL | extern crate rustc_ezilaires; use rustc_ezilaires as rustc_serialize;
[01:14:20]    |
[01:14:20]    = note: #[warn(unused_imports)] on by default
[01:14:20] 
[01:14:20] 
[01:14:20] 
[01:14:20] 
[01:14:20] 
[01:14:20] The actual stderr differed from the expected stderr.
[01:14:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-4036/issue-4036.stderr
[01:14:20] To update references, rerun the tests and pass the `--bless` flag
[01:14:20] To only update this specific test, also pass `--test-args issue-4036.rs`
[01:14:20] error: 1 errors occurred comparing output.
[01:14:20] status: exit code: 0
[01:14:20] status: exit code: 0
[01:14:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/issue-4036.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-4036/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-4036/auxiliary"
[01:14:20] ------------------------------------------
[01:14:20] 
[01:14:20] ------------------------------------------
[01:14:20] stderr:
[01:14:20] stderr:
[01:14:20] ------------------------------------------
[01:14:20] {"message":"unused import: `rustc_ezilaires as rustc_serialize`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/issue-4036.rs","byte_start":679,"byte_end":713,"line_start":19,"line_end":19,"column_start":35,"column_end":69,"is_primary":true,"text":[{"text":"extern crate rustc_ezilaires; use rustc_ezilaires as rustc_serialize;","highlight_start":35,"highlight_end":69}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `rustc_ezilaires as rustc_serialize`\n  --> /checkout/src/test/run-pass-fulldeps/issue-4036.rs:19:35\n   |\nLL | extern crate rustc_ezilaires; use rustc_ezilaires as rustc_serialize;\n   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:14:20] ------------------------------------------
[01:14:20] 
[01:14:20] thread '[run-pass] run-pass-fulldeps/issue-4036.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:14:20] 
---
[01:14:20] test result: FAILED. 87 passed; 10 failed; 0 ignored; 0 measured; 0 filtered out
[01:14:20] 
[01:14:20] 
[01:14:20] 
[01:14:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:20] 
[01:14:20] 
[01:14:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:20] Build completed unsuccessfully in 0:26:51
[01:14:20] Build completed unsuccessfully in 0:26:51
[01:14:20] make: *** [check] Error 1
[01:14:20] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:257f85f3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Nov 29 09:35:53 UTC 2018
