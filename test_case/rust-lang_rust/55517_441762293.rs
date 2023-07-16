plain
travis_time:end:25a554e0:start=1543256858540605600,finish=1543256860782170155,duration=2241564555
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:48:59] ........................................................................i........................... 3100/5073
[00:49:02] .................................................................................................... 3200/5073
[00:49:05] ...................................ii..i..ii........................................................ 3300/5073
[00:49:09] .................................................................................................... 3400/5073
[00:49:13] .................................................................F.................................. 3500/5073
[00:49:17] ..................................i................................................................. 3700/5073
[00:49:18] ..........................................................................................i......... 3800/5073
[00:49:20] .................................................................................................... 3900/5073
[00:49:23] .................................................................................................... 4000/5073
---
[00:49:59] 
[00:49:59] ---- [ui] ui/nll/user-annotations/dump-fn-method.rs stdout ----
[00:49:59] diff of stderr:
[00:49:59] 
[00:49:59] 4 LL |     let x = foo::<u32>; //~ ERROR [u32]
[00:49:59] 6 
[00:49:59] 6 
[00:49:59] - error: user substs: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }], value: UserSubsts { substs: [^0, u32, ^1], user_self_ty: None } }
[00:49:59] + error: user substs: Canonical { max_universe: U1, variables: [CanonicalVarInfo { kind: Ty(General(U1)) }, CanonicalVarInfo { kind: Ty(General(U1)) }], value: UserSubsts { substs: [^0, u32, ^1], user_self_ty: None } }
[00:49:59] 9    |
[00:49:59] 9    |
[00:49:59] 10 LL |     let x = <_ as Bazoom<u32>>::method::<_>; //~ ERROR [^0, u32, ^1]
[00:49:59] 
[00:49:59] 16 LL |     let x = <u8 as Bazoom<u16>>::method::<u32>; //~ ERROR [u8, u16, u32]
[00:49:59] 18 
[00:49:59] 18 
[00:49:59] - error: user substs: Canonical { max_universe: U1, variables: [CanonicalVarInfo { kind: Ty(General(U1)) }, CanonicalVarInfo { kind: Ty(General(U1)) }], value: UserSubsts { substs: [^0, ^1, u32], user_self_ty: None } }
[00:49:59] + error: user substs: Canonical { max_universe: U4, variables: [CanonicalVarInfo { kind: Ty(General(U4)) }, CanonicalVarInfo { kind: Ty(General(U4)) }], value: UserSubsts { substs: [^0, ^1, u32], user_self_ty: None } }
[00:49:59] 21    |
[00:49:59] 21    |
[00:49:59] 22 LL |     y.method::<u32>(44, 66); //~ ERROR [^0, ^1, u32]
[00:49:59] 
[00:49:59] The actual stderr differed from the expected stderr.
[00:49:59] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/dump-fn-method/dump-fn-method.stderr
[00:49:59] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/dump-fn-method/dump-fn-method.stderr
[00:49:59] To update references, rerun the tests and pass the `--bless` flag
[00:49:59] To only update this specific test, also pass `--test-args nll/user-annotations/dump-fn-method.rs`
[00:49:59] error: 1 errors occurred comparing output.
[00:49:59] status: exit code: 1
[00:49:59] status: exit code: 1
[00:49:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/dump-fn-method.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/dump-fn-method/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/dump-fn-method/auxiliary" "-A" "unused"
[00:49:59] ------------------------------------------
[00:49:59] 
[00:49:59] ------------------------------------------
[00:49:59] stderr:
[00:49:59] stderr:
[00:49:59] ------------------------------------------
[00:49:59] {"message":"user substs: Canonical { max_universe: U0, variables: [], value: UserSubsts { substs: [u32], user_self_ty: None } }","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/dump-fn-method.rs","byte_start":975,"byte_end":985,"line_start":36,"line_end":36,"column_start":13,"column_end":23,"is_primary":true,"text":[{"text":"    let x = foo::<u32>; //~ ERROR [u32]","highlight_start":13,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: user substs: Canonical { max_universe: U0, variables: [], value: UserSubsts { substs: [u32], user_self_ty: None } }\n  --> /checkout/src/test/ui/nll/user-annotations/dump-fn-method.rs:36:13\n   |\nLL |     let x = foo::<u32>; //~ ERROR [u32]\n   |             ^^^^^^^^^^\n\n"}
[00:49:59] {"message":"user substs: Canonical { max_universe: U1, variables: [CanonicalVarInfo { kind: Ty(General(U1)) }, CanonicalVarInfo { kind: Ty(General(U1)) }], value: UserSubsts { substs: [^0, u32, ^1], user_self_ty: None } }","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/dump-fn-method.rs","byte_start":1162,"byte_end":1193,"line_start":42,"line_end":42,"column_start":13,"column_end":44,"is_primary":true,"text":[{"text":"    let x = <_ as Bazoom<u32>>::method::<_>; //~ ERROR [^0, u32, ^1]","highlight_start":13,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: user substs: Canonical { max_universe: U1, variables: [CanonicalVarInfo { kind: Ty(General(U1)) }, CanonicalVarInfo { kind: Ty(General(U1)) }], value: UserSubsts { substs: [^0, u32, ^1], user_self_ty: None } }\n  --> /checkout/src/test/ui/nll/user-annotations/dump-fn-method.rs:42:13\n   |\nLL |     let x = <_ as Bazoom<u32>>::method::<_>; //~ ERROR [^0, u32, ^1]\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:49:59] {"message":"user substs: Canonical { max_universe: U0, variables: [], value: UserSubsts { substs: [u8, u16, u32], user_self_ty: None } }","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/dump-fn-method.rs","byte_start":1279,"byte_end":1313,"line_start":46,"line_end":46,"column_start":13,"column_end":47,"is_primary":true,"text":[{"text":"    let x = <u8 as Bazoom<u16>>::method::<u32>; //~ ERROR [u8, u16, u32]","highlight_start":13,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: user substs: Canonical { max_universe: U0, variables: [], value: UserSubsts { substs: [u8, u16, u32], user_self_ty: None } }\n  --> /checkout/src/test/ui/nll/user-annotations/dump-fn-method.rs:46:13\n   |\nLL |     let x = <u8 as Bazoom<u16>>::method::<u32>; //~ ERROR [u8, u16, u32]\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:49:59] {"message":"user substs: Canonical { max_universe: U4, variables: [CanonicalVarInfo { kind: Ty(General(U4)) }, CanonicalVarInfo { kind: Ty(General(U4)) }], value: UserSubsts { substs: [^0, ^1, u32], user_self_ty: None } }","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/dump-fn-method.rs","byte_start":1557,"byte_end":1580,"line_start":54,"line_end":54,"column_start":5,"column_end":28,"is_primary":true,"text":[{"text":"    y.method::<u32>(44, 66); //~ ERROR [^0, ^1, u32]","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: user substs: Canonical { max_universe: U4, variables: [CanonicalVarInfo { kind: Ty(General(U4)) }, CanonicalVarInfo { kind: Ty(General(U4)) }], value: UserSubsts { substs: [^0, ^1, u32], user_self_ty: None } }\n  --> /checkout/src/test/ui/nll/user-annotations/dump-fn-method.rs:54:5\n   |\nLL |     y.method::<u32>(44, 66); //~ ERROR [^0, ^1, u32]\n   |     ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:49:59] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:49:59] ------------------------------------------
[00:49:59] 
[00:49:59] thread '[ui] ui/nll/user-annotations/dump-fn-method.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:49:59] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:49:59] 
[00:49:59] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:49:59] 
[00:49:59] 
[00:49:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:49:59] 
[00:49:59] 
[00:49:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:59] Build completed unsuccessfully in 0:03:53
[00:49:59] Build completed unsuccessfully in 0:03:53
[00:49:59] make: *** [check] Error 1
[00:49:59] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09b7d827
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Nov 26 19:17:49 UTC 2018
---
travis_time:end:0057faa7:start=1543259870937791417,finish=1543259870942942123,duration=5150706
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07e18c8a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/
