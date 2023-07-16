plain
2019-08-26T18:15:48.0889421Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-26T18:15:48.1098168Z ##[command]git config gc.auto 0
2019-08-26T18:15:48.1178125Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-26T18:15:48.1236407Z ##[command]git config --get-all http.proxy
2019-08-26T18:15:48.9852947Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63880/merge:refs/remotes/pull/63880/merge
---
2019-08-26T18:16:24.1505327Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-26T18:16:24.1538240Z 
2019-08-26T18:16:24.1539058Z   git checkout -b <new-branch-name>
2019-08-26T18:16:24.1540038Z 
2019-08-26T18:16:24.1540357Z HEAD is now at 032c4ef9c Merge ae788620588029bef52d27d01ce66010a3c9a4b2 into 9fa8f140233047fb0211dbaee531a290bcfeae7e
2019-08-26T18:16:24.1706583Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-26T18:16:24.1709864Z ==============================================================================
2019-08-26T18:16:24.1709938Z Task         : Bash
2019-08-26T18:16:24.1709985Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-26T19:22:51.2865116Z ......................F............................................................................. 1500/8953
2019-08-26T19:22:57.3196568Z .................................................................................................... 1600/8953
2019-08-26T19:23:10.9249779Z .............................................i...............i...................................... 1700/8953
2019-08-26T19:23:19.6380441Z .................................................................................................... 1800/8953
2019-08-26T19:23:35.1661034Z .....................................iiiii.......................................................... 1900/8953
2019-08-26T19:23:46.3901981Z .................................................................................................... 2100/8953
2019-08-26T19:23:49.1945231Z .................................................................................................... 2200/8953
2019-08-26T19:23:53.7675547Z .................................................................................................... 2300/8953
2019-08-26T19:24:01.5051219Z .................................................................................................... 2400/8953
---
2019-08-26T19:27:12.0850365Z .........................i...............i.......................................................... 4700/8953
2019-08-26T19:27:24.5464106Z .................................................................................................... 4800/8953
2019-08-26T19:27:31.0924005Z .................................................................................................... 4900/8953
2019-08-26T19:27:42.6985322Z .................................................................................................... 5000/8953
2019-08-26T19:27:48.5070451Z ......ii.ii......................................................................................... 5100/8953
2019-08-26T19:28:03.7343108Z .................................................................................................... 5300/8953
2019-08-26T19:28:11.1638181Z ..............................................................i..................................... 5400/8953
2019-08-26T19:28:18.7920650Z .................................................................................................... 5500/8953
2019-08-26T19:28:27.1275430Z .................................................................................................... 5600/8953
2019-08-26T19:28:27.1275430Z .................................................................................................... 5600/8953
2019-08-26T19:28:37.3022505Z ........................................................ii...i..ii...........i...................... 5700/8953
2019-08-26T19:29:01.3087075Z .................................................................................................... 5900/8953
2019-08-26T19:29:06.5214487Z .................................................................................................... 6000/8953
2019-08-26T19:29:06.5214487Z .................................................................................................... 6000/8953
2019-08-26T19:29:14.1462286Z .........................................................i..ii...................................... 6100/8953
2019-08-26T19:29:43.6937560Z .................................................................................................... 6300/8953
2019-08-26T19:29:46.1078522Z ...i................................................................................................ 6400/8953
2019-08-26T19:29:48.5085197Z ...........................................................................i........................ 6500/8953
2019-08-26T19:29:51.4903053Z .................................................................................................... 6600/8953
---
2019-08-26T19:33:59.8440342Z ..............................................................i..................................... 8900/8953
2019-08-26T19:34:06.2146150Z .....................................................
2019-08-26T19:34:06.2147362Z failures:
2019-08-26T19:34:06.2188971Z 
2019-08-26T19:34:06.2189890Z ---- [ui] ui/consts/const-eval/ub-wide-ptr.rs stdout ----
2019-08-26T19:34:06.2190936Z 
2019-08-26T19:34:06.2190936Z 
2019-08-26T19:34:06.2191619Z 87    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-26T19:34:06.2192169Z 89 error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2192169Z 89 error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2193018Z -   --> $DIR/ub-wide-ptr.rs:129:1
2019-08-26T19:34:06.2193505Z -    |
2019-08-26T19:34:06.2194127Z - LL | const RAW_SLICE_MUCH_TOO_LONG: *const [u8] = unsafe { SliceTransmute { repr: SliceRepr { ptr: &42, len: usize::max_value() } }.raw_slice};
2019-08-26T19:34:06.2194825Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered too large slice (longer than isize::MAX bytes)
2019-08-26T19:34:06.2195331Z -    |
2019-08-26T19:34:06.2195984Z -    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-26T19:34:06.2197119Z - error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2197119Z - error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2197578Z 98   --> $DIR/ub-wide-ptr.rs:131:1
2019-08-26T19:34:06.2197861Z 99    |
2019-08-26T19:34:06.2198069Z 100 LL | const RAW_SLICE_LENGTH_UNINIT: *const [u8] = unsafe { SliceTransmute { addr: 42 }.raw_slice};
2019-08-26T19:34:06.2198475Z 150    |
2019-08-26T19:34:06.2198475Z 150    |
2019-08-26T19:34:06.2199560Z 151    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-26T19:34:06.2200248Z - error: aborting due to 19 previous errors
2019-08-26T19:34:06.2200440Z + error: aborting due to 18 previous errors
2019-08-26T19:34:06.2200565Z 154 
2019-08-26T19:34:06.2200925Z 155 For more information about this error, try `rustc --explain E0080`.
2019-08-26T19:34:06.2200925Z 155 For more information about this error, try `rustc --explain E0080`.
2019-08-26T19:34:06.2201105Z 156 
2019-08-26T19:34:06.2201211Z 
2019-08-26T19:34:06.2201312Z 
2019-08-26T19:34:06.2201441Z The actual stderr differed from the expected stderr.
2019-08-26T19:34:06.2202060Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-wide-ptr/ub-wide-ptr.stderr
2019-08-26T19:34:06.2203167Z To update references, rerun the tests and pass the `--bless` flag
2019-08-26T19:34:06.2203672Z To only update this specific test, also pass `--test-args consts/const-eval/ub-wide-ptr.rs`
2019-08-26T19:34:06.2203978Z error: 1 errors occurred comparing output.
2019-08-26T19:34:06.2204128Z status: exit code: 1
2019-08-26T19:34:06.2204128Z status: exit code: 1
2019-08-26T19:34:06.2205029Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-wide-ptr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-wide-ptr/auxiliary" "-A" "unused"
2019-08-26T19:34:06.2205616Z ------------------------------------------
2019-08-26T19:34:06.2205785Z 
2019-08-26T19:34:06.2206130Z ------------------------------------------
2019-08-26T19:34:06.2206289Z stderr:
2019-08-26T19:34:06.2206289Z stderr:
2019-08-26T19:34:06.2206974Z ------------------------------------------
2019-08-26T19:34:06.2207149Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2207512Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:84:1
2019-08-26T19:34:06.2207727Z    |
2019-08-26T19:34:06.2207868Z LL | const STR_TOO_LONG: &str = unsafe { SliceTransmute { repr: SliceRepr { ptr: &42, len: 999 } }.str};
2019-08-26T19:34:06.2208044Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling reference (not entirely in bounds)
2019-08-26T19:34:06.2208191Z    |
2019-08-26T19:34:06.2208742Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-26T19:34:06.2209051Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2209051Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2209447Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:87:1
2019-08-26T19:34:06.2209614Z    |
2019-08-26T19:34:06.2209753Z LL | const STR_LENGTH_PTR: &str = unsafe { SliceTransmute { bad: BadSliceRepr { ptr: &42, len: &3 } }.str};
2019-08-26T19:34:06.2210578Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
2019-08-26T19:34:06.2214304Z    |
2019-08-26T19:34:06.2215086Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-26T19:34:06.2215526Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2215526Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2215954Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:90:1
2019-08-26T19:34:06.2216127Z    |
2019-08-26T19:34:06.2216285Z LL | const MY_STR_LENGTH_PTR: &MyStr = unsafe { SliceTransmute { bad: BadSliceRepr { ptr: &42, len: &3 } }.my_str};
2019-08-26T19:34:06.2216798Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
2019-08-26T19:34:06.2216975Z    |
2019-08-26T19:34:06.2217498Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-26T19:34:06.2217942Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2217942Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2218411Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:94:1
2019-08-26T19:34:06.2218585Z    |
2019-08-26T19:34:06.2218892Z LL | const J1: &str = unsafe { SliceTransmute { slice: &[0xFF] }.str };
2019-08-26T19:34:06.2219512Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized or non-UTF-8 data in str at .<deref>
2019-08-26T19:34:06.2219870Z    |
2019-08-26T19:34:06.2220530Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-26T19:34:06.2220876Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2220876Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2221260Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:97:1
2019-08-26T19:34:06.2221429Z    |
2019-08-26T19:34:06.2221574Z LL | const J2: &MyStr = unsafe { SliceTransmute { slice: &[0xFF] }.my_str };
2019-08-26T19:34:06.2222054Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized or non-UTF-8 data in str at .<deref>.0
2019-08-26T19:34:06.2222226Z    |
2019-08-26T19:34:06.2223319Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-26T19:34:06.2223646Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2223646Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2224008Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:104:1
2019-08-26T19:34:06.2224193Z    |
2019-08-26T19:34:06.2224328Z LL | const SLICE_LENGTH_UNINIT: &[u8] = unsafe { SliceTransmute { addr: 42 }.slice};
2019-08-26T19:34:06.2224507Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized data in wide pointer metadata
2019-08-26T19:34:06.2224647Z    |
2019-08-26T19:34:06.2225156Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-26T19:34:06.2225489Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2225489Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2225853Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:107:1
2019-08-26T19:34:06.2226040Z    |
2019-08-26T19:34:06.2226178Z LL | const SLICE_TOO_LONG: &[u8] = unsafe { SliceTransmute { repr: SliceRepr { ptr: &42, len: 999 } }.slice};
2019-08-26T19:34:06.2226370Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling reference (not entirely in bounds)
2019-08-26T19:34:06.2226548Z    |
2019-08-26T19:34:06.2227067Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-26T19:34:06.2227371Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2227371Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2227798Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:110:1
2019-08-26T19:34:06.2230625Z    |
2019-08-26T19:34:06.2230852Z LL | const SLICE_LENGTH_PTR: &[u8] = unsafe { SliceTransmute { bad: BadSliceRepr { ptr: &42, len: &3 } }.slice};
2019-08-26T19:34:06.2231440Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
2019-08-26T19:34:06.2231656Z    |
2019-08-26T19:34:06.2232332Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-26T19:34:06.2233362Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2233362Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2233813Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:114:1
2019-08-26T19:34:06.2234029Z    |
2019-08-26T19:34:06.2234168Z LL | const H: &[bool] = &[unsafe { BoolTransmute { val: 3 }.bl }];
2019-08-26T19:34:06.2234420Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 3 at .<deref>[0], but expected something less or equal to 1
2019-08-26T19:34:06.2234591Z    |
2019-08-26T19:34:06.2235136Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-26T19:34:06.2235481Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2235481Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2235861Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:120:1
2019-08-26T19:34:06.2236032Z    |
2019-08-26T19:34:06.2236339Z LL | const I2: &MySliceBool = &MySlice(unsafe { BoolTransmute { val: 3 }.bl }, [false]);
2019-08-26T19:34:06.2236588Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 3 at .<deref>.0, but expected something less or equal to 1
2019-08-26T19:34:06.2236880Z    |
2019-08-26T19:34:06.2237386Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-26T19:34:06.2237697Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2237697Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2238073Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:123:1
2019-08-26T19:34:06.2240848Z    |
2019-08-26T19:34:06.2240922Z LL | const I3: &MySliceBool = &MySlice(true, [unsafe { BoolTransmute { val: 3 }.bl }]);
2019-08-26T19:34:06.2241012Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 3 at .<deref>.1[0], but expected something less or equal to 1
2019-08-26T19:34:06.2241076Z    |
2019-08-26T19:34:06.2241609Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-26T19:34:06.2241732Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2241732Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2241988Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:131:1
2019-08-26T19:34:06.2242054Z    |
2019-08-26T19:34:06.2242108Z LL | const RAW_SLICE_LENGTH_UNINIT: *const [u8] = unsafe { SliceTransmute { addr: 42 }.raw_slice};
2019-08-26T19:34:06.2242185Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized data in wide pointer metadata
2019-08-26T19:34:06.2242257Z    |
2019-08-26T19:34:06.2243055Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-26T19:34:06.2243172Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2243172Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2243477Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:136:1
2019-08-26T19:34:06.2243526Z    |
2019-08-26T19:34:06.2243577Z LL | const D: &dyn Trait = unsafe { DynTransmute { repr: DynRepr { ptr: &92, vtable: &3 } }.rust};
2019-08-26T19:34:06.2243802Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling or unaligned vtable pointer in wide pointer or too small vtable
2019-08-26T19:34:06.2243871Z    |
2019-08-26T19:34:06.2244315Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-26T19:34:06.2244425Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2244425Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2244673Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:139:1
2019-08-26T19:34:06.2244738Z    |
2019-08-26T19:34:06.2244792Z LL | const E: &dyn Trait = unsafe { DynTransmute { repr2: DynRepr2 { ptr: &92, vtable: &3 } }.rust};
2019-08-26T19:34:06.2244864Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling or unaligned vtable pointer in wide pointer or too small vtable
2019-08-26T19:34:06.2244937Z    |
2019-08-26T19:34:06.2245332Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-26T19:34:06.2245443Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2245443Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2245800Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:142:1
2019-08-26T19:34:06.2246014Z    |
2019-08-26T19:34:06.2246079Z LL | const F: &dyn Trait = unsafe { DynTransmute { bad: BadDynRepr { ptr: &92, vtable: 3 } }.rust};
2019-08-26T19:34:06.2246145Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling or unaligned vtable pointer in wide pointer or too small vtable
2019-08-26T19:34:06.2246196Z    |
2019-08-26T19:34:06.2246572Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-26T19:34:06.2246656Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2246656Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2246894Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:146:1
2019-08-26T19:34:06.2246945Z    |
2019-08-26T19:34:06.2246989Z LL | const G: &dyn Trait = &unsafe { BoolTransmute { val: 3 }.bl };
2019-08-26T19:34:06.2247320Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 3 at .<deref>.<dyn-downcast>, but expected something less or equal to 1
2019-08-26T19:34:06.2247372Z    |
2019-08-26T19:34:06.2247717Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-26T19:34:06.2247823Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2247823Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2248045Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:150:1
2019-08-26T19:34:06.2248103Z    |
2019-08-26T19:34:06.2248332Z LL | const RAW_TRAIT_OBJ_VTABLE_NULL: *const dyn Trait = unsafe { DynTransmute { bad: BadDynRepr { ptr: &92, vtable: 0 } }.rust};
2019-08-26T19:34:06.2248414Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling or unaligned vtable pointer in wide pointer or too small vtable
2019-08-26T19:34:06.2248490Z    |
2019-08-26T19:34:06.2248854Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-26T19:34:06.2248973Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2248973Z error[E0080]: it is undefined behavior to use this value
2019-08-26T19:34:06.2249279Z   --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:152:1
2019-08-26T19:34:06.2249332Z    |
2019-08-26T19:34:06.2249402Z LL | const RAW_TRAIT_OBJ_VTABLE_INVALID: *const dyn Trait = unsafe { DynTransmute { repr2: DynRepr2 { ptr: &92, vtable: &3 } }.raw_rust};
2019-08-26T19:34:06.2249476Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling or unaligned vtable pointer in wide pointer or too small vtable
2019-08-26T19:34:06.2249558Z    |
2019-08-26T19:34:06.2249943Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-26T19:34:06.2250052Z error: aborting due to 18 previous errors
2019-08-26T19:34:06.2250080Z 
2019-08-26T19:34:06.2250316Z For more information about this error, try `rustc --explain E0080`.
2019-08-26T19:34:06.2250350Z 
2019-08-26T19:34:06.2250350Z 
2019-08-26T19:34:06.2250573Z ------------------------------------------
2019-08-26T19:34:06.2250604Z 
2019-08-26T19:34:06.2250628Z 
2019-08-26T19:34:06.2250652Z 
2019-08-26T19:34:06.2250691Z failures:
2019-08-26T19:34:06.2250918Z     [ui] ui/consts/const-eval/ub-wide-ptr.rs
2019-08-26T19:34:06.2251747Z test result: FAILED. 8914 passed; 1 failed; 38 ignored; 0 measured; 0 filtered out
2019-08-26T19:34:06.2251785Z 
2019-08-26T19:34:06.2254680Z 
2019-08-26T19:34:06.2254886Z 
2019-08-26T19:34:06.2254886Z 
2019-08-26T19:34:06.2257329Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-26T19:34:06.2258219Z 
2019-08-26T19:34:06.2258267Z 
2019-08-26T19:34:06.2258327Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-26T19:34:06.2258383Z Build completed unsuccessfully in 1:10:43
2019-08-26T19:34:06.2258383Z Build completed unsuccessfully in 1:10:43
2019-08-26T19:34:06.2260003Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-26T19:34:06.2260083Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-26T19:34:06.2295021Z == clock drift check ==
2019-08-26T19:34:06.2309763Z   local time: Mon Aug 26 19:34:06 UTC 2019
2019-08-26T19:34:06.4604263Z   network time: Mon, 26 Aug 2019 19:34:06 GMT
2019-08-26T19:34:06.4608461Z == end clock drift check ==
2019-08-26T19:34:07.4031525Z ##[error]Bash exited with code '1'.
2019-08-26T19:34:07.4080457Z ##[section]Starting: Checkout
2019-08-26T19:34:07.4082210Z ==============================================================================
2019-08-26T19:34:07.4082263Z Task         : Get sources
2019-08-26T19:34:07.4082308Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
