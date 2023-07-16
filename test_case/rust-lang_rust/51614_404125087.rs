plain
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2d/99/b2c4e9d5a30f6471e410a146232b4118e697fa3ffc06d6a65efde84debd0/futures-3.2.0-py2-none-any.whl
Requirement already satisfied: six>=1.5 in /usr/lib/python2.7/dist-packages (from python-dateutil<3.0.0,>=2.1; python_version >= "2.7"->botocore==1.10.55->awscli)
Building wheels for collected packages: awscli
  Running setup.py bdist_wheel for awscli ... - \ | / - \ done
Successfully built awscli
Installing collected packages: docutils, jmespath, python-dateutil, botocore, colorama, pyasn1, rsa, futures, s3transfer, awscli
Successfully installed awscli-1.15.56 botocore-1.10.55 colorama-0.3.9 docutils-0.14 futures-3.2.0 jmespath-0.9.3 pyasn1-0.4.3 python-dateutil-2.7.3 rsa-3.4.2 s3transfer-0.1.13
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
---
[00:44:38] ....................................................................................................
[00:44:41] ....................................................................................................
[00:44:44] ....................................................................................................
[00:44:48] ....................................................................................................
[00:44:51] ..................................................................................F.................
[00:44:58] ........................i...........................................................................
[00:45:02] ....................................................................................................
[00:45:05] ....................................................................................................
[00:45:09] .......................................................................i............................
[00:45:09] .......................................................................i............................
[00:45:11] .........................................................
[00:45:11] failures:
[00:45:11] 
[00:45:11] ---- [ui] ui/macros/bad_hello.rs stdout ----
[00:45:11] diff of stderr:
[00:45:11] 
[00:45:11] 8 LL |     println!("{}", 3 + 4); //~ ERROR expected a literal
[00:45:11] 10 
[00:45:11] + error: aborting due to previous error
[00:45:11] + 
[00:45:11] 11 
[00:45:11] 11 
[00:45:11] 
[00:45:11] 
[00:45:11] The actual stderr differed from the expected stderr.
[00:45:11] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/bad_hello/bad_hello.stderr
[00:45:11] To update references, rerun the tests and pass the `--bless` flag
[00:45:11] To only update this specific test, also pass `--test-args macros/bad_hello.rs`
[00:45:11] error: 1 errors occurred comparing output.
[00:45:11] status: exit code: 101
[00:45:11] status: exit code: 101
[00:45:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/bad_hello.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/bad_hello/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/bad_hello/auxiliary" "-A" "unused"
[00:45:11] ------------------------------------------
[00:45:11] 
[00:45:11] ------------------------------------------
[00:45:11] stderr:
[00:45:11] stderr:
[00:45:11] ------------------------------------------
[00:45:11] {"message":"expected a literal","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/macros/bad_hello.rs","byte_start":492,"byte_end":497,"line_start":12,"line_end":12,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"    println!(3 + 4); //~ ERROR expected a literal","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you might be missing a string literal to format with","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/macros/bad_hello.rs","byte_start":492,"byte_end":497,"line_start":12,"line_end":12,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"    println!(3 + 4); //~ ERROR expected a literal","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":"\"{}\", 3 + 4","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: expected a literal\n  --> /checkout/src/test/ui/macros/bad_hello.rs:12:14\n   |\nLL |     println!(3 + 4); //~ ERROR expected a literal\n   |              ^^^^^\nhelp: you might be missing a string literal to format with\n   |\nLL |     println!(\"{}\", 3 + 4); //~ ERROR expected a literal\n   |              ^^^^^^^^^^^\n\n"}
[00:45:11] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:11] ------------------------------------------
[00:45:11] 
[00:45:11] thread '[ui] ui/macros/bad_hello.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:45:11] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:45:11] 
[00:45:11] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:45:11] 
[00:45:11] 
[00:45:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:45:11] 
[00:45:11] 
[00:45:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:45:11] Build completed unsuccessfully in 0:01:26
[00:45:11] Build completed unsuccessfully in 0:01:26
[00:45:11] make: *** [check] Error 1
[00:45:11] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01dbbff3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
