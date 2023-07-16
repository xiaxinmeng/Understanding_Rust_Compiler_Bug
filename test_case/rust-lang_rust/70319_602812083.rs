plain
2020-03-23T18:23:11.2738820Z ========================== Starting Command Output ===========================
2020-03-23T18:23:11.2744505Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/22375e29-7914-4550-b5b1-6dbf26c8c84c.sh
2020-03-23T18:23:11.2745011Z 
2020-03-23T18:23:11.2749371Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-23T18:23:11.2768900Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70319/merge to s
2020-03-23T18:23:11.2772344Z Task         : Get sources
2020-03-23T18:23:11.2772622Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T18:23:11.2772890Z Version      : 1.0.0
2020-03-23T18:23:11.2773138Z Author       : Microsoft
---
2020-03-23T18:23:12.2633477Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-23T18:23:12.2638813Z ##[command]git config gc.auto 0
2020-03-23T18:23:12.2642396Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-23T18:23:12.2645499Z ##[command]git config --get-all http.proxy
2020-03-23T18:23:12.2651288Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70319/merge:refs/remotes/pull/70319/merge
---
2020-03-23T19:22:53.4665424Z .................................................................................................... 1700/9824
2020-03-23T19:22:57.7603824Z .................................................................................................... 1800/9824
2020-03-23T19:23:08.5398344Z ..................................................................................i................. 1900/9824
2020-03-23T19:23:16.1488678Z .................................................................................................... 2000/9824
2020-03-23T19:23:22.7113091Z ........................................................................iiiii....................... 2100/9824
2020-03-23T19:23:44.4120524Z .................................................................................................... 2300/9824
2020-03-23T19:23:46.6874585Z .................................................................................................... 2400/9824
2020-03-23T19:23:49.3208603Z .................................................................................................... 2500/9824
2020-03-23T19:24:02.9540504Z .................................................................................................... 2600/9824
---
2020-03-23T19:26:55.4275292Z ...............................................i...............i.................................... 5000/9824
2020-03-23T19:27:03.7683417Z .................................................................................................... 5100/9824
2020-03-23T19:27:11.2224929Z ...........................................................................................i........ 5200/9824
2020-03-23T19:27:17.0888021Z .................................................................................................... 5300/9824
2020-03-23T19:27:27.6901238Z ..........................................................................ii.ii........i...i........ 5400/9824
2020-03-23T19:27:35.9025458Z ..............i..................................................................................... 5600/9824
2020-03-23T19:27:44.9154957Z ...................i................................................................................ 5700/9824
2020-03-23T19:27:51.4929010Z ....................................ii...................................i.......................... 5800/9824
2020-03-23T19:27:58.7408250Z .................................................................................................... 5900/9824
2020-03-23T19:27:58.7408250Z .................................................................................................... 5900/9824
2020-03-23T19:28:05.2252545Z .................................................................................................... 6000/9824
2020-03-23T19:28:15.2855180Z ...................................................................ii...i..ii...........i........... 6100/9824
2020-03-23T19:28:35.9513852Z .................................................................................................... 6300/9824
2020-03-23T19:28:43.3082945Z .................................................................................................... 6400/9824
2020-03-23T19:28:50.8379261Z .................................................................................................i.. 6500/9824
2020-03-23T19:29:07.6417996Z ii.................................................................................................. 6600/9824
---
2020-03-23T19:31:22.2933354Z .................................................................................................... 7800/9824
2020-03-23T19:31:27.2448143Z .................................................................................................... 7900/9824
2020-03-23T19:31:34.0452818Z ......................................................................................i............. 8000/9824
2020-03-23T19:31:42.8520227Z .................................................................................................... 8100/9824
2020-03-23T19:31:50.3251625Z ...................................iiiiiiiiii.i..................................................... 8200/9824
2020-03-23T19:32:05.7235225Z .................................................................................................... 8400/9824
2020-03-23T19:32:11.3961490Z .................................................................................................... 8500/9824
2020-03-23T19:32:27.0694601Z .................................................................................................... 8600/9824
2020-03-23T19:32:35.0583884Z .................................................................................................... 8700/9824
---
2020-03-23T19:34:32.1777328Z 
2020-03-23T19:34:32.1778273Z ---- [ui] ui/associated-const/defaults-cyclic-fail.rs stdout ----
2020-03-23T19:34:32.1778706Z diff of stderr:
2020-03-23T19:34:32.1778962Z 
2020-03-23T19:34:32.1779557Z - error[E0391]: cycle detected when const-evaluating + checking `Tr::A`
2020-03-23T19:34:32.1780427Z + error[E0391]: cycle detected when normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing, def_id: None }, value: Const { ty: u8, val: Unevaluated(DefId(0:4 ~ defaults_cyclic_fail[317d]::Tr[0]::A[0]), [()], None) } }`
2020-03-23T19:34:32.1781114Z +    |
2020-03-23T19:34:32.1781724Z + note: ...which requires const-evaluating + checking `Tr::A`...
2020-03-23T19:34:32.1782670Z 3    |
2020-03-23T19:34:32.1782976Z 4 LL |     const A: u8 = Self::B;
2020-03-23T19:34:32.1783228Z 
2020-03-23T19:34:32.1783473Z 5    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-03-23T19:34:32.1783473Z 5    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-03-23T19:34:32.1784069Z + note: ...which requires const-evaluating + checking `Tr::A`...
2020-03-23T19:34:32.1785173Z 6    |
2020-03-23T19:34:32.1785466Z + LL |     const A: u8 = Self::B;
2020-03-23T19:34:32.1785781Z +    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-03-23T19:34:32.1785781Z +    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-03-23T19:34:32.1786349Z 7 note: ...which requires const-evaluating `Tr::A`...
2020-03-23T19:34:32.1787479Z +   --> $DIR/defaults-cyclic-fail.rs:5:5
2020-03-23T19:34:32.1787815Z 9    |
2020-03-23T19:34:32.1788093Z 10 LL |     const A: u8 = Self::B;
2020-03-23T19:34:32.1788637Z -    |                   ^^^^^^^
2020-03-23T19:34:32.1788637Z -    |                   ^^^^^^^
2020-03-23T19:34:32.1788986Z +    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-03-23T19:34:32.1789816Z +    = note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing, def_id: None }, value: Const { ty: u8, val: Unevaluated(DefId(0:5 ~ defaults_cyclic_fail[317d]::Tr[0]::B[0]), [()], None) } }`...
2020-03-23T19:34:32.1790895Z 12 note: ...which requires const-evaluating + checking `Tr::B`...
2020-03-23T19:34:32.1791833Z 14    |
2020-03-23T19:34:32.1792029Z 
2020-03-23T19:34:32.1792319Z 15 LL |     const B: u8 = Self::A;
2020-03-23T19:34:32.1792638Z 16    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-03-23T19:34:32.1792638Z 16    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-03-23T19:34:32.1793212Z + note: ...which requires const-evaluating + checking `Tr::B`...
2020-03-23T19:34:32.1794137Z +    |
2020-03-23T19:34:32.1794137Z +    |
2020-03-23T19:34:32.1794435Z + LL |     const B: u8 = Self::A;
2020-03-23T19:34:32.1794744Z +    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-03-23T19:34:32.1795288Z 17 note: ...which requires const-evaluating `Tr::B`...
2020-03-23T19:34:32.1796414Z +   --> $DIR/defaults-cyclic-fail.rs:8:5
2020-03-23T19:34:32.1796764Z 19    |
2020-03-23T19:34:32.1797043Z 20 LL |     const B: u8 = Self::A;
2020-03-23T19:34:32.1797521Z -    |                   ^^^^^^^
2020-03-23T19:34:32.1797521Z -    |                   ^^^^^^^
2020-03-23T19:34:32.1798184Z -    = note: ...which again requires const-evaluating + checking `Tr::A`, completing the cycle
2020-03-23T19:34:32.1798667Z +    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-03-23T19:34:32.1799443Z +    = note: ...which again requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing, def_id: None }, value: Const { ty: u8, val: Unevaluated(DefId(0:4 ~ defaults_cyclic_fail[317d]::Tr[0]::A[0]), [()], None) } }`, completing the cycle
2020-03-23T19:34:32.1800412Z 23 note: cycle used when const-evaluating `main`
2020-03-23T19:34:32.1801499Z +   --> $DIR/defaults-cyclic-fail.rs:14:1
2020-03-23T19:34:32.1801833Z 25    |
2020-03-23T19:34:32.1801833Z 25    |
2020-03-23T19:34:32.1802587Z - LL |     assert_eq!(<() as Tr>::A, 0);
2020-03-23T19:34:32.1804949Z + LL | fn main() {
2020-03-23T19:34:32.1805342Z +    | ^^^^^^^^^
2020-03-23T19:34:32.1805635Z 28 
2020-03-23T19:34:32.1805955Z 29 error: aborting due to previous error
2020-03-23T19:34:32.1805955Z 29 error: aborting due to previous error
2020-03-23T19:34:32.1806279Z 30 
2020-03-23T19:34:32.1806495Z 
2020-03-23T19:34:32.1806726Z 
2020-03-23T19:34:32.1807060Z The actual stderr differed from the expected stderr.
2020-03-23T19:34:32.1811191Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/defaults-cyclic-fail/defaults-cyclic-fail.stderr
2020-03-23T19:34:32.1812729Z To update references, rerun the tests and pass the `--bless` flag
2020-03-23T19:34:32.1814065Z To only update this specific test, also pass `--test-args associated-const/defaults-cyclic-fail.rs`
2020-03-23T19:34:32.1815720Z error: 1 errors occurred comparing output.
2020-03-23T19:34:32.1816214Z status: exit code: 1
2020-03-23T19:34:32.1816214Z status: exit code: 1
2020-03-23T19:34:32.1818577Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-const/defaults-cyclic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/defaults-cyclic-fail" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/defaults-cyclic-fail/auxiliary"
2020-03-23T19:34:32.1823243Z ------------------------------------------
2020-03-23T19:34:32.1827430Z 
2020-03-23T19:34:32.1828101Z ------------------------------------------
2020-03-23T19:34:32.1828640Z stderr:
2020-03-23T19:34:32.1828640Z stderr:
2020-03-23T19:34:32.1829308Z ------------------------------------------
2020-03-23T19:34:32.1830595Z error[E0391]: cycle detected when normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing, def_id: None }, value: Const { ty: u8, val: Unevaluated(DefId(0:4 ~ defaults_cyclic_fail[317d]::Tr[0]::A[0]), [()], None) } }`
2020-03-23T19:34:32.1831389Z    |
2020-03-23T19:34:32.1832157Z note: ...which requires const-evaluating + checking `Tr::A`...
2020-03-23T19:34:32.1833386Z    |
2020-03-23T19:34:32.1833831Z LL |     const A: u8 = Self::B;
2020-03-23T19:34:32.1834336Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-03-23T19:34:32.1834336Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-03-23T19:34:32.1834919Z note: ...which requires const-evaluating + checking `Tr::A`...
2020-03-23T19:34:32.1836196Z    |
2020-03-23T19:34:32.1836668Z LL |     const A: u8 = Self::B;
2020-03-23T19:34:32.1837003Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-03-23T19:34:32.1837003Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-03-23T19:34:32.1837584Z note: ...which requires const-evaluating `Tr::A`...
2020-03-23T19:34:32.1839063Z    |
2020-03-23T19:34:32.1839361Z LL |     const A: u8 = Self::B;
2020-03-23T19:34:32.1839855Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-03-23T19:34:32.1839855Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-03-23T19:34:32.1840624Z    = note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing, def_id: None }, value: Const { ty: u8, val: Unevaluated(DefId(0:5 ~ defaults_cyclic_fail[317d]::Tr[0]::B[0]), [()], None) } }`...
2020-03-23T19:34:32.1842050Z note: ...which requires const-evaluating + checking `Tr::B`...
2020-03-23T19:34:32.1843159Z    |
2020-03-23T19:34:32.1843442Z LL |     const B: u8 = Self::A;
2020-03-23T19:34:32.1843784Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-03-23T19:34:32.1843784Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-03-23T19:34:32.1844715Z note: ...which requires const-evaluating + checking `Tr::B`...
2020-03-23T19:34:32.1845826Z    |
2020-03-23T19:34:32.1846131Z LL |     const B: u8 = Self::A;
2020-03-23T19:34:32.1846452Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-03-23T19:34:32.1846452Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-03-23T19:34:32.1847037Z note: ...which requires const-evaluating `Tr::B`...
2020-03-23T19:34:32.1848310Z    |
2020-03-23T19:34:32.1848603Z LL |     const B: u8 = Self::A;
2020-03-23T19:34:32.1848959Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-03-23T19:34:32.1848959Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-03-23T19:34:32.1849808Z    = note: ...which again requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing, def_id: None }, value: Const { ty: u8, val: Unevaluated(DefId(0:4 ~ defaults_cyclic_fail[317d]::Tr[0]::A[0]), [()], None) } }`, completing the cycle
2020-03-23T19:34:32.1851258Z note: cycle used when const-evaluating `main`
2020-03-23T19:34:32.1854237Z    |
2020-03-23T19:34:32.1854429Z LL | fn main() {
2020-03-23T19:34:32.1854893Z    | ^^^^^^^^^
2020-03-23T19:34:32.1855505Z 
---
2020-03-23T19:34:32.1859328Z test result: FAILED. 9763 passed; 1 failed; 60 ignored; 0 measured; 0 filtered out
2020-03-23T19:34:32.1859715Z 
2020-03-23T19:34:32.1859845Z 
2020-03-23T19:34:32.1859975Z 
2020-03-23T19:34:32.1864350Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-23T19:34:32.1867281Z 
2020-03-23T19:34:32.1867403Z 
2020-03-23T19:34:32.1867950Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-23T19:34:32.1868720Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-23T19:34:32.1868720Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-23T19:34:32.1869670Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-23T19:34:32.1870193Z Build completed unsuccessfully in 1:05:36
2020-03-23T19:34:32.1890528Z == clock drift check ==
2020-03-23T19:34:32.1912567Z   local time: Mon Mar 23 19:34:32 UTC 2020
2020-03-23T19:34:32.3564450Z   network time: Mon, 23 Mar 2020 19:34:32 GMT
2020-03-23T19:34:32.3568202Z == end clock drift check ==
2020-03-23T19:34:33.0186635Z 
2020-03-23T19:34:33.0271145Z ##[error]Bash exited with code '1'.
2020-03-23T19:34:33.0285954Z ##[section]Finishing: Run build
2020-03-23T19:34:33.0339837Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70319/merge to s
2020-03-23T19:34:33.0345301Z Task         : Get sources
2020-03-23T19:34:33.0345644Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T19:34:33.0345977Z Version      : 1.0.0
2020-03-23T19:34:33.0346195Z Author       : Microsoft
2020-03-23T19:34:33.0346195Z Author       : Microsoft
2020-03-23T19:34:33.0346544Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-23T19:34:33.0346966Z ==============================================================================
2020-03-23T19:34:33.3850939Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-23T19:34:33.3895344Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70319/merge to s
2020-03-23T19:34:33.3984037Z Cleaning up task key
2020-03-23T19:34:33.3985146Z Start cleaning up orphan processes.
2020-03-23T19:34:33.4173235Z Terminate orphan process: pid (3733) (python)
2020-03-23T19:34:33.4348834Z ##[section]Finishing: Finalize Job
