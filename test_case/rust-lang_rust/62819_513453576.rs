plain
2019-07-20T08:48:39.1063339Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-20T08:48:39.1231286Z ##[command]git config gc.auto 0
2019-07-20T08:48:39.8544219Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-20T08:48:39.8552048Z ##[command]git config --get-all http.proxy
2019-07-20T08:48:39.8557820Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62819/merge:refs/remotes/pull/62819/merge
---
2019-07-20T08:49:12.5073030Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-20T08:49:12.5073874Z 
2019-07-20T08:49:12.5074406Z   git checkout -b <new-branch-name>
2019-07-20T08:49:12.5074558Z 
2019-07-20T08:49:12.5074702Z HEAD is now at bdba43467 Merge 70d9041cce29a27cc370f8d9763c03a36e9f7fa3 into e9d22273283dce210b26362aa0dcc3fc10bf7e81
2019-07-20T08:49:12.5201523Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-20T08:49:12.5203720Z ==============================================================================
2019-07-20T08:49:12.5203763Z Task         : Bash
2019-07-20T08:49:12.5203813Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-20T09:43:49.9736024Z .................................................................................................... 200/5841
2019-07-20T09:43:53.9704762Z .................................................................................................... 300/5841
2019-07-20T09:43:57.4198426Z .................................................................................................... 400/5841
2019-07-20T09:44:00.9677633Z .................................................................................................... 500/5841
2019-07-20T09:44:04.5103864Z ........................................................................i........................... 600/5841
2019-07-20T09:44:12.6175868Z .................................................................................................... 800/5841
2019-07-20T09:44:17.6450117Z .................................................................................................... 900/5841
2019-07-20T09:44:22.1523822Z ...................................................................................................i 1000/5841
2019-07-20T09:44:22.1523822Z ...................................................................................................i 1000/5841
2019-07-20T09:44:27.1259576Z ...........i........................................................................................ 1100/5841
2019-07-20T09:44:30.7051253Z .............................iiiii.................................................................. 1200/5841
2019-07-20T09:44:36.0363743Z .................................................................................................... 1400/5841
2019-07-20T09:44:38.4436846Z .................................................................................................... 1500/5841
2019-07-20T09:44:41.8129216Z .................................................................................................... 1600/5841
2019-07-20T09:44:44.1374012Z .................................................................................................... 1700/5841
2019-07-20T09:44:44.1374012Z .................................................................................................... 1700/5841
2019-07-20T09:44:47.2145207Z ....................F.........F......................................i.............................. 1800/5841
2019-07-20T09:44:55.0355081Z .................................................................................................... 2000/5841
2019-07-20T09:44:58.9571618Z .................................................................................................... 2100/5841
2019-07-20T09:45:02.3729306Z .................................................................................................... 2200/5841
2019-07-20T09:45:02.3729306Z .................................................................................................... 2200/5841
2019-07-20T09:45:05.9262255Z .....................................................i.............................................. 2300/5841
2019-07-20T09:45:14.9051517Z .................................................................................................... 2500/5841
2019-07-20T09:45:18.5872205Z .................................................................................................... 2600/5841
2019-07-20T09:45:23.2180913Z .................................................................................................... 2700/5841
2019-07-20T09:45:26.8030368Z .................................................................................................... 2800/5841
2019-07-20T09:45:26.8030368Z .................................................................................................... 2800/5841
2019-07-20T09:45:30.7737282Z .................................................................................................... 2900/5841
2019-07-20T09:45:35.4423736Z .................................................................................................... 3000/5841
2019-07-20T09:45:39.4808667Z .................................................................................................... 3100/5841
2019-07-20T09:45:44.2462489Z .................................................................................................... 3200/5841
2019-07-20T09:45:47.4204561Z .................................................................................................... 3300/5841
2019-07-20T09:45:50.7781906Z .................................................................................................... 3400/5841
2019-07-20T09:45:55.4322525Z .................................................................................................... 3500/5841
2019-07-20T09:45:58.8150448Z ..................i................................................................................. 3600/5841
2019-07-20T09:46:02.6290490Z ............................................................................................ii...i.. 3700/5841
2019-07-20T09:46:06.2537303Z ii.................................................................................................. 3800/5841
2019-07-20T09:46:14.2201000Z .................................................................................................... 4000/5841
2019-07-20T09:46:14.2201000Z .................................................................................................... 4000/5841
2019-07-20T09:46:17.6497165Z ......ii............................................................................................ 4100/5841
2019-07-20T09:46:19.5243567Z ...........................i........................................................................ 4200/5841
2019-07-20T09:46:21.4351575Z .............................................................................................i...... 4300/5841
2019-07-20T09:46:28.0688956Z .................................................................................................... 4500/5841
2019-07-20T09:46:44.5418710Z .................................................................................................... 4600/5841
2019-07-20T09:46:47.8631767Z .................................................................................................... 4700/5841
2019-07-20T09:46:51.4713785Z .................................................................................................... 4800/5841
---
2019-07-20T09:47:22.4156812Z .................................................................................................... 5400/5841
2019-07-20T09:47:26.7085022Z .................................................................................................... 5500/5841
2019-07-20T09:47:30.4741819Z .................................................................................................... 5600/5841
2019-07-20T09:47:33.2922765Z .................................................................................................... 5700/5841
2019-07-20T09:47:35.8778652Z .................................................................................i.................. 5800/5841
2019-07-20T09:47:37.1825886Z failures:
2019-07-20T09:47:37.1862538Z 
2019-07-20T09:47:37.1865105Z ---- [ui] ui/feature-gates/feature-gate-rustc_private-libc.rs stdout ----
2019-07-20T09:47:37.1865161Z diff of stderr:
2019-07-20T09:47:37.1865161Z diff of stderr:
2019-07-20T09:47:37.1865190Z 
2019-07-20T09:47:37.1865224Z 11    |     ^^^^^^^^^^^^^^^^^^
2019-07-20T09:47:37.1865275Z 12    |
2019-07-20T09:47:37.1865613Z 13    = note: for more information, see ***/issues/27812
2019-07-20T09:47:37.1865856Z -    = help: add #![feature(rustc_private)] to the crate attributes to enable
2019-07-20T09:47:37.1865907Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-20T09:47:37.1866187Z 16 error: aborting due to 2 previous errors
2019-07-20T09:47:37.1866221Z 17 
2019-07-20T09:47:37.1866244Z 
2019-07-20T09:47:37.1866457Z - Some errors occurred: E0432, E0658.
2019-07-20T09:47:37.1866457Z - Some errors occurred: E0432, E0658.
2019-07-20T09:47:37.1866525Z + Some errors have detailed explanations: E0432, E0658.
2019-07-20T09:47:37.1866732Z 19 For more information about an error, try `rustc --explain E0432`.
2019-07-20T09:47:37.1866770Z 20 
2019-07-20T09:47:37.1866792Z 
2019-07-20T09:47:37.1866829Z 
2019-07-20T09:47:37.1866866Z The actual stderr differed from the expected stderr.
2019-07-20T09:47:37.1867143Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-rustc_private-libc/feature-gate-rustc_private-libc.stderr
2019-07-20T09:47:37.1867365Z To update references, rerun the tests and pass the `--bless` flag
2019-07-20T09:47:37.1867600Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-rustc_private-libc.rs`
2019-07-20T09:47:37.1867690Z error: 1 errors occurred comparing output.
2019-07-20T09:47:37.1867727Z status: exit code: 1
2019-07-20T09:47:37.1867727Z status: exit code: 1
2019-07-20T09:47:37.1868451Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-rustc_private-libc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-rustc_private-libc" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-rustc_private-libc/auxiliary" "-A" "unused"
2019-07-20T09:47:37.1868763Z ------------------------------------------
2019-07-20T09:47:37.1868807Z 
2019-07-20T09:47:37.1868996Z ------------------------------------------
2019-07-20T09:47:37.1869034Z stderr:
2019-07-20T09:47:37.1869034Z stderr:
2019-07-20T09:47:37.1869203Z ------------------------------------------
2019-07-20T09:47:37.1869258Z error[E0432]: unresolved import `libc`
2019-07-20T09:47:37.1869471Z   --> /checkout/src/test/ui/feature-gates/feature-gate-rustc_private-libc.rs:3:9
2019-07-20T09:47:37.1869513Z    |
2019-07-20T09:47:37.1869569Z LL |     use libc::*; //~ ERROR unresolved import
2019-07-20T09:47:37.1869609Z    |         ^^^^ maybe a missing `extern crate libc;`?
2019-07-20T09:47:37.1869633Z 
2019-07-20T09:47:37.1869954Z error[E0658]: use of unstable library feature 'rustc_private': crate "libc" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-20T09:47:37.1870173Z   --> /checkout/src/test/ui/feature-gates/feature-gate-rustc_private-libc.rs:2:5
2019-07-20T09:47:37.1870215Z    |
2019-07-20T09:47:37.1870267Z LL |     extern crate libc; //~ ERROR use of unstable
2019-07-20T09:47:37.1870345Z    |
2019-07-20T09:47:37.1870345Z    |
2019-07-20T09:47:37.1871216Z    = note: for more information, see ***/issues/27812
2019-07-20T09:47:37.1871280Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-20T09:47:37.1871387Z error: aborting due to 2 previous errors
2019-07-20T09:47:37.1871417Z 
2019-07-20T09:47:37.1871462Z Some errors have detailed explanations: E0432, E0658.
2019-07-20T09:47:37.1871727Z For more information about an error, try `rustc --explain E0432`.
---
2019-07-20T09:47:37.1872348Z diff of stderr:
2019-07-20T09:47:37.1872376Z 
2019-07-20T09:47:37.1872419Z 11    |     ^^^^^^^^^^^^^^^^^^
2019-07-20T09:47:37.1872479Z 12    |
2019-07-20T09:47:37.1872898Z 13    = note: for more information, see ***/issues/27812
2019-07-20T09:47:37.1873158Z -    = help: add #![feature(test)] to the crate attributes to enable
2019-07-20T09:47:37.1873232Z +    = help: add `#![feature(test)]` to the crate attributes to enable
2019-07-20T09:47:37.1873333Z 16 error: aborting due to 2 previous errors
2019-07-20T09:47:37.1873395Z 17 
2019-07-20T09:47:37.1873422Z 
2019-07-20T09:47:37.1873642Z - Some errors occurred: E0432, E0658.
2019-07-20T09:47:37.1873642Z - Some errors occurred: E0432, E0658.
2019-07-20T09:47:37.1873695Z + Some errors have detailed explanations: E0432, E0658.
2019-07-20T09:47:37.1873960Z 19 For more information about an error, try `rustc --explain E0432`.
2019-07-20T09:47:37.1874165Z 20 
2019-07-20T09:47:37.1874190Z 
2019-07-20T09:47:37.1874439Z 
2019-07-20T09:47:37.1874498Z The actual stderr differed from the expected stderr.
2019-07-20T09:47:37.1875127Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-test/feature-gate-test.stderr
2019-07-20T09:47:37.1875720Z To update references, rerun the tests and pass the `--bless` flag
2019-07-20T09:47:37.1875955Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-test.rs`
2019-07-20T09:47:37.1876021Z error: 1 errors occurred comparing output.
2019-07-20T09:47:37.1876138Z status: exit code: 1
2019-07-20T09:47:37.1876138Z status: exit code: 1
2019-07-20T09:47:37.1876777Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-test" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-test/auxiliary" "-A" "unused"
2019-07-20T09:47:37.1877062Z ------------------------------------------
2019-07-20T09:47:37.1877090Z 
2019-07-20T09:47:37.1877286Z ------------------------------------------
2019-07-20T09:47:37.1877323Z stderr:
2019-07-20T09:47:37.1877323Z stderr:
2019-07-20T09:47:37.1877498Z ------------------------------------------
2019-07-20T09:47:37.1877547Z error[E0432]: unresolved import `test`
2019-07-20T09:47:37.1877765Z   --> /checkout/src/test/ui/feature-gates/feature-gate-test.rs:3:9
2019-07-20T09:47:37.1877807Z    |
2019-07-20T09:47:37.1877847Z LL |     use test::*; //~ ERROR unresolved import
2019-07-20T09:47:37.1877904Z    |         ^^^^ maybe a missing `extern crate test;`?
2019-07-20T09:47:37.1877929Z 
2019-07-20T09:47:37.1878120Z error[E0658]: use of unstable library feature 'test'
2019-07-20T09:47:37.1878371Z    |
2019-07-20T09:47:37.1878371Z    |
2019-07-20T09:47:37.1878408Z LL |     extern crate test; //~ ERROR use of unstable
2019-07-20T09:47:37.1878505Z    |
2019-07-20T09:47:37.1878505Z    |
2019-07-20T09:47:37.1880425Z    = note: for more information, see ***/issues/27812
2019-07-20T09:47:37.1881467Z    = help: add `#![feature(test)]` to the crate attributes to enable
2019-07-20T09:47:37.1881594Z error: aborting due to 2 previous errors
2019-07-20T09:47:37.1881625Z 
2019-07-20T09:47:37.1881669Z Some errors have detailed explanations: E0432, E0658.
2019-07-20T09:47:37.1882085Z For more information about an error, try `rustc --explain E0432`.
---
2019-07-20T09:47:37.1887133Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-20T09:47:37.1887194Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-20T09:47:37.1892681Z 
2019-07-20T09:47:37.1892881Z 
2019-07-20T09:47:37.1901724Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-20T09:47:37.1902090Z 
2019-07-20T09:47:37.1902125Z 
2019-07-20T09:47:37.1905818Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-20T09:47:37.1905917Z Build completed unsuccessfully in 0:52:17
2019-07-20T09:47:37.1905917Z Build completed unsuccessfully in 0:52:17
2019-07-20T09:47:38.2124784Z ##[error]Bash exited with code '1'.
2019-07-20T09:47:38.2177800Z ##[section]Starting: Checkout
2019-07-20T09:47:38.2179760Z ==============================================================================
2019-07-20T09:47:38.2179820Z Task         : Get sources
2019-07-20T09:47:38.2179861Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
