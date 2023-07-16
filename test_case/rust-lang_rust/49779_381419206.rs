plain
[01:04:56] - warning: constant evaluation error
[01:04:56] + warning: attempt to subtract with overflow
[01:04:56] 2   --> $DIR/promoted_errors.rs:14:20
[01:04:56] 3    |
[01:04:56] 4 LL |     println!("{}", 0u32 - 1);
[01:04:56]
[01:04:56] -    |                    ^^^^^^^^ attempted to do overflowing math
[01:04:56] +    |                    ^^^^^^^^
[01:04:56] 6    |
[01:04:56] 7    = note: #[warn(const_err)] on by default
[01:04:56] 8
[01:04:56]
[01:04:56] - warning: constant evaluation error
[01:04:56] -   --> $DIR/promoted_errors.rs:14:20
[01:04:56] -    |
[01:04:56] - LL |     println!("{}", 0u32 - 1);
[01:04:56] -    |                    ^^^^^^^^ attempted to do overflowing math
[01:04:56] -
[01:04:56] - warning: constant evaluation error
[01:04:56] + warning: attempt to subtract with overflow
[01:04:56] 16   --> $DIR/promoted_errors.rs:17:14
[01:04:56] 17    |
[01:04:56] 18 LL |     let _x = 0u32 - 1;
[01:04:56]
[01:04:56] -    |              ^^^^^^^^ attempted to do overflowing math
[01:04:56] +    |              ^^^^^^^^
[01:04:56] 20
[01:04:56] 21 warning: attempt to divide by zero
[01:04:56] 22   --> $DIR/promoted_errors.rs:19:20
[01:04:56]
[01:04:56]
[01:04:56] The actual stderr differed from the expected stderr.
[01:04:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/promoted_errors.stderr
[01:04:56] To update references, run this command from build directory:
[01:04:56] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'const-eval/promoted_errors.rs'
[01:04:56]
[01:04:56] error: 1 errors occurred comparing output.
[01:04:56] status: exit code: 0
[01:04:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval/promoted_errors.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/promoted_errors.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/promoted_errors.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[01:04:56] {"message":"attempt to subtract with overflow","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/promoted_errors.rs","byte_start":515,"byte_end":523,"line_start":14,"line_end":14,"column_start":20,"column_end":28,"is_primary":true,"text":[{"text":"    println!(\"{}\", 0u32 - 1);","highlight_start":20,"highlight_end":28}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"#[warn(const_err)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: attempt to subtract with overflow\n  --> /checkout/src/test/ui/const-eval/promoted_errors.rs:14:20\n   |\nLL |     println!(\"{}\", 0u32 - 1);\n   |                    ^^^^^^^^\n   |\n   = note: #[warn(const_err)] on by default\n\n"}
[01:04:56] {"message":"attempt to subtract with overflow","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/promoted_errors.rs","byte_start":587,"byte_end":595,"line_start":17,"line_end":17,"column_start":14,"column_end":22,"is_primary":true,"text":[{"text":"    let _x = 0u32 - 1;","highlight_start":14,"highlight_end":22}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: attempt to subtract with overflow\n  --> /checkout/src/test/ui/const-eval/promoted_errors.rs:17:14\n   |\nLL |     let _x = 0u32 - 1;\n   |              ^^^^^^^^\n\n"}
[01:04:56] {"message":"attempt to divide by zero","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/promoted_errors.rs","byte_start":640,"byte_end":647,"line_start":19,"line_end":19,"column_start":20,"column_end":27,"is_primary":true,"text":[{"text":"    println!(\"{}\", 1/(1-1));","highlight_start":20,"highlight_end":27}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: attempt to divide by zero\n  --> /checkout/src/test/ui/const-eval/promoted_errors.rs:19:20\n   |\nLL |     println!(\"{}\", 1/(1-1));\n   |                    ^^^^^^^\n\n"}
[01:04:56] {"message":"constant evaluation error","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/promoted_errors.rs","byte_start":640,"byte_end":647,"line_start":19,"line_end":19,"column_start":20,"column_end":27,"is_primary":true,"text":[{"text":"    println!(\"{}\", 1/(1-1));","highlight_start":20,"highlight_end":27}],"label":"attempted to do overflowing math","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: constant evaluation error\n  --> /checkout/src/test/ui/const-eval/promoted_errors.rs:19:20\n   |\nLL |     println!(\"{}\", 1/(1-1));\n   |                    ^^^^^^^ attempted to do overflowing math\n\n"}
[01:04:56] {"message":"attempt to divide by zero","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/promoted_errors.rs","byte_start":711,"byte_end":718,"line_start":22,"line_end":22,"column_start":14,"column_end":21,"is_primary":true,"text":[{"text":"    let _x = 1/(1-1);","highlight_start":14,"highlight_end":21}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: attempt to divide by zero\n  --> /checkout/src/test/ui/const-eval/promoted_errors.rs:22:14\n   |\nLL |     let _x = 1/(1-1);\n   |              ^^^^^^^\n\n"}
[01:04:56] {"message":"constant evaluation error","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/promoted_errors.rs","byte_start":711,"byte_end":718,"line_start":22,"line_end":22,"column_start":14,"column_end":21,"is_primary":true,"text":[{"text":"    let _x = 1/(1-1);","highlight_start":14,"highlight_end":21}],"label":"attempted to do overflowing math","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: constant evaluation error\n  --> /checkout/src/test/ui/const-eval/promoted_errors.rs:22:14\n   |\nLL |     let _x = 1/(1-1);\n   |              ^^^^^^^ attempted to do overflowing math\n\n"}
[01:04:56] {"message":"constant evaluation error","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/promoted_errors.rs","byte_start":787,"byte_end":803,"line_start":25,"line_end":25,"column_start":20,"column_end":36,"is_primary":true,"text":[{"text":"    println!(\"{}\", 1/(false as u32));","highlight_start":20,"highlight_end":36}],"label":"attempted to do overflowing math","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: constant evaluation error\n  --> /checkout/src/test/ui/const-eval/promoted_errors.rs:25:20\n   |\nLL |     println!(\"{}\", 1/(false as u32));\n   |                    ^^^^^^^^^^^^^^^^ attempted to do overflowing math\n\n"}
---
[01:04:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options " "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "6.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:56] expected success, got: exit code: 101
[01:04:56]
[01:04:56]
[01:04:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:56] Build completed unsuccessfully in 0:02:16
[01:04:56] Makefile:58: recipe for target 'check' failed
[01:04:56] make: *** [check] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0567e25e:start=1523809675127277837,finish=1523809675140897733,duration=13619896
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0b1707b2
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0b1707b2:start=1523809675151427221,finish=1523809675158667600,duration=7240379
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04ce05f4
$ dmesg | grep -i kill
[   10.789765] init: failsafe main process (1092) killed by TERM signal
