plain
travis_time:end:0910872c:start=1549257012648784783,finish=1549257089502524442,duration=76853739659
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:02:31] .................................................................................................... 2700/5367
[01:02:35] .................................................................................................... 2800/5367
[01:02:39] .................................................................................................... 2900/5367
[01:02:43] .................................................................................................... 3000/5367
[01:02:46] .............................................................F...................................... 3100/5367
[01:02:50] ...............................................F.................................................... 3200/5367
[01:02:57] ..........................................................................................ii...i..ii 3400/5367
[01:03:01] .................................................................................................... 3500/5367
[01:03:05] .................................................................................................... 3600/5367
[01:03:08] .......................................................................................ii........... 3700/5367
---
[01:04:09] .................................................................................................... 5300/5367
[01:04:11] ......i............................................................
[01:04:11] failures:
[01:04:11] 
[01:04:11] ---- [ui] ui/lint/group-missing-impls.rs stdout ----
[01:04:11] 
[01:04:11] 11    |         ^^^^^^^^^^^^^
[01:04:11] 11    |         ^^^^^^^^^^^^^
[01:04:11] 12    = note: #[deny(missing_debug_implementations)] implied by #[deny(missing_impls)]
[01:04:11] 13 
[01:04:11] + error: type does not implement `Eq`; consider adding #[derive(Eq)] or a manual implementation
[01:04:11] +   --> $DIR/group-missing-impls.rs:6:1
[01:04:11] +    |
[01:04:11] + LL | pub struct A; //~ ERROR type does not implement `fmt::Debug`
[01:04:11] +    |
[01:04:11] + note: lint level defined here
[01:04:11] +   --> $DIR/group-missing-impls.rs:2:9
[01:04:11] +    |
[01:04:11] +    |
[01:04:11] + LL | #![deny(missing_impls)]
[01:04:11] +    |         ^^^^^^^^^^^^^
[01:04:11] +    = note: #[deny(missing_eq_implementations)] implied by #[deny(missing_impls)]
[01:04:11] + 
[01:04:11] 14 error: type does not implement `Copy`; consider adding #[derive(Copy)] or a manual implementation
[01:04:11] 15   --> $DIR/group-missing-impls.rs:9:1
[01:04:11] 
[01:04:11] 37    |         ^^^^^^^^^^^^^
[01:04:11] 37    |         ^^^^^^^^^^^^^
[01:04:11] 38    = note: #[deny(missing_clone_implementations)] implied by #[deny(missing_impls)]
[01:04:11] 39 
[01:04:11] + error: type does not implement `Eq`; consider adding #[derive(Eq)] or a manual implementation
[01:04:11] +   --> $DIR/group-missing-impls.rs:9:1
[01:04:11] + LL | pub struct B;
[01:04:11] +    | ^^^^^^^^^^^^^
[01:04:11] + 
[01:04:11] + 
[01:04:11] 40 error: type does not implement `Copy`; consider adding #[derive(Copy)] or a manual implementation
[01:04:11] 41   --> $DIR/group-missing-impls.rs:14:1
[01:04:11] 
[01:04:11] 
[01:04:11] 43 LL | pub struct C; //~ ERROR type does not implement `Copy`
[01:04:11] 45 
[01:04:11] - error: aborting due to 4 previous errors
[01:04:11] - error: aborting due to 4 previous errors
[01:04:11] + error: type does not implement `Eq`; consider adding #[derive(Eq)] or a manual implementation
[01:04:11] +   --> $DIR/group-missing-impls.rs:14:1
[01:04:11] +    |
[01:04:11] + LL | pub struct C; //~ ERROR type does not implement `Copy`
[01:04:11] + 
[01:04:11] + 
[01:04:11] + error: type does not implement `Eq`; consider adding #[derive(Eq)] or a manual implementation
[01:04:11] +   --> $DIR/group-missing-impls.rs:17:1
[01:04:11] + LL | pub struct D;
[01:04:11] +    | ^^^^^^^^^^^^^
[01:04:11] + 
[01:04:11] + error: aborting due to 8 previous errors
[01:04:11] + error: aborting due to 8 previous errors
[01:04:11] 47 
[01:04:11] 48 
[01:04:11] 
[01:04:11] 
[01:04:11] The actual stderr differed from the expected stderr.
[01:04:11] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/group-missing-impls/group-missing-impls.stderr
[01:04:11] To update references, rerun the tests and pass the `--bless` flag
[01:04:11] To only update this specific test, also pass `--test-args lint/group-missing-impls.rs`
[01:04:11] error: 1 errors occurred comparing output.
[01:04:11] status: exit code: 1
[01:04:11] status: exit code: 1
[01:04:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/group-missing-impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/group-missing-impls/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/group-missing-impls/auxiliary" "-A" "unused"
[01:04:11] ------------------------------------------
[01:04:11] 
[01:04:11] ------------------------------------------
[01:04:11] stderr:
[01:04:11] stderr:
[01:04:11] ------------------------------------------
[01:04:11] {"message":"type does not implement `fmt::Debug`; consider adding #[derive(Debug)] or a manual implementation","code":{"code":"missing_debug_implementations","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/group-missing-impls.rs","byte_start":104,"byte_end":117,"line_start":6,"line_end":6,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"pub struct A; //~ ERROR type does not implement `fmt::Debug`","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/group-missing-impls.rs","byte_start":43,"byte_end":56,"line_start":2,"line_end":2,"column_start":9,"column_end":22,"is_primary":true,"text":[{"text":"#![deny(missing_impls)]","highlight_start":9,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"#[deny(missing_debug_implementations)] implied by #[deny(missing_impls)]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: type does not implement `fmt::Debug`; consider adding #[derive(Debug)] or a manual implementation\n  --> /checkout/src/test/ui/lint/group-missing-impls.rs:6:1\n   |\nLL | pub struct A; //~ ERROR type does not implement `fmt::Debug`\n   | ^^^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/group-missing-impls.rs:2:9\n   |\nLL | #![deny(missing_impls)]\n   |         ^^^^^^^^^^^^^\n   = note: #[deny(missing_debug_implementations)] implied by #[deny(missing_impls)]\n\n"}
[01:04:11] {"message":"type does not implement `Eq`; consider adding #[derive(Eq)] or a manual implementation","code":{"code":"missing_eq_implementations","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/group-missing-impls.rs","byte_start":104,"byte_end":117,"line_start":6,"line_end":6,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"pub struct A; //~ ERROR type does not implement `fmt::Debug`","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/group-missing-impls.rs","byte_start":43,"byte_end":56,"line_start":2,"line_end":2,"column_start":9,"column_end":22,"is_primary":true,"text":[{"text":"#![deny(missing_impls)]","highlight_start":9,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"#[deny(missing_eq_implementations)] implied by #[deny(missing_impls)]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: type does not implement `Eq`; consider adding #[derive(Eq)] or a manual implementation\n  --> /checkout/src/test/ui/lint/group-missing-impls.rs:6:1\n   |\nLL | pub struct A; //~ ERROR type does not implement `fmt::Debug`\n   | ^^^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/group-missing-impls.rs:2:9\n   |\nLL | #![deny(missing_impls)]\n   |         ^^^^^^^^^^^^^\n   = note: #[deny(missing_eq_implementations)] implied by #[deny(missing_impls)]\n\n"}
[01:04:11] {"message":"type does not implement `Copy`; consider adding #[derive(Copy)] or a manual implementation","code":{"code":"missing_copy_implementations","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/group-missing-impls.rs","byte_start":183,"byte_end":196,"line_start":9,"line_end":9,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"pub struct B;","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/group-missing-impls.rs","byte_start":43,"byte_end":56,"line_start":2,"line_end":2,"column_start":9,"column_end":22,"is_primary":true,"text":[{"text":"#![deny(missing_impls)]","highlight_start":9,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"#[deny(missing_copy_implementations)] implied by #[deny(missing_impls)]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: type does not implement `Copy`; consider adding #[derive(Copy)] or a manual implementation\n  --> /checkout/src/test/ui/lint/group-missing-impls.rs:9:1\n   |\nLL | pub struct B;\n   | ^^^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/group-missing-impls.rs:2:9\n   |\nLL | #![deny(missing_impls)]\n   |         ^^^^^^^^^^^^^\n   = note: #[deny(missing_copy_implementations)] implied by #[deny(missing_impls)]\n\n"}
[01:04:11] {"message":"type does not implement `Clone`; consider adding #[derive(Clone)] or a manual implementation","code":{"code":"missing_clone_implementations","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/group-missing-impls.rs","byte_start":183,"byte_end":196,"line_start":9,"line_end":9,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"pub struct B;","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/group-missing-impls.rs","byte_start":43,"byte_end":56,"line_start":2,"line_end":2,"column_start":9,"column_end":22,"is_primary":true,"text":[{"text":"#![deny(missing_impls)]","highlight_start":9,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"#[deny(missing_clone_implementations)] implied by #[deny(missing_impls)]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: type does not implement `Clone`; consider adding #[derive(Clone)] or a manual implementation\n  --> /checkout/src/test/ui/lint/group-missing-impls.rs:9:1\n   |\nLL | pub struct B;\n   | ^^^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/group-missing-impls.rs:2:9\n   |\nLL | #![deny(missing_impls)]\n   |         ^^^^^^^^^^^^^\n   = note: #[deny(missing_clone_implementations)] implied by #[deny(missing_impls)]\n\n"}
[01:04:11] {"message":"type does not implement `Eq`; consider adding #[derive(Eq)] or a manual implementation","code":{"code":"missing_eq_implementations","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/group-missing-impls.rs","byte_start":183,"byte_end":196,"line_start":9,"line_end":9,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"pub struct B;","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: type does not implement `Eq`; consider adding #[derive(Eq)] or a manual implementation\n  --> /checkout/src/test/ui/lint/group-missing-impls.rs:9:1\n   |\nLL | pub struct B;\n   | ^^^^^^^^^^^^^\n\n"}
[01:04:11] {"message":"type does not implement `Copy`; consider adding #[derive(Copy)] or a manual implementation","code":{"code":"missing_copy_implementations","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/group-missing-impls.rs","byte_start":307,"byte_end":320,"line_start":14,"line_end":14,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"pub struct C; //~ ERROR type does not implement `Copy`","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: type does not implement `Copy`; consider adding #[derive(Copy)] or a manual implementation\n  --> /checkout/src/test/ui/lint/group-missing-impls.rs:14:1\n   |\nLL | pub struct C; //~ ERROR type does not implement `Copy`\n   | ^^^^^^^^^^^^^\n\n"}
[01:04:11] {"message":"type does not implement `Eq`; consider adding #[derive(Eq)] or a manual implementation","code":{"code":"missing_eq_implementations","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/group-missing-impls.rs","byte_start":307,"byte_end":320,"line_start":14,"line_end":14,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"pub struct C; //~ ERROR type does not implement `Copy`","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: type does not implement `Eq`; consider adding #[derive(Eq)] or a manual implementation\n  --> /checkout/src/test/ui/lint/group-missing-impls.rs:14:1\n   |\nLL | pub struct C; //~ ERROR type does not implement `Copy`\n   | ^^^^^^^^^^^^^\n\n"}
[01:04:11] {"message":"type does not implement `Eq`; consider adding #[derive(Eq)] or a manual implementation","code":{"code":"missing_eq_implementations","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/group-missing-impls.rs","byte_start":393,"byte_end":406,"line_start":17,"line_end":17,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"pub struct D;","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: type does not implement `Eq`; consider adding #[derive(Eq)] or a manual implementation\n  --> /checkout/src/test/ui/lint/group-missing-impls.rs:17:1\n   |\nLL | pub struct D;\n   | ^^^^^^^^^^^^^\n\n"}
[01:04:11] 
[01:04:11] ------------------------------------------
[01:04:11] 
[01:04:11] 
[01:04:11] thread '[ui] ui/lint/group-missing-impls.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:04:11] 
[01:04:11] ---- [ui] ui/lint/missing-eq-implementations.rs stdout ----
[01:04:11] diff of stderr:
[01:04:11] 
[01:04:11] 
[01:04:11] 11    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:04:11] 12 
[01:04:11] 13 error: type does not implement `Eq`; consider adding #[derive(Eq)] or a manual implementation
[01:04:11] -   --> $DIR/missing-eq-implementations.rs:16:1
[01:04:11] -    |
[01:04:11] - LL | pub enum D {} //~ ERROR type does not implement `Eq`
[01:04:11] - 
[01:04:11] - 
[01:04:11] - error: type does not implement `Eq`; consider adding #[derive(Eq)] or a manual implementation
[01:04:11] 20   --> $DIR/missing-eq-implementations.rs:18:1
[01:04:11] 21    |
[01:04:11] 22 LL | pub struct Foo; //~ ERROR type does not implement `Eq`
[01:04:11] 23    | ^^^^^^^^^^^^^^^
[01:04:11] 24 
[01:04:11] 24 
[01:04:11] - error: type does not implement `Eq`; consider adding #[derive(Eq)] or a manual implementation
[01:04:11] -   --> $DIR/missing-eq-implementations.rs:29:1
[01:04:11] -    |
[01:04:11] - LL | pub struct Qux; //~ ERROR type does not implement `Eq`
[01:04:11] - 
[01:04:11] - error: aborting due to 4 previous errors
[01:04:11] + error: aborting due to 2 previous errors
[01:04:11] 32 
[01:04:11] 32 
[01:04:11] 33 
[01:04:11] 
[01:04:11] 
[01:04:11] The actual stderr differed from the expected stderr.
[01:04:11] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/missing-eq-implementations/missing-eq-implementations.stderr
[01:04:11] To update references, rerun the tests and pass the `--bless` flag
[01:04:11] To only update this specific test, also pass `--test-args lint/missing-eq-implementations.rs`
[01:04:11] error: 1 errors occurred comparing output.
[01:04:11] status: exit code: 1
[01:04:11] status: exit code: 1
[01:04:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/missing-eq-implementations.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/missing-eq-implementations/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/missing-eq-implementations/auxiliary" "-A" "unused"
[01:04:11] ------------------------------------------
[01:04:11] 
[01:04:11] ------------------------------------------
[01:04:11] stderr:
[01:04:11] stderr:
[01:04:11] ------------------------------------------
[01:04:11] {"message":"type does not implement `Eq`; consider adding #[derive(Eq)] or a manual implementation","code":{"code":"missing_eq_implementations","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/missing-eq-implementations.rs","byte_start":91,"byte_end":104,"line_start":5,"line_end":5,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"pub enum A {} //~ ERROR type does not implement `Eq`","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/missing-eq-implementations.rs","byte_start":43,"byte_end":69,"line_start":2,"line_end":2,"column_start":9,"column_end":35,"is_primary":true,"text":[{"text":"#![deny(missing_eq_implementations)]","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: type does not implement `Eq`; consider adding #[derive(Eq)] or a manual implementation\n  --> /checkout/src/test/ui/lint/missing-eq-implementations.rs:5:1\n   |\nLL | pub enum A {} //~ ERROR type does not implement `Eq`\n   | ^^^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/missing-eq-implementations.rs:2:9\n   |\nLL | #![deny(missing_eq_implementations)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:04:11] {"message":"type does not implement `Eq`; consider adding #[derive(Eq)] or a manual implementation","code":{"code":"missing_eq_implementations","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/missing-eq-implementations.rs","byte_start":314,"byte_end":329,"line_start":18,"line_end":18,"column_start":1,"column_end":16,"is_primary":true,"text":[{"text":"pub struct Foo; //~ ERROR type does not implement `Eq`","highlight_start":1,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: type does not implement `Eq`; consider adding #[derive(Eq)] or a manual implementation\n  --> /checkout/src/test/ui/lint/missing-eq-implementations.rs:18:1\n   |\nLL | pub struct Foo; //~ ERROR type does not implement `Eq`\n   | ^^^^^^^^^^^^^^^\n\n"}
[01:04:11] 
[01:04:11] ------------------------------------------
[01:04:11] 
[01:04:11] thread '[ui] ui/lint/missing-eq-implementations.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:04:11] thread '[ui] ui/lint/missing-eq-implementations.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:04:11] 
[01:04:11] 
[01:04:11] failures:
[01:04:11]     [ui] ui/lint/group-missing-impls.rs
[01:04:11]     [ui] ui/lint/missing-eq-implementations.rs
[01:04:11] test result: FAILED. 5342 passed; 2 failed; 23 ignored; 0 measured; 0 filtered out
[01:04:11] 
[01:04:11] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:04:11] 
[01:04:11] 
[01:04:11] 
[01:04:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:11] 
[01:04:11] 
[01:04:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:11] Build completed unsuccessfully in 0:04:15
[01:04:11] Build completed unsuccessfully in 0:04:15
[01:04:11] make: *** [check] Error 1
[01:04:11] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b647dc7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb  4 06:15:50 UTC 2019
---
travis_time:end:30796788:start=1549260952034832792,finish=1549260952039911318,duration=5078526
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:038549a6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:086338e4
travis_time:start:086338e4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0272f5ef
$ dmesg | grep -i kill
