plain
2019-09-18T21:41:32.1689247Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-18T21:41:32.1708359Z ##[command]git config gc.auto 0
2019-09-18T21:41:32.1712735Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-18T21:41:32.1716052Z ##[command]git config --get-all http.proxy
2019-09-18T21:41:32.1719761Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64582/merge:refs/remotes/pull/64582/merge
---
2019-09-18T22:43:59.5271559Z .................................................................................................... 1500/9024
2019-09-18T22:44:05.6700770Z .................................................................................................... 1600/9024
2019-09-18T22:44:18.3561535Z .................................................................i...............i.................. 1700/9024
2019-09-18T22:44:25.5948723Z .................................................................................................... 1800/9024
2019-09-18T22:44:40.9217541Z ........................................................iiiii....................................... 1900/9024
2019-09-18T22:44:52.4401733Z .................................................................................................... 2100/9024
2019-09-18T22:44:54.9697740Z .................................................................................................... 2200/9024
2019-09-18T22:44:58.3018765Z .................................................................................................... 2300/9024
2019-09-18T22:45:06.7978213Z .................................................................................................... 2400/9024
---
2019-09-18T22:48:05.5764052Z ............................................i...............i....................................... 4700/9024
2019-09-18T22:48:16.1071092Z .................................................................................................... 4800/9024
2019-09-18T22:48:23.0456165Z .................................................................................................... 4900/9024
2019-09-18T22:48:32.6177334Z .................................................................................................... 5000/9024
2019-09-18T22:48:40.4565657Z ............................ii.ii................................................................... 5100/9024
2019-09-18T22:48:50.4164374Z .................................................................................................... 5300/9024
2019-09-18T22:49:00.7762217Z ............................................................................................i....... 5400/9024
2019-09-18T22:49:09.0708052Z .................................................................................................... 5500/9024
2019-09-18T22:49:13.9768170Z .................................................................................................... 5600/9024
2019-09-18T22:49:13.9768170Z .................................................................................................... 5600/9024
2019-09-18T22:49:24.5654756Z .......................................................................................ii...i..ii... 5700/9024
2019-09-18T22:49:49.6267983Z .................................................................................................... 5900/9024
2019-09-18T22:49:59.7367689Z .................................................................................................... 6000/9024
2019-09-18T22:49:59.7367689Z .................................................................................................... 6000/9024
2019-09-18T22:50:06.7425623Z .........................................................................................i..ii...... 6100/9024
2019-09-18T22:50:34.9822639Z .................................................................................................... 6300/9024
2019-09-18T22:50:39.3616451Z ................................................i................................................... 6400/9024
2019-09-18T22:50:41.5761080Z ...............................................F.................................................... 6500/9024
2019-09-18T22:50:44.0860650Z ....................i............................................................................... 6600/9024
---
2019-09-18T22:54:47.5704277Z 
2019-09-18T22:54:47.5705027Z ---- [ui] ui/codemap_tests/unicode.rs stdout ----
2019-09-18T22:54:47.5705312Z diff of stderr:
2019-09-18T22:54:47.5705481Z 
2019-09-18T22:54:47.5705929Z 4 LL | extern "路濫狼á́́" fn foo() {}
2019-09-18T22:54:47.5706164Z 5    |        ^^^^^^^^^ invalid ABI
2019-09-18T22:54:47.5706342Z 6    |
2019-09-18T22:54:47.5706890Z -    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, Rust, C, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
2019-09-18T22:54:47.5707960Z +    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, Rust, C, Swift, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
2019-09-18T22:54:47.5708426Z 9 error: aborting due to previous error
2019-09-18T22:54:47.5708633Z 10 
2019-09-18T22:54:47.5708816Z 
2019-09-18T22:54:47.5708969Z 
2019-09-18T22:54:47.5708969Z 
2019-09-18T22:54:47.5709144Z The actual stderr differed from the expected stderr.
2019-09-18T22:54:47.5710298Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode/unicode.stderr
2019-09-18T22:54:47.5710921Z To update references, rerun the tests and pass the `--bless` flag
2019-09-18T22:54:47.5711486Z To only update this specific test, also pass `--test-args codemap_tests/unicode.rs`
2019-09-18T22:54:47.5711999Z error: 1 errors occurred comparing output.
2019-09-18T22:54:47.5712231Z status: exit code: 1
2019-09-18T22:54:47.5712231Z status: exit code: 1
2019-09-18T22:54:47.5713745Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/codemap_tests/unicode.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode/auxiliary" "-A" "unused"
2019-09-18T22:54:47.5714512Z ------------------------------------------
2019-09-18T22:54:47.5714738Z 
2019-09-18T22:54:47.5715352Z ------------------------------------------
2019-09-18T22:54:47.5715608Z stderr:
2019-09-18T22:54:47.5715608Z stderr:
2019-09-18T22:54:47.5715996Z ------------------------------------------
2019-09-18T22:54:47.5716458Z error[E0703]: invalid ABI: found `路濫狼á́́`
2019-09-18T22:54:47.5717180Z    |
2019-09-18T22:54:47.5717180Z    |
2019-09-18T22:54:47.5717801Z LL | extern "路濫狼á́́" fn foo() {} //~ ERROR invalid ABI
2019-09-18T22:54:47.5718209Z    |        ^^^^^^^^^ invalid ABI
2019-09-18T22:54:47.5718398Z    |
2019-09-18T22:54:47.5719137Z    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, Rust, C, Swift, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
2019-09-18T22:54:47.5719644Z error: aborting due to previous error
2019-09-18T22:54:47.5719810Z 
2019-09-18T22:54:47.5720451Z 
2019-09-18T22:54:47.5721036Z ------------------------------------------
---
2019-09-18T22:54:47.5722923Z 516 
2019-09-18T22:54:47.5723157Z 517 error[E0658]: vectorcall is experimental and subject to change
2019-09-18T22:54:47.5723916Z -    --> $DIR/feature-gate-abi.rs:100:1
2019-09-18T22:54:47.5724333Z -     |
2019-09-18T22:54:47.5725280Z - LLL | extern "vectorcall" {}
2019-09-18T22:54:47.5726335Z -     |
2019-09-18T22:54:47.5726736Z -     = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-18T22:54:47.5727084Z +   --> $DIR/feature-gate-abi.rs:100:1
2019-09-18T22:54:47.5727254Z +    |
2019-09-18T22:54:47.5727254Z +    |
2019-09-18T22:54:47.5727370Z + LL | extern "vectorcall" {}
2019-09-18T22:54:47.5727621Z +    |
2019-09-18T22:54:47.5727736Z +    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-18T22:54:47.5727971Z 524 
2019-09-18T22:54:47.5728977Z 525 error[E0658]: rust-call ABI is subject to change
2019-09-18T22:54:47.5728977Z 525 error[E0658]: rust-call ABI is subject to change
2019-09-18T22:54:47.5731429Z -    --> $DIR/feature-gate-abi.rs:101:1
2019-09-18T22:54:47.5732163Z -     |
2019-09-18T22:54:47.5732650Z - LLL | extern "rust-call" {}
2019-09-18T22:54:47.5742245Z -     |
2019-09-18T22:54:47.5742245Z -     |
2019-09-18T22:54:47.5743456Z -     = note: for more information, see ***/issues/29625
2019-09-18T22:54:47.5744466Z -     = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-09-18T22:54:47.5744671Z +   --> $DIR/feature-gate-abi.rs:101:1
2019-09-18T22:54:47.5744714Z +    |
2019-09-18T22:54:47.5744904Z + LL | extern "rust-call" {}
2019-09-18T22:54:47.5744980Z +    |
2019-09-18T22:54:47.5744980Z +    |
2019-09-18T22:54:47.5745244Z +    = note: for more information, see ***/issues/29625
2019-09-18T22:54:47.5745293Z +    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-09-18T22:54:47.5745333Z 533 
2019-09-18T22:54:47.5745589Z 534 error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-09-18T22:54:47.5745936Z -     |
2019-09-18T22:54:47.5745936Z -     |
2019-09-18T22:54:47.5746115Z - LLL | extern "msp430-interrupt" {}
2019-09-18T22:54:47.5746473Z -     |
2019-09-18T22:54:47.5746473Z -     |
2019-09-18T22:54:47.5746735Z -     = note: for more information, see ***/issues/38487
2019-09-18T22:54:47.5746974Z -     = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-09-18T22:54:47.5747160Z +   --> $DIR/feature-gate-abi.rs:102:1
2019-09-18T22:54:47.5747197Z +    |
2019-09-18T22:54:47.5747393Z + LL | extern "msp430-interrupt" {}
2019-09-18T22:54:47.5747467Z +    |
2019-09-18T22:54:47.5747467Z +    |
2019-09-18T22:54:47.5747719Z +    = note: for more information, see ***/issues/38487
2019-09-18T22:54:47.5747768Z +    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-09-18T22:54:47.5747813Z 542 
2019-09-18T22:54:47.5747870Z 543 error[E0658]: PTX ABIs are experimental and subject to change
2019-09-18T22:54:47.5748223Z -     |
2019-09-18T22:54:47.5748223Z -     |
2019-09-18T22:54:47.5748415Z - LLL | extern "ptx-kernel" {}
2019-09-18T22:54:47.5748934Z -     |
2019-09-18T22:54:47.5748934Z -     |
2019-09-18T22:54:47.5749270Z -     = note: for more information, see ***/issues/38788
2019-09-18T22:54:47.5749682Z -     = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-09-18T22:54:47.5754613Z +   --> $DIR/feature-gate-abi.rs:103:1
2019-09-18T22:54:47.5754706Z +    |
2019-09-18T22:54:47.5754949Z + LL | extern "ptx-kernel" {}
2019-09-18T22:54:47.5755042Z +    |
2019-09-18T22:54:47.5755042Z +    |
2019-09-18T22:54:47.5755316Z +    = note: for more information, see ***/issues/38788
2019-09-18T22:54:47.5755365Z +    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-09-18T22:54:47.5755438Z 551 
2019-09-18T22:54:47.5755672Z 552 error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-09-18T22:54:47.5756032Z -     |
2019-09-18T22:54:47.5756032Z -     |
2019-09-18T22:54:47.5756209Z - LLL | extern "x86-interrupt" {}
2019-09-18T22:54:47.5756548Z -     |
2019-09-18T22:54:47.5756548Z -     |
2019-09-18T22:54:47.5756810Z -     = note: for more information, see ***/issues/40180
2019-09-18T22:54:47.5757042Z -     = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-09-18T22:54:47.5757247Z +   --> $DIR/feature-gate-abi.rs:104:1
2019-09-18T22:54:47.5757285Z +    |
2019-09-18T22:54:47.5757457Z + LL | extern "x86-interrupt" {}
2019-09-18T22:54:47.5757548Z +    |
2019-09-18T22:54:47.5757548Z +    |
2019-09-18T22:54:47.5757783Z +    = note: for more information, see ***/issues/40180
2019-09-18T22:54:47.5757856Z +    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-09-18T22:54:47.5758145Z 561 error[E0658]: thiscall is experimental and subject to change
2019-09-18T22:54:47.5758422Z -    --> $DIR/feature-gate-abi.rs:105:1
2019-09-18T22:54:47.5758791Z -     |
2019-09-18T22:54:47.5758791Z -     |
2019-09-18T22:54:47.5758971Z - LLL | extern "thiscall" {}
2019-09-18T22:54:47.5759344Z -     |
2019-09-18T22:54:47.5759559Z -     = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-09-18T22:54:47.5760582Z +   --> $DIR/feature-gate-abi.rs:105:1
2019-09-18T22:54:47.5760664Z +    |
2019-09-18T22:54:47.5760664Z +    |
2019-09-18T22:54:47.5760708Z + LL | extern "thiscall" {}
2019-09-18T22:54:47.5760793Z +    |
2019-09-18T22:54:47.5760862Z +    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-09-18T22:54:47.5760913Z 568 
2019-09-18T22:54:47.5760913Z 568 
2019-09-18T22:54:47.5761234Z 569 error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-09-18T22:54:47.5761746Z -     |
2019-09-18T22:54:47.5761746Z -     |
2019-09-18T22:54:47.5761981Z - LLL | extern "amdgpu-kernel" {}
2019-09-18T22:54:47.5762455Z -     |
2019-09-18T22:54:47.5762455Z -     |
2019-09-18T22:54:47.5762799Z -     = note: for more information, see ***/issues/51575
2019-09-18T22:54:47.5764107Z -     = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-09-18T22:54:47.5764402Z +   --> $DIR/feature-gate-abi.rs:106:1
2019-09-18T22:54:47.5764609Z +    |
2019-09-18T22:54:47.5764836Z + LL | extern "amdgpu-kernel" {}
2019-09-18T22:54:47.5764923Z +    |
2019-09-18T22:54:47.5764923Z +    |
2019-09-18T22:54:47.5765240Z +    = note: for more information, see ***/issues/51575
2019-09-18T22:54:47.5765297Z +    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-09-18T22:54:47.5765341Z 577 
2019-09-18T22:54:47.5765414Z 578 error[E0658]: Swift ABI is experimental and subject to change
2019-09-18T22:54:47.5766309Z -     |
2019-09-18T22:54:47.5766309Z -     |
2019-09-18T22:54:47.5766494Z - LLL | extern "Swift" {}
2019-09-18T22:54:47.5766818Z -     |
2019-09-18T22:54:47.5766818Z -     |
2019-09-18T22:54:47.5767175Z -     = help: add `#![feature(abi_swift)]` to the crate attributes to enable
2019-09-18T22:54:47.5767443Z +   --> $DIR/feature-gate-abi.rs:107:1
2019-09-18T22:54:47.5767481Z +    |
2019-09-18T22:54:47.5767515Z + LL | extern "Swift" {}
2019-09-18T22:54:47.5767600Z +    |
2019-09-18T22:54:47.5767600Z +    |
2019-09-18T22:54:47.5767639Z +    = help: add `#![feature(abi_swift)]` to the crate attributes to enable
2019-09-18T22:54:47.5767692Z 585 
2019-09-18T22:54:47.5767902Z 586 error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
2019-09-18T22:54:47.5768307Z 
2019-09-18T22:54:47.5768347Z 
2019-09-18T22:54:47.5768385Z The actual stderr differed from the expected stderr.
2019-09-18T22:54:47.5768385Z The actual stderr differed from the expected stderr.
2019-09-18T22:54:47.5768652Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi/feature-gate-abi.stderr
2019-09-18T22:54:47.5768885Z To update references, rerun the tests and pass the `--bless` flag
2019-09-18T22:54:47.5769123Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-abi.rs`
2019-09-18T22:54:47.5769191Z error: 1 errors occurred comparing output.
2019-09-18T22:54:47.5769250Z status: exit code: 1
2019-09-18T22:54:47.5769250Z status: exit code: 1
2019-09-18T22:54:47.5770254Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-abi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi/auxiliary" "-A" "unused"
2019-09-18T22:54:47.5770806Z ------------------------------------------
2019-09-18T22:54:47.5770844Z 
2019-09-18T22:54:47.5771110Z ------------------------------------------
2019-09-18T22:54:47.5771159Z stderr:
2019-09-18T22:54:47.5771159Z stderr:
2019-09-18T22:54:47.5771387Z ------------------------------------------
2019-09-18T22:54:47.5771457Z error[E0658]: intrinsics are subject to change
2019-09-18T22:54:47.5771713Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:13:1
2019-09-18T22:54:47.5771766Z    |
2019-09-18T22:54:47.5773258Z LL | extern "rust-intrinsic" fn f1() {} //~ ERROR intrinsics are subject to change
2019-09-18T22:54:47.5773400Z    |
2019-09-18T22:54:47.5773449Z    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-09-18T22:54:47.5773518Z 
2019-09-18T22:54:47.5773565Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T22:54:47.5773565Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T22:54:47.5773842Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:15:1
2019-09-18T22:54:47.5773911Z    |
2019-09-18T22:54:47.5774196Z LL | extern "platform-intrinsic" fn f2() {} //~ ERROR platform intrinsics are experimental
2019-09-18T22:54:47.5774325Z    |
2019-09-18T22:54:47.5774325Z    |
2019-09-18T22:54:47.5774653Z    = note: for more information, see ***/issues/27731
2019-09-18T22:54:47.5774716Z    = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
2019-09-18T22:54:47.5774817Z error[E0658]: vectorcall is experimental and subject to change
2019-09-18T22:54:47.5775088Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:17:1
2019-09-18T22:54:47.5775157Z    |
2019-09-18T22:54:47.5775157Z    |
2019-09-18T22:54:47.5775218Z LL | extern "vectorcall" fn f3() {} //~ ERROR vectorcall is experimental and subject to change
2019-09-18T22:54:47.5775312Z    |
2019-09-18T22:54:47.5775382Z    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-18T22:54:47.5775415Z 
2019-09-18T22:54:47.5776200Z error[E0658]: rust-call ABI is subject to change
2019-09-18T22:54:47.5776200Z error[E0658]: rust-call ABI is subject to change
2019-09-18T22:54:47.5776505Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:18:1
2019-09-18T22:54:47.5776548Z    |
2019-09-18T22:54:47.5776779Z LL | extern "rust-call" fn f4() {} //~ ERROR rust-call ABI is subject to change
2019-09-18T22:54:47.5776880Z    |
2019-09-18T22:54:47.5776880Z    |
2019-09-18T22:54:47.5777145Z    = note: for more information, see ***/issues/29625
2019-09-18T22:54:47.5777217Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-09-18T22:54:47.5777247Z 
2019-09-18T22:54:47.5777665Z error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-09-18T22:54:47.5778797Z    |
2019-09-18T22:54:47.5778797Z    |
2019-09-18T22:54:47.5779108Z LL | extern "msp430-interrupt" fn f5() {} //~ ERROR msp430-interrupt ABI is experimental
2019-09-18T22:54:47.5779225Z    |
2019-09-18T22:54:47.5779225Z    |
2019-09-18T22:54:47.5779539Z    = note: for more information, see ***/issues/38487
2019-09-18T22:54:47.5779604Z    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-09-18T22:54:47.5779659Z 
2019-09-18T22:54:47.5779705Z error[E0658]: PTX ABIs are experimental and subject to change
2019-09-18T22:54:47.5780133Z    |
2019-09-18T22:54:47.5780133Z    |
2019-09-18T22:54:47.5780410Z LL | extern "ptx-kernel" fn f6() {} //~ ERROR PTX ABIs are experimental and subject to change
2019-09-18T22:54:47.5780660Z    |
2019-09-18T22:54:47.5780660Z    |
2019-09-18T22:54:47.5780999Z    = note: for more information, see ***/issues/38788
2019-09-18T22:54:47.5781059Z    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-09-18T22:54:47.5781092Z 
2019-09-18T22:54:47.5781382Z error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-09-18T22:54:47.5781701Z    |
2019-09-18T22:54:47.5781701Z    |
2019-09-18T22:54:47.5781992Z LL | extern "x86-interrupt" fn f7() {} //~ ERROR x86-interrupt ABI is experimental
2019-09-18T22:54:47.5782088Z    |
2019-09-18T22:54:47.5782088Z    |
2019-09-18T22:54:47.5782399Z    = note: for more information, see ***/issues/40180
2019-09-18T22:54:47.5782463Z    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-09-18T22:54:47.5782568Z error[E0658]: thiscall is experimental and subject to change
2019-09-18T22:54:47.5782876Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:22:1
2019-09-18T22:54:47.5782929Z    |
2019-09-18T22:54:47.5782929Z    |
2019-09-18T22:54:47.5783002Z LL | extern "thiscall" fn f8() {} //~ ERROR thiscall is experimental and subject to change
2019-09-18T22:54:47.5783100Z    |
2019-09-18T22:54:47.5783179Z    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-09-18T22:54:47.5783264Z 
2019-09-18T22:54:47.5783264Z 
2019-09-18T22:54:47.5783552Z error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-09-18T22:54:47.5784052Z    |
2019-09-18T22:54:47.5784052Z    |
2019-09-18T22:54:47.5784331Z LL | extern "amdgpu-kernel" fn f9() {} //~ ERROR amdgpu-kernel ABI is experimental and subject to change
2019-09-18T22:54:47.5784446Z    |
2019-09-18T22:54:47.5784446Z    |
2019-09-18T22:54:47.5784878Z    = note: for more information, see ***/issues/51575
2019-09-18T22:54:47.5784945Z    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-09-18T22:54:47.5785155Z 
2019-09-18T22:54:47.5785196Z error[E0658]: Swift ABI is experimental and subject to change
2019-09-18T22:54:47.5785901Z    |
2019-09-18T22:54:47.5785901Z    |
2019-09-18T22:54:47.5786384Z LL | extern "Swift" fn f10() {} //~ ERROR Swift ABI is experimental and subject to change
2019-09-18T22:54:47.5786502Z    |
2019-09-18T22:54:47.5786502Z    |
2019-09-18T22:54:47.5786546Z    = help: add `#![feature(abi_swift)]` to the crate attributes to enable
2019-09-18T22:54:47.5786614Z error[E0658]: intrinsics are subject to change
2019-09-18T22:54:47.5786899Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:28:5
2019-09-18T22:54:47.5786944Z    |
2019-09-18T22:54:47.5786944Z    |
2019-09-18T22:54:47.5787790Z LL |     extern "rust-intrinsic" fn m1(); //~ ERROR intrinsics are subject to change
2019-09-18T22:54:47.5787975Z    |
2019-09-18T22:54:47.5788019Z    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-09-18T22:54:47.5788051Z 
2019-09-18T22:54:47.5788111Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T22:54:47.5788111Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T22:54:47.5788396Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:30:5
2019-09-18T22:54:47.5788443Z    |
2019-09-18T22:54:47.5788708Z LL |     extern "platform-intrinsic" fn m2(); //~ ERROR platform intrinsics are experimental
2019-09-18T22:54:47.5788798Z    |
2019-09-18T22:54:47.5788798Z    |
2019-09-18T22:54:47.5789124Z    = note: for more information, see ***/issues/27731
2019-09-18T22:54:47.5789365Z    = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
2019-09-18T22:54:47.5789606Z error[E0658]: vectorcall is experimental and subject to change
2019-09-18T22:54:47.5790507Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:32:5
2019-09-18T22:54:47.5790566Z    |
2019-09-18T22:54:47.5790566Z    |
2019-09-18T22:54:47.5790618Z LL |     extern "vectorcall" fn m3(); //~ ERROR vectorcall is experimental and subject to change
2019-09-18T22:54:47.5790741Z    |
2019-09-18T22:54:47.5790802Z    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-18T22:54:47.5790855Z 
2019-09-18T22:54:47.5791110Z error[E0658]: rust-call ABI is subject to change
2019-09-18T22:54:47.5791110Z error[E0658]: rust-call ABI is subject to change
2019-09-18T22:54:47.5791373Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:33:5
2019-09-18T22:54:47.5791443Z    |
2019-09-18T22:54:47.5791713Z LL |     extern "rust-call" fn m4(); //~ ERROR rust-call ABI is subject to change
2019-09-18T22:54:47.5791809Z    |
2019-09-18T22:54:47.5791809Z    |
2019-09-18T22:54:47.5792144Z    = note: for more information, see ***/issues/29625
2019-09-18T22:54:47.5792216Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-09-18T22:54:47.5792250Z 
2019-09-18T22:54:47.5792551Z error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-09-18T22:54:47.5792861Z    |
2019-09-18T22:54:47.5792861Z    |
2019-09-18T22:54:47.5793169Z LL |     extern "msp430-interrupt" fn m5(); //~ ERROR msp430-interrupt ABI is experimental
2019-09-18T22:54:47.5793270Z    |
2019-09-18T22:54:47.5793270Z    |
2019-09-18T22:54:47.5793889Z    = note: for more information, see ***/issues/38487
2019-09-18T22:54:47.5793938Z    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-09-18T22:54:47.5793966Z 
2019-09-18T22:54:47.5794022Z error[E0658]: PTX ABIs are experimental and subject to change
2019-09-18T22:54:47.5794285Z    |
2019-09-18T22:54:47.5794285Z    |
2019-09-18T22:54:47.5794547Z LL |     extern "ptx-kernel" fn m6(); //~ ERROR PTX ABIs are experimental and subject to change
2019-09-18T22:54:47.5794627Z    |
2019-09-18T22:54:47.5794627Z    |
2019-09-18T22:54:47.5795171Z    = note: for more information, see ***/issues/38788
2019-09-18T22:54:47.5795227Z    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-09-18T22:54:47.5795392Z 
2019-09-18T22:54:47.5795858Z error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-09-18T22:54:47.5796109Z    |
2019-09-18T22:54:47.5796109Z    |
2019-09-18T22:54:47.5796325Z LL |     extern "x86-interrupt" fn m7(); //~ ERROR x86-interrupt ABI is experimental
2019-09-18T22:54:47.5796421Z    |
2019-09-18T22:54:47.5796421Z    |
2019-09-18T22:54:47.5796683Z    = note: for more information, see ***/issues/40180
2019-09-18T22:54:47.5796734Z    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-09-18T22:54:47.5796812Z error[E0658]: thiscall is experimental and subject to change
2019-09-18T22:54:47.5797073Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:37:5
2019-09-18T22:54:47.5797116Z    |
2019-09-18T22:54:47.5797116Z    |
2019-09-18T22:54:47.5797167Z LL |     extern "thiscall" fn m8(); //~ ERROR thiscall is experimental and subject to change
2019-09-18T22:54:47.5797265Z    |
2019-09-18T22:54:47.5797306Z    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-09-18T22:54:47.5797334Z 
2019-09-18T22:54:47.5797334Z 
2019-09-18T22:54:47.5797583Z error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-09-18T22:54:47.5797847Z    |
2019-09-18T22:54:47.5797847Z    |
2019-09-18T22:54:47.5798112Z LL |     extern "amdgpu-kernel" fn m9(); //~ ERROR amdgpu-kernel ABI is experimental and subject to change
2019-09-18T22:54:47.5798300Z    |
2019-09-18T22:54:47.5798300Z    |
2019-09-18T22:54:47.5798613Z    = note: for more information, see ***/issues/51575
2019-09-18T22:54:47.5798664Z    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-09-18T22:54:47.5798692Z 
2019-09-18T22:54:47.5798759Z error[E0658]: Swift ABI is experimental and subject to change
2019-09-18T22:54:47.5799039Z    |
2019-09-18T22:54:47.5799039Z    |
2019-09-18T22:54:47.5799099Z LL |     extern "Swift" fn m10(); //~ ERROR Swift ABI is experimental and subject to change
2019-09-18T22:54:47.5799176Z    |
2019-09-18T22:54:47.5799176Z    |
2019-09-18T22:54:47.5799233Z    = help: add `#![feature(abi_swift)]` to the crate attributes to enable
2019-09-18T22:54:47.5799299Z error[E0658]: vectorcall is experimental and subject to change
2019-09-18T22:54:47.5799530Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:41:5
2019-09-18T22:54:47.5799708Z    |
2019-09-18T22:54:47.5799708Z    |
2019-09-18T22:54:47.5799750Z LL |     extern "vectorcall" fn dm3() {} //~ ERROR vectorcall is experimental and subject to change
2019-09-18T22:54:47.5799847Z    |
2019-09-18T22:54:47.5799895Z    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-18T22:54:47.5799923Z 
2019-09-18T22:54:47.5800866Z error[E0658]: rust-call ABI is subject to change
2019-09-18T22:54:47.5800866Z error[E0658]: rust-call ABI is subject to change
2019-09-18T22:54:47.5801200Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:42:5
2019-09-18T22:54:47.5801250Z    |
2019-09-18T22:54:47.5801522Z LL |     extern "rust-call" fn dm4() {} //~ ERROR rust-call ABI is subject to change
2019-09-18T22:54:47.5801639Z    |
2019-09-18T22:54:47.5801639Z    |
2019-09-18T22:54:47.5802246Z    = note: for more information, see ***/issues/29625
2019-09-18T22:54:47.5802346Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-09-18T22:54:47.5802398Z 
2019-09-18T22:54:47.5802722Z error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-09-18T22:54:47.5803055Z    |
2019-09-18T22:54:47.5803055Z    |
2019-09-18T22:54:47.5804676Z LL |     extern "msp430-interrupt" fn dm5() {} //~ ERROR msp430-interrupt ABI is experimental
2019-09-18T22:54:47.5804879Z    |
2019-09-18T22:54:47.5804879Z    |
2019-09-18T22:54:47.5805272Z    = note: for more information, see ***/issues/38487
2019-09-18T22:54:47.5805343Z    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-09-18T22:54:47.5805371Z 
2019-09-18T22:54:47.5805407Z error[E0658]: PTX ABIs are experimental and subject to change
2019-09-18T22:54:47.5805692Z    |
2019-09-18T22:54:47.5805692Z    |
2019-09-18T22:54:47.5805936Z LL |     extern "ptx-kernel" fn dm6() {} //~ ERROR PTX ABIs are experimental and subject to change
2019-09-18T22:54:47.5806033Z    |
2019-09-18T22:54:47.5806033Z    |
2019-09-18T22:54:47.5806271Z    = note: for more information, see ***/issues/38788
2019-09-18T22:54:47.5806320Z    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-09-18T22:54:47.5806375Z 
2019-09-18T22:54:47.5806607Z error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-09-18T22:54:47.5806891Z    |
2019-09-18T22:54:47.5806891Z    |
2019-09-18T22:54:47.5807125Z LL |     extern "x86-interrupt" fn dm7() {} //~ ERROR x86-interrupt ABI is experimental
2019-09-18T22:54:47.5807228Z    |
2019-09-18T22:54:47.5807228Z    |
2019-09-18T22:54:47.5807478Z    = note: for more information, see ***/issues/40180
2019-09-18T22:54:47.5807527Z    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-09-18T22:54:47.5807786Z error[E0658]: thiscall is experimental and subject to change
2019-09-18T22:54:47.5808054Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:46:5
2019-09-18T22:54:47.5808095Z    |
2019-09-18T22:54:47.5808095Z    |
2019-09-18T22:54:47.5808164Z LL |     extern "thiscall" fn dm8() {} //~ ERROR thiscall is experimental and subject to change
2019-09-18T22:54:47.5808243Z    |
2019-09-18T22:54:47.5808302Z    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-09-18T22:54:47.5808330Z 
2019-09-18T22:54:47.5808330Z 
2019-09-18T22:54:47.5808553Z error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-09-18T22:54:47.5808835Z    |
2019-09-18T22:54:47.5808835Z    |
2019-09-18T22:54:47.5810316Z LL |     extern "amdgpu-kernel" fn dm9() {} //~ ERROR amdgpu-kernel ABI is experimental and subject to change
2019-09-18T22:54:47.5810616Z    |
2019-09-18T22:54:47.5810616Z    |
2019-09-18T22:54:47.5811102Z    = note: for more information, see ***/issues/51575
2019-09-18T22:54:47.5811186Z    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-09-18T22:54:47.5811223Z 
2019-09-18T22:54:47.5811280Z error[E0658]: Swift ABI is experimental and subject to change
2019-09-18T22:54:47.5811633Z    |
2019-09-18T22:54:47.5811633Z    |
2019-09-18T22:54:47.5811684Z LL |     extern "Swift" fn dm10() {} //~ ERROR Swift ABI is experimental and subject to change
2019-09-18T22:54:47.5812576Z    |
2019-09-18T22:54:47.5812576Z    |
2019-09-18T22:54:47.5812638Z    = help: add `#![feature(abi_swift)]` to the crate attributes to enable
2019-09-18T22:54:47.5812715Z error[E0658]: intrinsics are subject to change
2019-09-18T22:54:47.5813147Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:55:5
2019-09-18T22:54:47.5813198Z    |
2019-09-18T22:54:47.5813198Z    |
2019-09-18T22:54:47.5813476Z LL |     extern "rust-intrinsic" fn m1() {} //~ ERROR intrinsics are subject to change
2019-09-18T22:54:47.5813740Z    |
2019-09-18T22:54:47.5813921Z    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-09-18T22:54:47.5813980Z 
2019-09-18T22:54:47.5814019Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T22:54:47.5814019Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T22:54:47.5814265Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:57:5
2019-09-18T22:54:47.5814303Z    |
2019-09-18T22:54:47.5814718Z LL |     extern "platform-intrinsic" fn m2() {} //~ ERROR platform intrinsics are experimental
2019-09-18T22:54:47.5814798Z    |
2019-09-18T22:54:47.5814798Z    |
2019-09-18T22:54:47.5815086Z    = note: for more information, see ***/issues/27731
2019-09-18T22:54:47.5815146Z    = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
2019-09-18T22:54:47.5815229Z error[E0658]: vectorcall is experimental and subject to change
2019-09-18T22:54:47.5815451Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:59:5
2019-09-18T22:54:47.5815490Z    |
2019-09-18T22:54:47.5815490Z    |
2019-09-18T22:54:47.5815557Z LL |     extern "vectorcall" fn m3() {} //~ ERROR vectorcall is experimental and subject to change
2019-09-18T22:54:47.5815631Z    |
2019-09-18T22:54:47.5815687Z    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-18T22:54:47.5815713Z 
2019-09-18T22:54:47.5815909Z error[E0658]: rust-call ABI is subject to change
2019-09-18T22:54:47.5815909Z error[E0658]: rust-call ABI is subject to change
2019-09-18T22:54:47.5816117Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:60:5
2019-09-18T22:54:47.5816175Z    |
2019-09-18T22:54:47.5816390Z LL |     extern "rust-call" fn m4() {} //~ ERROR rust-call ABI is subject to change
2019-09-18T22:54:47.5816597Z    |
2019-09-18T22:54:47.5816597Z    |
2019-09-18T22:54:47.5818411Z    = note: for more information, see ***/issues/29625
2019-09-18T22:54:47.5818472Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-09-18T22:54:47.5818521Z 
2019-09-18T22:54:47.5818799Z error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-09-18T22:54:47.5819071Z    |
2019-09-18T22:54:47.5819071Z    |
2019-09-18T22:54:47.5819294Z LL |     extern "msp430-interrupt" fn m5() {} //~ ERROR msp430-interrupt ABI is experimental
2019-09-18T22:54:47.5819391Z    |
2019-09-18T22:54:47.5819391Z    |
2019-09-18T22:54:47.5819788Z    = note: for more information, see ***/issues/38487
2019-09-18T22:54:47.5819839Z    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-09-18T22:54:47.5819893Z 
2019-09-18T22:54:47.5819931Z error[E0658]: PTX ABIs are experimental and subject to change
2019-09-18T22:54:47.5820634Z    |
2019-09-18T22:54:47.5820634Z    |
2019-09-18T22:54:47.5820942Z LL |     extern "ptx-kernel" fn m6() {} //~ ERROR PTX ABIs are experimental and subject to change
2019-09-18T22:54:47.5821054Z    |
2019-09-18T22:54:47.5821054Z    |
2019-09-18T22:54:47.5821378Z    = note: for more information, see ***/issues/38788
2019-09-18T22:54:47.5821439Z    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-09-18T22:54:47.5821471Z 
2019-09-18T22:54:47.5821758Z error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-09-18T22:54:47.5822068Z    |
2019-09-18T22:54:47.5822068Z    |
2019-09-18T22:54:47.5822360Z LL |     extern "x86-interrupt" fn m7() {} //~ ERROR x86-interrupt ABI is experimental
2019-09-18T22:54:47.5822469Z    |
2019-09-18T22:54:47.5822469Z    |
2019-09-18T22:54:47.5822779Z    = note: for more information, see ***/issues/40180
2019-09-18T22:54:47.5822838Z    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-09-18T22:54:47.5822936Z error[E0658]: thiscall is experimental and subject to change
2019-09-18T22:54:47.5823367Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:64:5
2019-09-18T22:54:47.5823434Z    |
2019-09-18T22:54:47.5823434Z    |
2019-09-18T22:54:47.5823487Z LL |     extern "thiscall" fn m8() {} //~ ERROR thiscall is experimental and subject to change
2019-09-18T22:54:47.5823787Z    |
2019-09-18T22:54:47.5823995Z    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-09-18T22:54:47.5824043Z 
2019-09-18T22:54:47.5824043Z 
2019-09-18T22:54:47.5824303Z error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-09-18T22:54:47.5824591Z    |
2019-09-18T22:54:47.5824591Z    |
2019-09-18T22:54:47.5824834Z LL |     extern "amdgpu-kernel" fn m9() {} //~ ERROR amdgpu-kernel ABI is experimental and subject to change
2019-09-18T22:54:47.5824914Z    |
2019-09-18T22:54:47.5824914Z    |
2019-09-18T22:54:47.5825191Z    = note: for more information, see ***/issues/51575
2019-09-18T22:54:47.5825241Z    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-09-18T22:54:47.5825267Z 
2019-09-18T22:54:47.5825323Z error[E0658]: Swift ABI is experimental and subject to change
2019-09-18T22:54:47.5825592Z    |
2019-09-18T22:54:47.5825592Z    |
2019-09-18T22:54:47.5825652Z LL |     extern "Swift" fn m10() {} //~ ERROR Swift ABI is experimental and subject to change
2019-09-18T22:54:47.5825727Z    |
2019-09-18T22:54:47.5825727Z    |
2019-09-18T22:54:47.5825892Z    = help: add `#![feature(abi_swift)]` to the crate attributes to enable
2019-09-18T22:54:47.5825953Z error[E0658]: intrinsics are subject to change
2019-09-18T22:54:47.5826202Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:71:5
2019-09-18T22:54:47.5826263Z    |
2019-09-18T22:54:47.5826263Z    |
2019-09-18T22:54:47.5826498Z LL |     extern "rust-intrinsic" fn im1() {} //~ ERROR intrinsics are subject to change
2019-09-18T22:54:47.5826602Z    |
2019-09-18T22:54:47.5826641Z    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-09-18T22:54:47.5826667Z 
2019-09-18T22:54:47.5826721Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T22:54:47.5826721Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-18T22:54:47.5828017Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:73:5
2019-09-18T22:54:47.5828083Z    |
2019-09-18T22:54:47.5828371Z LL |     extern "platform-intrinsic" fn im2() {} //~ ERROR platform intrinsics are experimental
2019-09-18T22:54:47.5828493Z    |
2019-09-18T22:54:47.5828493Z    |
2019-09-18T22:54:47.5828794Z    = note: for more information, see ***/issues/27731
2019-09-18T22:54:47.5828844Z    = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
2019-09-18T22:54:47.5828918Z error[E0658]: vectorcall is experimental and subject to change
2019-09-18T22:54:47.5829170Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:75:5
2019-09-18T22:54:47.5829211Z    |
2019-09-18T22:54:47.5829211Z    |
2019-09-18T22:54:47.5829252Z LL |     extern "vectorcall" fn im3() {} //~ ERROR vectorcall is experimental and subject to change
2019-09-18T22:54:47.5829345Z    |
2019-09-18T22:54:47.5829383Z    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-18T22:54:47.5829410Z 
2019-09-18T22:54:47.5829630Z error[E0658]: rust-call ABI is subject to change
2019-09-18T22:54:47.5829630Z error[E0658]: rust-call ABI is subject to change
2019-09-18T22:54:47.5831288Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:76:5
2019-09-18T22:54:47.5831364Z    |
2019-09-18T22:54:47.5831688Z LL |     extern "rust-call" fn im4() {} //~ ERROR rust-call ABI is subject to change
2019-09-18T22:54:47.5831788Z    |
2019-09-18T22:54:47.5831788Z    |
2019-09-18T22:54:47.5832303Z    = note: for more information, see ***/issues/29625
---
2019-09-18T22:54:47.5888403Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-18T22:54:47.5888452Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-18T22:54:47.5888494Z 
2019-09-18T22:54:47.5888515Z 
2019-09-18T22:54:47.5889695Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-18T22:54:47.5890355Z 
2019-09-18T22:54:47.5890385Z 
2019-09-18T22:54:47.5890496Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-18T22:54:47.5890565Z Build completed unsuccessfully in 1:05:59
2019-09-18T22:54:47.5890565Z Build completed unsuccessfully in 1:05:59
2019-09-18T22:54:47.5890612Z == clock drift check ==
2019-09-18T22:54:47.5890657Z   local time: Wed Sep 18 22:54:47 UTC 2019
2019-09-18T22:54:47.6707372Z   network time: Wed, 18 Sep 2019 22:54:47 GMT
2019-09-18T22:54:47.6707512Z == end clock drift check ==
2019-09-18T22:54:48.7781660Z ##[error]Bash exited with code '1'.
2019-09-18T22:54:48.7824835Z ##[section]Starting: Checkout
2019-09-18T22:54:48.7826668Z ==============================================================================
2019-09-18T22:54:48.7826744Z Task         : Get sources
2019-09-18T22:54:48.7826792Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
