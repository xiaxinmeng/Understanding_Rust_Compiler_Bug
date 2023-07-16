plain
2019-09-11T00:10:24.9647161Z test [ui] ui/hr-subtype/hr-subtype.rs#bound_inv_a_vs_bound_inv_b ... ok
2019-09-11T00:10:24.9647442Z test [ui] ui/hr-subtype/hr-subtype.rs#bound_inv_a_b_vs_bound_inv_a ... ok
2019-09-11T00:10:24.9647813Z test [ui] ui/hr-subtype/hr-subtype.rs#free_inv_x_vs_free_inv_y ... ok
2019-09-11T00:10:24.9648073Z test [ui] ui/hr-subtype/hr-subtype.rs#free_x_vs_free_x ... ok
2019-09-11T00:10:24.9648335Z test [ui] ui/hrtb/due-to-where-clause.rs ... ok
2019-09-11T00:10:24.9887298Z test [ui] ui/hrtb/hrtb-cache-issue-54302.rs ... ok
2019-09-11T00:10:25.0250456Z test [ui] ui/hrtb/hrtb-conflate-regions.rs ... ok
2019-09-11T00:10:25.0807191Z test [ui] ui/hrtb/hrtb-debruijn-in-receiver.rs ... ok
2019-09-11T00:10:25.1355520Z test [ui] ui/hrtb/hrtb-exists-forall-trait-contravariant.rs ... ok
---
2019-09-11T00:23:35.1809742Z test [ui (nll)] ui/hr-subtype/hr-subtype.rs#free_inv_x_vs_free_inv_y ... ok
2019-09-11T00:23:35.1809999Z test [ui (nll)] ui/hr-subtype/hr-subtype.rs#free_x_vs_free_x ... ok
2019-09-11T00:23:35.1810278Z test [ui (nll)] ui/hr-subtype/hr-subtype.rs#free_x_vs_free_y ... ok
2019-09-11T00:23:35.1810528Z test [ui (nll)] ui/hrtb/hrtb-cache-issue-54302.rs ... ok
2019-09-11T00:23:35.1865611Z test [ui (nll)] ui/hrtb/due-to-where-clause.rs ... FAILED
2019-09-11T00:23:35.2654998Z test [ui (nll)] ui/hrtb/hrtb-debruijn-in-receiver.rs ... ok
2019-09-11T00:23:35.3189054Z test [ui (nll)] ui/hrtb/hrtb-exists-forall-trait-contravariant.rs ... ok
2019-09-11T00:23:35.3714622Z test [ui (nll)] ui/hrtb/hrtb-exists-forall-trait-covariant.rs ... ok
2019-09-11T00:23:35.3949453Z test [ui (nll)] ui/hrtb/hrtb-exists-forall-fn.rs ... ok
---
2019-09-11T00:32:05.4395557Z test [ui (nll)] ui/wrapping-int-combinations.rs ... ok
2019-09-11T00:32:05.4400815Z 
2019-09-11T00:32:05.4401589Z failures:
2019-09-11T00:32:05.4431103Z 
2019-09-11T00:32:05.4431874Z ---- [ui (nll)] ui/hrtb/due-to-where-clause.rs stdout ----
2019-09-11T00:32:05.4432376Z error: ui test compiled successfully!
2019-09-11T00:32:05.4432585Z status: exit code: 0
2019-09-11T00:32:05.4432585Z status: exit code: 0
2019-09-11T00:32:05.4433554Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hrtb/due-to-where-clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/due-to-where-clause.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/due-to-where-clause.nll/auxiliary" "-A" "unused"
2019-09-11T00:32:05.4435471Z ------------------------------------------
2019-09-11T00:32:05.4435661Z 
2019-09-11T00:32:05.4436064Z ------------------------------------------
2019-09-11T00:32:05.4436262Z stderr:
2019-09-11T00:32:05.4436262Z stderr:
2019-09-11T00:32:05.4436662Z ------------------------------------------
2019-09-11T00:32:05.4436828Z 
2019-09-11T00:32:05.4437211Z ------------------------------------------
2019-09-11T00:32:05.4437379Z 
2019-09-11T00:32:05.4437510Z 
2019-09-11T00:32:05.4437668Z 
2019-09-11T00:32:05.4437978Z failures:
2019-09-11T00:32:05.4438293Z     [ui (nll)] ui/hrtb/due-to-where-clause.rs
2019-09-11T00:32:05.4438789Z test result: FAILED. 8938 passed; 1 failed; 70 ignored; 0 measured; 0 filtered out
2019-09-11T00:32:05.4439145Z 
2019-09-11T00:32:05.4466357Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-11T00:32:05.4466748Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-11T00:32:05.4466748Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-11T00:32:05.4481607Z 
2019-09-11T00:32:05.4481859Z 
2019-09-11T00:32:05.4483900Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-09-11T00:32:05.4485354Z 
2019-09-11T00:32:05.4485503Z 
2019-09-11T00:32:05.4493108Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-11T00:32:05.4493954Z Build completed unsuccessfully in 1:55:43
2019-09-11T00:32:05.4493954Z Build completed unsuccessfully in 1:55:43
2019-09-11T00:32:05.4555876Z == clock drift check ==
2019-09-11T00:32:05.4571017Z   local time: Wed Sep 11 00:32:05 UTC 2019
2019-09-11T00:32:05.6182877Z   network time: Wed, 11 Sep 2019 00:32:05 GMT
2019-09-11T00:32:05.6188041Z == end clock drift check ==
2019-09-11T00:32:06.5271040Z ##[error]Bash exited with code '1'.
2019-09-11T00:32:06.5307643Z ##[section]Starting: Upload CPU usage statistics
2019-09-11T00:32:06.5317037Z ==============================================================================
2019-09-11T00:32:06.5317158Z Task         : Bash
2019-09-11T00:32:06.5317246Z Description  : Run a Bash script on macOS, Linux, or Windows
