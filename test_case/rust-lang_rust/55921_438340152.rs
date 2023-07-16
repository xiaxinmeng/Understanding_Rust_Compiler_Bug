plain
travis_time:end:0989e860:start=1542122302904040142,finish=1542122305346150585,duration=2442110443
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[01:17:01] .........................................i.......................................................... 3100/5017
[01:17:05] .................................................................................................... 3200/5017
[01:17:08] ...i.i..ii.......................................................................................... 3300/5017
[01:17:12] .................................................................................................... 3400/5017
[01:17:15] .............................F.................................................i.ii................. 3500/5017
[01:17:19] i................................................................................................... 3700/5017
[01:17:20] .......................................................i............................................ 3800/5017
[01:17:22] .................................................................................................... 3900/5017
[01:17:25] .................................................................................................... 4000/5017
---
[01:17:59] 
[01:17:59] ---- [ui] ui/nll/user-annotations/dump-fn-method.rs stdout ----
[01:17:59] diff of stderr:
[01:17:59] 
[01:17:59] 4 LL |     let x = foo::<u32>; //~ ERROR [u32]
[01:17:59] 6 
[01:17:59] 6 
[01:17:59] - error: user substs: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General) }, CanonicalVarInfo { kind: Ty(General) }], value: UserSubsts { substs: [^0, u32, ^1], user_self_ty: None } }
[01:17:59] + error: user substs: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }], value: UserSubsts { substs: [^0, u32, ^1], user_self_ty: None } }
[01:17:59] 9    |
[01:17:59] 9    |
[01:17:59] 10 LL |     let x = <_ as Bazoom<u32>>::method::<_>; //~ ERROR [^0, u32, ^1]
[01:17:59] 
[01:17:59] 16 LL |     let x = <u8 as Bazoom<u16>>::method::<u32>; //~ ERROR [u8, u16, u32]
[01:17:59] 18 
[01:17:59] 18 
[01:17:59] - error: user substs: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General) }, CanonicalVarInfo { kind: Ty(General) }], value: UserSubsts { substs: [^0, ^1, u32], user_self_ty: None } }
[01:17:59] + error: user substs: Canonical { max_universe: U1, variables: [CanonicalVarInfo { kind: Ty(General(U1)) }, CanonicalVarInfo { kind: Ty(General(U1)) }], value: UserSubsts { substs: [^0, ^1, u32], user_self_ty: None } }
[01:17:59] 21    |
[01:17:59] 21    |
[01:17:59] 22 LL |     y.method::<u32>(44, 66); //~ ERROR [^0, ^1, u32]
[01:17:59] 
[01:17:59] The actual stderr differed from the expected stderr.
[01:17:59] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/dump-fn-method/dump-fn-method.stderr
[01:17:59] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/dump-fn-method/dump-fn-method.stderr
[01:17:59] To update references, rerun the tests and pass the `--bless` flag
[01:17:59] To only update this specific test, also pass `--test-args nll/user-annotations/dump-fn-method.rs`
[01:17:59] error: 1 errors occurred comparing output.
[01:17:59] status: exit code: 1
[01:17:59] status: exit code: 1
[01:17:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/dump-fn-method.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/dump-fn-method/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/dump-fn-method/auxiliary" "-A" "unused"
[01:17:59] ------------------------------------------
[01:17:59] 
[01:17:59] ------------------------------------------
[01:17:59] stderr:
[01:17:59] stderr:
[01:17:59] ------------------------------------------
[01:17:59] {"message":"user substs: Canonical { max_universe: U0, variables: [], value: UserSubsts { substs: [u32], user_self_ty: None } }","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/dump-fn-method.rs","byte_start":975,"byte_end":985,"line_start":36,"line_end":36,"column_start":13,"column_end":23,"is_primary":true,"text":[{"text":"    let x = foo::<u32>; //~ ERROR [u32]","highlight_start":13,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: user substs: Canonical { max_universe: U0, variables: [], value: UserSubsts { substs: [u32], user_self_ty: None } }\n  --> /checkout/src/test/ui/nll/user-annotations/dump-fn-method.rs:36:13\n   |\nLL |     let x = foo::<u32>; //~ ERROR [u32]\n   |             ^^^^^^^^^^\n\n"}
[01:17:59] {"message":"user substs: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }], value: UserSubsts { substs: [^0, u32, ^1], user_self_ty: None } }","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/dump-fn-method.rs","byte_start":1162,"byte_end":1193,"line_start":42,"line_end":42,"column_start":13,"column_end":44,"is_primary":true,"text":[{"text":"    let x = <_ as Bazoom<u32>>::method::<_>; //~ ERROR [^0, u32, ^1]","highlight_start":13,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: user substs: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }], value: UserSubsts { substs: [^0, u32, ^1], user_self_ty: None } }\n  --> /checkout/src/test/ui/nll/user-annotations/dump-fn-method.rs:42:13\n   |\nLL |     let x = <_ as Bazoom<u32>>::method::<_>; //~ ERROR [^0, u32, ^1]\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:17:59] {"message":"user substs: Canonical { max_universe: U0, variables: [], value: UserSubsts { substs: [u8, u16, u32], user_self_ty: None } }","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/dump-fn-method.rs","byte_start":1279,"byte_end":1313,"line_start":46,"line_end":46,"column_start":13,"column_end":47,"is_primary":true,"text":[{"text":"    let x = <u8 as Bazoom<u16>>::method::<u32>; //~ ERROR [u8, u16, u32]","highlight_start":13,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: user substs: Canonical { max_universe: U0, variables: [], value: UserSubsts { substs: [u8, u16, u32], user_self_ty: None } }\n  --> /checkout/src/test/ui/nll/user-annotations/dump-fn-method.rs:46:13\n   |\nLL |     let x = <u8 as Bazoom<u16>>::method::<u32>; //~ ERROR [u8, u16, u32]\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:17:59] {"message":"user substs: Canonical { max_universe: U1, variables: [CanonicalVarInfo { kind: Ty(General(U1)) }, CanonicalVarInfo { kind: Ty(General(U1)) }], value: UserSubsts { substs: [^0, ^1, u32], user_self_ty: None } }","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/dump-fn-method.rs","byte_start":1557,"byte_end":1580,"line_start":54,"line_end":54,"column_start":5,"column_end":28,"is_primary":true,"text":[{"text":"    y.method::<u32>(44, 66); //~ ERROR [^0, ^1, u32]","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: user substs: Canonical { max_universe: U1, variables: [CanonicalVarInfo { kind: Ty(General(U1)) }, CanonicalVarInfo { kind: Ty(General(U1)) }], value: UserSubsts { substs: [^0, ^1, u32], user_self_ty: None } }\n  --> /checkout/src/test/ui/nll/user-annotations/dump-fn-method.rs:54:5\n   |\nLL |     y.method::<u32>(44, 66); //~ ERROR [^0, ^1, u32]\n   |     ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:17:59] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[01:17:59] ------------------------------------------
[01:17:59] 
[01:17:59] thread '[ui] ui/nll/user-annotations/dump-fn-method.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[01:17:59] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:17:59] 
[01:17:59] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:17:59] 
[01:17:59] 
[01:17:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:17:59] 
[01:17:59] 
[01:17:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:59] Build completed unsuccessfully in 0:03:37
[01:17:59] Build completed unsuccessfully in 0:03:37
[01:17:59] make: *** [check] Error 1
[01:17:59] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0232d948
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Nov 13 16:36:35 UTC 2018
---
travis_time:end:2fa34044:start=1542126996392344754,finish=1542126996405053521,duration=12708767
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:001f7660
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:043c0cdc
$ dmesg | grep -i kill
