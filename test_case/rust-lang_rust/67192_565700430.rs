plain
2019-12-14T08:13:52.0284554Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-14T08:13:52.0461615Z ##[command]git config gc.auto 0
2019-12-14T08:13:52.0542749Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-14T08:13:52.0602925Z ##[command]git config --get-all http.proxy
2019-12-14T08:13:52.0751889Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67192/merge:refs/remotes/pull/67192/merge
---
2019-12-14T09:12:21.0402412Z .................................................................................................... 1600/9376
2019-12-14T09:12:25.4203732Z .................................................................................................... 1700/9376
2019-12-14T09:12:37.4505956Z .................................................................i.................................. 1800/9376
2019-12-14T09:12:45.0645739Z .................................................................................................... 1900/9376
2019-12-14T09:12:59.7743530Z ..................................................iiiii............................................. 2000/9376
2019-12-14T09:13:10.2105846Z .................................................................................................... 2200/9376
2019-12-14T09:13:12.6729919Z .................................................................................................... 2300/9376
2019-12-14T09:13:15.9828248Z .................................................................................................... 2400/9376
2019-12-14T09:13:38.3235402Z .................................................................................................... 2500/9376
---
2019-12-14T09:16:09.0594849Z .................................................................................................... 4700/9376
2019-12-14T09:16:14.1563735Z ..........................................................i...............i......................... 4800/9376
2019-12-14T09:16:21.9172141Z .................................................................................................... 4900/9376
2019-12-14T09:16:30.0858798Z .................................................................................................... 5000/9376
2019-12-14T09:16:35.3064149Z ..i................................................................................................. 5100/9376
2019-12-14T09:16:45.5456714Z ....................................................................ii.ii...........i............... 5200/9376
2019-12-14T09:16:54.2706764Z ....i............................................................................................... 5400/9376
2019-12-14T09:17:04.4436773Z .................................................................................................... 5500/9376
2019-12-14T09:17:10.7535193Z ..................................................i................................................. 5600/9376
2019-12-14T09:17:17.7746584Z .................................................................................................... 5700/9376
2019-12-14T09:17:17.7746584Z .................................................................................................... 5700/9376
2019-12-14T09:17:27.8602649Z .................................................................................................... 5800/9376
2019-12-14T09:17:37.4714819Z ......................................ii...i..ii...........i........................................ 5900/9376
2019-12-14T09:17:56.7512271Z .................................................................................................... 6100/9376
2019-12-14T09:18:04.9046379Z .................................................................................................... 6200/9376
2019-12-14T09:18:04.9046379Z .................................................................................................... 6200/9376
2019-12-14T09:18:15.7365875Z ..............................................................i..ii................................. 6300/9376
2019-12-14T09:18:42.9792809Z .................................................................................................... 6500/9376
2019-12-14T09:18:45.0773350Z ..................................i................................................................. 6600/9376
2019-12-14T09:18:47.3207689Z .................................................................................................... 6700/9376
2019-12-14T09:18:49.6287745Z ...........................i........................................................................ 6800/9376
---
2019-12-14T09:20:26.0697389Z .................................................................................................... 7400/9376
2019-12-14T09:20:31.1051608Z .................................................................................................... 7500/9376
2019-12-14T09:20:36.6532693Z .................................................................................................... 7600/9376
2019-12-14T09:20:45.8289285Z .................................................................................................... 7700/9376
2019-12-14T09:20:54.1690755Z ...............................................iiii................................................. 7800/9376
2019-12-14T09:21:08.5664448Z .................................................................................................... 8000/9376
2019-12-14T09:21:15.1973629Z .................................................................................................... 8100/9376
2019-12-14T09:21:30.6364112Z .................................................................................................... 8200/9376
2019-12-14T09:21:38.4620270Z .................................................................................................... 8300/9376
---
2019-12-14T09:23:34.1573831Z failures:
2019-12-14T09:23:34.1575035Z 
2019-12-14T09:23:34.1581975Z ---- [ui] ui/consts/const-eval/ub-wide-ptr.rs stdout ----
2019-12-14T09:23:34.1582999Z 
2019-12-14T09:23:34.1583368Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-12-14T09:23:34.1584354Z status: exit code: 101
2019-12-14T09:23:34.1588161Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-wide-ptr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-wide-ptr/auxiliary" "-A" "unused"
2019-12-14T09:23:34.1589372Z ------------------------------------------
2019-12-14T09:23:34.1589674Z 
2019-12-14T09:23:34.1590185Z ------------------------------------------
2019-12-14T09:23:34.1590497Z stderr:
2019-12-14T09:23:34.1590497Z stderr:
2019-12-14T09:23:34.1590971Z ------------------------------------------
2019-12-14T09:23:34.1592736Z error[E0080]: it is undefined behavior to use this value
2019-12-14T09:23:34.1593426Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:87:1
2019-12-14T09:23:34.1593751Z    |
2019-12-14T09:23:34.1594032Z LL | const STR_TOO_LONG: &str = unsafe { SliceTransmute { repr: SliceRepr { ptr: &42, len: 999 } }.str};
2019-12-14T09:23:34.1594301Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling reference (not entirely in bounds)
2019-12-14T09:23:34.1594577Z    |
2019-12-14T09:23:34.1595242Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-14T09:23:34.1595818Z error[E0080]: it is undefined behavior to use this value
2019-12-14T09:23:34.1596318Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:90:1
2019-12-14T09:23:34.1596647Z    |
2019-12-14T09:23:34.1596647Z    |
2019-12-14T09:23:34.1596885Z LL | const STR_LENGTH_PTR: &str = unsafe { SliceTransmute { bad: BadSliceRepr { ptr: &42, len: &3 } }.str};
2019-12-14T09:23:34.1597991Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
2019-12-14T09:23:34.1598505Z    |
2019-12-14T09:23:34.1599481Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-14T09:23:34.1600727Z error[E0080]: it is undefined behavior to use this value
2019-12-14T09:23:34.1601251Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:93:1
2019-12-14T09:23:34.1601477Z    |
2019-12-14T09:23:34.1601477Z    |
2019-12-14T09:23:34.1601634Z LL | const MY_STR_LENGTH_PTR: &MyStr = unsafe { SliceTransmute { bad: BadSliceRepr { ptr: &42, len: &3 } }.my_str};
2019-12-14T09:23:34.1602198Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
2019-12-14T09:23:34.1602431Z    |
2019-12-14T09:23:34.1602997Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-14T09:23:34.1603364Z error[E0080]: it is undefined behavior to use this value
2019-12-14T09:23:34.1603778Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:97:1
2019-12-14T09:23:34.1603967Z    |
2019-12-14T09:23:34.1603967Z    |
2019-12-14T09:23:34.1604110Z LL | const STR_NO_UTF8: &str = unsafe { SliceTransmute { slice: &[0xFF] }.str };
2019-12-14T09:23:34.1604636Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized or non-UTF-8 data in str at .<deref>
2019-12-14T09:23:34.1604836Z    |
2019-12-14T09:23:34.1605375Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-14T09:23:34.1605745Z error[E0080]: it is undefined behavior to use this value
2019-12-14T09:23:34.1606148Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:100:1
2019-12-14T09:23:34.1606359Z    |
2019-12-14T09:23:34.1606359Z    |
2019-12-14T09:23:34.1606503Z LL | const MYSTR_NO_UTF8: &MyStr = unsafe { SliceTransmute { slice: &[0xFF] }.my_str };
2019-12-14T09:23:34.1607030Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized or non-UTF-8 data in str at .<deref>.0
2019-12-14T09:23:34.1607228Z    |
2019-12-14T09:23:34.1608015Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-14T09:23:34.1608592Z error[E0080]: it is undefined behavior to use this value
2019-12-14T09:23:34.1609083Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:107:1
2019-12-14T09:23:34.1609318Z    |
2019-12-14T09:23:34.1609318Z    |
2019-12-14T09:23:34.1609478Z LL | const SLICE_LENGTH_UNINIT: &[u8] = unsafe { SliceTransmute { addr: 42 }.slice};
2019-12-14T09:23:34.1609653Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered undefined pointer
2019-12-14T09:23:34.1609800Z    |
2019-12-14T09:23:34.1610350Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-14T09:23:34.1610703Z error[E0080]: it is undefined behavior to use this value
2019-12-14T09:23:34.1611091Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:110:1
2019-12-14T09:23:34.1611301Z    |
2019-12-14T09:23:34.1611301Z    |
2019-12-14T09:23:34.1611474Z LL | const SLICE_TOO_LONG: &[u8] = unsafe { SliceTransmute { repr: SliceRepr { ptr: &42, len: 999 } }.slice};
2019-12-14T09:23:34.1611808Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling reference (not entirely in bounds)
2019-12-14T09:23:34.1612083Z    |
2019-12-14T09:23:34.1612670Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-14T09:23:34.1613028Z error[E0080]: it is undefined behavior to use this value
2019-12-14T09:23:34.1613421Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:113:1
2019-12-14T09:23:34.1613631Z    |
2019-12-14T09:23:34.1613631Z    |
2019-12-14T09:23:34.1613778Z LL | const SLICE_LENGTH_PTR: &[u8] = unsafe { SliceTransmute { bad: BadSliceRepr { ptr: &42, len: &3 } }.slice};
2019-12-14T09:23:34.1614326Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
2019-12-14T09:23:34.1614536Z    |
2019-12-14T09:23:34.1615080Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-14T09:23:34.1615434Z error[E0080]: it is undefined behavior to use this value
2019-12-14T09:23:34.1615825Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:117:1
2019-12-14T09:23:34.1616035Z    |
2019-12-14T09:23:34.1616035Z    |
2019-12-14T09:23:34.1616181Z LL | const SLICE_CONTENT_INVALID: &[bool] = &[unsafe { BoolTransmute { val: 3 }.bl }];
2019-12-14T09:23:34.1616361Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 3 at .<deref>[0], but expected something less or equal to 1
2019-12-14T09:23:34.1616511Z    |
2019-12-14T09:23:34.1617063Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-14T09:23:34.1617420Z error[E0080]: it is undefined behavior to use this value
2019-12-14T09:23:34.1618080Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:123:1
2019-12-14T09:23:34.1618371Z    |
2019-12-14T09:23:34.1618371Z    |
2019-12-14T09:23:34.1618522Z LL | const MYSLICE_PREFIX_BAD: &MySliceBool = &MySlice(unsafe { BoolTransmute { val: 3 }.bl }, [false]);
2019-12-14T09:23:34.1618709Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 3 at .<deref>.0, but expected something less or equal to 1
2019-12-14T09:23:34.1618861Z    |
2019-12-14T09:23:34.1619464Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-14T09:23:34.1619841Z error[E0080]: it is undefined behavior to use this value
2019-12-14T09:23:34.1620236Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:126:1
2019-12-14T09:23:34.1620472Z    |
2019-12-14T09:23:34.1620472Z    |
2019-12-14T09:23:34.1620622Z LL | const MYSLICE_SUFFIX_BAD: &MySliceBool = &MySlice(true, [unsafe { BoolTransmute { val: 3 }.bl }]);
2019-12-14T09:23:34.1620804Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 3 at .<deref>.1[0], but expected something less or equal to 1
2019-12-14T09:23:34.1620958Z    |
2019-12-14T09:23:34.1621499Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-14T09:23:34.1621991Z error[E0080]: it is undefined behavior to use this value
2019-12-14T09:23:34.1622470Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:133:1
2019-12-14T09:23:34.1622839Z    |
2019-12-14T09:23:34.1622839Z    |
2019-12-14T09:23:34.1622991Z LL | const RAW_SLICE_LENGTH_UNINIT: *const [u8] = unsafe { SliceTransmute { addr: 42 }.raw_slice};
2019-12-14T09:23:34.1623256Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered undefined pointer
2019-12-14T09:23:34.1623411Z    |
2019-12-14T09:23:34.1623998Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-14T09:23:34.1624371Z error[E0080]: it is undefined behavior to use this value
2019-12-14T09:23:34.1624768Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:138:1
2019-12-14T09:23:34.1624956Z    |
2019-12-14T09:23:34.1624956Z    |
2019-12-14T09:23:34.1625144Z LL | const TRAIT_OBJ_SHORT_VTABLE_1: &dyn Trait = unsafe { DynTransmute { repr: DynRepr { ptr: &92, vtable: &3 } }.rust};
2019-12-14T09:23:34.1625328Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling or unaligned vtable pointer in wide pointer or too small vtable
2019-12-14T09:23:34.1625509Z    |
2019-12-14T09:23:34.1626066Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-14T09:23:34.1626437Z error[E0080]: it is undefined behavior to use this value
2019-12-14T09:23:34.1628971Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:141:1
2019-12-14T09:23:34.1629262Z    |
2019-12-14T09:23:34.1629262Z    |
2019-12-14T09:23:34.1629475Z LL | const TRAIT_OBJ_SHORT_VTABLE_2: &dyn Trait = unsafe { DynTransmute { repr2: DynRepr2 { ptr: &92, vtable: &3 } }.rust};
2019-12-14T09:23:34.1629653Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling or unaligned vtable pointer in wide pointer or too small vtable
2019-12-14T09:23:34.1629850Z    |
2019-12-14T09:23:34.1630464Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-14T09:23:34.1630660Z 
2019-12-14T09:23:34.1631818Z thread 'rustc' panicked at 'expected a Pointer but got Raw bits: InterpErrorInfo { kind: a memory access tried to interpret some bytes as a pointer, backtrace: None }', src/libcore/result.rs:1189:5
2019-12-14T09:23:34.1632025Z 
2019-12-14T09:23:34.1632080Z error: internal compiler error: unexpected panic
2019-12-14T09:23:34.1632111Z 
2019-12-14T09:23:34.1632172Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-14T09:23:34.1632172Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-14T09:23:34.1632211Z 
2019-12-14T09:23:34.1632759Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-12-14T09:23:34.1633084Z note: rustc 1.41.0-dev running on x86_64-unknown-linux-gnu
2019-12-14T09:23:34.1633118Z 
2019-12-14T09:23:34.1633118Z 
2019-12-14T09:23:34.1633405Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-12-14T09:23:34.1633509Z error: aborting due to 14 previous errors
2019-12-14T09:23:34.1633538Z 
2019-12-14T09:23:34.1633790Z For more information about this error, try `rustc --explain E0080`.
2019-12-14T09:23:34.1633825Z 
---
2019-12-14T09:23:34.1650272Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-14T09:23:34.1650356Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-14T09:23:34.1671463Z 
2019-12-14T09:23:34.1671578Z 
2019-12-14T09:23:34.1673559Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-14T09:23:34.1673848Z 
2019-12-14T09:23:34.1673879Z 
2019-12-14T09:23:34.1679271Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-14T09:23:34.1679372Z Build completed unsuccessfully in 1:03:50
2019-12-14T09:23:34.1679372Z Build completed unsuccessfully in 1:03:50
2019-12-14T09:23:34.1734637Z == clock drift check ==
2019-12-14T09:23:34.1754452Z   local time: Sat Dec 14 09:23:34 UTC 2019
2019-12-14T09:23:34.7179422Z   network time: Sat, 14 Dec 2019 09:23:34 GMT
2019-12-14T09:23:34.7179560Z == end clock drift check ==
2019-12-14T09:23:35.4925298Z 
2019-12-14T09:23:35.5025110Z ##[error]Bash exited with code '1'.
2019-12-14T09:23:35.5069663Z ##[section]Starting: Checkout
2019-12-14T09:23:35.5072309Z ==============================================================================
2019-12-14T09:23:35.5072370Z Task         : Get sources
2019-12-14T09:23:35.5072417Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
