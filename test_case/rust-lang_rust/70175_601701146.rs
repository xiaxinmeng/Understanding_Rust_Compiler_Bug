plain
2020-03-20T12:24:41.4815962Z ========================== Starting Command Output ===========================
2020-03-20T12:24:41.4819192Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/433685ba-f6db-4acc-8bc7-bafe0e02501e.sh
2020-03-20T12:24:41.4819609Z 
2020-03-20T12:24:41.4824021Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-20T12:24:41.4843091Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70175/merge to s
2020-03-20T12:24:41.4846073Z Task         : Get sources
2020-03-20T12:24:41.4846364Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-20T12:24:41.4846646Z Version      : 1.0.0
2020-03-20T12:24:41.4846854Z Author       : Microsoft
---
2020-03-20T12:24:42.4744796Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-20T12:24:42.4753831Z ##[command]git config gc.auto 0
2020-03-20T12:24:42.4760106Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-20T12:24:42.4765651Z ##[command]git config --get-all http.proxy
2020-03-20T12:24:42.4771503Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70175/merge:refs/remotes/pull/70175/merge
---
2020-03-20T13:18:25.8642918Z .................................................................................................... 1700/9803
2020-03-20T13:18:29.8972777Z .................................................................................................... 1800/9803
2020-03-20T13:18:40.5768432Z ..........................................................................i......................... 1900/9803
2020-03-20T13:18:46.5407890Z .................................................................................................... 2000/9803
2020-03-20T13:18:53.9756992Z ................................................................iiiii............................... 2100/9803
2020-03-20T13:19:10.7365720Z .................................................................................................... 2300/9803
2020-03-20T13:19:12.9018023Z .................................................................................................... 2400/9803
2020-03-20T13:19:15.7568107Z .................................................................................................... 2500/9803
2020-03-20T13:19:34.6013751Z .................................................................................................... 2600/9803
---
2020-03-20T13:22:04.4204522Z .....................................i...............i.............................................. 5000/9803
2020-03-20T13:22:13.0918388Z .................................................................................................... 5100/9803
2020-03-20T13:22:19.0522204Z ................................................................................i................... 5200/9803
2020-03-20T13:22:24.2083715Z .................................................................................................... 5300/9803
2020-03-20T13:22:33.5752404Z .............................................................ii.ii........i...i..................... 5400/9803
2020-03-20T13:22:41.1740577Z i................................................................................................... 5600/9803
2020-03-20T13:22:49.9412959Z .....i.............................................................................................. 5700/9803
2020-03-20T13:22:55.8465261Z ........................................................i........................................... 5800/9803
2020-03-20T13:23:01.9386038Z .................................................................................................... 5900/9803
2020-03-20T13:23:01.9386038Z .................................................................................................... 5900/9803
2020-03-20T13:23:09.6038888Z .................................................................................................... 6000/9803
2020-03-20T13:23:17.1524086Z ..................................................ii...i..ii...........i............................ 6100/9803
2020-03-20T13:23:36.3489908Z .................................................................................................... 6300/9803
2020-03-20T13:23:43.1661607Z .................................................................................................... 6400/9803
2020-03-20T13:23:43.1661607Z .................................................................................................... 6400/9803
2020-03-20T13:23:47.2160879Z ................................................................................i..ii............... 6500/9803
2020-03-20T13:24:09.2084033Z .................................................................................................... 6700/9803
2020-03-20T13:24:18.5293919Z ...............................................................................i.................... 6800/9803
2020-03-20T13:24:20.6886453Z .................................................................................................... 6900/9803
2020-03-20T13:24:22.7937061Z .................................................................................................... 7000/9803
---
2020-03-20T13:26:01.2558105Z .................................................................................................... 7800/9803
2020-03-20T13:26:06.4025271Z .................................................................................................... 7900/9803
2020-03-20T13:26:12.0844783Z ...................................................................i................................ 8000/9803
2020-03-20T13:26:21.6810718Z .................................................................................................... 8100/9803
2020-03-20T13:26:26.8085307Z ...............iiiiiiiiii.i......................................................................... 8200/9803
2020-03-20T13:26:39.9326433Z .................................................................................................... 8400/9803
2020-03-20T13:26:45.7667824Z .................................................................................................... 8500/9803
2020-03-20T13:26:59.8776493Z .................................................................................................... 8600/9803
2020-03-20T13:27:06.0618974Z .................................................................................................... 8700/9803
---
2020-03-20T13:29:08.1934331Z ---- [run-fail] run-fail/mir_codegen_no_landing_pads.rs stdout ----
2020-03-20T13:29:08.1934547Z 
2020-03-20T13:29:08.1934731Z error: compilation failed!
2020-03-20T13:29:08.1935258Z status: exit code: 1
2020-03-20T13:29:08.1937099Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/mir_codegen_no_landing_pads.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/mir_codegen_no_landing_pads/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-C" "codegen-units=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/mir_codegen_no_landing_pads/auxiliary"
2020-03-20T13:29:08.1938611Z ------------------------------------------
2020-03-20T13:29:08.1938783Z 
2020-03-20T13:29:08.1939142Z ------------------------------------------
2020-03-20T13:29:08.1939345Z stderr:
2020-03-20T13:29:08.1939345Z stderr:
2020-03-20T13:29:08.1939726Z ------------------------------------------
2020-03-20T13:29:08.1940107Z warning: unused imports: `Write`, `self`
2020-03-20T13:29:08.1940592Z  --> /checkout/src/test/run-fail/mir_codegen_no_landing_pads.rs:5:15
2020-03-20T13:29:08.1940851Z   |
2020-03-20T13:29:08.1941027Z 5 | use std::io::{self, Write};
2020-03-20T13:29:08.1941413Z   |
2020-03-20T13:29:08.1941612Z   = note: `#[warn(unused_imports)]` on by default
2020-03-20T13:29:08.1941784Z 
2020-03-20T13:29:08.1941784Z 
2020-03-20T13:29:08.1942313Z error: the linked panic runtime `panic_unwind` is not compiled with this crate's panic strategy `abort`
2020-03-20T13:29:08.1942745Z error: aborting due to previous error
2020-03-20T13:29:08.1942895Z 
2020-03-20T13:29:08.1943003Z 
2020-03-20T13:29:08.1943335Z ------------------------------------------
2020-03-20T13:29:08.1943335Z ------------------------------------------
2020-03-20T13:29:08.1943491Z 
2020-03-20T13:29:08.1943741Z 
2020-03-20T13:29:08.1944366Z ---- [run-fail] run-fail/mir_codegen_no_landing_pads_diverging.rs stdout ----
2020-03-20T13:29:08.1944612Z 
2020-03-20T13:29:08.1944779Z error: compilation failed!
2020-03-20T13:29:08.1944986Z status: exit code: 1
2020-03-20T13:29:08.1947094Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/mir_codegen_no_landing_pads_diverging.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/mir_codegen_no_landing_pads_diverging/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-C" "codegen-units=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/mir_codegen_no_landing_pads_diverging/auxiliary"
2020-03-20T13:29:08.1948761Z ------------------------------------------
2020-03-20T13:29:08.1948938Z 
2020-03-20T13:29:08.1949289Z ------------------------------------------
2020-03-20T13:29:08.1949508Z stderr:
2020-03-20T13:29:08.1949508Z stderr:
2020-03-20T13:29:08.1949871Z ------------------------------------------
2020-03-20T13:29:08.1950139Z warning: unused imports: `Write`, `self`
2020-03-20T13:29:08.1950712Z  --> /checkout/src/test/run-fail/mir_codegen_no_landing_pads_diverging.rs:5:15
2020-03-20T13:29:08.1950991Z   |
2020-03-20T13:29:08.1951178Z 5 | use std::io::{self, Write};
2020-03-20T13:29:08.1951594Z   |
2020-03-20T13:29:08.1951806Z   = note: `#[warn(unused_imports)]` on by default
2020-03-20T13:29:08.1951992Z 
2020-03-20T13:29:08.1952179Z warning: unreachable statement
---
2020-03-20T13:29:08.1956944Z 
2020-03-20T13:29:08.1957121Z warning: unused variable: `d`
2020-03-20T13:29:08.1957658Z   --> /checkout/src/test/run-fail/mir_codegen_no_landing_pads_diverging.rs:18:8
2020-03-20T13:29:08.1957953Z    |
2020-03-20T13:29:08.1958133Z 18 | fn mir(d: Droppable) {
2020-03-20T13:29:08.1958444Z    |        ^ help: consider prefixing with an underscore: `_d`
2020-03-20T13:29:08.1958693Z 
2020-03-20T13:29:08.1959212Z error: the linked panic runtime `panic_unwind` is not compiled with this crate's panic strategy `abort`
2020-03-20T13:29:08.1959691Z error: aborting due to previous error
2020-03-20T13:29:08.1959857Z 
2020-03-20T13:29:08.1959953Z 
2020-03-20T13:29:08.1960306Z ------------------------------------------
---
2020-03-20T13:29:08.1981609Z test result: FAILED. 137 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
2020-03-20T13:29:08.1981977Z 
2020-03-20T13:29:08.1982155Z 
2020-03-20T13:29:08.1982246Z 
2020-03-20T13:29:08.1986583Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-20T13:29:08.1989638Z 
2020-03-20T13:29:08.1989783Z 
2020-03-20T13:29:08.1990366Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-20T13:29:08.2038527Z Build completed unsuccessfully in 1:00:17
2020-03-20T13:29:08.2038527Z Build completed unsuccessfully in 1:00:17
2020-03-20T13:29:08.2038809Z == clock drift check ==
2020-03-20T13:29:08.2056419Z   local time: Fri Mar 20 13:29:08 UTC 2020
2020-03-20T13:29:08.6951163Z   network time: Fri, 20 Mar 2020 13:29:08 GMT
2020-03-20T13:29:08.6958195Z == end clock drift check ==
2020-03-20T13:29:08.6959981Z 
2020-03-20T13:29:08.6995897Z ##[error]Bash exited with code '1'.
2020-03-20T13:29:08.7009994Z ##[section]Finishing: Run build
2020-03-20T13:29:08.7055367Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70175/merge to s
2020-03-20T13:29:08.7060462Z Task         : Get sources
2020-03-20T13:29:08.7060782Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-20T13:29:08.7061079Z Version      : 1.0.0
2020-03-20T13:29:08.7061304Z Author       : Microsoft
2020-03-20T13:29:08.7061304Z Author       : Microsoft
2020-03-20T13:29:08.7061636Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-20T13:29:08.7062010Z ==============================================================================
2020-03-20T13:29:09.0306473Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-20T13:29:09.0312194Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70175/merge to s
2020-03-20T13:29:09.0396800Z Cleaning up task key
2020-03-20T13:29:09.0397936Z Start cleaning up orphan processes.
2020-03-20T13:29:09.0567944Z Terminate orphan process: pid (3487) (python)
2020-03-20T13:29:09.0735714Z ##[section]Finishing: Finalize Job
