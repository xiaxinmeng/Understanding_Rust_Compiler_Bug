plain
2019-09-18T18:36:29.7247107Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-18T18:36:29.7441322Z ##[command]git config gc.auto 0
2019-09-18T18:36:30.7093255Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-18T18:36:30.7096085Z ##[command]git config --get-all http.proxy
2019-09-18T18:36:30.7101540Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64582/merge:refs/remotes/pull/64582/merge
---
2019-09-18T19:38:30.3628736Z .................................................................................................... 1500/9024
2019-09-18T19:38:36.2576658Z .................................................................................................... 1600/9024
2019-09-18T19:38:48.5698694Z .................................................................i...............i.................. 1700/9024
2019-09-18T19:38:55.6527506Z .................................................................................................... 1800/9024
2019-09-18T19:39:10.6707284Z ........................................................iiiii....................................... 1900/9024
2019-09-18T19:39:22.1820274Z .................................................................................................... 2100/9024
2019-09-18T19:39:24.6073818Z .................................................................................................... 2200/9024
2019-09-18T19:39:27.8989703Z .................................................................................................... 2300/9024
2019-09-18T19:39:36.2126089Z .................................................................................................... 2400/9024
---
2019-09-18T19:42:34.4689134Z ............................................i...............i....................................... 4700/9024
2019-09-18T19:42:45.1281574Z .................................................................................................... 4800/9024
2019-09-18T19:42:52.2692928Z .................................................................................................... 4900/9024
2019-09-18T19:43:02.0537079Z .................................................................................................... 5000/9024
2019-09-18T19:43:09.8909075Z ............................ii.ii................................................................... 5100/9024
2019-09-18T19:43:20.1598701Z .................................................................................................... 5300/9024
2019-09-18T19:43:30.6484368Z ............................................................................................i....... 5400/9024
2019-09-18T19:43:38.9689901Z .................................................................................................... 5500/9024
2019-09-18T19:43:43.9200274Z .................................................................................................... 5600/9024
2019-09-18T19:43:43.9200274Z .................................................................................................... 5600/9024
2019-09-18T19:43:54.6558200Z .......................................................................................ii...i..ii... 5700/9024
2019-09-18T19:44:20.2397243Z .................................................................................................... 5900/9024
2019-09-18T19:44:30.6018115Z .................................................................................................... 6000/9024
2019-09-18T19:44:30.6018115Z .................................................................................................... 6000/9024
2019-09-18T19:44:39.9671308Z .........................................................................................i..ii...... 6100/9024
2019-09-18T19:45:09.1764947Z .................................................................................................... 6300/9024
2019-09-18T19:45:13.3285787Z ................................................i................................................... 6400/9024
2019-09-18T19:45:15.4892593Z ...............................................F.................................................... 6500/9024
2019-09-18T19:45:17.8846279Z ....................i............................................................................... 6600/9024
---
2019-09-18T19:49:20.3385596Z 
2019-09-18T19:49:20.3406571Z ---- [ui] ui/codemap_tests/unicode.rs stdout ----
2019-09-18T19:49:20.3408961Z diff of stderr:
2019-09-18T19:49:20.3409725Z 
2019-09-18T19:49:20.3411293Z 4 LL | extern "路濫狼á́́" fn foo() {}
2019-09-18T19:49:20.3413517Z 5    |        ^^^^^^^^^ invalid ABI
2019-09-18T19:49:20.3415516Z 6    |
2019-09-18T19:49:20.3417783Z -    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, Rust, C, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
2019-09-18T19:49:20.3420368Z +    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, Rust, C, Swift, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
2019-09-18T19:49:20.3421713Z 9 error: aborting due to previous error
2019-09-18T19:49:20.3423225Z 10 
2019-09-18T19:49:20.3423437Z 
2019-09-18T19:49:20.3424450Z 
2019-09-18T19:49:20.3424450Z 
2019-09-18T19:49:20.3424715Z The actual stderr differed from the expected stderr.
2019-09-18T19:49:20.3425280Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode/unicode.stderr
2019-09-18T19:49:20.3426956Z To update references, rerun the tests and pass the `--bless` flag
2019-09-18T19:49:20.3428726Z To only update this specific test, also pass `--test-args codemap_tests/unicode.rs`
2019-09-18T19:49:20.3430889Z error: 1 errors occurred comparing output.
2019-09-18T19:49:20.3431104Z status: exit code: 1
2019-09-18T19:49:20.3431104Z status: exit code: 1
2019-09-18T19:49:20.3432699Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/codemap_tests/unicode.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode/auxiliary" "-A" "unused"
2019-09-18T19:49:20.3435469Z ------------------------------------------
2019-09-18T19:49:20.3435675Z 
2019-09-18T19:49:20.3436422Z ------------------------------------------
2019-09-18T19:49:20.3436750Z stderr:
2019-09-18T19:49:20.3436750Z stderr:
2019-09-18T19:49:20.3437235Z ------------------------------------------
2019-09-18T19:49:20.3437586Z error[E0703]: invalid ABI: found `路濫狼á́́`
2019-09-18T19:49:20.3438104Z    |
2019-09-18T19:49:20.3438104Z    |
2019-09-18T19:49:20.3438420Z LL | extern "路濫狼á́́" fn foo() {} //~ ERROR invalid ABI
2019-09-18T19:49:20.3438587Z    |        ^^^^^^^^^ invalid ABI
2019-09-18T19:49:20.3438715Z    |
2019-09-18T19:49:20.3439153Z    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, Rust, C, Swift, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
2019-09-18T19:49:20.3439616Z error: aborting due to previous error
2019-09-18T19:49:20.3439715Z 
2019-09-18T19:49:20.3439840Z 
2019-09-18T19:49:20.3440296Z ------------------------------------------
2019-09-18T19:49:20.3440296Z ------------------------------------------
2019-09-18T19:49:20.3440620Z 
2019-09-18T19:49:20.3440898Z 
2019-09-18T19:49:20.3441257Z ---- [ui] ui/feature-gates/feature-gate-abi.rs stdout ----
2019-09-18T19:49:20.3441422Z diff of stderr:
2019-09-18T19:49:20.3441526Z 
2019-09-18T19:49:20.3442124Z + error[E0407]: method `m10` is not a member of trait `Tr`
2019-09-18T19:49:20.3442563Z +   --> $DIR/feature-gate-abi.rs:65:5
2019-09-18T19:49:20.3442757Z +    |
2019-09-18T19:49:20.3442901Z + LL |     extern "Swift" fn m10() {}
2019-09-18T19:49:20.3443085Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ not a member of trait `Tr`
2019-09-18T19:49:20.3443376Z 1 error[E0658]: intrinsics are subject to change
2019-09-18T19:49:20.3443754Z -   --> $DIR/feature-gate-abi.rs:12:1
2019-09-18T19:49:20.3444539Z +   --> $DIR/feature-gate-abi.rs:13:1
2019-09-18T19:49:20.3444762Z 3    |
2019-09-18T19:49:20.3444762Z 3    |
2019-09-18T19:49:20.3445130Z 4 LL | extern "rust-intrinsic" fn f1() {}
2019-09-18T19:49:20.3445439Z 
2019-09-18T19:49:20.3445586Z 7    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-09-18T19:49:20.3445748Z 8 
2019-09-18T19:49:20.3446087Z 9 error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T19:49:20.3446087Z 9 error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T19:49:20.3446553Z -   --> $DIR/feature-gate-abi.rs:14:1
2019-09-18T19:49:20.3446950Z +   --> $DIR/feature-gate-abi.rs:15:1
2019-09-18T19:49:20.3447283Z 11    |
2019-09-18T19:49:20.3447676Z 12 LL | extern "platform-intrinsic" fn f2() {}
2019-09-18T19:49:20.3448212Z 
2019-09-18T19:49:20.3448581Z 16    = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
2019-09-18T19:49:20.3448708Z 17 
2019-09-18T19:49:20.3448838Z 18 error[E0658]: vectorcall is experimental and subject to change
2019-09-18T19:49:20.3448838Z 18 error[E0658]: vectorcall is experimental and subject to change
2019-09-18T19:49:20.3449357Z -   --> $DIR/feature-gate-abi.rs:16:1
2019-09-18T19:49:20.3450522Z +   --> $DIR/feature-gate-abi.rs:17:1
2019-09-18T19:49:20.3451055Z 20    |
2019-09-18T19:49:20.3451177Z 21 LL | extern "vectorcall" fn f3() {}
2019-09-18T19:49:20.3451820Z 
2019-09-18T19:49:20.3452014Z 24    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-18T19:49:20.3452184Z 25 
2019-09-18T19:49:20.3452627Z 26 error[E0658]: rust-call ABI is subject to change
2019-09-18T19:49:20.3452627Z 26 error[E0658]: rust-call ABI is subject to change
2019-09-18T19:49:20.3453037Z -   --> $DIR/feature-gate-abi.rs:17:1
2019-09-18T19:49:20.3453825Z +   --> $DIR/feature-gate-abi.rs:18:1
2019-09-18T19:49:20.3454250Z 28    |
2019-09-18T19:49:20.3459044Z 29 LL | extern "rust-call" fn f4() {}
2019-09-18T19:49:20.3466855Z 
2019-09-18T19:49:20.3466895Z 33    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-09-18T19:49:20.3466933Z 34 
2019-09-18T19:49:20.3466933Z 34 
2019-09-18T19:49:20.3468628Z 35 error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-09-18T19:49:20.3469200Z +   --> $DIR/feature-gate-abi.rs:19:1
2019-09-18T19:49:20.3469264Z 37    |
2019-09-18T19:49:20.3469264Z 37    |
2019-09-18T19:49:20.3469448Z 38 LL | extern "msp430-interrupt" fn f5() {}
2019-09-18T19:49:20.3469531Z 
2019-09-18T19:49:20.3469531Z 
2019-09-18T19:49:20.3469588Z 42    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-09-18T19:49:20.3469625Z 43 
2019-09-18T19:49:20.3469664Z 44 error[E0658]: PTX ABIs are experimental and subject to change
2019-09-18T19:49:20.3470049Z +   --> $DIR/feature-gate-abi.rs:20:1
2019-09-18T19:49:20.3470087Z 46    |
2019-09-18T19:49:20.3470087Z 46    |
2019-09-18T19:49:20.3470273Z 47 LL | extern "ptx-kernel" fn f6() {}
2019-09-18T19:49:20.3470336Z 
2019-09-18T19:49:20.3470336Z 
2019-09-18T19:49:20.3470375Z 51    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-09-18T19:49:20.3470427Z 52 
2019-09-18T19:49:20.3470629Z 53 error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-09-18T19:49:20.3471174Z +   --> $DIR/feature-gate-abi.rs:21:1
2019-09-18T19:49:20.3471217Z 55    |
2019-09-18T19:49:20.3471217Z 55    |
2019-09-18T19:49:20.3471387Z 56 LL | extern "x86-interrupt" fn f7() {}
2019-09-18T19:49:20.3471466Z 
2019-09-18T19:49:20.3471466Z 
2019-09-18T19:49:20.3471504Z 60    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-09-18T19:49:20.3477818Z 62 error[E0658]: thiscall is experimental and subject to change
2019-09-18T19:49:20.3478152Z -   --> $DIR/feature-gate-abi.rs:21:1
2019-09-18T19:49:20.3478334Z +   --> $DIR/feature-gate-abi.rs:22:1
2019-09-18T19:49:20.3478392Z 64    |
2019-09-18T19:49:20.3478392Z 64    |
2019-09-18T19:49:20.3478425Z 65 LL | extern "thiscall" fn f8() {}
2019-09-18T19:49:20.3478484Z 
2019-09-18T19:49:20.3478538Z 68    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-09-18T19:49:20.3478574Z 69 
2019-09-18T19:49:20.3478574Z 69 
2019-09-18T19:49:20.3492740Z 70 error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-09-18T19:49:20.3493679Z +   --> $DIR/feature-gate-abi.rs:23:1
2019-09-18T19:49:20.3493730Z 72    |
2019-09-18T19:49:20.3493730Z 72    |
2019-09-18T19:49:20.3493980Z 73 LL | extern "amdgpu-kernel" fn f9() {}
2019-09-18T19:49:20.3494077Z 
2019-09-18T19:49:20.3494077Z 
2019-09-18T19:49:20.3494128Z 77    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-09-18T19:49:20.3494195Z 78 
2019-09-18T19:49:20.3494244Z 79 error[E0658]: Swift ABI is experimental and subject to change
2019-09-18T19:49:20.3494700Z +   --> $DIR/feature-gate-abi.rs:24:1
2019-09-18T19:49:20.3494748Z 81    |
2019-09-18T19:49:20.3494748Z 81    |
2019-09-18T19:49:20.3494791Z 82 LL | extern "Swift" fn f10() {}
2019-09-18T19:49:20.3495077Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-18T19:49:20.3495121Z 84    |
2019-09-18T19:49:20.3495121Z 84    |
2019-09-18T19:49:20.3496023Z -    = note: for more information, see ***/issues/0
2019-09-18T19:49:20.3496327Z 86    = help: add `#![feature(abi_swift)]` to the crate attributes to enable
2019-09-18T19:49:20.3496407Z 88 error[E0658]: intrinsics are subject to change
2019-09-18T19:49:20.3496564Z 
2019-09-18T19:49:20.3496797Z -   --> $DIR/feature-gate-abi.rs:26:5
2019-09-18T19:49:20.3496987Z +   --> $DIR/feature-gate-abi.rs:28:5
2019-09-18T19:49:20.3496987Z +   --> $DIR/feature-gate-abi.rs:28:5
2019-09-18T19:49:20.3497045Z 90    |
2019-09-18T19:49:20.3497238Z 91 LL |     extern "rust-intrinsic" fn m1();
2019-09-18T19:49:20.3497309Z 
2019-09-18T19:49:20.3497369Z 94    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-09-18T19:49:20.3497408Z 95 
2019-09-18T19:49:20.3497449Z 96 error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T19:49:20.3497449Z 96 error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T19:49:20.3497657Z -   --> $DIR/feature-gate-abi.rs:28:5
2019-09-18T19:49:20.3497848Z +   --> $DIR/feature-gate-abi.rs:30:5
2019-09-18T19:49:20.3497897Z 98    |
2019-09-18T19:49:20.3498115Z 99 LL |     extern "platform-intrinsic" fn m2();
2019-09-18T19:49:20.3498188Z 
2019-09-18T19:49:20.3498232Z 103    = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
2019-09-18T19:49:20.3498297Z 104 
2019-09-18T19:49:20.3498338Z 105 error[E0658]: vectorcall is experimental and subject to change
2019-09-18T19:49:20.3498338Z 105 error[E0658]: vectorcall is experimental and subject to change
2019-09-18T19:49:20.3498653Z -   --> $DIR/feature-gate-abi.rs:30:5
2019-09-18T19:49:20.3498865Z +   --> $DIR/feature-gate-abi.rs:32:5
2019-09-18T19:49:20.3498908Z 107    |
2019-09-18T19:49:20.3498946Z 108 LL |     extern "vectorcall" fn m3();
2019-09-18T19:49:20.3499029Z 
2019-09-18T19:49:20.3499071Z 111    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-18T19:49:20.3499111Z 112 
2019-09-18T19:49:20.3499343Z 113 error[E0658]: rust-call ABI is subject to change
2019-09-18T19:49:20.3499343Z 113 error[E0658]: rust-call ABI is subject to change
2019-09-18T19:49:20.3499717Z -   --> $DIR/feature-gate-abi.rs:31:5
2019-09-18T19:49:20.3499900Z +   --> $DIR/feature-gate-abi.rs:33:5
2019-09-18T19:49:20.3499956Z 115    |
2019-09-18T19:49:20.3500379Z 116 LL |     extern "rust-call" fn m4();
2019-09-18T19:49:20.3500456Z 
2019-09-18T19:49:20.3500583Z 120    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-09-18T19:49:20.3500624Z 121 
2019-09-18T19:49:20.3500624Z 121 
2019-09-18T19:49:20.3500847Z 122 error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-09-18T19:49:20.3501515Z +   --> $DIR/feature-gate-abi.rs:34:5
2019-09-18T19:49:20.3501557Z 124    |
2019-09-18T19:49:20.3501557Z 124    |
2019-09-18T19:49:20.3502735Z 125 LL |     extern "msp430-interrupt" fn m5();
2019-09-18T19:49:20.3502851Z 
2019-09-18T19:49:20.3502851Z 
2019-09-18T19:49:20.3503024Z 129    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-09-18T19:49:20.3503103Z 130 
2019-09-18T19:49:20.3503150Z 131 error[E0658]: PTX ABIs are experimental and subject to change
2019-09-18T19:49:20.3503688Z +   --> $DIR/feature-gate-abi.rs:35:5
2019-09-18T19:49:20.3503736Z 133    |
2019-09-18T19:49:20.3503736Z 133    |
2019-09-18T19:49:20.3503955Z 134 LL |     extern "ptx-kernel" fn m6();
2019-09-18T19:49:20.3504052Z 
2019-09-18T19:49:20.3504052Z 
2019-09-18T19:49:20.3504100Z 138    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-09-18T19:49:20.3504146Z 139 
2019-09-18T19:49:20.3504415Z 140 error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-09-18T19:49:20.3504850Z +   --> $DIR/feature-gate-abi.rs:36:5
2019-09-18T19:49:20.3504895Z 142    |
2019-09-18T19:49:20.3504895Z 142    |
2019-09-18T19:49:20.3505132Z 143 LL |     extern "x86-interrupt" fn m7();
2019-09-18T19:49:20.3505221Z 
2019-09-18T19:49:20.3505221Z 
2019-09-18T19:49:20.3505287Z 147    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-09-18T19:49:20.3505471Z 149 error[E0658]: thiscall is experimental and subject to change
2019-09-18T19:49:20.3505734Z -   --> $DIR/feature-gate-abi.rs:35:5
2019-09-18T19:49:20.3505955Z +   --> $DIR/feature-gate-abi.rs:37:5
2019-09-18T19:49:20.3506002Z 151    |
2019-09-18T19:49:20.3506002Z 151    |
2019-09-18T19:49:20.3506046Z 152 LL |     extern "thiscall" fn m8();
2019-09-18T19:49:20.3506137Z 
2019-09-18T19:49:20.3506186Z 155    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-09-18T19:49:20.3506250Z 156 
2019-09-18T19:49:20.3506250Z 156 
2019-09-18T19:49:20.3506503Z 157 error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-09-18T19:49:20.3506941Z +   --> $DIR/feature-gate-abi.rs:38:5
2019-09-18T19:49:20.3507009Z 159    |
2019-09-18T19:49:20.3507009Z 159    |
2019-09-18T19:49:20.3507235Z 160 LL |     extern "amdgpu-kernel" fn m9();
2019-09-18T19:49:20.3507332Z 
2019-09-18T19:49:20.3507332Z 
2019-09-18T19:49:20.3507654Z 163    = note: for more information, see ***/issues/51575
2019-09-18T19:49:20.3507715Z 164    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-09-18T19:49:20.3507779Z 165 
2019-09-18T19:49:20.3508032Z - error[E0658]: Swift ABI is experimental and subject to change
2019-09-18T19:49:20.3508762Z -    |
2019-09-18T19:49:20.3508762Z -    |
2019-09-18T19:49:20.3509120Z - LL |     extern "Swift" fn m10();
2019-09-18T19:49:20.3510719Z -    |
2019-09-18T19:49:20.3510719Z -    |
2019-09-18T19:49:20.3511036Z -    = note: for more information, see ***/issues/0
2019-09-18T19:49:20.3511462Z -    = help: add `#![feature(abi_swift)]` to the crate attributes to enable
2019-09-18T19:49:20.3511947Z 175 error[E0658]: vectorcall is experimental and subject to change
2019-09-18T19:49:20.3512168Z -   --> $DIR/feature-gate-abi.rs:38:5
2019-09-18T19:49:20.3512385Z +   --> $DIR/feature-gate-abi.rs:40:5
2019-09-18T19:49:20.3512459Z 177    |
2019-09-18T19:49:20.3512459Z 177    |
2019-09-18T19:49:20.3512505Z 178 LL |     extern "vectorcall" fn dm3() {}
2019-09-18T19:49:20.3512600Z 
2019-09-18T19:49:20.3512649Z 181    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-18T19:49:20.3512696Z 182 
2019-09-18T19:49:20.3512949Z 183 error[E0658]: rust-call ABI is subject to change
2019-09-18T19:49:20.3512949Z 183 error[E0658]: rust-call ABI is subject to change
2019-09-18T19:49:20.3513170Z -   --> $DIR/feature-gate-abi.rs:39:5
2019-09-18T19:49:20.3513387Z +   --> $DIR/feature-gate-abi.rs:41:5
2019-09-18T19:49:20.3513436Z 185    |
2019-09-18T19:49:20.3513676Z 186 LL |     extern "rust-call" fn dm4() {}
2019-09-18T19:49:20.3513942Z 
2019-09-18T19:49:20.3514011Z 190    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-09-18T19:49:20.3514058Z 191 
2019-09-18T19:49:20.3514058Z 191 
2019-09-18T19:49:20.3514349Z 192 error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-09-18T19:49:20.3515042Z +   --> $DIR/feature-gate-abi.rs:42:5
2019-09-18T19:49:20.3515086Z 194    |
2019-09-18T19:49:20.3515086Z 194    |
2019-09-18T19:49:20.3515478Z 195 LL |     extern "msp430-interrupt" fn dm5() {}
2019-09-18T19:49:20.3515576Z 
2019-09-18T19:49:20.3515576Z 
2019-09-18T19:49:20.3515623Z 199    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-09-18T19:49:20.3515900Z 200 
2019-09-18T19:49:20.3515943Z 201 error[E0658]: PTX ABIs are experimental and subject to change
2019-09-18T19:49:20.3516353Z +   --> $DIR/feature-gate-abi.rs:43:5
2019-09-18T19:49:20.3516414Z 203    |
2019-09-18T19:49:20.3516414Z 203    |
2019-09-18T19:49:20.3516852Z 204 LL |     extern "ptx-kernel" fn dm6() {}
2019-09-18T19:49:20.3517120Z 
2019-09-18T19:49:20.3517120Z 
2019-09-18T19:49:20.3517255Z 208    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-09-18T19:49:20.3517297Z 209 
2019-09-18T19:49:20.3517574Z 210 error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-09-18T19:49:20.3517982Z +   --> $DIR/feature-gate-abi.rs:44:5
2019-09-18T19:49:20.3518024Z 212    |
2019-09-18T19:49:20.3518024Z 212    |
2019-09-18T19:49:20.3518249Z 213 LL |     extern "x86-interrupt" fn dm7() {}
2019-09-18T19:49:20.3518323Z 
2019-09-18T19:49:20.3518323Z 
2019-09-18T19:49:20.3518385Z 217    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-09-18T19:49:20.3518478Z 219 error[E0658]: thiscall is experimental and subject to change
2019-09-18T19:49:20.3518865Z -   --> $DIR/feature-gate-abi.rs:43:5
2019-09-18T19:49:20.3519462Z +   --> $DIR/feature-gate-abi.rs:45:5
2019-09-18T19:49:20.3519506Z 221    |
2019-09-18T19:49:20.3519506Z 221    |
2019-09-18T19:49:20.3519547Z 222 LL |     extern "thiscall" fn dm8() {}
2019-09-18T19:49:20.3519797Z 
2019-09-18T19:49:20.3519841Z 225    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-09-18T19:49:20.3519899Z 226 
2019-09-18T19:49:20.3519899Z 226 
2019-09-18T19:49:20.3520466Z 227 error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-09-18T19:49:20.3521171Z +   --> $DIR/feature-gate-abi.rs:46:5
2019-09-18T19:49:20.3521227Z 229    |
2019-09-18T19:49:20.3521227Z 229    |
2019-09-18T19:49:20.3521592Z 230 LL |     extern "amdgpu-kernel" fn dm9() {}
2019-09-18T19:49:20.3521676Z 
2019-09-18T19:49:20.3521676Z 
2019-09-18T19:49:20.3521726Z 234    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-09-18T19:49:20.3521765Z 235 
2019-09-18T19:49:20.3521805Z 236 error[E0658]: Swift ABI is experimental and subject to change
2019-09-18T19:49:20.3522570Z +   --> $DIR/feature-gate-abi.rs:47:5
2019-09-18T19:49:20.3522608Z 238    |
2019-09-18T19:49:20.3522608Z 238    |
2019-09-18T19:49:20.3522660Z 239 LL |     extern "Swift" fn dm10() {}
2019-09-18T19:49:20.3522884Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-18T19:49:20.3522934Z 241    |
2019-09-18T19:49:20.3522934Z 241    |
2019-09-18T19:49:20.3523178Z -    = note: for more information, see ***/issues/0
2019-09-18T19:49:20.3523382Z 243    = help: add `#![feature(abi_swift)]` to the crate attributes to enable
2019-09-18T19:49:20.3523471Z 245 error[E0658]: intrinsics are subject to change
2019-09-18T19:49:20.3523494Z 
2019-09-18T19:49:20.3523757Z -   --> $DIR/feature-gate-abi.rs:51:5
2019-09-18T19:49:20.3524400Z +   --> $DIR/feature-gate-abi.rs:54:5
2019-09-18T19:49:20.3524400Z +   --> $DIR/feature-gate-abi.rs:54:5
2019-09-18T19:49:20.3524443Z 247    |
2019-09-18T19:49:20.3524628Z 248 LL |     extern "rust-intrinsic" fn m1() {}
2019-09-18T19:49:20.3524721Z 
2019-09-18T19:49:20.3524760Z 251    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-09-18T19:49:20.3524812Z 252 
2019-09-18T19:49:20.3524851Z 253 error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T19:49:20.3524851Z 253 error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T19:49:20.3525034Z -   --> $DIR/feature-gate-abi.rs:53:5
2019-09-18T19:49:20.3525211Z +   --> $DIR/feature-gate-abi.rs:56:5
2019-09-18T19:49:20.3525266Z 255    |
2019-09-18T19:49:20.3525628Z 256 LL |     extern "platform-intrinsic" fn m2() {}
2019-09-18T19:49:20.3525714Z 
2019-09-18T19:49:20.3525762Z 260    = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
2019-09-18T19:49:20.3525801Z 261 
2019-09-18T19:49:20.3525840Z 262 error[E0658]: vectorcall is experimental and subject to change
2019-09-18T19:49:20.3525840Z 262 error[E0658]: vectorcall is experimental and subject to change
2019-09-18T19:49:20.3526504Z -   --> $DIR/feature-gate-abi.rs:55:5
2019-09-18T19:49:20.3526728Z +   --> $DIR/feature-gate-abi.rs:58:5
2019-09-18T19:49:20.3526910Z 264    |
2019-09-18T19:49:20.3526976Z 265 LL |     extern "vectorcall" fn m3() {}
2019-09-18T19:49:20.3527052Z 
2019-09-18T19:49:20.3527117Z 268    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-18T19:49:20.3527164Z 269 
2019-09-18T19:49:20.3527428Z 270 error[E0658]: rust-call ABI is subject to change
2019-09-18T19:49:20.3527428Z 270 error[E0658]: rust-call ABI is subject to change
2019-09-18T19:49:20.3527647Z -   --> $DIR/feature-gate-abi.rs:56:5
2019-09-18T19:49:20.3527880Z +   --> $DIR/feature-gate-abi.rs:59:5
2019-09-18T19:49:20.3527926Z 272    |
2019-09-18T19:49:20.3528142Z 273 LL |     extern "rust-call" fn m4() {}
2019-09-18T19:49:20.3528247Z 
2019-09-18T19:49:20.3528296Z 277    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-09-18T19:49:20.3528341Z 278 
2019-09-18T19:49:20.3528341Z 278 
2019-09-18T19:49:20.3528614Z 279 error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-09-18T19:49:20.3529058Z +   --> $DIR/feature-gate-abi.rs:60:5
2019-09-18T19:49:20.3529122Z 281    |
2019-09-18T19:49:20.3529122Z 281    |
2019-09-18T19:49:20.3529349Z 282 LL |     extern "msp430-interrupt" fn m5() {}
2019-09-18T19:49:20.3529447Z 
2019-09-18T19:49:20.3529447Z 
2019-09-18T19:49:20.3529497Z 286    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-09-18T19:49:20.3529701Z 287 
2019-09-18T19:49:20.3529745Z 288 error[E0658]: PTX ABIs are experimental and subject to change
2019-09-18T19:49:20.3530550Z +   --> $DIR/feature-gate-abi.rs:61:5
2019-09-18T19:49:20.3530591Z 290    |
2019-09-18T19:49:20.3530591Z 290    |
2019-09-18T19:49:20.3530790Z 291 LL |     extern "ptx-kernel" fn m6() {}
2019-09-18T19:49:20.3530855Z 
2019-09-18T19:49:20.3530855Z 
2019-09-18T19:49:20.3530893Z 295    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-09-18T19:49:20.3530954Z 296 
2019-09-18T19:49:20.3531159Z 297 error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-09-18T19:49:20.3531533Z +   --> $DIR/feature-gate-abi.rs:62:5
2019-09-18T19:49:20.3531570Z 299    |
2019-09-18T19:49:20.3531570Z 299    |
2019-09-18T19:49:20.3532139Z 300 LL |     extern "x86-interrupt" fn m7() {}
2019-09-18T19:49:20.3532244Z 
2019-09-18T19:49:20.3532244Z 
2019-09-18T19:49:20.3532293Z 304    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-09-18T19:49:20.3532513Z 306 error[E0658]: thiscall is experimental and subject to change
2019-09-18T19:49:20.3532772Z -   --> $DIR/feature-gate-abi.rs:60:5
2019-09-18T19:49:20.3532986Z +   --> $DIR/feature-gate-abi.rs:63:5
2019-09-18T19:49:20.3533049Z 308    |
2019-09-18T19:49:20.3533049Z 308    |
2019-09-18T19:49:20.3533093Z 309 LL |     extern "thiscall" fn m8() {}
2019-09-18T19:49:20.3533178Z 
2019-09-18T19:49:20.3533244Z 312    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-09-18T19:49:20.3533290Z 313 
2019-09-18T19:49:20.3533290Z 313 
2019-09-18T19:49:20.3533545Z 314 error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-09-18T19:49:20.3533997Z +   --> $DIR/feature-gate-abi.rs:64:5
2019-09-18T19:49:20.3534044Z 316    |
2019-09-18T19:49:20.3534044Z 316    |
2019-09-18T19:49:20.3534278Z 317 LL |     extern "amdgpu-kernel" fn m9() {}
2019-09-18T19:49:20.3534358Z 
2019-09-18T19:49:20.3534358Z 
2019-09-18T19:49:20.3534415Z 321    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-09-18T19:49:20.3534477Z 322 
2019-09-18T19:49:20.3534523Z 323 error[E0658]: Swift ABI is experimental and subject to change
2019-09-18T19:49:20.3535092Z +   --> $DIR/feature-gate-abi.rs:65:5
2019-09-18T19:49:20.3535139Z 325    |
2019-09-18T19:49:20.3535139Z 325    |
2019-09-18T19:49:20.3535183Z 326 LL |     extern "Swift" fn m10() {}
2019-09-18T19:49:20.3535618Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-18T19:49:20.3535650Z 328    |
2019-09-18T19:49:20.3535650Z 328    |
2019-09-18T19:49:20.3535911Z -    = note: for more information, see ***/issues/0
2019-09-18T19:49:20.3535959Z 330    = help: add `#![feature(abi_swift)]` to the crate attributes to enable
2019-09-18T19:49:20.3536046Z 332 error[E0658]: intrinsics are subject to change
2019-09-18T19:49:20.3536071Z 
2019-09-18T19:49:20.3536262Z -   --> $DIR/feature-gate-abi.rs:66:5
2019-09-18T19:49:20.3536439Z +   --> $DIR/feature-gate-abi.rs:70:5
2019-09-18T19:49:20.3536439Z +   --> $DIR/feature-gate-abi.rs:70:5
2019-09-18T19:49:20.3536491Z 334    |
2019-09-18T19:49:20.3536669Z 335 LL |     extern "rust-intrinsic" fn im1() {}
2019-09-18T19:49:20.3536742Z 
2019-09-18T19:49:20.3536796Z 338    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-09-18T19:49:20.3536832Z 339 
2019-09-18T19:49:20.3536869Z 340 error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T19:49:20.3536869Z 340 error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T19:49:20.3537063Z -   --> $DIR/feature-gate-abi.rs:68:5
2019-09-18T19:49:20.3537235Z +   --> $DIR/feature-gate-abi.rs:72:5
2019-09-18T19:49:20.3537271Z 342    |
2019-09-18T19:49:20.3537464Z 343 LL |     extern "platform-intrinsic" fn im2() {}
2019-09-18T19:49:20.3537531Z 
2019-09-18T19:49:20.3537570Z 347    = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
2019-09-18T19:49:20.3537628Z 348 
2019-09-18T19:49:20.3537665Z 349 error[E0658]: vectorcall is experimental and subject to change
2019-09-18T19:49:20.3537665Z 349 error[E0658]: vectorcall is experimental and subject to change
2019-09-18T19:49:20.3537844Z -   --> $DIR/feature-gate-abi.rs:70:5
2019-09-18T19:49:20.3538032Z +   --> $DIR/feature-gate-abi.rs:74:5
2019-09-18T19:49:20.3538075Z 351    |
2019-09-18T19:49:20.3538110Z 352 LL |     extern "vectorcall" fn im3() {}
2019-09-18T19:49:20.3538186Z 
2019-09-18T19:49:20.3538223Z 355    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-18T19:49:20.3538260Z 356 
2019-09-18T19:49:20.3538461Z 357 error[E0658]: rust-call ABI is subject to change
2019-09-18T19:49:20.3538461Z 357 error[E0658]: rust-call ABI is subject to change
2019-09-18T19:49:20.3538634Z -   --> $DIR/feature-gate-abi.rs:71:5
2019-09-18T19:49:20.3538808Z +   --> $DIR/feature-gate-abi.rs:75:5
2019-09-18T19:49:20.3538860Z 359    |
2019-09-18T19:49:20.3539035Z 360 LL |     extern "rust-call" fn im4() {}
2019-09-18T19:49:20.3539174Z 
2019-09-18T19:49:20.3539240Z 364    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-09-18T19:49:20.3539277Z 365 
2019-09-18T19:49:20.3539277Z 365 
2019-09-18T19:49:20.3539499Z 366 error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-09-18T19:49:20.3539867Z +   --> $DIR/feature-gate-abi.rs:76:5
2019-09-18T19:49:20.3539904Z 368    |
2019-09-18T19:49:20.3539904Z 368    |
2019-09-18T19:49:20.3540097Z 369 LL |     extern "msp430-interrupt" fn im5() {}
2019-09-18T19:49:20.3540164Z 
2019-09-18T19:49:20.3540164Z 
2019-09-18T19:49:20.3540202Z 373    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-09-18T19:49:20.3540255Z 374 
2019-09-18T19:49:20.3540291Z 375 error[E0658]: PTX ABIs are experimental and subject to change
2019-09-18T19:49:20.3540662Z +   --> $DIR/feature-gate-abi.rs:77:5
2019-09-18T19:49:20.3540701Z 377    |
2019-09-18T19:49:20.3540701Z 377    |
2019-09-18T19:49:20.3540879Z 378 LL |     extern "ptx-kernel" fn im6() {}
2019-09-18T19:49:20.3540957Z 
2019-09-18T19:49:20.3540957Z 
2019-09-18T19:49:20.3540994Z 382    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-09-18T19:49:20.3541103Z 383 
2019-09-18T19:49:20.3541341Z 384 error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-09-18T19:49:20.3542115Z +   --> $DIR/feature-gate-abi.rs:78:5
2019-09-18T19:49:20.3542187Z 386    |
2019-09-18T19:49:20.3542187Z 386    |
2019-09-18T19:49:20.3542413Z 387 LL |     extern "x86-interrupt" fn im7() {}
2019-09-18T19:49:20.3542495Z 
2019-09-18T19:49:20.3542495Z 
2019-09-18T19:49:20.3542560Z 391    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-09-18T19:49:20.3542664Z 393 error[E0658]: thiscall is experimental and subject to change
2019-09-18T19:49:20.3542901Z -   --> $DIR/feature-gate-abi.rs:75:5
2019-09-18T19:49:20.3543121Z +   --> $DIR/feature-gate-abi.rs:79:5
2019-09-18T19:49:20.3543167Z 395    |
2019-09-18T19:49:20.3543167Z 395    |
2019-09-18T19:49:20.3543227Z 396 LL |     extern "thiscall" fn im8() {}
2019-09-18T19:49:20.3543313Z 
2019-09-18T19:49:20.3543361Z 399    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-09-18T19:49:20.3543423Z 400 
2019-09-18T19:49:20.3543423Z 400 
2019-09-18T19:49:20.3543677Z 401 error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-09-18T19:49:20.3544126Z +   --> $DIR/feature-gate-abi.rs:80:5
2019-09-18T19:49:20.3544172Z 403    |
2019-09-18T19:49:20.3544172Z 403    |
2019-09-18T19:49:20.3544395Z 404 LL |     extern "amdgpu-kernel" fn im9() {}
2019-09-18T19:49:20.3544491Z 
2019-09-18T19:49:20.3544491Z 
2019-09-18T19:49:20.3544548Z 408    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-09-18T19:49:20.3544596Z 409 
2019-09-18T19:49:20.3544659Z 410 error[E0658]: Swift ABI is experimental and subject to change
2019-09-18T19:49:20.3545107Z +   --> $DIR/feature-gate-abi.rs:81:5
2019-09-18T19:49:20.3545168Z 412    |
2019-09-18T19:49:20.3545168Z 412    |
2019-09-18T19:49:20.3545212Z 413 LL |     extern "Swift" fn im10() {}
2019-09-18T19:49:20.3545794Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-18T19:49:20.3545843Z 415    |
2019-09-18T19:49:20.3545843Z 415    |
2019-09-18T19:49:20.3546251Z -    = note: for more information, see ***/issues/0
2019-09-18T19:49:20.3546296Z 417    = help: add `#![feature(abi_swift)]` to the crate attributes to enable
2019-09-18T19:49:20.3546381Z 419 error[E0658]: intrinsics are subject to change
2019-09-18T19:49:20.3546404Z 
2019-09-18T19:49:20.3546696Z -   --> $DIR/feature-gate-abi.rs:80:11
2019-09-18T19:49:20.3546902Z +   --> $DIR/feature-gate-abi.rs:85:11
2019-09-18T19:49:20.3546902Z +   --> $DIR/feature-gate-abi.rs:85:11
2019-09-18T19:49:20.3546938Z 421    |
---
2019-09-18T19:49:20.3581581Z 644 
2019-09-18T19:49:20.3581755Z 
2019-09-18T19:49:20.3582127Z 
2019-09-18T19:49:20.3582185Z The actual stderr differed from the expected stderr.
2019-09-18T19:49:20.3582574Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi/feature-gate-abi.stderr
2019-09-18T19:49:20.3582839Z To update references, rerun the tests and pass the `--bless` flag
2019-09-18T19:49:20.3583108Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-abi.rs`
2019-09-18T19:49:20.3583205Z error: 1 errors occurred comparing output.
2019-09-18T19:49:20.3583249Z status: exit code: 1
2019-09-18T19:49:20.3583249Z status: exit code: 1
2019-09-18T19:49:20.3584025Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-abi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi/auxiliary" "-A" "unused"
2019-09-18T19:49:20.3584467Z ------------------------------------------
2019-09-18T19:49:20.3584502Z 
2019-09-18T19:49:20.3584724Z ------------------------------------------
2019-09-18T19:49:20.3584829Z stderr:
2019-09-18T19:49:20.3584829Z stderr:
2019-09-18T19:49:20.3585070Z ------------------------------------------
2019-09-18T19:49:20.3585122Z error[E0407]: method `m10` is not a member of trait `Tr`
2019-09-18T19:49:20.3585606Z    |
2019-09-18T19:49:20.3585606Z    |
2019-09-18T19:49:20.3585645Z LL |     extern "Swift" fn m10() {} //~ ERROR Swift ABI is experimental and subject to change
2019-09-18T19:49:20.3585848Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ not a member of trait `Tr`
2019-09-18T19:49:20.3585927Z error[E0658]: intrinsics are subject to change
2019-09-18T19:49:20.3586117Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:13:1
2019-09-18T19:49:20.3586169Z    |
2019-09-18T19:49:20.3586169Z    |
2019-09-18T19:49:20.3586360Z LL | extern "rust-intrinsic" fn f1() {} //~ ERROR intrinsics are subject to change
2019-09-18T19:49:20.3586437Z    |
2019-09-18T19:49:20.3586491Z    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-09-18T19:49:20.3586515Z 
2019-09-18T19:49:20.3586549Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T19:49:20.3586549Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T19:49:20.3586758Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:15:1
2019-09-18T19:49:20.3586795Z    |
2019-09-18T19:49:20.3586992Z LL | extern "platform-intrinsic" fn f2() {} //~ ERROR platform intrinsics are experimental
2019-09-18T19:49:20.3587080Z    |
2019-09-18T19:49:20.3587080Z    |
2019-09-18T19:49:20.3587315Z    = note: for more information, see ***/issues/27731
2019-09-18T19:49:20.3587378Z    = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
2019-09-18T19:49:20.3587438Z error[E0658]: vectorcall is experimental and subject to change
2019-09-18T19:49:20.3587643Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:17:1
2019-09-18T19:49:20.3587696Z    |
2019-09-18T19:49:20.3587696Z    |
2019-09-18T19:49:20.3587733Z LL | extern "vectorcall" fn f3() {} //~ ERROR vectorcall is experimental and subject to change
2019-09-18T19:49:20.3587818Z    |
2019-09-18T19:49:20.3587853Z    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-18T19:49:20.3587878Z 
2019-09-18T19:49:20.3588064Z error[E0658]: rust-call ABI is subject to change
2019-09-18T19:49:20.3588064Z error[E0658]: rust-call ABI is subject to change
2019-09-18T19:49:20.3588250Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:18:1
2019-09-18T19:49:20.3588285Z    |
2019-09-18T19:49:20.3588551Z LL | extern "rust-call" fn f4() {} //~ ERROR rust-call ABI is subject to change
2019-09-18T19:49:20.3588652Z    |
2019-09-18T19:49:20.3588652Z    |
2019-09-18T19:49:20.3588906Z    = note: for more information, see ***/issues/29625
2019-09-18T19:49:20.3588959Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-09-18T19:49:20.3588983Z 
2019-09-18T19:49:20.3589181Z error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-09-18T19:49:20.3589419Z    |
2019-09-18T19:49:20.3589419Z    |
2019-09-18T19:49:20.3589613Z LL | extern "msp430-interrupt" fn f5() {} //~ ERROR msp430-interrupt ABI is experimental
2019-09-18T19:49:20.3589699Z    |
2019-09-18T19:49:20.3589699Z    |
2019-09-18T19:49:20.3589912Z    = note: for more information, see ***/issues/38487
2019-09-18T19:49:20.3589977Z    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-09-18T19:49:20.3590002Z 
2019-09-18T19:49:20.3590036Z error[E0658]: PTX ABIs are experimental and subject to change
2019-09-18T19:49:20.3590288Z    |
2019-09-18T19:49:20.3590288Z    |
2019-09-18T19:49:20.3590587Z LL | extern "ptx-kernel" fn f6() {} //~ ERROR PTX ABIs are experimental and subject to change
2019-09-18T19:49:20.3590678Z    |
2019-09-18T19:49:20.3590678Z    |
2019-09-18T19:49:20.3590893Z    = note: for more information, see ***/issues/38788
2019-09-18T19:49:20.3590951Z    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-09-18T19:49:20.3590976Z 
2019-09-18T19:49:20.3591171Z error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-09-18T19:49:20.3591407Z    |
2019-09-18T19:49:20.3591407Z    |
2019-09-18T19:49:20.3592059Z LL | extern "x86-interrupt" fn f7() {} //~ ERROR x86-interrupt ABI is experimental
2019-09-18T19:49:20.3592189Z    |
2019-09-18T19:49:20.3592189Z    |
2019-09-18T19:49:20.3592519Z    = note: for more information, see ***/issues/40180
2019-09-18T19:49:20.3592583Z    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-09-18T19:49:20.3592693Z error[E0658]: thiscall is experimental and subject to change
2019-09-18T19:49:20.3592983Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:22:1
2019-09-18T19:49:20.3593053Z    |
2019-09-18T19:49:20.3593053Z    |
2019-09-18T19:49:20.3593106Z LL | extern "thiscall" fn f8() {} //~ ERROR thiscall is experimental and subject to change
2019-09-18T19:49:20.3593202Z    |
2019-09-18T19:49:20.3593269Z    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-09-18T19:49:20.3593303Z 
2019-09-18T19:49:20.3593303Z 
2019-09-18T19:49:20.3593574Z error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-09-18T19:49:20.3593924Z    |
2019-09-18T19:49:20.3593924Z    |
2019-09-18T19:49:20.3594221Z LL | extern "amdgpu-kernel" fn f9() {} //~ ERROR amdgpu-kernel ABI is experimental and subject to change
2019-09-18T19:49:20.3594350Z    |
2019-09-18T19:49:20.3594350Z    |
2019-09-18T19:49:20.3594652Z    = note: for more information, see ***/issues/51575
2019-09-18T19:49:20.3594731Z    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-09-18T19:49:20.3594764Z 
2019-09-18T19:49:20.3594813Z error[E0658]: Swift ABI is experimental and subject to change
2019-09-18T19:49:20.3595317Z    |
2019-09-18T19:49:20.3595317Z    |
2019-09-18T19:49:20.3595560Z LL | extern "Swift" fn f10() {} //~ ERROR Swift ABI is experimental and subject to change
2019-09-18T19:49:20.3595813Z    |
2019-09-18T19:49:20.3595813Z    |
2019-09-18T19:49:20.3596122Z    = help: add `#![feature(abi_swift)]` to the crate attributes to enable
2019-09-18T19:49:20.3596208Z error[E0658]: intrinsics are subject to change
2019-09-18T19:49:20.3596431Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:28:5
2019-09-18T19:49:20.3596477Z    |
2019-09-18T19:49:20.3596477Z    |
2019-09-18T19:49:20.3596843Z LL |     extern "rust-intrinsic" fn m1(); //~ ERROR intrinsics are subject to change
2019-09-18T19:49:20.3596934Z    |
2019-09-18T19:49:20.3596969Z    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-09-18T19:49:20.3597010Z 
2019-09-18T19:49:20.3597044Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T19:49:20.3597044Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T19:49:20.3597230Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:30:5
2019-09-18T19:49:20.3597282Z    |
2019-09-18T19:49:20.3597486Z LL |     extern "platform-intrinsic" fn m2(); //~ ERROR platform intrinsics are experimental
2019-09-18T19:49:20.3597560Z    |
2019-09-18T19:49:20.3597560Z    |
2019-09-18T19:49:20.3597796Z    = note: for more information, see ***/issues/27731
2019-09-18T19:49:20.3597842Z    = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
2019-09-18T19:49:20.3598005Z error[E0658]: vectorcall is experimental and subject to change
2019-09-18T19:49:20.3598238Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:32:5
2019-09-18T19:49:20.3598500Z    |
2019-09-18T19:49:20.3598500Z    |
2019-09-18T19:49:20.3598555Z LL |     extern "vectorcall" fn m3(); //~ ERROR vectorcall is experimental and subject to change
2019-09-18T19:49:20.3598627Z    |
2019-09-18T19:49:20.3598679Z    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-18T19:49:20.3598704Z 
2019-09-18T19:49:20.3598882Z error[E0658]: rust-call ABI is subject to change
2019-09-18T19:49:20.3598882Z error[E0658]: rust-call ABI is subject to change
2019-09-18T19:49:20.3599078Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:33:5
2019-09-18T19:49:20.3599134Z    |
2019-09-18T19:49:20.3599330Z LL |     extern "rust-call" fn m4(); //~ ERROR rust-call ABI is subject to change
2019-09-18T19:49:20.3599427Z    |
2019-09-18T19:49:20.3599427Z    |
2019-09-18T19:49:20.3599651Z    = note: for more information, see ***/issues/29625
2019-09-18T19:49:20.3599695Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-09-18T19:49:20.3599736Z 
2019-09-18T19:49:20.3599940Z error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-09-18T19:49:20.3600183Z    |
2019-09-18T19:49:20.3600183Z    |
2019-09-18T19:49:20.3600388Z LL |     extern "msp430-interrupt" fn m5(); //~ ERROR msp430-interrupt ABI is experimental
2019-09-18T19:49:20.3600478Z    |
2019-09-18T19:49:20.3600478Z    |
2019-09-18T19:49:20.3600699Z    = note: for more information, see ***/issues/38487
2019-09-18T19:49:20.3600746Z    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-09-18T19:49:20.3600788Z 
2019-09-18T19:49:20.3600825Z error[E0658]: PTX ABIs are experimental and subject to change
2019-09-18T19:49:20.3601091Z    |
2019-09-18T19:49:20.3601091Z    |
2019-09-18T19:49:20.3601329Z LL |     extern "ptx-kernel" fn m6(); //~ ERROR PTX ABIs are experimental and subject to change
2019-09-18T19:49:20.3601406Z    |
2019-09-18T19:49:20.3601406Z    |
2019-09-18T19:49:20.3602354Z    = note: for more information, see ***/issues/38788
2019-09-18T19:49:20.3602420Z    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-09-18T19:49:20.3602454Z 
2019-09-18T19:49:20.3602948Z error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-09-18T19:49:20.3603452Z    |
2019-09-18T19:49:20.3603452Z    |
2019-09-18T19:49:20.3603759Z LL |     extern "x86-interrupt" fn m7(); //~ ERROR x86-interrupt ABI is experimental
2019-09-18T19:49:20.3604315Z    |
2019-09-18T19:49:20.3604315Z    |
2019-09-18T19:49:20.3604685Z    = note: for more information, see ***/issues/40180
2019-09-18T19:49:20.3604744Z    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-09-18T19:49:20.3604841Z error[E0658]: thiscall is experimental and subject to change
2019-09-18T19:49:20.3605103Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:37:5
2019-09-18T19:49:20.3605152Z    |
2019-09-18T19:49:20.3605152Z    |
2019-09-18T19:49:20.3605202Z LL |     extern "thiscall" fn m8(); //~ ERROR thiscall is experimental and subject to change
2019-09-18T19:49:20.3605312Z    |
2019-09-18T19:49:20.3605371Z    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-09-18T19:49:20.3605420Z 
2019-09-18T19:49:20.3605420Z 
2019-09-18T19:49:20.3605842Z error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-09-18T19:49:20.3606686Z    |
2019-09-18T19:49:20.3606686Z    |
2019-09-18T19:49:20.3607111Z LL |     extern "amdgpu-kernel" fn m9(); //~ ERROR amdgpu-kernel ABI is experimental and subject to change
2019-09-18T19:49:20.3607204Z    |
2019-09-18T19:49:20.3607204Z    |
2019-09-18T19:49:20.3607429Z    = note: for more information, see ***/issues/51575
2019-09-18T19:49:20.3607475Z    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-09-18T19:49:20.3607553Z error[E0658]: vectorcall is experimental and subject to change
2019-09-18T19:49:20.3607755Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:40:5
2019-09-18T19:49:20.3607792Z    |
2019-09-18T19:49:20.3607792Z    |
2019-09-18T19:49:20.3607855Z LL |     extern "vectorcall" fn dm3() {} //~ ERROR vectorcall is experimental and subject to change
2019-09-18T19:49:20.3607926Z    |
2019-09-18T19:49:20.3607979Z    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-18T19:49:20.3608010Z 
2019-09-18T19:49:20.3608188Z error[E0658]: rust-call ABI is subject to change
2019-09-18T19:49:20.3608188Z error[E0658]: rust-call ABI is subject to change
2019-09-18T19:49:20.3608396Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:41:5
2019-09-18T19:49:20.3608434Z    |
2019-09-18T19:49:20.3608633Z LL |     extern "rust-call" fn dm4() {} //~ ERROR rust-call ABI is subject to change
2019-09-18T19:49:20.3608723Z    |
2019-09-18T19:49:20.3608723Z    |
2019-09-18T19:49:20.3608937Z    = note: for more information, see ***/issues/29625
2019-09-18T19:49:20.3608998Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-09-18T19:49:20.3609025Z 
2019-09-18T19:49:20.3609397Z error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-09-18T19:49:20.3609812Z    |
2019-09-18T19:49:20.3609812Z    |
2019-09-18T19:49:20.3610170Z LL |     extern "msp430-interrupt" fn dm5() {} //~ ERROR msp430-interrupt ABI is experimental
2019-09-18T19:49:20.3610262Z    |
2019-09-18T19:49:20.3610262Z    |
2019-09-18T19:49:20.3610471Z    = note: for more information, see ***/issues/38487
2019-09-18T19:49:20.3610513Z    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-09-18T19:49:20.3610553Z 
2019-09-18T19:49:20.3610586Z error[E0658]: PTX ABIs are experimental and subject to change
2019-09-18T19:49:20.3611012Z    |
2019-09-18T19:49:20.3611012Z    |
2019-09-18T19:49:20.3611220Z LL |     extern "ptx-kernel" fn dm6() {} //~ ERROR PTX ABIs are experimental and subject to change
2019-09-18T19:49:20.3611560Z    |
2019-09-18T19:49:20.3611560Z    |
2019-09-18T19:49:20.3611829Z    = note: for more information, see ***/issues/38788
2019-09-18T19:49:20.3612211Z    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-09-18T19:49:20.3612259Z 
2019-09-18T19:49:20.3612576Z error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-09-18T19:49:20.3612871Z    |
2019-09-18T19:49:20.3612871Z    |
2019-09-18T19:49:20.3613147Z LL |     extern "x86-interrupt" fn dm7() {} //~ ERROR x86-interrupt ABI is experimental
2019-09-18T19:49:20.3613245Z    |
2019-09-18T19:49:20.3613245Z    |
2019-09-18T19:49:20.3613546Z    = note: for more information, see ***/issues/40180
2019-09-18T19:49:20.3613608Z    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-09-18T19:49:20.3613716Z error[E0658]: thiscall is experimental and subject to change
2019-09-18T19:49:20.3614000Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:45:5
2019-09-18T19:49:20.3614051Z    |
2019-09-18T19:49:20.3614051Z    |
2019-09-18T19:49:20.3614122Z LL |     extern "thiscall" fn dm8() {} //~ ERROR thiscall is experimental and subject to change
2019-09-18T19:49:20.3614347Z    |
2019-09-18T19:49:20.3614415Z    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-09-18T19:49:20.3614450Z 
2019-09-18T19:49:20.3614450Z 
2019-09-18T19:49:20.3614747Z error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-09-18T19:49:20.3615086Z    |
2019-09-18T19:49:20.3615086Z    |
2019-09-18T19:49:20.3615547Z LL |     extern "amdgpu-kernel" fn dm9() {} //~ ERROR amdgpu-kernel ABI is experimental and subject to change
2019-09-18T19:49:20.3615876Z    |
2019-09-18T19:49:20.3615876Z    |
2019-09-18T19:49:20.3616133Z    = note: for more information, see ***/issues/51575
2019-09-18T19:49:20.3616185Z    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-09-18T19:49:20.3616229Z 
2019-09-18T19:49:20.3616269Z error[E0658]: Swift ABI is experimental and subject to change
2019-09-18T19:49:20.3616574Z    |
2019-09-18T19:49:20.3616574Z    |
2019-09-18T19:49:20.3616620Z LL |     extern "Swift" fn dm10() {} //~ ERROR Swift ABI is experimental and subject to change
2019-09-18T19:49:20.3616700Z    |
2019-09-18T19:49:20.3616700Z    |
2019-09-18T19:49:20.3616763Z    = help: add `#![feature(abi_swift)]` to the crate attributes to enable
2019-09-18T19:49:20.3616830Z error[E0658]: intrinsics are subject to change
2019-09-18T19:49:20.3617224Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:54:5
2019-09-18T19:49:20.3617265Z    |
2019-09-18T19:49:20.3617265Z    |
2019-09-18T19:49:20.3617489Z LL |     extern "rust-intrinsic" fn m1() {} //~ ERROR intrinsics are subject to change
2019-09-18T19:49:20.3617586Z    |
2019-09-18T19:49:20.3617625Z    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-09-18T19:49:20.3617658Z 
2019-09-18T19:49:20.3617711Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T19:49:20.3617711Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T19:49:20.3617918Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:56:5
2019-09-18T19:49:20.3618129Z    |
2019-09-18T19:49:20.3618348Z LL |     extern "platform-intrinsic" fn m2() {} //~ ERROR platform intrinsics are experimental
2019-09-18T19:49:20.3618424Z    |
2019-09-18T19:49:20.3618424Z    |
2019-09-18T19:49:20.3618872Z    = note: for more information, see ***/issues/27731
2019-09-18T19:49:20.3618919Z    = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
2019-09-18T19:49:20.3619057Z error[E0658]: vectorcall is experimental and subject to change
2019-09-18T19:49:20.3619311Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:58:5
2019-09-18T19:49:20.3619349Z    |
2019-09-18T19:49:20.3619349Z    |
2019-09-18T19:49:20.3619388Z LL |     extern "vectorcall" fn m3() {} //~ ERROR vectorcall is experimental and subject to change
2019-09-18T19:49:20.3619484Z    |
2019-09-18T19:49:20.3619522Z    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-18T19:49:20.3619741Z 
2019-09-18T19:49:20.3620100Z error[E0658]: rust-call ABI is subject to change
2019-09-18T19:49:20.3620100Z error[E0658]: rust-call ABI is subject to change
2019-09-18T19:49:20.3620297Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:59:5
2019-09-18T19:49:20.3620349Z    |
2019-09-18T19:49:20.3620553Z LL |     extern "rust-call" fn m4() {} //~ ERROR rust-call ABI is subject to change
2019-09-18T19:49:20.3620630Z    |
2019-09-18T19:49:20.3620630Z    |
2019-09-18T19:49:20.3620880Z    = note: for more information, see ***/issues/29625
2019-09-18T19:49:20.3620930Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-09-18T19:49:20.3620956Z 
2019-09-18T19:49:20.3621198Z error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-09-18T19:49:20.3622409Z    |
2019-09-18T19:49:20.3622409Z    |
2019-09-18T19:49:20.3622710Z LL |     extern "msp430-interrupt" fn m5() {} //~ ERROR msp430-interrupt ABI is experimental
2019-09-18T19:49:20.3622811Z    |
2019-09-18T19:49:20.3622811Z    |
2019-09-18T19:49:20.3623122Z    = note: for more information, see ***/issues/38487
2019-09-18T19:49:20.3623217Z    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-09-18T19:49:20.3623255Z 
2019-09-18T19:49:20.3623318Z error[E0658]: PTX ABIs are experimental and subject to change
2019-09-18T19:49:20.3623675Z    |
2019-09-18T19:49:20.3623675Z    |
2019-09-18T19:49:20.3623982Z LL |     extern "ptx-kernel" fn m6() {} //~ ERROR PTX ABIs are experimental and subject to change
2019-09-18T19:49:20.3624092Z    |
2019-09-18T19:49:20.3624092Z    |
2019-09-18T19:49:20.3624411Z    = note: for more information, see ***/issues/38788
2019-09-18T19:49:20.3624472Z    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-09-18T19:49:20.3624506Z 
2019-09-18T19:49:20.3624783Z error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-09-18T19:49:20.3625117Z    |
2019-09-18T19:49:20.3625117Z    |
2019-09-18T19:49:20.3625566Z LL |     extern "x86-interrupt" fn m7() {} //~ ERROR x86-interrupt ABI is experimental
2019-09-18T19:49:20.3625656Z    |
2019-09-18T19:49:20.3625656Z    |
2019-09-18T19:49:20.3625881Z    = note: for more information, see ***/issues/40180
2019-09-18T19:49:20.3625947Z    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-09-18T19:49:20.3626009Z error[E0658]: thiscall is experimental and subject to change
2019-09-18T19:49:20.3626245Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:63:5
2019-09-18T19:49:20.3626284Z    |
2019-09-18T19:49:20.3626284Z    |
2019-09-18T19:49:20.3626323Z LL |     extern "thiscall" fn m8() {} //~ ERROR thiscall is experimental and subject to change
2019-09-18T19:49:20.3626412Z    |
2019-09-18T19:49:20.3626450Z    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-09-18T19:49:20.3626476Z 
2019-09-18T19:49:20.3626476Z 
2019-09-18T19:49:20.3626690Z error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-09-18T19:49:20.3626931Z    |
2019-09-18T19:49:20.3626931Z    |
2019-09-18T19:49:20.3627276Z LL |     extern "amdgpu-kernel" fn m9() {} //~ ERROR amdgpu-kernel ABI is experimental and subject to change
2019-09-18T19:49:20.3627365Z    |
2019-09-18T19:49:20.3627365Z    |
2019-09-18T19:49:20.3627639Z    = note: for more information, see ***/issues/51575
2019-09-18T19:49:20.3627694Z    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-09-18T19:49:20.3627720Z 
2019-09-18T19:49:20.3627771Z error[E0658]: Swift ABI is experimental and subject to change
2019-09-18T19:49:20.3628023Z    |
2019-09-18T19:49:20.3628023Z    |
2019-09-18T19:49:20.3628062Z LL |     extern "Swift" fn m10() {} //~ ERROR Swift ABI is experimental and subject to change
2019-09-18T19:49:20.3628149Z    |
2019-09-18T19:49:20.3628149Z    |
2019-09-18T19:49:20.3628187Z    = help: add `#![feature(abi_swift)]` to the crate attributes to enable
2019-09-18T19:49:20.3628269Z error[E0658]: intrinsics are subject to change
2019-09-18T19:49:20.3628472Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:70:5
2019-09-18T19:49:20.3628526Z    |
2019-09-18T19:49:20.3628526Z    |
2019-09-18T19:49:20.3628737Z LL |     extern "rust-intrinsic" fn im1() {} //~ ERROR intrinsics are subject to change
2019-09-18T19:49:20.3628895Z    |
2019-09-18T19:49:20.3628957Z    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-09-18T19:49:20.3628982Z 
2019-09-18T19:49:20.3629018Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T19:49:20.3629018Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T19:49:20.3629252Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:72:5
2019-09-18T19:49:20.3629294Z    |
2019-09-18T19:49:20.3629511Z LL |     extern "platform-intrinsic" fn im2() {} //~ ERROR platform intrinsics are experimental
2019-09-18T19:49:20.3629605Z    |
2019-09-18T19:49:20.3629605Z    |
2019-09-18T19:49:20.3629842Z    = note: for more information, see ***/issues/27731
2019-09-18T19:49:20.3629904Z    = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
2019-09-18T19:49:20.3629966Z error[E0658]: vectorcall is experimental and subject to change
2019-09-18T19:49:20.3630182Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:74:5
2019-09-18T19:49:20.3630238Z    |
2019-09-18T19:49:20.3630238Z    |
2019-09-18T19:49:20.3630277Z LL |     extern "vectorcall" fn im3() {} //~ ERROR vectorcall is experimental and subject to change
2019-09-18T19:49:20.3630366Z    |
2019-09-18T19:49:20.3630404Z    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-18T19:49:20.3630429Z 
2019-09-18T19:49:20.3630615Z error[E0658]: rust-call ABI is subject to change
2019-09-18T19:49:20.3630615Z error[E0658]: rust-call ABI is subject to change
2019-09-18T19:49:20.3630832Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:75:5
2019-09-18T19:49:20.3630870Z    |
2019-09-18T19:49:20.3631080Z LL |     extern "rust-call" fn im4() {} //~ ERROR rust-call ABI is subject to change
2019-09-18T19:49:20.3631173Z    |
2019-09-18T19:49:20.3631173Z    |
2019-09-18T19:49:20.3631398Z    = note: for more information, see ***/issues/29625
2019-09-18T19:49:20.3631468Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-09-18T19:49:20.3631493Z 
2019-09-18T19:49:20.3632346Z error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-09-18T19:49:20.3632711Z    |
2019-09-18T19:49:20.3632711Z    |
2019-09-18T19:49:20.3632982Z LL |     extern "msp430-interrupt" fn im5() {} //~ ERROR msp430-interrupt ABI is experimental
2019-09-18T19:49:20.3633100Z    |
2019-09-18T19:49:20.3633100Z    |
2019-09-18T19:49:20.3633392Z    = note: for more information, see ***/issues/38487
2019-09-18T19:49:20.3633582Z    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-09-18T19:49:20.3633627Z 
2019-09-18T19:49:20.3633675Z error[E0658]: PTX ABIs are experimental and subject to change
2019-09-18T19:49:20.3634036Z    |
2019-09-18T19:49:20.3634036Z    |
2019-09-18T19:49:20.3634308Z LL |     extern "ptx-kernel" fn im6() {} //~ ERROR PTX ABIs are experimental and subject to change
2019-09-18T19:49:20.3634423Z    |
2019-09-18T19:49:20.3634423Z    |
2019-09-18T19:49:20.3634705Z    = note: for more information, see ***/issues/38788
2019-09-18T19:49:20.3634780Z    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-09-18T19:49:20.3634813Z 
2019-09-18T19:49:20.3635072Z error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-09-18T19:49:20.3635387Z    |
2019-09-18T19:49:20.3635387Z    |
2019-09-18T19:49:20.3635940Z LL |     extern "x86-interrupt" fn im7() {} //~ ERROR x86-interrupt ABI is experimental
2019-09-18T19:49:20.3636033Z    |
2019-09-18T19:49:20.3636033Z    |
2019-09-18T19:49:20.3636246Z    = note: for more information, see ***/issues/40180
2019-09-18T19:49:20.3636370Z    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-09-18T19:49:20.3636447Z error[E0658]: thiscall is experimental and subject to change
2019-09-18T19:49:20.3636662Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:79:5
2019-09-18T19:49:20.3636716Z    |
2019-09-18T19:49:20.3636716Z    |
2019-09-18T19:49:20.3636754Z LL |     extern "thiscall" fn im8() {} //~ ERROR thiscall is experimental and subject to change
2019-09-18T19:49:20.3636823Z    |
2019-09-18T19:49:20.3636876Z    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-09-18T19:49:20.3636901Z 
2019-09-18T19:49:20.3636901Z 
2019-09-18T19:49:20.3637110Z error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-09-18T19:49:20.3637367Z    |
2019-09-18T19:49:20.3637367Z    |
2019-09-18T19:49:20.3637591Z LL |     extern "amdgpu-kernel" fn im9() {} //~ ERROR amdgpu-kernel ABI is experimental and subject to change
2019-09-18T19:49:20.3637693Z    |
2019-09-18T19:49:20.3637693Z    |
2019-09-18T19:49:20.3637921Z    = note: for more information, see ***/issues/51575
2019-09-18T19:49:20.3637983Z    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-09-18T19:49:20.3638009Z 
2019-09-18T19:49:20.3638044Z error[E0658]: Swift ABI is experimental and subject to change
2019-09-18T19:49:20.3638311Z    |
2019-09-18T19:49:20.3638311Z    |
2019-09-18T19:49:20.3638358Z LL |     extern "Swift" fn im10() {} //~ ERROR Swift ABI is experimental and subject to change
2019-09-18T19:49:20.3638622Z    |
2019-09-18T19:49:20.3638622Z    |
2019-09-18T19:49:20.3638659Z    = help: add `#![feature(abi_swift)]` to the crate attributes to enable
2019-09-18T19:49:20.3638739Z error[E0658]: intrinsics are subject to change
2019-09-18T19:49:20.3638937Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:85:11
2019-09-18T19:49:20.3638976Z    |
2019-09-18T19:49:20.3638976Z    |
2019-09-18T19:49:20.3639176Z LL | type A1 = extern "rust-intrinsic" fn(); //~ ERROR intrinsics are subject to change
2019-09-18T19:49:20.3639269Z    |
2019-09-18T19:49:20.3639305Z    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-09-18T19:49:20.3639347Z 
2019-09-18T19:49:20.3639383Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T19:49:20.3639383Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T19:49:20.3639578Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:86:11
2019-09-18T19:49:20.3639703Z    |
2019-09-18T19:49:20.3639939Z LL | type A2 = extern "platform-intrinsic" fn(); //~ ERROR platform intrinsics are experimental
2019-09-18T19:49:20.3640015Z    |
2019-09-18T19:49:20.3640015Z    |
2019-09-18T19:49:20.3640263Z    = note: for more information, see ***/issues/27731
2019-09-18T19:49:20.3640311Z    = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
2019-09-18T19:49:20.3640391Z error[E0658]: vectorcall is experimental and subject to change
2019-09-18T19:49:20.3640611Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:87:11
2019-09-18T19:49:20.3640651Z    |
2019-09-18T19:49:20.3640651Z    |
2019-09-18T19:49:20.3640710Z LL | type A3 = extern "vectorcall" fn(); //~ ERROR vectorcall is experimental and subject to change
2019-09-18T19:49:20.3640786Z    |
2019-09-18T19:49:20.3640848Z    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-18T19:49:20.3640874Z 
2019-09-18T19:49:20.3641215Z error[E0658]: rust-call ABI is subject to change
2019-09-18T19:49:20.3641215Z error[E0658]: rust-call ABI is subject to change
2019-09-18T19:49:20.3641401Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:88:11
2019-09-18T19:49:20.3641454Z    |
2019-09-18T19:49:20.3642406Z LL | type A4 = extern "rust-call" fn(); //~ ERROR rust-call ABI is subject to change
2019-09-18T19:49:20.3642531Z    |
2019-09-18T19:49:20.3642531Z    |
2019-09-18T19:49:20.3642830Z    = note: for more information, see ***/issues/29625
2019-09-18T19:49:20.3642889Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-09-18T19:49:20.3642942Z 
2019-09-18T19:49:20.3643205Z error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-09-18T19:49:20.3643521Z    |
2019-09-18T19:49:20.3643521Z    |
2019-09-18T19:49:20.3643799Z LL | type A5 = extern "msp430-interrupt" fn(); //~ ERROR msp430-interrupt ABI is experimental
2019-09-18T19:49:20.3644071Z    |
2019-09-18T19:49:20.3644071Z    |
2019-09-18T19:49:20.3644390Z    = note: for more information, see ***/issues/38487
2019-09-18T19:49:20.3644452Z    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-09-18T19:49:20.3644517Z 
2019-09-18T19:49:20.3644565Z error[E0658]: PTX ABIs are experimental and subject to change
2019-09-18T19:49:20.3644916Z    |
2019-09-18T19:49:20.3644916Z    |
2019-09-18T19:49:20.3645210Z LL | type A6 = extern "ptx-kernel" fn (); //~ ERROR PTX ABIs are experimental and subject to change
2019-09-18T19:49:20.3645492Z    |
2019-09-18T19:49:20.3645492Z    |
2019-09-18T19:49:20.3645907Z    = note: for more information, see ***/issues/38788
2019-09-18T19:49:20.3645953Z    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-09-18T19:49:20.3645978Z 
2019-09-18T19:49:20.3646215Z error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-09-18T19:49:20.3646465Z    |
2019-09-18T19:49:20.3646465Z    |
2019-09-18T19:49:20.3646700Z LL | type A7 = extern "x86-interrupt" fn(); //~ ERROR x86-interrupt ABI is experimental
2019-09-18T19:49:20.3646785Z    |
2019-09-18T19:49:20.3646785Z    |
2019-09-18T19:49:20.3647031Z    = note: for more information, see ***/issues/40180
2019-09-18T19:49:20.3647078Z    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-09-18T19:49:20.3647157Z error[E0658]: thiscall is experimental and subject to change
2019-09-18T19:49:20.3647515Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:92:11
2019-09-18T19:49:20.3647550Z    |
2019-09-18T19:49:20.3647550Z    |
2019-09-18T19:49:20.3647603Z LL | type A8 = extern "thiscall" fn(); //~ ERROR thiscall is experimental and subject to change
2019-09-18T19:49:20.3647783Z    |
2019-09-18T19:49:20.3647818Z    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-09-18T19:49:20.3647860Z 
2019-09-18T19:49:20.3647860Z 
2019-09-18T19:49:20.3648085Z error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-09-18T19:49:20.3648334Z    |
2019-09-18T19:49:20.3648334Z    |
---
2019-09-18T19:49:20.3668373Z 
2019-09-18T19:49:20.3668536Z ---- [ui] ui/parser/issue-8537.rs stdout ----
2019-09-18T19:49:20.3668571Z diff of stderr:
2019-09-18T19:49:20.3668591Z 
2019-09-18T19:49:20.3668758Z 4 LL |   "invalid-ab_isize"
2019-09-18T19:49:20.3668794Z 5    |   ^^^^^^^^^^^^^^^^^^ invalid ABI
2019-09-18T19:49:20.3668825Z 6    |
2019-09-18T19:49:20.3669137Z -    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, Rust, C, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
2019-09-18T19:49:20.3669523Z +    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, Rust, C, Swift, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
2019-09-18T19:49:20.3669636Z 9 error: aborting due to previous error
2019-09-18T19:49:20.3669667Z 10 
2019-09-18T19:49:20.3669687Z 
2019-09-18T19:49:20.3669706Z 
2019-09-18T19:49:20.3669706Z 
2019-09-18T19:49:20.3669755Z The actual stderr differed from the expected stderr.
2019-09-18T19:49:20.3669993Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-8537/issue-8537.stderr
2019-09-18T19:49:20.3670176Z To update references, rerun the tests and pass the `--bless` flag
2019-09-18T19:49:20.3670383Z To only update this specific test, also pass `--test-args parser/issue-8537.rs`
2019-09-18T19:49:20.3670442Z error: 1 errors occurred comparing output.
2019-09-18T19:49:20.3670490Z status: exit code: 1
2019-09-18T19:49:20.3670490Z status: exit code: 1
2019-09-18T19:49:20.3671013Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-8537.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-8537" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-8537/auxiliary" "-A" "unused"
2019-09-18T19:49:20.3671339Z ------------------------------------------
2019-09-18T19:49:20.3671365Z 
2019-09-18T19:49:20.3671524Z ------------------------------------------
2019-09-18T19:49:20.3671575Z stderr:
2019-09-18T19:49:20.3671575Z stderr:
2019-09-18T19:49:20.3671732Z ------------------------------------------
2019-09-18T19:49:20.3671901Z error[E0703]: invalid ABI: found `invalid-ab_isize`
2019-09-18T19:49:20.3672095Z   --> /checkout/src/test/ui/parser/issue-8537.rs:2:3
2019-09-18T19:49:20.3672130Z    |
2019-09-18T19:49:20.3672292Z LL |   "invalid-ab_isize" //~ ERROR invalid ABI
2019-09-18T19:49:20.3672346Z    |   ^^^^^^^^^^^^^^^^^^ invalid ABI
2019-09-18T19:49:20.3672378Z    |
2019-09-18T19:49:20.3672678Z    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, Rust, C, Swift, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
2019-09-18T19:49:20.3672765Z error: aborting due to previous error
2019-09-18T19:49:20.3672785Z 
2019-09-18T19:49:20.3672804Z 
2019-09-18T19:49:20.3672979Z ------------------------------------------
2019-09-18T19:49:20.3672979Z ------------------------------------------
2019-09-18T19:49:20.3673002Z 
2019-09-18T19:49:20.3673021Z 
2019-09-18T19:49:20.3673179Z ---- [ui] ui/reify-intrinsic.rs stdout ----
2019-09-18T19:49:20.3673231Z diff of stderr:
2019-09-18T19:49:20.3673251Z 
2019-09-18T19:49:20.3673298Z 8    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-09-18T19:49:20.3673335Z 9    |
2019-09-18T19:49:20.3673541Z 10    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-09-18T19:49:20.3673749Z -               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}`
2019-09-18T19:49:20.3673959Z +               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-09-18T19:49:20.3674014Z 12 
2019-09-18T19:49:20.3674263Z - error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-18T19:49:20.3674516Z + error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-18T19:49:20.3674704Z 14   --> $DIR/reify-intrinsic.rs:11:13
2019-09-18T19:49:20.3674803Z 15    |
2019-09-18T19:49:20.3675021Z 16 LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-09-18T19:49:20.3675084Z 
2019-09-18T19:49:20.3675117Z The actual stderr differed from the expected stderr.
2019-09-18T19:49:20.3675338Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-09-18T19:49:20.3675338Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-09-18T19:49:20.3675538Z To update references, rerun the tests and pass the `--bless` flag
2019-09-18T19:49:20.3675723Z To only update this specific test, also pass `--test-args reify-intrinsic.rs`
2019-09-18T19:49:20.3675796Z error: 1 errors occurred comparing output.
2019-09-18T19:49:20.3675828Z status: exit code: 1
2019-09-18T19:49:20.3675828Z status: exit code: 1
2019-09-18T19:49:20.3676529Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/reify-intrinsic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/auxiliary" "-A" "unused"
2019-09-18T19:49:20.3676873Z ------------------------------------------
2019-09-18T19:49:20.3676916Z 
2019-09-18T19:49:20.3677088Z ------------------------------------------
2019-09-18T19:49:20.3677123Z stderr:
2019-09-18T19:49:20.3677123Z stderr:
2019-09-18T19:49:20.3677287Z ------------------------------------------
2019-09-18T19:49:20.3677345Z error[E0308]: cannot coerce intrinsics to function pointers
2019-09-18T19:49:20.3677525Z   --> /checkout/src/test/ui/reify-intrinsic.rs:6:64
2019-09-18T19:49:20.3677564Z    |
2019-09-18T19:49:20.3677780Z LL |     let _: unsafe extern "rust-intrinsic" fn(isize) -> usize = std::mem::transmute;
2019-09-18T19:49:20.3677892Z    |                                                                |
2019-09-18T19:49:20.3677935Z    |                                                                cannot coerce intrinsics to function pointers
2019-09-18T19:49:20.3677991Z    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-09-18T19:49:20.3678046Z    |
2019-09-18T19:49:20.3678046Z    |
2019-09-18T19:49:20.3678246Z    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-09-18T19:49:20.3678460Z               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-09-18T19:49:20.3678505Z 
2019-09-18T19:49:20.3678760Z error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-18T19:49:20.3679009Z    |
2019-09-18T19:49:20.3679009Z    |
2019-09-18T19:49:20.3679615Z LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-09-18T19:49:20.3679725Z 
2019-09-18T19:49:20.3679787Z error: aborting due to 2 previous errors
2019-09-18T19:49:20.3679816Z 
2019-09-18T19:49:20.3679861Z Some errors have detailed explanations: E0308, E0606.
---
2019-09-18T19:49:20.3682578Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-18T19:49:20.3682637Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-18T19:49:20.3682687Z 
2019-09-18T19:49:20.3682712Z 
2019-09-18T19:49:20.3684344Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-18T19:49:20.3684998Z 
2019-09-18T19:49:20.3685022Z 
2019-09-18T19:49:20.3685063Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-18T19:49:20.3685118Z Build completed unsuccessfully in 1:05:38
2019-09-18T19:49:20.3685118Z Build completed unsuccessfully in 1:05:38
2019-09-18T19:49:20.3685153Z == clock drift check ==
2019-09-18T19:49:20.3685231Z   local time: Wed Sep 18 19:49:20 UTC 2019
2019-09-18T19:49:20.5066122Z   network time: Wed, 18 Sep 2019 19:49:20 GMT
2019-09-18T19:49:20.5069616Z == end clock drift check ==
2019-09-18T19:49:21.3298034Z ##[error]Bash exited with code '1'.
2019-09-18T19:49:21.3344829Z ##[section]Starting: Checkout
2019-09-18T19:49:21.3347219Z ==============================================================================
2019-09-18T19:49:21.3347280Z Task         : Get sources
2019-09-18T19:49:21.3347354Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
