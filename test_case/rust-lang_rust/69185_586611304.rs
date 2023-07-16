plain
2020-02-15T14:42:43.9823392Z ========================== Starting Command Output ===========================
2020-02-15T14:42:43.9824612Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9ef9a4c1-8df6-4218-a0c7-584bd59e56d2.sh
2020-02-15T14:42:43.9824652Z 
2020-02-15T14:42:43.9826983Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-15T14:42:43.9831376Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69185/merge to s
2020-02-15T14:42:43.9832633Z Task         : Get sources
2020-02-15T14:42:43.9832661Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-15T14:42:43.9832688Z Version      : 1.0.0
2020-02-15T14:42:43.9832713Z Author       : Microsoft
---
2020-02-15T14:42:44.9795354Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-15T14:42:44.9806090Z ##[command]git config gc.auto 0
2020-02-15T14:42:44.9808543Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-15T14:42:44.9810656Z ##[command]git config --get-all http.proxy
2020-02-15T14:42:44.9816887Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69185/merge:refs/remotes/pull/69185/merge
---
2020-02-15T15:40:21.6605986Z .................................................................................................... 1700/9664
2020-02-15T15:40:26.5995341Z .................................................................................................... 1800/9664
2020-02-15T15:40:38.2310267Z ............................................i....................................................... 1900/9664
2020-02-15T15:40:45.6671650Z .................................................................................................... 2000/9664
2020-02-15T15:40:58.3771995Z ..................................iiiii............................................................. 2100/9664
2020-02-15T15:41:08.3106954Z .................................................................................................... 2300/9664
2020-02-15T15:41:10.8653650Z .................................................................................................... 2400/9664
2020-02-15T15:41:15.5091421Z .................................................................................................... 2500/9664
2020-02-15T15:41:36.9641583Z .................................................................................................... 2600/9664
---
2020-02-15T15:44:09.0654327Z .......i............................................................................................ 5000/9664
2020-02-15T15:44:17.1292354Z .................................................................................................... 5100/9664
2020-02-15T15:44:21.4643040Z .................................i.................................................................. 5200/9664
2020-02-15T15:44:30.0321311Z .................................................................................................... 5300/9664
2020-02-15T15:44:35.4806421Z .........ii.ii........i...i......................................................................... 5400/9664
2020-02-15T15:44:42.6889078Z .................................................................................................... 5600/9664
2020-02-15T15:44:54.2911673Z .................................................................................................... 5700/9664
2020-02-15T15:45:02.1182506Z .i.................................................................................................. 5800/9664
2020-02-15T15:45:07.3257923Z ...................................................................................................i 5900/9664
2020-02-15T15:45:07.3257923Z ...................................................................................................i 5900/9664
2020-02-15T15:45:17.7254851Z .............................................................................................ii...i. 6000/9664
2020-02-15T15:45:30.1528543Z .ii...........i..................................................................................... 6100/9664
2020-02-15T15:45:47.4524436Z .................................................................................................... 6300/9664
2020-02-15T15:45:53.9680503Z .................................................................................................... 6400/9664
2020-02-15T15:45:53.9680503Z .................................................................................................... 6400/9664
2020-02-15T15:46:11.6726860Z .....................i..ii.......................................................................... 6500/9664
2020-02-15T15:46:34.2278170Z .................................................................................................... 6700/9664
2020-02-15T15:46:36.5755859Z .........i.......................................................................................... 6800/9664
2020-02-15T15:46:38.9288387Z .................................................................................................... 6900/9664
2020-02-15T15:46:41.4259101Z ...................i................................................................................ 7000/9664
---
2020-02-15T15:48:26.6464891Z .................................................................................................... 7700/9664
2020-02-15T15:48:32.3246083Z .................................................................................................... 7800/9664
2020-02-15T15:48:38.4415796Z .................................................................................................... 7900/9664
2020-02-15T15:48:48.8936749Z .................................................................................................... 8000/9664
2020-02-15T15:48:54.9470105Z .iiiiiii.i.......................................................................................... 8100/9664
2020-02-15T15:49:09.9181240Z .................................................................................................... 8300/9664
2020-02-15T15:49:21.6963559Z .................................................................................................... 8400/9664
2020-02-15T15:49:34.2041428Z .................................................................................................... 8500/9664
2020-02-15T15:49:39.7000020Z .................................................................................................... 8600/9664
---
2020-02-15T15:51:39.0456737Z  finished in 1.492
2020-02-15T15:51:39.0674652Z Check compiletest suite=run-fail mode=run-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-15T15:51:39.2581425Z 
2020-02-15T15:51:39.2581671Z running 139 tests
2020-02-15T15:51:48.8532893Z ........................................................................FF.FF....F.F........F....... 100/139
2020-02-15T15:51:53.6872903Z .......F.F.............................
2020-02-15T15:51:53.6887078Z 
2020-02-15T15:51:53.6888070Z ---- [run-fail] run-fail/mir_indexing_oob_1.rs stdout ----
2020-02-15T15:51:53.6888409Z 
2020-02-15T15:51:53.6888675Z error: compilation failed!
2020-02-15T15:51:53.6888675Z error: compilation failed!
2020-02-15T15:51:53.6888903Z status: exit code: 1
2020-02-15T15:51:53.6889895Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/mir_indexing_oob_1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/mir_indexing_oob_1/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/mir_indexing_oob_1/auxiliary"
2020-02-15T15:51:53.6890764Z ------------------------------------------
2020-02-15T15:51:53.6891028Z 
2020-02-15T15:51:53.6891512Z ------------------------------------------
2020-02-15T15:51:53.6891797Z stderr:
2020-02-15T15:51:53.6891797Z stderr:
2020-02-15T15:51:53.6892260Z ------------------------------------------
2020-02-15T15:51:53.6892584Z error: this operation will panic at runtime
2020-02-15T15:51:53.6893081Z  --> /checkout/src/test/run-fail/mir_indexing_oob_1.rs:7:5
2020-02-15T15:51:53.6893372Z   |
2020-02-15T15:51:53.6893628Z 7 |     C[10]
2020-02-15T15:51:53.6893849Z   |     ^^^^^ index out of bounds: the len is 5 but the index is 10
2020-02-15T15:51:53.6894088Z   |
2020-02-15T15:51:53.6894312Z   = note: `#[deny(panic)]` on by default
2020-02-15T15:51:53.6894727Z error: aborting due to previous error
2020-02-15T15:51:53.6894946Z 
2020-02-15T15:51:53.6895143Z 
2020-02-15T15:51:53.6895605Z ------------------------------------------
2020-02-15T15:51:53.6895605Z ------------------------------------------
2020-02-15T15:51:53.6895865Z 
2020-02-15T15:51:53.6896549Z 
2020-02-15T15:51:53.6897118Z ---- [run-fail] run-fail/mir_indexing_oob_2.rs stdout ----
2020-02-15T15:51:53.6897411Z 
2020-02-15T15:51:53.6897678Z error: compilation failed!
2020-02-15T15:51:53.6897923Z status: exit code: 1
2020-02-15T15:51:53.6898915Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/mir_indexing_oob_2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/mir_indexing_oob_2/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/mir_indexing_oob_2/auxiliary"
2020-02-15T15:51:53.6900592Z ------------------------------------------
2020-02-15T15:51:53.6900784Z 
2020-02-15T15:51:53.6901174Z ------------------------------------------
2020-02-15T15:51:53.6901359Z stderr:
2020-02-15T15:51:53.6901359Z stderr:
2020-02-15T15:51:53.6901713Z ------------------------------------------
2020-02-15T15:51:53.6902152Z error: this operation will panic at runtime
2020-02-15T15:51:53.6902580Z  --> /checkout/src/test/run-fail/mir_indexing_oob_2.rs:7:5
2020-02-15T15:51:53.6902767Z   |
2020-02-15T15:51:53.6902935Z 7 |     C[10]
2020-02-15T15:51:53.6903084Z   |     ^^^^^ index out of bounds: the len is 5 but the index is 10
2020-02-15T15:51:53.6903354Z   |
2020-02-15T15:51:53.6903566Z   = note: `#[deny(panic)]` on by default
2020-02-15T15:51:53.6903836Z error: aborting due to previous error
2020-02-15T15:51:53.6903983Z 
2020-02-15T15:51:53.6904105Z 
2020-02-15T15:51:53.6904516Z ------------------------------------------
2020-02-15T15:51:53.6904516Z ------------------------------------------
2020-02-15T15:51:53.6904679Z 
2020-02-15T15:51:53.6904834Z 
2020-02-15T15:51:53.6905209Z ---- [run-fail] run-fail/mir_indexing_oob_3.rs stdout ----
2020-02-15T15:51:53.6905370Z 
2020-02-15T15:51:53.6905541Z error: compilation failed!
2020-02-15T15:51:53.6905690Z status: exit code: 1
2020-02-15T15:51:53.6906520Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/mir_indexing_oob_3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/mir_indexing_oob_3/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/mir_indexing_oob_3/auxiliary"
2020-02-15T15:51:53.6907145Z ------------------------------------------
2020-02-15T15:51:53.6907335Z 
2020-02-15T15:51:53.6907696Z ------------------------------------------
2020-02-15T15:51:53.6907874Z stderr:
2020-02-15T15:51:53.6907874Z stderr:
2020-02-15T15:51:53.6908250Z ------------------------------------------
2020-02-15T15:51:53.6908433Z error: this operation will panic at runtime
2020-02-15T15:51:53.6908802Z  --> /checkout/src/test/run-fail/mir_indexing_oob_3.rs:7:5
2020-02-15T15:51:53.6909012Z   |
2020-02-15T15:51:53.6909287Z 7 |     C[10]
2020-02-15T15:51:53.6909445Z   |     ^^^^^ index out of bounds: the len is 5 but the index is 10
2020-02-15T15:51:53.6909590Z   |
2020-02-15T15:51:53.6909728Z   = note: `#[deny(panic)]` on by default
2020-02-15T15:51:53.6910015Z error: aborting due to previous error
2020-02-15T15:51:53.6910137Z 
2020-02-15T15:51:53.6910262Z 
2020-02-15T15:51:53.6910635Z ------------------------------------------
2020-02-15T15:51:53.6910635Z ------------------------------------------
2020-02-15T15:51:53.6910797Z 
2020-02-15T15:51:53.6910917Z 
2020-02-15T15:51:53.6911270Z ---- [run-fail] run-fail/overflowing-add.rs stdout ----
2020-02-15T15:51:53.6911453Z 
2020-02-15T15:51:53.6911593Z error: compilation failed!
2020-02-15T15:51:53.6911729Z status: exit code: 1
2020-02-15T15:51:53.6912558Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/overflowing-add.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-add/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-add/auxiliary"
2020-02-15T15:51:53.6913170Z ------------------------------------------
2020-02-15T15:51:53.6913336Z 
2020-02-15T15:51:53.6913701Z ------------------------------------------
2020-02-15T15:51:53.6913878Z stderr:
2020-02-15T15:51:53.6913878Z stderr:
2020-02-15T15:51:53.6914224Z ------------------------------------------
2020-02-15T15:51:53.6914421Z error: this arithmetic operation will overflow
2020-02-15T15:51:53.6914874Z  --> /checkout/src/test/run-fail/overflowing-add.rs:7:14
2020-02-15T15:51:53.6915057Z   |
2020-02-15T15:51:53.6915249Z 7 |     let _x = 200u8 + 200u8 + 200u8;
2020-02-15T15:51:53.6915394Z   |              ^^^^^^^^^^^^^ attempt to add with overflow
2020-02-15T15:51:53.6915532Z   |
2020-02-15T15:51:53.6915698Z   = note: `#[deny(overflow)]` on by default
2020-02-15T15:51:53.6916116Z error: aborting due to previous error
2020-02-15T15:51:53.6916261Z 
2020-02-15T15:51:53.6916379Z 
2020-02-15T15:51:53.6916755Z ------------------------------------------
2020-02-15T15:51:53.6916755Z ------------------------------------------
2020-02-15T15:51:53.6916913Z 
2020-02-15T15:51:53.6917060Z 
2020-02-15T15:51:53.6918018Z ---- [run-fail] run-fail/overflowing-mul.rs stdout ----
2020-02-15T15:51:53.6918215Z 
2020-02-15T15:51:53.6918383Z error: compilation failed!
2020-02-15T15:51:53.6918534Z status: exit code: 1
2020-02-15T15:51:53.6921305Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/overflowing-mul.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-mul/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-mul/auxiliary"
2020-02-15T15:51:53.6922474Z ------------------------------------------
2020-02-15T15:51:53.6922662Z 
2020-02-15T15:51:53.6923012Z ------------------------------------------
2020-02-15T15:51:53.6923188Z stderr:
---
2020-02-15T15:51:53.6928749Z   |
2020-02-15T15:51:53.6928937Z 7 |     let x = 200u8 * 4;
2020-02-15T15:51:53.6929088Z   |             ^^^^^^^^^ attempt to multiply with overflow
2020-02-15T15:51:53.6929229Z   |
2020-02-15T15:51:53.6929374Z   = note: `#[deny(overflow)]` on by default
2020-02-15T15:51:53.6929683Z error: aborting due to previous error
2020-02-15T15:51:53.6929808Z 
2020-02-15T15:51:53.6929946Z 
2020-02-15T15:51:53.6930358Z ------------------------------------------
2020-02-15T15:51:53.6930358Z ------------------------------------------
2020-02-15T15:51:53.6930523Z 
2020-02-15T15:51:53.6930644Z 
2020-02-15T15:51:53.6931013Z ---- [run-fail] run-fail/overflowing-neg.rs stdout ----
2020-02-15T15:51:53.6931200Z 
2020-02-15T15:51:53.6931367Z error: compilation failed!
2020-02-15T15:51:53.6931514Z status: exit code: 1
2020-02-15T15:51:53.6932351Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/overflowing-neg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-neg/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-neg/auxiliary"
2020-02-15T15:51:53.6932984Z ------------------------------------------
2020-02-15T15:51:53.6933147Z 
2020-02-15T15:51:53.6933485Z ------------------------------------------
2020-02-15T15:51:53.6933684Z stderr:
2020-02-15T15:51:53.6933684Z stderr:
2020-02-15T15:51:53.6934062Z ------------------------------------------
2020-02-15T15:51:53.6934246Z error: this arithmetic operation will overflow
2020-02-15T15:51:53.6934621Z  --> /checkout/src/test/run-fail/overflowing-neg.rs:7:14
2020-02-15T15:51:53.6934840Z   |
2020-02-15T15:51:53.6935185Z 7 |     let _x = -std::i8::MIN;
2020-02-15T15:51:53.6935372Z   |              ^^^^^^^^^^^^^ attempt to negate with overflow
2020-02-15T15:51:53.6935517Z   |
2020-02-15T15:51:53.6935881Z   = note: `#[deny(overflow)]` on by default
2020-02-15T15:51:53.6936152Z error: aborting due to previous error
2020-02-15T15:51:53.6936295Z 
2020-02-15T15:51:53.6936629Z 
2020-02-15T15:51:53.6937067Z ------------------------------------------
2020-02-15T15:51:53.6937067Z ------------------------------------------
2020-02-15T15:51:53.6937245Z 
2020-02-15T15:51:53.6937384Z 
2020-02-15T15:51:53.6937893Z ---- [run-fail] run-fail/overflowing-sub.rs stdout ----
2020-02-15T15:51:53.6938130Z 
2020-02-15T15:51:53.6938296Z error: compilation failed!
2020-02-15T15:51:53.6938467Z status: exit code: 1
2020-02-15T15:51:53.6939395Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/overflowing-sub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-sub/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-sub/auxiliary"
2020-02-15T15:51:53.6940189Z ------------------------------------------
2020-02-15T15:51:53.6940345Z 
2020-02-15T15:51:53.6940707Z ------------------------------------------
2020-02-15T15:51:53.6940886Z stderr:
2020-02-15T15:51:53.6940886Z stderr:
2020-02-15T15:51:53.6941244Z ------------------------------------------
2020-02-15T15:51:53.6941433Z error: this arithmetic operation will overflow
2020-02-15T15:51:53.6941799Z  --> /checkout/src/test/run-fail/overflowing-sub.rs:7:14
2020-02-15T15:51:53.6942000Z   |
2020-02-15T15:51:53.6942375Z 7 |     let _x = 42u8 - (42u8 + 1);
2020-02-15T15:51:53.6942560Z   |              ^^^^^^^^^^^^^^^^^ attempt to subtract with overflow
2020-02-15T15:51:53.6942703Z   |
2020-02-15T15:51:53.6942842Z   = note: `#[deny(overflow)]` on by default
2020-02-15T15:51:53.6943133Z error: aborting due to previous error
2020-02-15T15:51:53.6943284Z 
2020-02-15T15:51:53.6943417Z 
2020-02-15T15:51:53.6943786Z ------------------------------------------
2020-02-15T15:51:53.6943786Z ------------------------------------------
2020-02-15T15:51:53.6943947Z 
2020-02-15T15:51:53.6944070Z 
2020-02-15T15:51:53.6944456Z ---- [run-fail] run-fail/promoted_div_by_zero.rs stdout ----
2020-02-15T15:51:53.6944621Z 
2020-02-15T15:51:53.6944764Z error: compilation failed!
2020-02-15T15:51:53.6944934Z status: exit code: 1
2020-02-15T15:51:53.6945980Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/promoted_div_by_zero.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/promoted_div_by_zero/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/promoted_div_by_zero/auxiliary"
2020-02-15T15:51:53.6946568Z ------------------------------------------
2020-02-15T15:51:53.6946758Z 
2020-02-15T15:51:53.6947092Z ------------------------------------------
2020-02-15T15:51:53.6947264Z stderr:
2020-02-15T15:51:53.6947264Z stderr:
2020-02-15T15:51:53.6947594Z ------------------------------------------
2020-02-15T15:51:53.6947765Z warning: unused variable: `x`
2020-02-15T15:51:53.6948156Z  --> /checkout/src/test/run-fail/promoted_div_by_zero.rs:6:9
2020-02-15T15:51:53.6948368Z   |
2020-02-15T15:51:53.6948707Z 6 |     let x = &(1 / (1 - 1));
2020-02-15T15:51:53.6948887Z   |         ^ help: consider prefixing with an underscore: `_x`
2020-02-15T15:51:53.6949195Z   = note: `#[warn(unused_variables)]` on by default
2020-02-15T15:51:53.6949345Z 
2020-02-15T15:51:53.6949646Z error: this operation will panic at runtime
2020-02-15T15:51:53.6950084Z  --> /checkout/src/test/run-fail/promoted_div_by_zero.rs:6:14
2020-02-15T15:51:53.6950084Z  --> /checkout/src/test/run-fail/promoted_div_by_zero.rs:6:14
2020-02-15T15:51:53.6950436Z   |
2020-02-15T15:51:53.6950860Z 6 |     let x = &(1 / (1 - 1));
2020-02-15T15:51:53.6951045Z   |              ^^^^^^^^^^^^^ attempt to divide by zero
2020-02-15T15:51:53.6951379Z   |
2020-02-15T15:51:53.6951681Z   = note: `#[deny(panic)]` on by default
2020-02-15T15:51:53.6951966Z error: aborting due to previous error
2020-02-15T15:51:53.6952119Z 
2020-02-15T15:51:53.6952374Z 
2020-02-15T15:51:53.6952822Z ------------------------------------------
2020-02-15T15:51:53.6952822Z ------------------------------------------
2020-02-15T15:51:53.6953103Z 
2020-02-15T15:51:53.6953297Z 
2020-02-15T15:51:53.6953688Z ---- [run-fail] run-fail/promoted_overflow.rs stdout ----
2020-02-15T15:51:53.6953853Z 
2020-02-15T15:51:53.6953992Z error: compilation failed!
2020-02-15T15:51:53.6954142Z status: exit code: 1
2020-02-15T15:51:53.6954968Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/promoted_overflow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/promoted_overflow/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "overflow-checks=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/promoted_overflow/auxiliary"
2020-02-15T15:51:53.6955595Z ------------------------------------------
2020-02-15T15:51:53.6955750Z 
2020-02-15T15:51:53.6956093Z ------------------------------------------
2020-02-15T15:51:53.6956298Z stderr:
2020-02-15T15:51:53.6956298Z stderr:
2020-02-15T15:51:53.6956643Z ------------------------------------------
2020-02-15T15:51:53.6956818Z warning: unused variable: `x`
2020-02-15T15:51:53.6957174Z  --> /checkout/src/test/run-fail/promoted_overflow.rs:7:9
2020-02-15T15:51:53.6957387Z   |
2020-02-15T15:51:53.6957753Z 7 |     let x: &'static u32 = &(0u32 - 1);
2020-02-15T15:51:53.6957939Z   |         ^ help: consider prefixing with an underscore: `_x`
2020-02-15T15:51:53.6958237Z   = note: `#[warn(unused_variables)]` on by default
2020-02-15T15:51:53.6958543Z 
2020-02-15T15:51:53.6958689Z error: this arithmetic operation will overflow
2020-02-15T15:51:53.6959877Z  --> /checkout/src/test/run-fail/promoted_overflow.rs:7:28
2020-02-15T15:51:53.6959877Z  --> /checkout/src/test/run-fail/promoted_overflow.rs:7:28
2020-02-15T15:51:53.6960088Z   |
2020-02-15T15:51:53.6960517Z 7 |     let x: &'static u32 = &(0u32 - 1);
2020-02-15T15:51:53.6960719Z   |                            ^^^^^^^^^^ attempt to subtract with overflow
2020-02-15T15:51:53.6960890Z   |
2020-02-15T15:51:53.6961099Z   = note: `#[deny(overflow)]` on by default
2020-02-15T15:51:53.6961398Z error: aborting due to previous error
2020-02-15T15:51:53.6961533Z 
2020-02-15T15:51:53.6961665Z 
2020-02-15T15:51:53.6962049Z ------------------------------------------
---
2020-02-15T15:51:53.6970406Z test result: FAILED. 130 passed; 9 failed; 0 ignored; 0 measured; 0 filtered out
2020-02-15T15:51:53.6970458Z 
2020-02-15T15:51:53.6970513Z 
2020-02-15T15:51:53.6970540Z 
2020-02-15T15:51:53.6972232Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-15T15:51:53.6972510Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-15T15:51:53.6973063Z 7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-15T15:51:53.6973205Z 
2020-02-15T15:51:53.6973232Z 
2020-02-15T15:51:53.6973289Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-15T15:51:53.6973359Z Build completed unsuccessfully in 1:03:07
2020-02-15T15:51:53.6973359Z Build completed unsuccessfully in 1:03:07
2020-02-15T15:51:53.6973405Z == clock drift check ==
2020-02-15T15:51:53.6973486Z   local time: Sat Feb 15 15:51:53 UTC 2020
2020-02-15T15:51:53.9679516Z   network time: Sat, 15 Feb 2020 15:51:53 GMT
2020-02-15T15:51:53.9680772Z == end clock drift check ==
2020-02-15T15:51:54.3772124Z 
2020-02-15T15:51:54.3870118Z ##[error]Bash exited with code '1'.
2020-02-15T15:51:54.3884168Z ##[section]Finishing: Run build
2020-02-15T15:51:54.3905170Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69185/merge to s
2020-02-15T15:51:54.3907110Z Task         : Get sources
2020-02-15T15:51:54.3907176Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-15T15:51:54.3907224Z Version      : 1.0.0
2020-02-15T15:51:54.3907267Z Author       : Microsoft
2020-02-15T15:51:54.3907267Z Author       : Microsoft
2020-02-15T15:51:54.3907335Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-15T15:51:54.3907386Z ==============================================================================
2020-02-15T15:51:54.8443363Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-15T15:51:54.8485581Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69185/merge to s
2020-02-15T15:51:54.8593161Z Cleaning up task key
2020-02-15T15:51:54.8594018Z Start cleaning up orphan processes.
2020-02-15T15:51:54.8709378Z Terminate orphan process: pid (3677) (python)
2020-02-15T15:51:54.8977441Z ##[section]Finishing: Finalize Job
