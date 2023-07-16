plain
travis_time:end:36cfb25c:start=1548194607502964450,finish=1548194609845261818,duration=2342297368
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:03:44] .................................................................................................... 2600/5321
[01:03:48] .................................................................................................... 2700/5321
[01:03:52] .................................................................................................... 2800/5321
[01:03:56] .................................................................................................... 2900/5321
[01:03:59] ..............................................................................FFF................... 3000/5321
[01:04:06] ..............................................................................................i..... 3200/5321
[01:04:09] .................................................................................................... 3300/5321
[01:04:12] ...........................................................ii...i..ii............................... 3400/5321
[01:04:16] .................................................................................................... 3500/5321
---
[01:05:14] .................................................................................................... 5000/5321
[01:05:17] .................................................................................................... 5100/5321
[01:05:20] .................................................................................................... 5200/5321
[01:05:23] ............................................................i....................................... 5300/5321
to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/homogeneous-aggr-phantom-data/homogeneous-aggr-phantom-data.stderr
[01:05:24] To update references, rerun the tests and pass the `--bless` flag
[01:05:24] To only update this specific test, also pass `--test-args layout/homogeneous-aggr-phantom-data.rs`
[01:05:24] error: 1 errors occurred comparing output.
[01:05:24] status: exit code: 1
[01:05:24] status: exit code: 1
[01:05:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/layout/homogeneous-aggr-phantom-data.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/homogeneous-aggr-phantom-data/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/homogeneous-aggr-phantom-data/auxiliary" "-A" "unused"
[01:05:24] ------------------------------------------
[01:05:24] 
[01:05:24] ------------------------------------------
[01:05:24] stderr:
[01:05:24] stderr:
[01:05:24] ------------------------------------------
[01:05:24] {"message":"attribute must be of the form `#[rustc_layout]`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/layout/homogeneous-aggr-phantom-data.rs","byte_start":358,"byte_end":396,"line_start":20,"line_end":20,"column_start":1,"column_end":39,"is_primary":true,"text":[{"text":"#[rustc_layout(homogeneous_aggregate)]","highlight_start":1,"highlight_end":39}],"label":null,"suggested_replacemen] 
[01:05:24] - error: abi: Aggregate { sized: true, has_vla: true }
[01:05:24] -   --> $DIR/has-vla-zero-length-array-struct.rs:10:1
[01:05:24] + error: attribute must be of the form `#[rustc_layout]`
[01:05:24] +   --> $DIR/has-vla-zero-length-array-struct.rs:9:1
[01:05:24] 3    |
[01:05:24] - LL | pub type Test0 = [u32; 0];
[01:05:24] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:05:24] + LL | #[rustc_layout(abi)]
[01:05:24] 6 
[01:05:24] 6 
[01:05:24] - error: abi: Aggregate { sized: true, has_vla: true }
[01:05:24] -   --> $DIR/has-vla-zero-length-array-struct.rs:22:1
[01:05:24] + error: attribute must be of the form `#[rustc_layout]`
[01:05:24] +   --> $DIR/has-vla-zero-length-array-struct.rs:21:1
[01:05:24] 9    |
[01:05:24] - LL | pub type Test1 = FinalField;
[01:05:24] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:05:24] + LL | #[rustc_layout(abi)]
[01:05:24] 12 
[01:05:24] 12 
[01:05:24] - error: abi: Aggregate { sized: true, has_vla: false }
[01:05:24] -   --> $DIR/has-vla-zero-length-array-struct.rs:34:1
[01:05:24] + error: attribute must be of the form `#[rustc_layout]`
[01:05:24] +   --> $DIR/has-vla-zero-length-array-struct.rs:33:1
[01:05:24] 15    |
[01:05:24] - LL | pub type Test2 = MiddleField;
[01:05:24] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:05:24] + LL | #[rustc_layout(abi)]
[01:05:24] 18 
[01:05:24] 18 
[01:05:24] - error: abi: Aggregate { sized: true, has_vla: true }
[01:05:24] -   --> $DIR/has-vla-zero-length-array-struct.rs:46:1
[01:05:24] + error: attribute must be of the form `#[rustc_layout]`
[01:05:24] +   --> $DIR/has-vla-zero-length-array-struct.rs:45:1
[01:05:24] 21    |
[01:05:24] - LL | pub type Test3 = FinalFieldTransitive;
[01:05:24] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:05:24] + LL | #[rustc_layout(abi)]
[01:05:24] 24 
[01:05:24] 24 
[01:05:24] - error: abi: Aggregate { sized: true, has_vla: false }
[01:05:24] -   --> $DIR/has-vla-zero-length-array-struct.rs:62:1
[01:05:24] + error: attribute must be of the form `#[rustc_layout]`
[01:05:24] +   --> $DIR/has-vla-zero-length-array-struct.rs:61:1
[01:05:24] 27    |
[01:05:24] - LL | pub type Test4 = MiddleFieldPhantom;
[01:05:24] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:05:24] + LL | #[rustc_layout(abi)]
[01:05:24] 30 
[01:05:24] 30 
[01:05:24] - error: abi: ScalarPair(Scalar { value: Float(f32), valid_range: 0..=4294967295 }, Scalar { value: Float(f32), valid_range: 0..=4294967295 })
[01:05:24] -   --> $DIR/has-vla-zero-length-array-struct.rs:74:1
[01:05:24] + error: attribute must be of the form `#[rustc_layout]`
[01:05:24] +   --> $DIR/has-vla-zero-length-array-struct.rs:73:1
[01:05:24] 33    |
[01:05:24] - LL | pub type Test5 = FinalFieldRust;
[01:05:24] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:05:24] + LL | #[rustc_layout(abi)]
[01:05:24] 36 
[01:05:24] 37 error: aborting due to 6 previous errors
[01:05:24] 38 
[01:05:24] 
[01:05:24] 
[01:05:24] 
[01:05:24] The actual stderr differed from the expected stderr.
[01:05:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/has-vla-zero-length-array-struct/has-vla-zero-length-array-struct.stderr
[01:05:24] To update references, rerun the tests and pass the `--bless` flag
[01:05:24] To only update this specific test, also pass `--test-args layout/has-vla-zero-length-array-struct.rs`
[01:05:24] error: 1 errors occurred comparing output.
[01:05:24] status: exit code: 1
[01:05:24] status: exit code: 1
[01:05:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/layout/has-vla-zero-length-array-struct.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/has-vla-zero-length-array-struct/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/has-vla-zero-length-array-struct/auxiliary" "-A" "unused"
[01:05:24] ------------------------------------------
[01:05:24] 
[01:05:24] ------------------------------------------
[01:05:24] stderr:
[01:05:24] stderr:
[01:05:24] ------------------------------------------
[01:05:24] {"message":"attribute must be of the form `#[rustc_layout]`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/layout/has-vla-zero-length-array-struct.rs","byte_start":230,"byte_end":250,"line_start":9,"line_end":9,"column_start":1,"column_end":21,"is_primary":true,"text":[{"text":"#[rustc_layout(abi)]","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: attribute must be of the form `#[rustc_layout]`\n  --> /checkout/src/test/ui/layout/has-vla-zero-length-array-struct.rs:9:1\n   |\nLL | #[rustc_layout(abi)]\n   | ^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:05:24] {"message":"attribute must be of the form `#[rustc_layout]`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/layout/has-vla-zero-length-array-struct.rs","byte_start":527,"byte_end":547,"line_start":21,"line_end":21,"column_start":1,"column_end":21,"is_primary":true,"text":[{"text":"#[rustc_layout(abi)]","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: attribute must be of the form `#[rustc_layout]`\n  --> /checkout/src/test/ui/layout/has-vla-zero-length-array-struct.rs:21:1\n   |\nLL | #[rustc_layout(abi)]\n   | ^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:05:24] {"message":"attribute must be of the form `#[rustc_layout]`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/layout/has-vla-zero-length-array-struct.rs","byte_start":827,"byte_end":847,"line_start":33,"line_end":33,"column_start":1,"column_end":21,"is_primary":true,"text":[{"text":"#[rustc_layout(abi)]","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: attribute must be of the form `#[rustc_layout]`\n  --> /checkout/src/test/ui/layout/has-vla-zero-length-array-struct.rs:33:1\n   |\nLL | #[rustc_layout(abi)]\n   | ^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:05:24] {"message":"attribute must be of the form `#[rustc_layout]`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/layout/has-vla-zero-length-array-struct.rs","byte_start":1139,"byte_end":1159,"line_start":45,"line_end":45,"column_start":1,"column_end":21,"is_primary":true,"text":[{"text":"#[rustc_layout(abi)]","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: attribute must be of the form `#[rustc_layout]`\n  --> /checkout/src/test/ui/layout/has-vla-zero-length-array-struct.rs:45:1\n   |\nLL | #[rustc_layout(abi)]\n   | ^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:05:24] {"message":"attribute must be of the form `#[rustc_layout]`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/layout/has-vla-zero-length-array-struct.rs","byte_start":1608,"byte_end":1628,"line_start":61,"line_end":61,"column_start":1,"column_end":21,"is_primary":true,"text":[{"text":"#[rustc_layout(abi)]","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: attribute must be of the form `#[rustc_layout]`\n  --> /checkout/src/test/ui/layout/has-vla-zero-length-array-struct.rs:61:1\n   |\nLL | #[rustc_layout(abi)]\n   | ^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:05:24] {"message":"attribute must be of the form `#[rustc_layout]`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/layout/has-vla-zero-length-array-struct.rs","byte_start":1886,"byte_end":1906,"line_start":73,"line_end":73,"column_start":1,"column_end":21,"is_primary":true,"text":[{"text":"#[rustc_layout(abi)]","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: attribute must be of the form `#[rustc_layout]`\n  --> /checkout/src/test/ui/layout/has-vla-zero-length-array-struct.rs:73:1\n   |\nLL | #[rustc_layout(abi)]\n   | ^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:05:24] {"message":"aborting due to 6 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 6 previous errors\n\n"}
[01:05:24] ------------------------------------------
[01:05:24] 
[01:05:24] 
[01:05:24] thread '[ui] ui/layout/has-vla-zero-length-array-struct.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:05:24] 
[01:05:24] ---- [ui] ui/layout/has-vla-union.rs stdout ----
[01:05:24] 
[01:05:24] 
[01:05:24] - error: abi: Aggregate { sized: true, has_vla: true }
[01:05:24] -   --> $DIR/has-vla-union.rs:27:1
[01:05:24] + error: attribute must be of the form `#[rustc_layout]`
[01:05:24] +   --> $DIR/has-vla-union.rs:26:1
[01:05:24] 3    |
[01:05:24] - LL | type TestEmpty3 = Empty3;
[01:05:24] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:05:24] + LL | #[rustc_layout(abi)]
[01:05:24] 6 
[01:05:24] 6 
[01:05:24] - error: abi: Aggregate { sized: true, has_vla: true }
[01:05:24] -   --> $DIR/has-vla-union.rs:52:1
[01:05:24] + error: attribute must be of the form `#[rustc_layout]`
[01:05:24] +   --> $DIR/has-vla-union.rs:51:1
[01:05:24] 9    |
[01:05:24] - LL | type TestU3 = U3;
[01:05:24] -    | ^^^^^^^^^^^^^^^^^
[01:05:24] + LL | #[rustc_layout(abi)]
[01:05:24] 12 
[01:05:24] 12 
[01:05:24] - error: homogeneous_aggregate: Some(Reg { kind: Float, size: Size { raw: 4 } })
[01:05:24] -   --> $DIR/has-vla-union.rs:68:1
[01:05:24] + error: attribute must be of the form `#[rustc_layout]`
[01:05:24] +   --> $DIR/has-vla-union.rs:67:1
[01:05:24] 15    |
[01:05:24] - LL | type TestBaz1 = Baz1;
[01:05:24] -    | ^^^^^^^^^^^^^^^^^^^^^
[01:05:24] + LL | #[rustc_layout(homogeneous_aggregate)]
[01:05:24] 18 
[01:05:24] 18 
[01:05:24] - error: homogeneous_aggregate: Some(Reg { kind: Float, size: Size { raw: 4 } })
[01:05:24] -   --> $DIR/has-vla-union.rs:79:1
[01:05:24] + error: attribute must be of the form `#[rustc_layout]`
[01:05:24] +   --> $DIR/has-vla-union.rs:78:1
[01:05:24] 21    |
[01:05:24] - LL | type TestBaz2 = Baz2;
[01:05:24] -    | ^^^^^^^^^^^^^^^^^^^^^
[01:05:24] + LL | #[rustc_layout(homogeneous_aggregate)]
[01:05:24] 24 
[01:05:24] 24 
[01:05:24] - error: homogeneous_aggregate: None
[01:05:24] -   --> $DIR/has-vla-union.rs:90:1
[01:05:24] + error: attribute must be of the form `#[rustc_layout]`
[01:05:24] +   --> $DIR/has-vla-union.rs:89:1
[01:05:24] 27    |
[01:05:24] - LL | type TestBaz3 = Baz3;
[01:05:24] -    | ^^^^^^^^^^^^^^^^^^^^^
[01:05:24] + LL | #[rustc_layout(homogeneous_aggregate)]
[01:05:24] 30 
[01:05:24] 30 
[01:05:24] - error: homogeneous_aggregate: None
[01:05:24] -   --> $DIR/has-vla-union.rs:101:1
[01:05:24] + error: attribute must be of the form `#[rustc_layout]`
[01:05:24] +   --> $DIR/has-vla-union.rs:100:1
[01:05:24] 33    |
[01:05:24] - LL | type TestBaz4 = Baz4;
[01:05:24] -    | ^^^^^^^^^^^^^^^^^^^^^
[01:05:24] + LL | #[rustc_layout(homogeneous_aggregate)]
[01:05:24] 36 
[01:05:24] 37 error: aborting due to 6 previous errors
[01:05:24] 38 
[01:05:24] 
[01:05:24] 
[01:05:24] 
[01:05:24] The actual stderr differed from the expected stderr.
[01:05:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/has-vla-union/has-vla-union.stderr
[01:05:24] To update references, rerun the tests and pass the `--bless` flag
[01:05:24] To only update this specific test, also pass `--test-args layout/has-vla-union.rs`
[01:05:24] error: 1 errors occurred comparing output.
[01:05:24] status: exit code: 1
[01:05:24] status: exit code: 1
[01:05:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/layout/has-vla-union.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/has-vla-union/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/has-vla-union/auxiliary" "-A" "unused"
[01:05:24] ------------------------------------------
[01:05:24] 
[01:05:24] ------------------------------------------
[01:05:24] stderr:
[01:05:24] stderr:
[01:05:24] ------------------------------------------
[01:05:24] {"message":"attribute must be of the form `#[rustc_layout]`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/layout/has-vla-union.rs","byte_start":449,"byte_end":469,"line_start":26,"line_end":26,"column_start":1,"column_end":21,"is_primary":true,"text":[{"text":"#[rustc_layout(abi)]","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: attribute must be of the form `#[rustc_layout]`\n  --> /checkout/src/test/ui/layout/has-vla-union.rs:26:1\n   |\nLL | #[rustc_layout(abi)]\n   | ^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:05:24] {"message":"attribute must be of the form `#[rustc_layout]`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/layout/has-vla-union.rs","byte_start":737,"byte_end":757,"line_start":51,"line_end":51,"column_start":1,"column_end":21,"is_primary":true,"text":[{"text":"#[rustc_layout(abi)]","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: attribute must be of the form `#[rustc_layout]`\n  --> /checkout/src/test/ui/layout/has-vla-union.rs:51:1\n   |\nLL | #[rustc_layout(abi)]\n   | ^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:05:24] {"message":"attribute must be of the form `#[rustc_layout]`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/layout/has-vla-union.rs","byte_start":936,"byte_end":974,"line_start":67,"line_end":67,"column_start":1,"column_end":39,"is_primary":true,"text":[{"text":"#[rustc_layout(homogeneous_aggregate)]","hig /checkout/src/test/ui/layout/has-vla-union.rs:89:1\n   |\nLL | #[rustc_layout(homogeneous_aggregate)]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:05:24] {"message":"attribute must be of the form `#[rustc_layout]`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/layout/has-vla-union.rs","byte_start":1428,"byte_end":1466,"line_start":100,"line_end":100,"column_start":1,"column_end":39,"is_primary":true,"text":[{"text":"#[rustc_layout(homogeneous_aggregate)]","highlight_start":1,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: attribute must be of the form `#[rustc_layout]`\n  --> /checkout/src/test/ui/layout/has-vla-union.rs:100:1\n   |\nLL | #[rustc_layout(homogeneous_aggregate)]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:05:24] {"message":"aborting due to 6 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 6 previous errors\n\n"}
[01:05:24] ------------------------------------------
[01:05:24] 
[01:05:24] 
[01:05:24] thread '[ui] ui/layout/has-vla-union.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:05:24] 
[01:05:24] failures:
[01:05:24] failures:
[01:05:24]     [ui] ui/layout/has-vla-union.rs
[01:05:24]     [ui] ui/layout/has-vla-zero-length-array-struct.rs
[01:05:24]     [ui] ui/layout/homogeneous-aggr-phantom-data.rs
[01:05:24] test result: FAILED. 5295 passed; 3 failed; 23 ignored; 0 measured; 0 filtered out
[01:05:24] 
[01:05:24] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:05:24] 
[01:05:24] 
[01:05:24] 
[01:05:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:24] 
[01:05:24] 
[01:05:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:24] Build completed unsuccessfully in 0:04:05
[01:05:24] Build completed unsuccessfully in 0:04:05
[01:05:24] make: *** [check] Error 1
[01:05:24] Makefile:48: recipe for target 'check' failed
2552332 ./obj
2552292 ./obj/build
1887180 ./obj/build/x86_64-unknown-linux-gnu
1131228 ./src
---
171496 ./obj/build/x86_64-unknown-linux-gnu/stage2/lib
163000 ./obj/build/bootstrap/debug/incremental
153276 ./src/tools/clang
146884 ./obj/build/bootstrap/debug/incremental/bootstrap-1o7ipylf5x405
146880 ./obj/build/bootstrap/debug/incremental/bootstrap-1o7ipylf5x405/s-f8seuzwwdy-1elglv3-3v9gdvsuxn2fo
139412 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
139408 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
137068 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
124936 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
---
26652 ./src/llvm-emscripten/test/Transforms
25820 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release
25820 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release
24100 ./src/tools/lldb/packages
24096 ./sold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00b804ca
travis_time:start:00b804ca
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09d949a8
$ dmesg | grep -i kill
