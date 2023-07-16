plain
[01:07:19] failures:
[01:07:19] 
[01:07:19] ---- [ui] ui/borrowck/borrowck-migrate-to-nll.rs#edition stdout ----
[01:07:19] 
[01:07:19] error in revision `edition`: test run failed!
[01:07:19] status: exit code: 1
[01:07:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.edition/a"
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.edition/a", waiting for result
[01:07:19] ------------------------------------------
[01:07:19] stderr:
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test0/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] thread '[ui] ui/borrowck/borrowck-migrate-to-nll.rs#edition' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:07:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:07:19] 
[01:07:19] ---- [ui] ui/borrowck/borrowck-migrate-to-nll.rs#zflag stdout ----
[01:07:19] 
[01:07:19] error in revision `zflag`: test run failed!
[01:07:19] status: exit code: 1
[01:07:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.zflag/a"
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.zflag/a", waiting for result
[01:07:19] ------------------------------------------
[01:07:19] stderr:
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test1/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] thread '[ui] ui/borrowck/borrowck-migrate-to-nll.rs#zflag' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:19] 
[01:07:19] 
[01:07:19] ---- [ui] ui/borrowck/issue-51348-multi-ref-mut-in-guard.rs stdout ----
[01:07:19] 
[01:07:19] error: test run failed!
[01:07:19] status: exit code: 1
[01:07:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-51348-multi-ref-mut-in-guard/a"
[01:07:19] ------------------------------------------
[01:07:19] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-51348-multi-ref-mut-in-guard/a", waiting for result
[01:07:19] 
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] stderr:
[01:07:19] ------------------------------------------
[01:07:19] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test2/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] thread '[ui] ui/borrowck/issue-51348-multi-ref-mut-in-guard.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:19] 
[01:07:19] 
[01:07:19] ---- [ui] ui/borrowck/issue-52967-edition-2018-needs-two-phase-borrows.rs#edition stdout ----
[01:07:19] 
[01:07:19] error in revision `edition`: test run failed!
[01:07:19] status: exit code: 1
[01:07:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-52967-edition-2018-needs-two-phase-borrows.edition/a"
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-52967-edition-2018-needs-two-phase-borrows.edition/a", waiting for result
[01:07:19] ------------------------------------------
[01:07:19] stderr:
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test4/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] thread '[ui] ui/borrowck/issue-52967-edition-2018-needs-two-phase-borrows.rs#edition' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:19] 
[01:07:19] 
[01:07:19] ---- [ui] ui/borrowck/issue-52967-edition-2018-needs-two-phase-borrows.rs#ast stdout ----
[01:07:19] 
[01:07:19] error in revision `ast`: test run failed!
[01:07:19] status: exit code: 1
[01:07:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-52967-edition-2018-needs-two-phase-borrows.ast/a"
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-52967-edition-2018-needs-two-phase-borrows.ast/a", waiting for result
[01:07:19] ------------------------------------------
[01:07:19] stderr:
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test3/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] thread '[ui] ui/borrowck/issue-52967-edition-2018-needs-two-phase-borrows.rs#ast' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:19] 
[01:07:19] 
[01:07:19] ---- [ui] ui/borrowck/issue-52967-edition-2018-needs-two-phase-borrows.rs#zflags stdout ----
[01:07:19] 
[01:07:19] error in revision `zflags`: test run failed!
[01:07:19] status: exit code: 1
[01:07:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-52967-edition-2018-needs-two-phase-borrows.zflags/a"
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-52967-edition-2018-needs-two-phase-borrows.zflags/a", waiting for result
[01:07:19] ------------------------------------------
[01:07:19] stderr:
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test5/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] thread '[ui] ui/borrowck/issue-52967-edition-2018-needs-two-phase-borrows.rs#zflags' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:19] 
[01:07:19] 
[01:07:19] ---- [ui] ui/borrowck/two-phase-method-receivers.rs stdout ----
[01:07:19] 
[01:07:19] error: test run failed!
[01:07:19] status: exit code: 1
[01:07:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-method-receivers/a"
[01:07:19] ------------------------------------------
[01:07:19] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-method-receivers/a", waiting for result
[01:07:19] 
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] stderr:
[01:07:19] ------------------------------------------
[01:07:19] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test6/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] thread '[ui] ui/borrowck/two-phase-method-receivers.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:19] 
[01:07:19] 
[01:07:19] ---- [ui] ui/borrowck/two-phase-multiple-activations.rs stdout ----
[01:07:19] 
[01:07:19] error: test run failed!
[01:07:19] status: exit code: 1
[01:07:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-multiple-activations/a"
[01:07:19] ------------------------------------------
[01:07:19] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-multiple-activations/a", waiting for result
[01:07:19] 
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] stderr:
[01:07:19] ------------------------------------------
[01:07:19] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test7/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] thread '[ui] ui/borrowck/two-phase-multiple-activations.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:19] 
[01:07:19] 
[01:07:19] ---- [ui] ui/consts/const-eval/const_transmute.rs stdout ----
[01:07:19] 
[01:07:19] error: test run failed!
[01:07:19] status: exit code: 1
[01:07:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_transmute/a"
[01:07:19] ------------------------------------------
[01:07:19] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_transmute/a", waiting for result
[01:07:19] 
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] stderr:
[01:07:19] ------------------------------------------
[01:07:19] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test8/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] thread '[ui] ui/consts/const-eval/const_transmute.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:19] 
[01:07:19] 
[01:07:19] ---- [ui] ui/consts/const-eval/enum_discr.rs stdout ----
[01:07:19] 
[01:07:19] error: test run failed!
[01:07:19] status: exit code: 1
[01:07:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/enum_discr/a"
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/enum_discr/a", waiting for result
[01:07:19] ------------------------------------------
[01:07:19] stderr:
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test9/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] thread '[ui] ui/consts/const-eval/enum_discr.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:19] 
[01:07:19] 
[01:07:19] ---- [ui] ui/consts/const-eval/strlen.rs stdout ----
[01:07:19] 
[01:07:19] error: test run failed!
[01:07:19] status: exit code: 1
[01:07:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/strlen/a"
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/strlen/a", waiting for result
[01:07:19] ------------------------------------------
[01:07:19] stderr:
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test10/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] thread '[ui] ui/consts/const-eval/strlen.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:19] 
[01:07:19] 
[01:07:19] ---- [ui] ui/custom-test-frameworks-simple.rs stdout ----
[01:07:19] 
[01:07:19] error: test run failed!
[01:07:19] status: exit code: 1
[01:07:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom-test-frameworks-simple/a"
[01:07:19] ------------------------------------------
[01:07:19] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom-test-frameworks-simple/a", waiting for result
[01:07:19] 
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] stderr:
[01:07:19] ------------------------------------------
[01:07:19] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test11/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] thread '[ui] ui/custom-test-frameworks-simple.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:19] 
[01:07:19] 
[01:07:19] ---- [ui] ui/custom_test_frameworks/full.rs stdout ----
[01:07:19] 
[01:07:19] error: test run failed!
[01:07:19] status: exit code: 1
[01:07:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/full/a:/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/full/auxiliary/libexample_runner.so"
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/full/a", waiting for result
[01:07:19] ------------------------------------------
[01:07:19] stderr:
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] CANNOT LINK EXECUTABLE: could not load library "libexample_runner.so" needed by "/data/tmp/work/test12/a"; caused by could not load library "libstd-f77e650adb99256d.so" needed by "libexample_runner.so"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] thread '[ui] ui/custom_test_frameworks/full.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:19] 
[01:07:19] 
[01:07:19] ---- [ui] ui/deprecation/deprecated-macro_escape-inner.rs stdout ----
[01:07:19] 
[01:07:19] error: test run failed!
[01:07:19] status: exit code: 1
[01:07:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deprecation/deprecated-macro_escape-inner/a"
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deprecation/deprecated-macro_escape-inner/a", waiting for result
[01:07:19] ------------------------------------------
[01:07:19] stderr:
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test13/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] thread '[ui] ui/deprecation/deprecated-macro_escape-inner.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:19] 
[01:07:19] 
[01:07:19] ---- [ui] ui/deprecation/deprecated-macro_escape.rs stdout ----
[01:07:19] 
[01:07:19] error: test run failed!
[01:07:19] status: exit code: 1
[01:07:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deprecation/deprecated-macro_escape/a"
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deprecation/deprecated-macro_escape/a", waiting for result
[01:07:19] ------------------------------------------
[01:07:19] stderr:
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test14/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] thread '[ui] ui/deprecation/deprecated-macro_escape.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:19] 
[01:07:19] 
[01:07:19] ---- [ui] ui/custom_test_frameworks/dynamic.rs stdout ----
[01:07:19] 
[01:07:19] error: test run failed!
[01:07:19] status: exit code: 1
[01:07:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/dynamic/a:/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/dynamic/auxiliary/libdynamic_runner.so"
[01:07:19] ------------------------------------------
[01:07:19] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/dynamic/a", waiting for result
[01:07:19] 
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] stderr:
[01:07:19] ------------------------------------------
[01:07:19] CANNOT LINK EXECUTABLE: could not load library "libdynamic_runner.so" needed by "/data/tmp/work/test15/a"; caused by could not load library "libstd-f77e650adb99256d.so" needed by "libdynamic_runner.so"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] thread '[ui] ui/custom_test_frameworks/dynamic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:19] 
[01:07:19] 
[01:07:19] ---- [ui] ui/derives/deriving-meta-empty-trait-list.rs stdout ----
[01:07:19] 
[01:07:19] error: test run failed!
[01:07:19] status: exit code: 1
[01:07:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-meta-empty-trait-list/a"
[01:07:19] ------------------------------------------
[01:07:19] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-meta-empty-trait-list/a", waiting for result
[01:07:19] 
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] stderr:
[01:07:19] ------------------------------------------
[01:07:19] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test16/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] thread '[ui] ui/derives/deriving-meta-empty-trait-list.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:19] 
[01:07:19] 
[01:07:19] ---- [ui] ui/enum/enum-size-variance.rs stdout ----
[01:07:19] 
[01:07:19] error: test run failed!
[01:07:19] status: exit code: 1
[01:07:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum/enum-size-variance/a"
[01:07:19] ------------------------------------------
[01:07:19] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum/enum-size-variance/a", waiting for result
[01:07:19] 
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] stderr:
[01:07:19] ------------------------------------------
[01:07:19] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test17/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:19] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] thread '[ui] ui/enum/enum-size-variance.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:19] thread '[ui] ui/enum/enum-size-variance.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:19] 
[01:07:19] ---- [ui] ui/feature-gates/feature-gate-trivial_bounds-lint.rs stdout ----
[01:07:19] 
[01:07:19] error: test run failed!
[01:07:19] status: exit code: 1
[01:07:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-trivial_bounds-lint/a"
[01:07:19] ------------------------------------------
[01:07:19] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-trivial_bounds-lint/a", waiting for result
[01:07:19] 
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] stderr:
[01:07:19] ------------------------------------------
[01:07:19] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test18/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] thread '[ui] ui/feature-gates/feature-gate-trivial_bounds-lint.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:19] 
[01:07:19] 
[01:07:19] ---- [ui] ui/impl-header-lifetime-elision/explicit-and-elided-same-header.rs stdout ----
[01:07:19] 
[01:07:19] error: test run failed!
[01:07:19] status: exit code: 1
[01:07:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-header-lifetime-elision/explicit-and-elided-same-header/a"
[01:07:19] ------------------------------------------
[01:07:19] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-header-lifetime-elision/explicit-and-elided-same-header/a", waiting for result
[01:07:19] 
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] stderr:
[01:07:19] ------------------------------------------
[01:07:19] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test19/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] thread '[ui] ui/impl-header-lifetime-elision/explicit-and-elided-same-header.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:19] 
[01:07:19] 
[01:07:19] ---- [ui] ui/impl-header-lifetime-elision/path-underscore.rs stdout ----
[01:07:19] 
[01:07:19] error: test run failed!
[01:07:19] status: exit code: 1
[01:07:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-header-lifetime-elision/path-underscore/a"
[01:07:19] ------------------------------------------
[01:07:19] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-header-lifetime-elision/path-underscore/a", waiting for result
[01:07:19] 
[01:07:19] ------------------------------------------
[01:07:19] ------------------------------------------
[01:07:19] stderr:
[01:07:19] ------------------------------------------
[01:07:19] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test20/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:19] ------------------------------------------
[01:07:19] 
[01:07:19] thread '[ui] ui/impl-header-lifetime-elision/path-underscore.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:20] 
[01:07:20] 
[01:07:20] ---- [ui] ui/impl-header-lifetime-elision/ref-underscore.rs stdout ----
[01:07:20] 
[01:07:20] error: test run failed!
[01:07:20] status: exit code: 1
[01:07:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-header-lifetime-elision/ref-underscore/a"
[01:07:20] ------------------------------------------
[01:07:20] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-header-lifetime-elision/ref-underscore/a", waiting for result
[01:07:20] 
[01:07:20] ------------------------------------------
[01:07:20] ------------------------------------------
[01:07:20] stderr:
[01:07:20] ------------------------------------------
[01:07:20] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test21/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:20] ------------------------------------------
[01:07:20] 
[01:07:20] thread '[ui] ui/impl-header-lifetime-elision/ref-underscore.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:20] 
[01:07:20] 
[01:07:20] ---- [ui] ui/impl-header-lifetime-elision/trait-underscore.rs stdout ----
[01:07:20] 
[01:07:20] error: test run failed!
[01:07:20] status: exit code: 1
[01:07:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-header-lifetime-elision/trait-underscore/a"
[01:07:20] ------------------------------------------
[01:07:20] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-header-lifetime-elision/trait-underscore/a", waiting for result
[01:07:20] 
[01:07:20] ------------------------------------------
[01:07:20] ------------------------------------------
[01:07:20] stderr:
[01:07:20] ------------------------------------------
[01:07:20] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test22/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:20] ------------------------------------------
[01:07:20] 
[01:07:20] thread '[ui] ui/impl-header-lifetime-elision/trait-underscore.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:20] 
[01:07:20] 
[01:07:20] ---- [ui] ui/impl-trait/closure-calling-parent-fn.rs stdout ----
[01:07:20] 
[01:07:20] error: test run failed!
[01:07:20] status: exit code: 1
[01:07:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/closure-calling-parent-fn/a"
[01:07:20] ------------------------------------------
[01:07:20] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/closure-calling-parent-fn/a", waiting for result
[01:07:20] 
[01:07:20] ------------------------------------------
[01:07:20] ------------------------------------------
[01:07:20] stderr:
[01:07:20] ------------------------------------------
[01:07:20] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test23/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:20] ------------------------------------------
[01:07:20] 
[01:07:20] thread '[ui] ui/impl-trait/closure-calling-parent-fn.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:20] 
[01:07:20] 
[01:07:20] ---- [ui] ui/impl-trait/region-escape-via-bound-contravariant-closure.rs stdout ----
[01:07:20] 
[01:07:20] error: test run failed!
[01:07:20] status: exit code: 1
[01:07:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/region-escape-via-bound-contravariant-closure/a"
[01:07:20] ------------------------------------------
[01:07:20] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/region-escape-via-bound-contravariant-closure/a", waiting for result
[01:07:20] 
[01:07:20] ------------------------------------------
[01:07:20] ------------------------------------------
[01:07:20] stderr:
[01:07:20] ------------------------------------------
[01:07:20] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test24/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:20] ------------------------------------------
[01:07:20] 
[01:07:20] thread '[ui] ui/impl-trait/region-escape-via-bound-contravariant-closure.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:20] 
[01:07:20] 
[01:07:20] ---- [ui] ui/impl-trait/region-escape-via-bound-contravariant.rs stdout ----
[01:07:20] 
[01:07:20] error: test run failed!
[01:07:20] status: exit code: 1
[01:07:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/region-escape-via-bound-contravariant/a"
[01:07:20] ------------------------------------------
[01:07:20] ------------------------------------------
[01:07:20] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/region-escape-via-bound-contravariant/a", waiting for result
[01:07:20] ------------------------------------------
[01:07:20] stderr:
[01:07:20] ------------------------------------------
[01:07:20] ------------------------------------------
[01:07:20] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test25/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:20] ------------------------------------------
[01:07:20] 
[01:07:20] thread '[ui] ui/impl-trait/region-escape-via-bound-contravariant.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:20] 
[01:07:20] 
[01:07:20] ---- [ui] ui/inference/inference_unstable.rs stdout ----
[01:07:20] 
[01:07:20] error: test run failed!
[01:07:20] status: exit code: 1
[01:07:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/inference_unstable/a:/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/inference_unstable/auxiliary/libinference_unstable_itertools.so:/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/inference_unstable/auxiliary/libinference_unstable_iterator.so"
[01:07:20] ------------------------------------------
[01:07:20] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/inference_unstable/a", waiting for result
[01:07:20] 
[01:07:20] ------------------------------------------
[01:07:20] ------------------------------------------
[01:07:20] stderr:
[01:07:20] ------------------------------------------
[01:07:20] CANNOT LINK EXECUTABLE: could not load library "libstd-f77e650adb99256d.so" needed by "/data/tmp/work/test26/a"; caused by cannot locate symbol "__sync_val_compare_and_swap_8" referenced by "libstd-f77e650adb99256d.so"...
[01:07:20] ------------------------------------------
[01:07:20] 
[01:07:20] thread '[ui] ui/inference/inference_unstable.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:07:20] 
[01:07:20] 
[01:07:20] ---- [ui] ui/issues/issue-17905.rs stdout ----
[01:07:20] 
[01:07:20] error: test run failed!
[01:07:20] status: exit code: 1
[01:07:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17905/a"
[01:07:20] ------------------------------------------
[01:07:20] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17905/a", waiting for result
[01:07:20] 
[01:07:20] ------------------------------------------
[01:07:20] ------------------------------------------
---
[01:07:20] test result: FAILED. 4865 passed; 79 failed; 33 ignored; 0 measured; 0 filtered out
[01:07:20] 
[01:07:20] 
[01:07:20] 
[01:07:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-arm-linux-androideabi" "--mode" "ui" "--target" "arm-linux-androideabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "/android/ndk/arm-14" "--color" "always"
[01:07:20] 
[01:07:20] 
[01:07:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
[01:07:20] Build completed unsuccessfully in 0:56:10
---
travis_time:end:0663f35d:start=1540885447816334088,finish=1540885447834285619,duration=17951531
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:186531a8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04717e63
travis_time:start:04717e63
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:052c27d6
$ dmesg | grep -i kill
