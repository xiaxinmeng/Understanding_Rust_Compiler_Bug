plain
travis_time:end:07edcfe0:start=1553007326531921066,finish=1553007420499162807,duration=93967241741
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:12:41] ..................i................................................................................. 4700/5479
[01:12:46] ...............................................................ii................................... 4800/5479
[01:12:50] .................................................................................................... 4900/5479
[01:12:53] .................................................................................................... 5000/5479
[01:12:57] .........................F.......................................................................... 5100/5479
[01:13:04] .................................................................................................... 5300/5479
[01:13:06] .................................................................................................... 5400/5479
[01:13:09] .................i.............................................................
[01:13:09] failures:
[01:13:09] failures:
[01:13:09] 
[01:13:09] ---- [ui] ui/target-feature-wrong.rs stdout ----
[01:13:09] diff of stderr:
[01:13:09] 
[01:13:09] 22 LL | #[target_feature(disable = "baz")]
[01:13:09] 24 
[01:13:09] 24 
[01:13:09] - error: #[target_feature(..)] can only be applied to `unsafe` function
[01:13:09] -    |
[01:13:09] -    |
[01:13:09] - LL | #[target_feature(enable = "sse2")]
[01:13:09] - 
[01:13:09] 31 error: attribute should be applied to a function
[01:13:09] 32   --> $DIR/target-feature-wrong.rs:30:1
[01:13:09] 33    |
[01:13:09] 33    |
[01:13:09] 
[01:13:09] 42    |
[01:13:09] 43 LL | #[inline(always)]
[01:13:09] 44    | ^^^^^^^^^^^^^^^^^
[01:13:09] + 
[01:13:09] + error: #[target_feature(..)] can only be applied to `unsafe` function
[01:13:09] +    |
[01:13:09] + LL | fn bar() {}
[01:13:09] +    | ^^^^^^^^^^^
[01:13:09] 45 
[01:13:09] 45 
[01:13:09] 46 error: aborting due to 7 previous errors
[01:13:09] 47 
[01:13:09] 
[01:13:09] 
[01:13:09] The actual stderr differed from the expected stderr.
[01:13:09] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature-wrong/target-feature-wrong.stderr
[01:13:09] To update references, rerun the tests and pass the `--bless` flag
[01:13:09] To only update this specific test, also pass `--test-args target-feature-wrong.rs`
[01:13:09] error: 1 errors occurred comparing output.
[01:13:09] status: exit code: 1
[01:13:09] status: exit code: 1
[01:13:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/target-feature-wrong.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature-wrong/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature-wrong/auxiliary" "-A" "unused"
[01:13:09] ------------------------------------------
[01:13:09] 
[01:13:09] ------------------------------------------
[01:13:09] stderr:
[01:13:09] stderr:
[01:13:09] ------------------------------------------
[01:13:09] {"message":"attribute must be of the form `#[target_feature(enable = \"name\")]`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":240,"byte_end":267,"line_start":16,"line_end":16,"column_start":1,"column_end":28,"is_primary":true,"text":[{"text":"#[target_feature = \"+sse2\"]","highlight_start":1,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: attribute must be of the form `#[target_feature(enable = \"name\")]`\n  --> /checkout/src/test/ui/target-feature-wrong.rs:16:1\n   |\nLL | #[target_feature = \"+sse2\"]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:13:09] {"message":"the feature named `foo` is not valid for this target","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":317,"byte_end":331,"line_start":18,"line_end":18,"column_start":18,"column_end":32,"is_primary":true,"text":[{"text":"#[target_feature(enable = \"foo\")]","highlight_start":18,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: the feature named `foo` is not valid for this target\n  --> /checkout/src/test/ui/target-feature-wrong.rs:18:18\n   |\nLL | #[target_feature(enable = \"foo\")]\n   |                  ^^^^^^^^^^^^^^\n\n"}
[01:13:09] {"message":"#[target_feature(..)] only accepts sub-keys of `enable` currently","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":389,"byte_end":392,"line_start":20,"line_end":20,"column_start":18,"column_end":21,"is_primary":true,"text":[{"text":"#[target_feature(bar)]","highlight_start":18,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: #[target_feature(..)] only accepts sub-keys of `enable` currently\n  --> /checkout/src/test/ui/target-feature-wrong.rs:20:18\n   |\nLL | #[target_feature(bar)]\n   |                  ^^^\n\n"}
[01:13:09] {"message":"#[target_feature(..)] only accepts sub-keys of `enable` currently","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":446,"byte_end":461,"line_start":22,"line_end":22,"column_start":18,"column_end":33,"is_primary":true,"text":[{"text":"#[target_feature(disable = \"baz\")]","highlight_start":18,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: #[target_feature(..)] only accepts sub-keys of `enable` currently\n  --> /checkout/src/test/ui/target-feature-wrong.rs:22:18\n   |\nLL | #[target_feature(disable = \"baz\")]\n   |                  ^^^^^^^^^^^^^^^\n\n"}
[01:13:09] {"message":"attribute should be applied to a function","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":698,"byte_end":712,"line_start":32,"line_end":32,"column_start":1,"column_end":15,"is_primary":false,"text":[{"text":"mod another {}","highlight_start":1,"highlight_end":15}],"label":"not a function","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":619,"byte_end":653,"line_start":30,"line_end":30,"column_start":1,"column_end":35,"is_primary":true,"text":[{"text":"#[target_feature(enable = \"sse2\")]","highlight_start":1,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: attribute should be applied to a function\n  --> /checkout/src/test/ui/target-feature-wrong.rs:30:1\n   |\nLL | #[target_feature(enable = \"sse2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\nLL | //~^ ERROR: should be applied to a function\nLL | mod another {}\n   | -------------- not a function\n\n"}
[01:13:09] {"message":"cannot use #[inline(always)] with #[target_feature]","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":714,"byte_end":731,"line_start":34,"line_end":34,"column_start":1,"column_end":18,"is_primary":true,"text":[{"text":"#[inline(always)]","highlight_start":1,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: cannot use #[inline(always)] with #[target_feature]\n  --> /checkout/src/test/ui/target-feature-wrong.rs:34:1\n   |\nLL | #[inline(always)]\n   | ^^^^^^^^^^^^^^^^^\n\n"}
[01:13:09] {"message":"#[target_feature(..)] can only be applied to `unsafe` function","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/target-feature-wrong.rs","byte_start":606,"byte_end":617,"line_start":28,"line_end":28,"column_start":1,"column_end":12,"is_primary":true,"text":[{"text":"fn bar() {}","highlight_start":1,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: #[target_feature(..)] can only be applied to `unsafe` function\n  --> /checkout/src/test/ui/target-feature-wrong.rs:28:1\n   |\nLL | fn bar() {}\n   | ^^^^^^^^^^^\n\n"}
[01:13:09] 
[01:13:09] ------------------------------------------
[01:13:09] 
[01:13:09] thread '[ui] ui/target-feature-wrong.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
---
[01:13:09] 
[01:13:09] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:13:09] 
[01:13:09] 
[01:13:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:09] 
[01:13:09] 
[01:13:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:09] Build completed unsuccessfully in 0:04:18
[01:13:09] Build completed unsuccessfully in 0:04:18
[01:13:09] Makefile:48: recipe for target 'check' failed
[01:13:09] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1e4057c0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 19 16:10:19 UTC 2019
---
travis_time:end:03ccd4cc:start=1553011820329179345,finish=1553011820334180620,duration=5001275
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0841b123
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00559376
travis_time:start:00559376
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0c04b9cb
$ dmesg | grep -i kill
