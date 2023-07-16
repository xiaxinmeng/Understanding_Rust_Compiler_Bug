plain
travis_time:end:03827787:start=1552698920542965218,finish=1552698998730287810,duration=78187322592
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:17:41] .................................................................................................... 4400/5472
[01:17:44] .................................................................................................... 4500/5472
[01:17:48] .................................................................................................... 4600/5472
[01:17:53] ..............i..................................................................................... 4700/5472
[01:17:59] ..............................F..................................................................... 4800/5472
[01:18:07] .................................................................................................... 5000/5472
[01:18:10] .................................................................................................... 5100/5472
[01:18:15] .................................................................................................... 5200/5472
[01:18:18] .................................................................................................... 5300/5472
---
[01:18:23] 
[01:18:23] 1 error: cannot use a built-in attribute through an import
[01:18:23] 2   --> $DIR/cross-crate.rs:7:3
[01:18:23] 3    |
[01:18:23] - LL | #[built_in_attr] //~ ERROR cannot use a built-in attribute through an import
[01:18:23] + LL | #[built_in_attr]
[01:18:23] 6    |
[01:18:23] 7 note: the built-in attribute imported here
[01:18:23] 
[01:18:23] 13 error: cannot use a tool module through an import
[01:18:23] 13 error: cannot use a tool module through an import
[01:18:23] 14   --> $DIR/cross-crate.rs:8:3
[01:18:23] 15    |
[01:18:23] - LL | #[tool_mod::skip] //~ ERROR cannot use a tool module through an import
[01:18:23] + LL | #[tool_mod::skip]
[01:18:23] 18    |
[01:18:23] 19 note: the tool module imported here
[01:18:23] 
[01:18:23] 
[01:18:23] 
[01:18:23] The actual stderr differed from the expected stderr.
[01:18:23] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/cross-crate/cross-crate.stderr
[01:18:23] To update references, rerun the tests and pass the `--bless` flag
[01:18:23] To only update this specific test, also pass `--test-args rust-2018/uniform-paths/cross-crate.rs`
[01:18:23] error: 1 errors occurred comparing output.
[01:18:23] status: exit code: 1
[01:18:23] status: exit code: 1
[01:18:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/uniform-paths/cross-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/cross-crate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/cross-crate/auxiliary" "-A" "unused"
[01:18:23] ------------------------------------------
[01:18:23] 
[01:18:23] ------------------------------------------
[01:18:23] stderr:
[01:18:23] stderr:
[01:18:23] ------------------------------------------
[01:18:23] {"message":"cannot use a built-in attribute through an import","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/cross-crate.rs","byte_start":94,"byte_end":107,"line_start":7,"line_end":7,"column_start":3,"column_end":16,"is_primary":true,"text":[{"text":"#[built_in_attr] //~ ERROR cannot use a built-in attribute through an import","highlight_start":3,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the built-in attribute imported here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/cross-crate.rs","byte_start":75,"byte_end":89,"line_start":5,"line_end":5,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"use cross_crate::*;","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: cannot use a built-in attribute through an import\n  --> /checkout/src/test/ui/rust-2018/uniform-paths/cross-crate.rs:7:3\n   |\nLL | #[built_in_attr] //~ ERROR cannot use a built-in attribute through an import\n   |   ^^^^^^^^^^^^^\n   |\nnote: the built-in attribute imported here\n  --> /checkout/src/test/ui/rust-2018/uniform-paths/cross-crate.rs:5:5\n   |\nLL | use cross_crate::*;\n   |     ^^^^^^^^^^^^^^\n\n"}
[01:18:23] {"message":"cannot use a tool module through an import","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/cross-crate.rs","byte_start":171,"byte_end":179,"line_start":8,"line_end":8,"column_start":3,"column_end":11,"is_primary":true,"text":[{"text":"#[tool_mod::skip] //~ ERROR cannot use a tool module through an import","highlight_start":3,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the tool module imported here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/cross-crate.rs","byte_start":75,"byte_end":89,"line_start":5,"line_end":5,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"use cross_crate::*;","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: cannot use a tool module through an import\n  --> /checkout/src/test/ui/rust-2018/uniform-paths/cross-crate.rs:8:3\n   |\nLL | #[tool_mod::skip] //~ ERROR cannot use a tool module through an import\n   |   ^^^^^^^^\n   |\nnote: the tool module imported here\n  --> /checkout/src/test/ui/rust-2018/uniform-paths/cross-crate.rs:5:5\n   |\nLL | use cross_crate::*;\n   |     ^^^^^^^^^^^^^^\n\n"}
[01:18:23] 
[01:18:23] ------------------------------------------
[01:18:23] 
[01:18:23] thread '[ui] ui/rust-2018/uniform-paths/cross-crate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
---
[01:18:23] 
[01:18:23] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:18:23] 
[01:18:23] 
[01:18:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:18:23] 
[01:18:23] 
[01:18:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:18:23] Build completed unsuccessfully in 0:04:44
[01:18:23] Build completed unsuccessfully in 0:04:44
[01:18:23] make: *** [check] Error 1
[01:18:23] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:10585443
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Mar 16 02:35:12 UTC 2019
