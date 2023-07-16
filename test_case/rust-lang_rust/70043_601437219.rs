plain
2020-03-19T20:52:43.9316476Z ========================== Starting Command Output ===========================
2020-03-19T20:52:43.9322586Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/00706bd6-fd39-4405-abed-f93919c453fd.sh
2020-03-19T20:52:43.9322838Z 
2020-03-19T20:52:43.9328564Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-19T20:52:43.9348973Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70043/merge to s
2020-03-19T20:52:43.9351773Z Task         : Get sources
2020-03-19T20:52:43.9351999Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T20:52:43.9352217Z Version      : 1.0.0
2020-03-19T20:52:43.9352363Z Author       : Microsoft
---
2020-03-19T20:52:44.9272222Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-19T20:52:44.9281166Z ##[command]git config gc.auto 0
2020-03-19T20:52:44.9286976Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-19T20:52:44.9292827Z ##[command]git config --get-all http.proxy
2020-03-19T20:52:44.9303858Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70043/merge:refs/remotes/pull/70043/merge
---
2020-03-19T21:50:02.3585695Z .................................................................................................... 1700/9803
2020-03-19T21:50:06.2597291Z .................................................................................................... 1800/9803
2020-03-19T21:50:16.7247445Z ..........................................................................i......................... 1900/9803
2020-03-19T21:50:22.6650974Z .................................................................................................... 2000/9803
2020-03-19T21:50:30.2111239Z ................................................................iiiii............................... 2100/9803
2020-03-19T21:50:46.5881775Z .................................................................................................... 2300/9803
2020-03-19T21:50:48.5652820Z .................................................................................................... 2400/9803
2020-03-19T21:50:51.2048495Z .................................................................................................... 2500/9803
2020-03-19T21:51:10.2562400Z .................................................................................................... 2600/9803
---
2020-03-19T21:53:47.4360105Z .....................................i...............i.............................................. 5000/9803
2020-03-19T21:53:55.6263409Z .................................................................................................... 5100/9803
2020-03-19T21:54:01.7348121Z ................................................................................i................... 5200/9803
2020-03-19T21:54:06.8120698Z .................................................................................................... 5300/9803
2020-03-19T21:54:16.4114363Z .............................................................ii.ii........i...i..................... 5400/9803
2020-03-19T21:54:23.9449230Z i................................................................................................... 5600/9803
2020-03-19T21:54:32.7072574Z .....i.............................................................................................. 5700/9803
2020-03-19T21:54:38.5368609Z ........................................................i........................................... 5800/9803
2020-03-19T21:54:44.4634651Z .................................................................................................... 5900/9803
2020-03-19T21:54:44.4634651Z .................................................................................................... 5900/9803
2020-03-19T21:54:51.9946185Z .................................................................................................... 6000/9803
2020-03-19T21:54:59.1732432Z ..................................................ii...i..ii...........i............................ 6100/9803
2020-03-19T21:55:17.8360675Z .................................................................................................... 6300/9803
2020-03-19T21:55:24.2596007Z .................................................................................................... 6400/9803
2020-03-19T21:55:24.2596007Z .................................................................................................... 6400/9803
2020-03-19T21:55:31.7375367Z ................................................................................i..ii............... 6500/9803
2020-03-19T21:55:57.2439409Z .................................................................................................... 6700/9803
2020-03-19T21:56:06.2279087Z ...............................................................................i.................... 6800/9803
2020-03-19T21:56:08.1622219Z .................................................................................................... 6900/9803
2020-03-19T21:56:10.0700985Z .................................................................................................... 7000/9803
---
2020-03-19T21:57:46.9470017Z .................................................................................................... 7800/9803
2020-03-19T21:57:51.9267662Z .................................................................................................... 7900/9803
2020-03-19T21:57:57.5121994Z ..................................................................i................................. 8000/9803
2020-03-19T21:58:07.0735851Z .................................................................................................... 8100/9803
2020-03-19T21:58:12.1865050Z ...............iiiiiiiiii.i........F................................................................ 8200/9803
2020-03-19T21:58:25.1768471Z .................................................................................................... 8400/9803
2020-03-19T21:58:30.9318469Z .................................................................................................... 8500/9803
2020-03-19T21:58:45.0069110Z .................................................................................................... 8600/9803
2020-03-19T21:58:51.0626514Z .................................................................................................... 8700/9803
---
2020-03-19T22:00:38.1705618Z failures:
2020-03-19T22:00:38.1732008Z 
2020-03-19T22:00:38.1733487Z ---- [ui] ui/save-analysis/issue-68621.rs stdout ----
2020-03-19T22:00:38.1733798Z 
2020-03-19T22:00:38.1734175Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-19T22:00:38.1734778Z status: exit code: 101
2020-03-19T22:00:38.1738540Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/save-analysis/issue-68621.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/save-analysis/issue-68621" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Zsave-analysis" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/save-analysis/issue-68621/auxiliary"
2020-03-19T22:00:38.1742949Z ------------------------------------------
2020-03-19T22:00:38.1743117Z 
2020-03-19T22:00:38.1743413Z ------------------------------------------
2020-03-19T22:00:38.1743575Z stderr:
---
2020-03-19T22:00:38.1747829Z error: internal compiler error: unexpected panic
2020-03-19T22:00:38.1747994Z 
2020-03-19T22:00:38.1748176Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-19T22:00:38.1748343Z 
2020-03-19T22:00:38.1749260Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-19T22:00:38.1749891Z note: rustc 1.44.0-nightly (7590f222f 2020-03-19) running on x86_64-unknown-linux-gnu
2020-03-19T22:00:38.1750092Z 
2020-03-19T22:00:38.1750092Z 
2020-03-19T22:00:38.1750639Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z save-analysis -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-19T22:00:38.1751110Z error: aborting due to previous error
2020-03-19T22:00:38.1751242Z 
2020-03-19T22:00:38.1751319Z 
2020-03-19T22:00:38.1751608Z ------------------------------------------
2020-03-19T22:00:38.1751608Z ------------------------------------------
2020-03-19T22:00:38.1751745Z 
2020-03-19T22:00:38.1751837Z 
2020-03-19T22:00:38.1752168Z ---- [ui] ui/type-alias-impl-trait/issue-63279.rs stdout ----
2020-03-19T22:00:38.1752333Z 
2020-03-19T22:00:38.1752538Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-19T22:00:38.1752795Z status: exit code: 101
2020-03-19T22:00:38.1754785Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-63279.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-63279" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Zsave-analysis" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-63279/auxiliary"
2020-03-19T22:00:38.1757028Z ------------------------------------------
2020-03-19T22:00:38.1757197Z 
2020-03-19T22:00:38.1757508Z ------------------------------------------
2020-03-19T22:00:38.1757683Z stderr:
2020-03-19T22:00:38.1757683Z stderr:
2020-03-19T22:00:38.1758010Z ------------------------------------------
2020-03-19T22:00:38.1758695Z error[E0271]: type mismatch resolving `<[closure@/checkout/src/test/ui/type-alias-impl-trait/issue-63279.rs:8:5: 8:28] as std::ops::FnOnce<()>>::Output == ()`
2020-03-19T22:00:38.1759729Z    |
2020-03-19T22:00:38.1759946Z LL | type Closure = impl FnOnce(); //~ ERROR: type mismatch resolving
2020-03-19T22:00:38.1763678Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected opaque type, found `()`
2020-03-19T22:00:38.1764709Z    |
---
2020-03-19T22:00:38.1775001Z error: internal compiler error: unexpected panic
2020-03-19T22:00:38.1775167Z 
2020-03-19T22:00:38.1775490Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-19T22:00:38.1775674Z 
2020-03-19T22:00:38.1776336Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-19T22:00:38.1777143Z note: rustc 1.44.0-nightly (7590f222f 2020-03-19) running on x86_64-unknown-linux-gnu
2020-03-19T22:00:38.1777357Z 
2020-03-19T22:00:38.1777357Z 
2020-03-19T22:00:38.1777927Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z save-analysis -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-19T22:00:38.1778421Z error: aborting due to previous error
2020-03-19T22:00:38.1778561Z 
2020-03-19T22:00:38.1778925Z For more information about this error, try `rustc --explain E0271`.
2020-03-19T22:00:38.1779126Z 
2020-03-19T22:00:38.1779126Z 
2020-03-19T22:00:38.1780365Z ------------------------------------------
2020-03-19T22:00:38.1780527Z 
2020-03-19T22:00:38.1780611Z 
2020-03-19T22:00:38.1781376Z ---- [ui] ui/type-alias-impl-trait/issue-65679-inst-opaque-ty-from-val-twice.rs stdout ----
2020-03-19T22:00:38.1781597Z 
2020-03-19T22:00:38.1781918Z error: test compilation failed although it shouldn't!
2020-03-19T22:00:38.1782137Z status: exit code: 101
2020-03-19T22:00:38.1784394Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-65679-inst-opaque-ty-from-val-twice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-65679-inst-opaque-ty-from-val-twice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Zsave-analysis" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-65679-inst-opaque-ty-from-val-twice/auxiliary"
2020-03-19T22:00:38.1785911Z ------------------------------------------
2020-03-19T22:00:38.1786055Z 
2020-03-19T22:00:38.1786368Z ------------------------------------------
2020-03-19T22:00:38.1786533Z stderr:
---
2020-03-19T22:00:38.1788224Z error: internal compiler error: unexpected panic
2020-03-19T22:00:38.1788439Z 
2020-03-19T22:00:38.1788618Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-19T22:00:38.1788780Z 
2020-03-19T22:00:38.1789492Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-19T22:00:38.1790141Z note: rustc 1.44.0-nightly (7590f222f 2020-03-19) running on x86_64-unknown-linux-gnu
2020-03-19T22:00:38.1790716Z 
2020-03-19T22:00:38.1790716Z 
2020-03-19T22:00:38.1791528Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z save-analysis -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-19T22:00:38.1791985Z 
2020-03-19T22:00:38.1793043Z ------------------------------------------
2020-03-19T22:00:38.1793222Z 
2020-03-19T22:00:38.1793310Z 
---
2020-03-19T22:00:38.1801540Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-19T22:00:38.1802011Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-19T22:00:38.1802321Z 
2020-03-19T22:00:38.1829666Z 
2020-03-19T22:00:38.1833265Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-19T22:00:38.1835674Z 
2020-03-19T22:00:38.1835761Z 
2020-03-19T22:00:38.1836639Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-19T22:00:38.1836967Z Build completed unsuccessfully in 1:02:17
2020-03-19T22:00:38.1836967Z Build completed unsuccessfully in 1:02:17
2020-03-19T22:00:38.1872381Z == clock drift check ==
2020-03-19T22:00:38.1894759Z   local time: Thu Mar 19 22:00:38 UTC 2020
2020-03-19T22:00:38.4726870Z   network time: Thu, 19 Mar 2020 22:00:38 GMT
2020-03-19T22:00:38.4727357Z == end clock drift check ==
2020-03-19T22:00:39.1549652Z 
2020-03-19T22:00:39.1624836Z ##[error]Bash exited with code '1'.
2020-03-19T22:00:39.1637252Z ##[section]Finishing: Run build
2020-03-19T22:00:39.1685149Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70043/merge to s
2020-03-19T22:00:39.1690082Z Task         : Get sources
2020-03-19T22:00:39.1690559Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T22:00:39.1690811Z Version      : 1.0.0
2020-03-19T22:00:39.1691000Z Author       : Microsoft
2020-03-19T22:00:39.1691000Z Author       : Microsoft
2020-03-19T22:00:39.1691282Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-19T22:00:39.1691621Z ==============================================================================
2020-03-19T22:00:39.4856776Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-19T22:00:39.4905310Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70043/merge to s
2020-03-19T22:00:39.4990014Z Cleaning up task key
2020-03-19T22:00:39.4991291Z Start cleaning up orphan processes.
2020-03-19T22:00:39.5268603Z Terminate orphan process: pid (3922) (python)
2020-03-19T22:00:39.5325195Z ##[section]Finishing: Finalize Job
