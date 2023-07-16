plain
travis_time:end:02428816:start=1545537723325581944,finish=1545537726490994844,duration=3165412900
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:17] 
[01:07:17] running 118 tests
[01:07:41] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:07:45] ......iii.i.....ii
[01:07:45] 
[01:07:45]  finished in 28.067
[01:07:45] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc-ui
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:36:50] 
[01:36:50] running 12 tests
[01:36:56] .F...F......
[01:36:56] 
[01:36:56] ---- [ui] rustdoc-ui/deprecated-attrs.rs stdout ----
[01:36:56] diff of stderr:
[01:36:56] 
[01:36:56] 
[01:36:56] 1 warning: the `#![doc(no_default_passes)]` attribute is considered deprecated
[01:36:56] -    = warning: please see https://github.com/rust-lang/rust/issues/44136
[01:36:56] +    = warning: see <https://github.com/rust-lang/rust/issues/44136>
[01:36:56] +    = warning: see <https://github.com/rust-lang/rust/issues/44136>
[01:36:56] 4    = help: you may want to use `#![doc(document_private_items)]`
[01:36:56] 5 
[01:36:56] 6 warning: the `#![doc(passes = "...")]` attribute is considered deprecated
[01:36:56] 7    |
[01:36:56] -    = warning: please see https://github.com/rust-lang/rust/issues/44136
[01:36:56] +    = warning: see <https://github.com/rust-lang/rust/issues/44136>
[01:36:56] 9 
[01:36:56] 9 
[01:36:56] 10 
[01:36:56] 
[01:36:56] 
[01:36:56] The actual stderr differed from the expected stderr.
[01:36:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deprecated-attrs/deprecated-attrs.stderr
[01:36:56] To update references, rerun the tests and pass the `--bless` flag
[01:36:56] To only update this specific test, also pass `--test-args deprecated-attrs.rs`
[01:36:56] error: 1 errors occurred comparing output.
[01:36:56] status: exit code: 0
[01:36:56] status: exit code: 0
[01:36:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/deprecated-attrs.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deprecated-attrs/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deprecated-attrs/auxiliary"
[01:36:56] ------------------------------------------
[01:36:56] 
[01:36:56] ------------------------------------------
[01:36:56] stderr:
[01:36:56] stderr:
[01:36:56] ------------------------------------------
[01:36:56] {"message":"the `#![doc(no_default_passes)]` attribute is considered deprecated","code":null,"level":"warning","spans":[],"children":[{"message":"see <https://github.com/rust-lang/rust/issues/44136>","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"you may want to use `#![doc(document_private_items)]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: the `#![doc(no_default_passes)]` attribute is considered deprecated\n   |\n   = warning: see <https://github.com/rust-lang/rust/issues/44136>\n   = help: you may want to use `#![doc(document_private_items)]`\n\n"}
[01:36:56] {"message":"the `#![doc(passes = \"...\")]` attribute is considered deprecated","code":null,"level":"warning","spans":[],"children":[{"message":"see <https://github.com/rust-lang/rust/issues/44136>","code":null,"level":"warning","spans":[],"children":[],"rendered":null}],"rendered":"warning: the `#![doc(passes = \"...\")]` attribute is considered deprecated\n   |\n   = warning: see <https://github.com/rust-lang/rust/issues/44136>\n\n"}
[01:36:56] ------------------------------------------
[01:36:56] 
[01:36:56] thread '[ui] rustdoc-ui/deprecated-attrs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3259:9
[01:36:56] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:36:56] 1 error: `[i]` cannot be resolved, ignoring it...
[01:36:56] -   --> $DIR/intra-link-span-ice-55723.rs:21:10
[01:36:56] +   --> $DIR/intra-link-span-ice-55723.rs:20:10
[01:36:56] 3    |
[01:36:56] 4 LL | /// （arr[i]）
[01:36:56] 
[01:36:56] 
[01:36:56] The actual stderr differed from the expected stderr.
[01:36:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-link-span-ice-55723/intra-link-span-ice-55723.stderr
[01:36:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-link-span-ice-55723/intra-link-span-ice-55723.stderr
[01:36:56] To update references, rerun the tests and pass the `--bless` flag
[01:36:56] To only update this specific test, also pass `--test-args intra-link-span-ice-55723.rs`
[01:36:56] error: 1 errors occurred comparing output.
[01:36:56] status: exit code: 1
[01:36:56] status: exit code: 1
[01:36:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-link-span-ice-55723.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-link-span-ice-55723/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-link-span-ice-55723/auxiliary"
[01:36:56] ------------------------------------------
[01:36:56] 
[01:36:56] ------------------------------------------
[01:36:56] stderr:
[01:36:56] stderr:
[01:36:56] ------------------------------------------
[01:36:56] {"message":"`[i]` cannot be resolved, ignoring it...","code":{"code":"intra_doc_link_resolution_failure","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-link-span-ice-55723.rs","byte_start":739,"byte_end":740,"line_start":20,"line_end":20,"column_start":10,"column_end":11,"is_primary":true,"text":[{"text":"/// （arr[i]）","highlight_start":10,"highlight_end":11}],"label":"cannot be resolved, ignoring","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/rustdoc-ui/intra-link-span-ice-55723.rs","byte_start":506,"byte_end":539,"line_start":13,"line_end":13,"column_start":9,"column_end":42,"is_primary":true,"text":[{"text":"#![deny(intra_doc_link_resolution_failure)]","highlight_start":9,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `[i]` cannot be resolved, ignoring it...\n  --> /checkout/src/test/rustdoc-ui/intra-link-span-ice-55723.rs:20:10\n   |\nLL | /// （arr[i]）\n   |           ^ cannot be resolved, ignoring\n   |\nnote: lint level defined here\n  --> /checkout/src/test/rustdoc-ui/intra-link-span-ice-55723.rs:13:9\n   |\nLL | #![deny(intra_doc_link_resolution_failure)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   = help: to escape `[` and `]` characters, just add '\\' before them like `\\[` or `\\]`\n\n"}
[01:36:56] ------------------------------------------
[01:36:56] 
[01:36:56] thread '[ui] rustdoc-ui/intra-link-span-ice-55723.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3259:9
[01:36:56] 
---
[01:36:56] test result: FAILED. 10 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:36:56] 
[01:36:56] 
[01:36:56] 
[01:36:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:36:56] 
[01:36:56] 
[01:36:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:36:56] Build completed unsuccessfully in 0:40:39
[01:36:56] Build completed unsuccessfully in 0:40:39
[01:36:56] Makefile:58: recipe for target 'check' failed
[01:36:56] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
