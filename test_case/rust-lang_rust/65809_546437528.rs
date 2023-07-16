plain
2019-10-25T16:08:46.8599094Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-25T16:08:46.8804598Z ##[command]git config gc.auto 0
2019-10-25T16:08:46.8887122Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-25T16:08:46.8944271Z ##[command]git config --get-all http.proxy
2019-10-25T16:08:46.9064392Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65809/merge:refs/remotes/pull/65809/merge
---
2019-10-25T17:05:42.6084167Z .................................................................................................... 1600/9251
2019-10-25T17:05:47.8338796Z .................................................................................................... 1700/9251
2019-10-25T17:05:59.7277581Z ........................................................i...............i........................... 1800/9251
2019-10-25T17:06:06.8267980Z .................................................................................................... 1900/9251
2019-10-25T17:06:20.6948827Z ..............................................iiiii................................................. 2000/9251
2019-10-25T17:06:30.6519385Z .................................................................................................... 2200/9251
2019-10-25T17:06:32.9649560Z .................................................................................................... 2300/9251
2019-10-25T17:06:36.5222146Z .................................................................................................... 2400/9251
2019-10-25T17:06:58.6009563Z .................................................................................................... 2500/9251
---
2019-10-25T17:09:42.0661947Z .................................................i...............i.................................. 4800/9251
2019-10-25T17:09:50.5066326Z .................................................................................................... 4900/9251
2019-10-25T17:09:58.7705703Z .................................................................................................... 5000/9251
2019-10-25T17:10:04.7680267Z .................................................................................................... 5100/9251
2019-10-25T17:10:14.5962591Z ..................................................ii.ii...........i................................. 5200/9251
2019-10-25T17:10:23.8879008Z .................................................................................................... 5400/9251
2019-10-25T17:10:32.5141807Z .................................................................................................... 5500/9251
2019-10-25T17:10:39.9394786Z ....................i............................................................................... 5600/9251
2019-10-25T17:10:45.1955422Z .................................................................................................... 5700/9251
2019-10-25T17:10:45.1955422Z .................................................................................................... 5700/9251
2019-10-25T17:10:56.8101753Z .................................................................................................... 5800/9251
2019-10-25T17:11:07.7698729Z .................ii...i..ii...........i............................................................. 5900/9251
2019-10-25T17:11:28.3629600Z .................................................................................................... 6100/9251
2019-10-25T17:11:34.3128909Z .................................................................................................... 6200/9251
2019-10-25T17:11:34.3128909Z .................................................................................................... 6200/9251
2019-10-25T17:11:46.5635681Z ........................................i..ii....................................................... 6300/9251
2019-10-25T17:12:07.6943864Z .................................................................................................... 6500/9251
2019-10-25T17:12:09.7856214Z ......i............................................................................................. 6600/9251
2019-10-25T17:12:11.7480249Z .................................................................................i.................. 6700/9251
2019-10-25T17:12:14.1936163Z .................................................................................................... 6800/9251
---
2019-10-25T17:16:07.4404752Z 
2019-10-25T17:16:07.4406084Z ---- [ui] ui/feature-gates/feature-gate-abi.rs stdout ----
2019-10-25T17:16:07.4406555Z diff of stderr:
2019-10-25T17:16:07.4406920Z 
2019-10-25T17:16:07.4407262Z 82 LL | extern "efiapi" fn f10() {}
2019-10-25T17:16:07.4407651Z 84    |
2019-10-25T17:16:07.4407651Z 84    |
2019-10-25T17:16:07.4409164Z -    = note: for more information, see ***/issues/1
2019-10-25T17:16:07.4410031Z +    = note: for more information, see ***/issues/65815
2019-10-25T17:16:07.4410562Z 86    = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
2019-10-25T17:16:07.4411232Z 88 error[E0658]: intrinsics are subject to change
2019-10-25T17:16:07.4411457Z 
2019-10-25T17:16:07.4411457Z 
2019-10-25T17:16:07.4412391Z 169 LL |     extern "efiapi" fn m10();
2019-10-25T17:16:07.4412820Z 171    |
2019-10-25T17:16:07.4412820Z 171    |
2019-10-25T17:16:07.4413421Z -    = note: for more information, see ***/issues/1
2019-10-25T17:16:07.4414040Z +    = note: for more information, see ***/issues/65815
2019-10-25T17:16:07.4414609Z 173    = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
2019-10-25T17:16:07.4415414Z 175 error[E0658]: vectorcall is experimental and subject to change
2019-10-25T17:16:07.4415605Z 
2019-10-25T17:16:07.4415605Z 
2019-10-25T17:16:07.4415816Z 239 LL |     extern "efiapi" fn dm10() {}
2019-10-25T17:16:07.4416285Z 241    |
2019-10-25T17:16:07.4416285Z 241    |
2019-10-25T17:16:07.4416842Z -    = note: for more information, see ***/issues/1
2019-10-25T17:16:07.4417493Z +    = note: for more information, see ***/issues/65815
2019-10-25T17:16:07.4418713Z 243    = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
2019-10-25T17:16:07.4419370Z 245 error[E0658]: intrinsics are subject to change
2019-10-25T17:16:07.4419559Z 
2019-10-25T17:16:07.4419559Z 
2019-10-25T17:16:07.4419974Z 326 LL |     extern "efiapi" fn m10() {}
2019-10-25T17:16:07.4420439Z 328    |
2019-10-25T17:16:07.4420439Z 328    |
2019-10-25T17:16:07.4421283Z -    = note: for more information, see ***/issues/1
2019-10-25T17:16:07.4422984Z +    = note: for more information, see ***/issues/65815
2019-10-25T17:16:07.4423407Z 330    = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
2019-10-25T17:16:07.4424300Z 332 error[E0658]: intrinsics are subject to change
2019-10-25T17:16:07.4424470Z 
2019-10-25T17:16:07.4424470Z 
2019-10-25T17:16:07.4424665Z 413 LL |     extern "efiapi" fn im10() {}
2019-10-25T17:16:07.4425219Z 415    |
2019-10-25T17:16:07.4425219Z 415    |
2019-10-25T17:16:07.4425697Z -    = note: for more information, see ***/issues/1
2019-10-25T17:16:07.4426372Z +    = note: for more information, see ***/issues/65815
2019-10-25T17:16:07.4426617Z 417    = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
2019-10-25T17:16:07.4427134Z 419 error[E0658]: intrinsics are subject to change
2019-10-25T17:16:07.4427284Z 
2019-10-25T17:16:07.4427284Z 
2019-10-25T17:16:07.4428025Z 500 LL | type A10 = extern "efiapi" fn();
2019-10-25T17:16:07.4428548Z 502    |
2019-10-25T17:16:07.4428548Z 502    |
2019-10-25T17:16:07.4429122Z -    = note: for more information, see ***/issues/1
2019-10-25T17:16:07.4429763Z +    = note: for more information, see ***/issues/65815
2019-10-25T17:16:07.4430822Z 504    = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
2019-10-25T17:16:07.4431564Z 506 error[E0658]: intrinsics are subject to change
2019-10-25T17:16:07.4431681Z 
2019-10-25T17:16:07.4431681Z 
2019-10-25T17:16:07.4431793Z 587 LL | extern "efiapi" {}
2019-10-25T17:16:07.4432031Z 589    |
2019-10-25T17:16:07.4432031Z 589    |
2019-10-25T17:16:07.4432468Z -    = note: for more information, see ***/issues/1
2019-10-25T17:16:07.4433037Z +    = note: for more information, see ***/issues/65815
2019-10-25T17:16:07.4433199Z 591    = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
2019-10-25T17:16:07.4433329Z 592 
2019-10-25T17:16:07.4433653Z 593 error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
2019-10-25T17:16:07.4433870Z 
2019-10-25T17:16:07.4433996Z The actual stderr differed from the expected stderr.
2019-10-25T17:16:07.4433996Z The actual stderr differed from the expected stderr.
2019-10-25T17:16:07.4434336Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi/feature-gate-abi.stderr
2019-10-25T17:16:07.4434665Z To update references, rerun the tests and pass the `--bless` flag
2019-10-25T17:16:07.4435063Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-abi.rs`
2019-10-25T17:16:07.4435317Z error: 1 errors occurred comparing output.
2019-10-25T17:16:07.4435446Z status: exit code: 1
2019-10-25T17:16:07.4435446Z status: exit code: 1
2019-10-25T17:16:07.4436246Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-abi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi/auxiliary" "-A" "unused"
2019-10-25T17:16:07.4436825Z ------------------------------------------
2019-10-25T17:16:07.4436986Z 
2019-10-25T17:16:07.4437283Z ------------------------------------------
2019-10-25T17:16:07.4437431Z stderr:
2019-10-25T17:16:07.4437431Z stderr:
2019-10-25T17:16:07.4437908Z ------------------------------------------
2019-10-25T17:16:07.4438064Z error[E0658]: intrinsics are subject to change
2019-10-25T17:16:07.4438931Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:13:1
2019-10-25T17:16:07.4439167Z    |
2019-10-25T17:16:07.4439577Z LL | extern "rust-intrinsic" fn f1() {} //~ ERROR intrinsics are subject to change
2019-10-25T17:16:07.4439955Z    |
2019-10-25T17:16:07.4440101Z    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-10-25T17:16:07.4440227Z 
2019-10-25T17:16:07.4440391Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-10-25T17:16:07.4440391Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-10-25T17:16:07.4440778Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:15:1
2019-10-25T17:16:07.4440979Z    |
2019-10-25T17:16:07.4441404Z LL | extern "platform-intrinsic" fn f2() {} //~ ERROR platform intrinsics are experimental
2019-10-25T17:16:07.4441771Z    |
2019-10-25T17:16:07.4441771Z    |
2019-10-25T17:16:07.4442297Z    = note: for more information, see ***/issues/27731
2019-10-25T17:16:07.4442488Z    = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
2019-10-25T17:16:07.4442702Z error[E0658]: vectorcall is experimental and subject to change
2019-10-25T17:16:07.4443210Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:17:1
2019-10-25T17:16:07.4443365Z    |
2019-10-25T17:16:07.4443365Z    |
2019-10-25T17:16:07.4443481Z LL | extern "vectorcall" fn f3() {} //~ ERROR vectorcall is experimental and subject to change
2019-10-25T17:16:07.4443717Z    |
2019-10-25T17:16:07.4443829Z    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-10-25T17:16:07.4443944Z 
2019-10-25T17:16:07.4444241Z error[E0658]: rust-call ABI is subject to change
2019-10-25T17:16:07.4444241Z error[E0658]: rust-call ABI is subject to change
2019-10-25T17:16:07.4444575Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:18:1
2019-10-25T17:16:07.4444744Z    |
2019-10-25T17:16:07.4445055Z LL | extern "rust-call" fn f4() {} //~ ERROR rust-call ABI is subject to change
2019-10-25T17:16:07.4445338Z    |
2019-10-25T17:16:07.4445338Z    |
2019-10-25T17:16:07.4445681Z    = note: for more information, see ***/issues/29625
2019-10-25T17:16:07.4445875Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-10-25T17:16:07.4445988Z 
2019-10-25T17:16:07.4446305Z error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-10-25T17:16:07.4446809Z    |
2019-10-25T17:16:07.4446809Z    |
2019-10-25T17:16:07.4447125Z LL | extern "msp430-interrupt" fn f5() {} //~ ERROR msp430-interrupt ABI is experimental
2019-10-25T17:16:07.4447406Z    |
2019-10-25T17:16:07.4447406Z    |
2019-10-25T17:16:07.4447939Z    = note: for more information, see ***/issues/38487
2019-10-25T17:16:07.4448488Z    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-10-25T17:16:07.4448635Z 
2019-10-25T17:16:07.4448803Z error[E0658]: PTX ABIs are experimental and subject to change
2019-10-25T17:16:07.4449425Z    |
2019-10-25T17:16:07.4449425Z    |
2019-10-25T17:16:07.4449970Z LL | extern "ptx-kernel" fn f6() {} //~ ERROR PTX ABIs are experimental and subject to change
2019-10-25T17:16:07.4450363Z    |
2019-10-25T17:16:07.4450363Z    |
2019-10-25T17:16:07.4450844Z    = note: for more information, see ***/issues/38788
2019-10-25T17:16:07.4451055Z    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-10-25T17:16:07.4451187Z 
2019-10-25T17:16:07.4451906Z error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-10-25T17:16:07.4452434Z    |
2019-10-25T17:16:07.4452434Z    |
2019-10-25T17:16:07.4452749Z LL | extern "x86-interrupt" fn f7() {} //~ ERROR x86-interrupt ABI is experimental
2019-10-25T17:16:07.4453037Z    |
2019-10-25T17:16:07.4453037Z    |
2019-10-25T17:16:07.4453369Z    = note: for more information, see ***/issues/40180
2019-10-25T17:16:07.4453568Z    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-10-25T17:16:07.4453961Z error[E0658]: thiscall is experimental and subject to change
2019-10-25T17:16:07.4454258Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:22:1
2019-10-25T17:16:07.4456770Z    |
2019-10-25T17:16:07.4456770Z    |
2019-10-25T17:16:07.4456970Z LL | extern "thiscall" fn f8() {} //~ ERROR thiscall is experimental and subject to change
2019-10-25T17:16:07.4457384Z    |
2019-10-25T17:16:07.4457746Z    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-10-25T17:16:07.4458295Z 
2019-10-25T17:16:07.4458295Z 
2019-10-25T17:16:07.4458827Z error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-10-25T17:16:07.4459490Z    |
2019-10-25T17:16:07.4459490Z    |
2019-10-25T17:16:07.4459917Z LL | extern "amdgpu-kernel" fn f9() {} //~ ERROR amdgpu-kernel ABI is experimental and subject to change
2019-10-25T17:16:07.4460529Z    |
2019-10-25T17:16:07.4460529Z    |
2019-10-25T17:16:07.4461047Z    = note: for more information, see ***/issues/51575
2019-10-25T17:16:07.4461279Z    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-10-25T17:16:07.4461574Z 
2019-10-25T17:16:07.4461722Z error[E0658]: efiapi ABI is experimental and subject to change
2019-10-25T17:16:07.4463078Z    |
2019-10-25T17:16:07.4463078Z    |
2019-10-25T17:16:07.4463256Z LL | extern "efiapi" fn f10() {} //~ ERROR efiapi ABI is experimental and subject to change
2019-10-25T17:16:07.4463641Z    |
2019-10-25T17:16:07.4463641Z    |
2019-10-25T17:16:07.4464043Z    = note: for more information, see ***/issues/65815
2019-10-25T17:16:07.4464218Z    = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
2019-10-25T17:16:07.4464457Z error[E0658]: intrinsics are subject to change
2019-10-25T17:16:07.4464814Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:28:5
2019-10-25T17:16:07.4464968Z    |
2019-10-25T17:16:07.4464968Z    |
2019-10-25T17:16:07.4465425Z LL |     extern "rust-intrinsic" fn m1(); //~ ERROR intrinsics are subject to change
2019-10-25T17:16:07.4465729Z    |
2019-10-25T17:16:07.4465845Z    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-10-25T17:16:07.4465944Z 
2019-10-25T17:16:07.4466092Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-10-25T17:16:07.4466092Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-10-25T17:16:07.4466429Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:30:5
2019-10-25T17:16:07.4466585Z    |
2019-10-25T17:16:07.4467106Z LL |     extern "platform-intrinsic" fn m2(); //~ ERROR platform intrinsics are experimental
2019-10-25T17:16:07.4467385Z    |
2019-10-25T17:16:07.4467385Z    |
2019-10-25T17:16:07.4468198Z    = note: for more information, see ***/issues/27731
2019-10-25T17:16:07.4468809Z    = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
2019-10-25T17:16:07.4472642Z error[E0658]: vectorcall is experimental and subject to change
2019-10-25T17:16:07.4476825Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:32:5
2019-10-25T17:16:07.4477135Z    |
2019-10-25T17:16:07.4477135Z    |
2019-10-25T17:16:07.4477177Z LL |     extern "vectorcall" fn m3(); //~ ERROR vectorcall is experimental and subject to change
2019-10-25T17:16:07.4477246Z    |
2019-10-25T17:16:07.4477299Z    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-10-25T17:16:07.4477325Z 
2019-10-25T17:16:07.4477535Z error[E0658]: rust-call ABI is subject to change
2019-10-25T17:16:07.4477535Z error[E0658]: rust-call ABI is subject to change
2019-10-25T17:16:07.4477917Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:33:5
2019-10-25T17:16:07.4477955Z    |
2019-10-25T17:16:07.4478965Z LL |     extern "rust-call" fn m4(); //~ ERROR rust-call ABI is subject to change
2019-10-25T17:16:07.4479101Z    |
2019-10-25T17:16:07.4479101Z    |
2019-10-25T17:16:07.4485450Z    = note: for more information, see ***/issues/29625
2019-10-25T17:16:07.4485554Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-10-25T17:16:07.4485584Z 
2019-10-25T17:16:07.4485914Z error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-10-25T17:16:07.4486514Z    |
2019-10-25T17:16:07.4486514Z    |
2019-10-25T17:16:07.4486723Z LL |     extern "msp430-interrupt" fn m5(); //~ ERROR msp430-interrupt ABI is experimental
2019-10-25T17:16:07.4486818Z    |
2019-10-25T17:16:07.4486818Z    |
2019-10-25T17:16:07.4487038Z    = note: for more information, see ***/issues/38487
2019-10-25T17:16:07.4487085Z    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-10-25T17:16:07.4487129Z 
2019-10-25T17:16:07.4487341Z error[E0658]: PTX ABIs are experimental and subject to change
2019-10-25T17:16:07.4487630Z    |
2019-10-25T17:16:07.4487630Z    |
2019-10-25T17:16:07.4488365Z LL |     extern "ptx-kernel" fn m6(); //~ ERROR PTX ABIs are experimental and subject to change
2019-10-25T17:16:07.4488492Z    |
2019-10-25T17:16:07.4488492Z    |
2019-10-25T17:16:07.4488819Z    = note: for more information, see ***/issues/38788
2019-10-25T17:16:07.4488879Z    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-10-25T17:16:07.4488931Z 
2019-10-25T17:16:07.4489182Z error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-10-25T17:16:07.4489471Z    |
2019-10-25T17:16:07.4489471Z    |
2019-10-25T17:16:07.4490131Z LL |     extern "x86-interrupt" fn m7(); //~ ERROR x86-interrupt ABI is experimental
2019-10-25T17:16:07.4490254Z    |
2019-10-25T17:16:07.4490254Z    |
2019-10-25T17:16:07.4490574Z    = note: for more information, see ***/issues/40180
2019-10-25T17:16:07.4490632Z    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-10-25T17:16:07.4490732Z error[E0658]: thiscall is experimental and subject to change
2019-10-25T17:16:07.4490981Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:37:5
2019-10-25T17:16:07.4491028Z    |
2019-10-25T17:16:07.4491028Z    |
2019-10-25T17:16:07.4491098Z LL |     extern "thiscall" fn m8(); //~ ERROR thiscall is experimental and subject to change
2019-10-25T17:16:07.4491190Z    |
2019-10-25T17:16:07.4491256Z    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-10-25T17:16:07.4491293Z 
2019-10-25T17:16:07.4491293Z 
2019-10-25T17:16:07.4491750Z error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-10-25T17:16:07.4492285Z    |
2019-10-25T17:16:07.4492285Z    |
2019-10-25T17:16:07.4492518Z LL |     extern "amdgpu-kernel" fn m9(); //~ ERROR amdgpu-kernel ABI is experimental and subject to change
2019-10-25T17:16:07.4492606Z    |
2019-10-25T17:16:07.4492606Z    |
2019-10-25T17:16:07.4492993Z    = note: for more information, see ***/issues/51575
2019-10-25T17:16:07.4493038Z    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-10-25T17:16:07.4493082Z 
2019-10-25T17:16:07.4493117Z error[E0658]: efiapi ABI is experimental and subject to change
2019-10-25T17:16:07.4493366Z    |
2019-10-25T17:16:07.4493366Z    |
2019-10-25T17:16:07.4493404Z LL |     extern "efiapi" fn m10(); //~ ERROR efiapi ABI is experimental and subject to change
2019-10-25T17:16:07.4493492Z    |
2019-10-25T17:16:07.4493492Z    |
2019-10-25T17:16:07.4493707Z    = note: for more information, see ***/issues/65815
2019-10-25T17:16:07.4493757Z    = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
2019-10-25T17:16:07.4493878Z error[E0658]: vectorcall is experimental and subject to change
2019-10-25T17:16:07.4494080Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:41:5
2019-10-25T17:16:07.4494116Z    |
2019-10-25T17:16:07.4494116Z    |
2019-10-25T17:16:07.4494171Z LL |     extern "vectorcall" fn dm3() {} //~ ERROR vectorcall is experimental and subject to change
2019-10-25T17:16:07.4494242Z    |
2019-10-25T17:16:07.4494298Z    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-10-25T17:16:07.4494324Z 
2019-10-25T17:16:07.4494497Z error[E0658]: rust-call ABI is subject to change
2019-10-25T17:16:07.4494497Z error[E0658]: rust-call ABI is subject to change
2019-10-25T17:16:07.4494698Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:42:5
2019-10-25T17:16:07.4494735Z    |
2019-10-25T17:16:07.4494933Z LL |     extern "rust-call" fn dm4() {} //~ ERROR rust-call ABI is subject to change
2019-10-25T17:16:07.4495108Z    |
2019-10-25T17:16:07.4495108Z    |
2019-10-25T17:16:07.4495348Z    = note: for more information, see ***/issues/29625
2019-10-25T17:16:07.4495409Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-10-25T17:16:07.4495436Z 
2019-10-25T17:16:07.4495630Z error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-10-25T17:16:07.4495869Z    |
2019-10-25T17:16:07.4495869Z    |
2019-10-25T17:16:07.4496225Z LL |     extern "msp430-interrupt" fn dm5() {} //~ ERROR msp430-interrupt ABI is experimental
2019-10-25T17:16:07.4496312Z    |
2019-10-25T17:16:07.4496312Z    |
2019-10-25T17:16:07.4496510Z    = note: for more information, see ***/issues/38487
2019-10-25T17:16:07.4496559Z    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-10-25T17:16:07.4496607Z 
2019-10-25T17:16:07.4496640Z error[E0658]: PTX ABIs are experimental and subject to change
2019-10-25T17:16:07.4496876Z    |
2019-10-25T17:16:07.4496876Z    |
2019-10-25T17:16:07.4497070Z LL |     extern "ptx-kernel" fn dm6() {} //~ ERROR PTX ABIs are experimental and subject to change
2019-10-25T17:16:07.4497156Z    |
2019-10-25T17:16:07.4497156Z    |
2019-10-25T17:16:07.4497352Z    = note: for more information, see ***/issues/38788
2019-10-25T17:16:07.4497392Z    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-10-25T17:16:07.4497415Z 
2019-10-25T17:16:07.4497611Z error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-10-25T17:16:07.4498165Z    |
2019-10-25T17:16:07.4498165Z    |
2019-10-25T17:16:07.4498818Z LL |     extern "x86-interrupt" fn dm7() {} //~ ERROR x86-interrupt ABI is experimental
2019-10-25T17:16:07.4498934Z    |
2019-10-25T17:16:07.4498934Z    |
2019-10-25T17:16:07.4499263Z    = note: for more information, see ***/issues/40180
2019-10-25T17:16:07.4499322Z    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-10-25T17:16:07.4499421Z error[E0658]: thiscall is experimental and subject to change
2019-10-25T17:16:07.4499669Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:46:5
2019-10-25T17:16:07.4499716Z    |
2019-10-25T17:16:07.4499716Z    |
2019-10-25T17:16:07.4499784Z LL |     extern "thiscall" fn dm8() {} //~ ERROR thiscall is experimental and subject to change
2019-10-25T17:16:07.4499877Z    |
2019-10-25T17:16:07.4499941Z    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-10-25T17:16:07.4499977Z 
2019-10-25T17:16:07.4499977Z 
2019-10-25T17:16:07.4500253Z error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-10-25T17:16:07.4500600Z    |
2019-10-25T17:16:07.4500600Z    |
2019-10-25T17:16:07.4500895Z LL |     extern "amdgpu-kernel" fn dm9() {} //~ ERROR amdgpu-kernel ABI is experimental and subject to change
2019-10-25T17:16:07.4501017Z    |
2019-10-25T17:16:07.4501017Z    |
2019-10-25T17:16:07.4501314Z    = note: for more information, see ***/issues/51575
2019-10-25T17:16:07.4501374Z    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-10-25T17:16:07.4501429Z 
2019-10-25T17:16:07.4501476Z error[E0658]: efiapi ABI is experimental and subject to change
2019-10-25T17:16:07.4501991Z    |
2019-10-25T17:16:07.4501991Z    |
2019-10-25T17:16:07.4502029Z LL |     extern "efiapi" fn dm10() {} //~ ERROR efiapi ABI is experimental and subject to change
2019-10-25T17:16:07.4502182Z    |
2019-10-25T17:16:07.4502182Z    |
2019-10-25T17:16:07.4503236Z    = note: for more information, see ***/issues/65815
2019-10-25T17:16:07.4503290Z    = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
2019-10-25T17:16:07.4503368Z error[E0658]: intrinsics are subject to change
2019-10-25T17:16:07.4503610Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:55:5
2019-10-25T17:16:07.4503645Z    |
2019-10-25T17:16:07.4503645Z    |
2019-10-25T17:16:07.4503849Z LL |     extern "rust-intrinsic" fn m1() {} //~ ERROR intrinsics are subject to change
2019-10-25T17:16:07.4503920Z    |
2019-10-25T17:16:07.4503975Z    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-10-25T17:16:07.4503999Z 
2019-10-25T17:16:07.4504033Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-10-25T17:16:07.4504033Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-10-25T17:16:07.4504221Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:57:5
2019-10-25T17:16:07.4504279Z    |
2019-10-25T17:16:07.4504654Z LL |     extern "platform-intrinsic" fn m2() {} //~ ERROR platform intrinsics are experimental
2019-10-25T17:16:07.4504747Z    |
2019-10-25T17:16:07.4504747Z    |
2019-10-25T17:16:07.4504963Z    = note: for more information, see ***/issues/27731
2019-10-25T17:16:07.4505010Z    = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
2019-10-25T17:16:07.4505091Z error[E0658]: vectorcall is experimental and subject to change
2019-10-25T17:16:07.4505298Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:59:5
2019-10-25T17:16:07.4505354Z    |
2019-10-25T17:16:07.4505354Z    |
2019-10-25T17:16:07.4505394Z LL |     extern "vectorcall" fn m3() {} //~ ERROR vectorcall is experimental and subject to change
2019-10-25T17:16:07.4505485Z    |
2019-10-25T17:16:07.4505632Z    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-10-25T17:16:07.4505676Z 
2019-10-25T17:16:07.4505897Z error[E0658]: rust-call ABI is subject to change
2019-10-25T17:16:07.4505897Z error[E0658]: rust-call ABI is subject to change
2019-10-25T17:16:07.4506119Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:60:5
2019-10-25T17:16:07.4506157Z    |
2019-10-25T17:16:07.4506364Z LL |     extern "rust-call" fn m4() {} //~ ERROR rust-call ABI is subject to change
2019-10-25T17:16:07.4506458Z    |
2019-10-25T17:16:07.4506458Z    |
2019-10-25T17:16:07.4506743Z    = note: for more information, see ***/issues/29625
2019-10-25T17:16:07.4506808Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-10-25T17:16:07.4506836Z 
2019-10-25T17:16:07.4507050Z error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-10-25T17:16:07.4507307Z    |
2019-10-25T17:16:07.4507307Z    |
2019-10-25T17:16:07.4507531Z LL |     extern "msp430-interrupt" fn m5() {} //~ ERROR msp430-interrupt ABI is experimental
2019-10-25T17:16:07.4507633Z    |
2019-10-25T17:16:07.4507633Z    |
2019-10-25T17:16:07.4508752Z    = note: for more information, see ***/issues/38487
2019-10-25T17:16:07.4508843Z    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-10-25T17:16:07.4508877Z 
2019-10-25T17:16:07.4508924Z error[E0658]: PTX ABIs are experimental and subject to change
2019-10-25T17:16:07.4509281Z    |
2019-10-25T17:16:07.4509281Z    |
2019-10-25T17:16:07.4509548Z LL |     extern "ptx-kernel" fn m6() {} //~ ERROR PTX ABIs are experimental and subject to change
2019-10-25T17:16:07.4509664Z    |
2019-10-25T17:16:07.4509664Z    |
2019-10-25T17:16:07.4509934Z    = note: for more information, see ***/issues/38788
2019-10-25T17:16:07.4510005Z    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-10-25T17:16:07.4510181Z 
2019-10-25T17:16:07.4510478Z error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-10-25T17:16:07.4510809Z    |
2019-10-25T17:16:07.4510809Z    |
2019-10-25T17:16:07.4511082Z LL |     extern "x86-interrupt" fn m7() {} //~ ERROR x86-interrupt ABI is experimental
2019-10-25T17:16:07.4511200Z    |
2019-10-25T17:16:07.4511200Z    |
2019-10-25T17:16:07.4511497Z    = note: for more information, see ***/issues/40180
2019-10-25T17:16:07.4511558Z    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-10-25T17:16:07.4511660Z error[E0658]: thiscall is experimental and subject to change
2019-10-25T17:16:07.4512052Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:64:5
2019-10-25T17:16:07.4512087Z    |
2019-10-25T17:16:07.4512087Z    |
2019-10-25T17:16:07.4512150Z LL |     extern "thiscall" fn m8() {} //~ ERROR thiscall is experimental and subject to change
2019-10-25T17:16:07.4512223Z    |
2019-10-25T17:16:07.4512278Z    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-10-25T17:16:07.4512302Z 
2019-10-25T17:16:07.4512302Z 
2019-10-25T17:16:07.4512483Z error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-10-25T17:16:07.4512712Z    |
2019-10-25T17:16:07.4512712Z    |
2019-10-25T17:16:07.4512912Z LL |     extern "amdgpu-kernel" fn m9() {} //~ ERROR amdgpu-kernel ABI is experimental and subject to change
2019-10-25T17:16:07.4513002Z    |
2019-10-25T17:16:07.4513002Z    |
2019-10-25T17:16:07.4513201Z    = note: for more information, see ***/issues/51575
2019-10-25T17:16:07.4513260Z    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-10-25T17:16:07.4513285Z 
2019-10-25T17:16:07.4513387Z error[E0658]: efiapi ABI is experimental and subject to change
2019-10-25T17:16:07.4513661Z    |
2019-10-25T17:16:07.4513661Z    |
2019-10-25T17:16:07.4513698Z LL |     extern "efiapi" fn m10() {} //~ ERROR efiapi ABI is experimental and subject to change
2019-10-25T17:16:07.4513953Z    |
2019-10-25T17:16:07.4513953Z    |
2019-10-25T17:16:07.4514167Z    = note: for more information, see ***/issues/65815
2019-10-25T17:16:07.4514211Z    = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
2019-10-25T17:16:07.4514287Z error[E0658]: intrinsics are subject to change
2019-10-25T17:16:07.4514477Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:71:5
2019-10-25T17:16:07.4514532Z    |
2019-10-25T17:16:07.4514532Z    |
2019-10-25T17:16:07.4514729Z LL |     extern "rust-intrinsic" fn im1() {} //~ ERROR intrinsics are subject to change
2019-10-25T17:16:07.4514815Z    |
2019-10-25T17:16:07.4514870Z    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-10-25T17:16:07.4514895Z 
2019-10-25T17:16:07.4514930Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-10-25T17:16:07.4514930Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-10-25T17:16:07.4515136Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:73:5
2019-10-25T17:16:07.4515172Z    |
2019-10-25T17:16:07.4515375Z LL |     extern "platform-intrinsic" fn im2() {} //~ ERROR platform intrinsics are experimental
2019-10-25T17:16:07.4515467Z    |
2019-10-25T17:16:07.4515467Z    |
2019-10-25T17:16:07.4515678Z    = note: for more information, see ***/issues/27731
2019-10-25T17:16:07.4515741Z    = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
2019-10-25T17:16:07.4515802Z error[E0658]: vectorcall is experimental and subject to change
2019-10-25T17:16:07.4515999Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:75:5
2019-10-25T17:16:07.4516124Z    |
2019-10-25T17:16:07.4516124Z    |
2019-10-25T17:16:07.4516163Z LL |     extern "vectorcall" fn im3() {} //~ ERROR vectorcall is experimental and subject to change
2019-10-25T17:16:07.4516251Z    |
2019-10-25T17:16:07.4516288Z    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-10-25T17:16:07.4516314Z 
2019-10-25T17:16:07.4516528Z error[E0658]: rust-call ABI is subject to change
2019-10-25T17:16:07.4516528Z error[E0658]: rust-call ABI is subject to change
2019-10-25T17:16:07.4516718Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:76:5
2019-10-25T17:16:07.4516754Z    |
2019-10-25T17:16:07.4516947Z LL |     extern "rust-call" fn im4() {} //~ ERROR rust-call ABI is subject to change
2019-10-25T17:16:07.4517037Z    |
2019-10-25T17:16:07.4517037Z    |
2019-10-25T17:16:07.4517411Z    = note: for more information, see ***/issues/29625
2019-10-25T17:16:07.4517460Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-10-25T17:16:07.4517489Z 
2019-10-25T17:16:07.4517857Z error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-10-25T17:16:07.4518487Z    |
2019-10-25T17:16:07.4518487Z    |
2019-10-25T17:16:07.4518751Z LL |     extern "msp430-interrupt" fn im5() {} //~ ERROR msp430-interrupt ABI is experimental
2019-10-25T17:16:07.4518870Z    |
2019-10-25T17:16:07.4518870Z    |
2019-10-25T17:16:07.4519143Z    = note: for more information, see ***/issues/38487
2019-10-25T17:16:07.4519219Z    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-10-25T17:16:07.4519252Z 
2019-10-25T17:16:07.4519297Z error[E0658]: PTX ABIs are experimental and subject to change
2019-10-25T17:16:07.4519605Z    |
2019-10-25T17:16:07.4519605Z    |
2019-10-25T17:16:07.4519960Z LL |     extern "ptx-kernel" fn im6() {} //~ ERROR PTX ABIs are experimental and subject to change
2019-10-25T17:16:07.4520098Z    |
2019-10-25T17:16:07.4520098Z    |
2019-10-25T17:16:07.4520402Z    = note: for more information, see ***/issues/38788
2019-10-25T17:16:07.4520477Z    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-10-25T17:16:07.4520509Z 
2019-10-25T17:16:07.4520756Z error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-10-25T17:16:07.4521059Z    |
2019-10-25T17:16:07.4521059Z    |
2019-10-25T17:16:07.4521315Z LL |     extern "x86-interrupt" fn im7() {} //~ ERROR x86-interrupt ABI is experimental
2019-10-25T17:16:07.4521588Z    |
2019-10-25T17:16:07.4521588Z    |
2019-10-25T17:16:07.4521980Z    = note: for more information, see ***/issues/40180
2019-10-25T17:16:07.4522021Z    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-10-25T17:16:07.4522108Z error[E0658]: thiscall is experimental and subject to change
2019-10-25T17:16:07.4522292Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:80:5
2019-10-25T17:16:07.4522523Z    |
2019-10-25T17:16:07.4522523Z    |
2019-10-25T17:16:07.4522561Z LL |     extern "thiscall" fn im8() {} //~ ERROR thiscall is experimental and subject to change
2019-10-25T17:16:07.4522649Z    |
2019-10-25T17:16:07.4522686Z    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-10-25T17:16:07.4522712Z 
2019-10-25T17:16:07.4522712Z 
2019-10-25T17:16:07.4522913Z error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-10-25T17:16:07.4523172Z    |
2019-10-25T17:16:07.4523172Z    |
2019-10-25T17:16:07.4523397Z LL |     extern "amdgpu-kernel" fn im9() {} //~ ERROR amdgpu-kernel ABI is experimental and subject to change
2019-10-25T17:16:07.4523579Z    |
2019-10-25T17:16:07.4523579Z    |
2019-10-25T17:16:07.4523835Z    = note: for more information, see ***/issues/51575
2019-10-25T17:16:07.4523901Z    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-10-25T17:16:07.4523928Z 
2019-10-25T17:16:07.4523965Z error[E0658]: efiapi ABI is experimental and subject to change
2019-10-25T17:16:07.4524226Z    |
2019-10-25T17:16:07.4524226Z    |
2019-10-25T17:16:07.4524265Z LL |     extern "efiapi" fn im10() {} //~ ERROR efiapi ABI is experimental and subject to change
2019-10-25T17:16:07.4524533Z    |
2019-10-25T17:16:07.4524533Z    |
2019-10-25T17:16:07.4524751Z    = note: for more information, see ***/issues/65815
2019-10-25T17:16:07.4524812Z    = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
2019-10-25T17:16:07.4524880Z error[E0658]: intrinsics are subject to change
---
2019-10-25T17:16:07.4640027Z 
2019-10-25T17:16:07.4640246Z ---- [ui] ui/symbol-names/impl1.rs#legacy stdout ----
2019-10-25T17:16:07.4640306Z diff of stderr:
2019-10-25T17:16:07.4640334Z 
2019-10-25T17:16:07.4640572Z - error: symbol-name(_ZN5impl13foo3Foo3bar17he53b9bee7600ed8dE)
2019-10-25T17:16:07.4640836Z + error: symbol-name(_ZN5impl13foo3Foo3bar17h92cf46db76791039E)
2019-10-25T17:16:07.4641091Z 3    |
2019-10-25T17:16:07.4641136Z 4 LL |         #[rustc_symbol_name]
2019-10-25T17:16:07.4641184Z 
2019-10-25T17:16:07.4641226Z 5    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4641226Z 5    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4641267Z 6 
2019-10-25T17:16:07.4641520Z - error: demangling(impl1::foo::Foo::bar::he53b9bee7600ed8d)
2019-10-25T17:16:07.4641915Z + error: demangling(impl1::foo::Foo::bar::h92cf46db76791039)
2019-10-25T17:16:07.4642341Z 9    |
2019-10-25T17:16:07.4642392Z 10 LL |         #[rustc_symbol_name]
2019-10-25T17:16:07.4642414Z 
2019-10-25T17:16:07.4642447Z 22 LL |         #[rustc_def_path]
2019-10-25T17:16:07.4642447Z 22 LL |         #[rustc_def_path]
2019-10-25T17:16:07.4642497Z 23    |         ^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4642529Z 24 
2019-10-25T17:16:07.4642745Z - error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h86c41f0462d901d4E)
2019-10-25T17:16:07.4642978Z + error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h90c4a800b1aa0df0E)
2019-10-25T17:16:07.4643178Z 27    |
2019-10-25T17:16:07.4643224Z 28 LL |         #[rustc_symbol_name]
2019-10-25T17:16:07.4643248Z 
2019-10-25T17:16:07.4643280Z 29    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4643280Z 29    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4643311Z 30 
2019-10-25T17:16:07.4643526Z - error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h86c41f0462d901d4)
2019-10-25T17:16:07.4643583Z + error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h90c4a800b1aa0df0)
2019-10-25T17:16:07.4643792Z 33    |
2019-10-25T17:16:07.4643826Z 34 LL |         #[rustc_symbol_name]
2019-10-25T17:16:07.4643848Z 
2019-10-25T17:16:07.4643880Z 46 LL |         #[rustc_def_path]
2019-10-25T17:16:07.4643880Z 46 LL |         #[rustc_def_path]
2019-10-25T17:16:07.4643927Z 47    |         ^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4643960Z 48 
2019-10-25T17:16:07.4644315Z - error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$_$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17h636bc933fc62ee2fE)
2019-10-25T17:16:07.4644698Z + error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$_$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17h61b0fcb05ebeeb79E)
2019-10-25T17:16:07.4645184Z 51    |
2019-10-25T17:16:07.4645216Z 52 LL |             #[rustc_symbol_name]
2019-10-25T17:16:07.4645238Z 
2019-10-25T17:16:07.4645268Z 53    |             ^^^^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4645268Z 53    |             ^^^^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4645316Z 54 
2019-10-25T17:16:07.4645556Z - error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; _] as impl1::main::{{closure}}::Bar>::method::h636bc933fc62ee2f)
2019-10-25T17:16:07.4645607Z + error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; _] as impl1::main::{{closure}}::Bar>::method::h61b0fcb05ebeeb79)
2019-10-25T17:16:07.4645810Z 57    |
2019-10-25T17:16:07.4645841Z 58 LL |             #[rustc_symbol_name]
2019-10-25T17:16:07.4645876Z 
2019-10-25T17:16:07.4645895Z 
2019-10-25T17:16:07.4645895Z 
2019-10-25T17:16:07.4645927Z The actual stderr differed from the expected stderr.
2019-10-25T17:16:07.4646151Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy/impl1.legacy.stderr
2019-10-25T17:16:07.4646350Z To update references, rerun the tests and pass the `--bless` flag
2019-10-25T17:16:07.4646535Z To only update this specific test, also pass `--test-args symbol-names/impl1.rs`
2019-10-25T17:16:07.4646561Z 
2019-10-25T17:16:07.4646594Z error in revision `legacy`: 1 errors occurred comparing output.
2019-10-25T17:16:07.4646642Z status: exit code: 1
2019-10-25T17:16:07.4647259Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/impl1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy/auxiliary" "-A" "unused"
2019-10-25T17:16:07.4647547Z ------------------------------------------
2019-10-25T17:16:07.4647574Z 
2019-10-25T17:16:07.4647934Z ------------------------------------------
2019-10-25T17:16:07.4647970Z stderr:
2019-10-25T17:16:07.4647970Z stderr:
2019-10-25T17:16:07.4648516Z ------------------------------------------
2019-10-25T17:16:07.4648785Z error: symbol-name(_ZN5impl13foo3Foo3bar17h92cf46db76791039E)
2019-10-25T17:16:07.4649068Z    |
2019-10-25T17:16:07.4649126Z LL |         #[rustc_symbol_name]
2019-10-25T17:16:07.4649171Z    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4649201Z 
2019-10-25T17:16:07.4649201Z 
2019-10-25T17:16:07.4649246Z error: demangling(impl1::foo::Foo::bar::h92cf46db76791039)
2019-10-25T17:16:07.4649534Z    |
2019-10-25T17:16:07.4649586Z LL |         #[rustc_symbol_name]
2019-10-25T17:16:07.4649648Z    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4649677Z 
2019-10-25T17:16:07.4649677Z 
2019-10-25T17:16:07.4649896Z error: demangling-alt(impl1::foo::Foo::bar)
2019-10-25T17:16:07.4650183Z    |
2019-10-25T17:16:07.4650225Z LL |         #[rustc_symbol_name]
2019-10-25T17:16:07.4650269Z    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4650309Z 
2019-10-25T17:16:07.4650309Z 
2019-10-25T17:16:07.4650512Z error: def-path(foo::Foo::bar)
2019-10-25T17:16:07.4650784Z    |
2019-10-25T17:16:07.4650842Z LL |         #[rustc_def_path]
2019-10-25T17:16:07.4650885Z    |         ^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4650913Z 
2019-10-25T17:16:07.4650913Z 
2019-10-25T17:16:07.4651199Z error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h90c4a800b1aa0df0E)
2019-10-25T17:16:07.4651746Z    |
2019-10-25T17:16:07.4651795Z LL |         #[rustc_symbol_name]
2019-10-25T17:16:07.4651826Z    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4651847Z 
2019-10-25T17:16:07.4651847Z 
2019-10-25T17:16:07.4651881Z error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h90c4a800b1aa0df0)
2019-10-25T17:16:07.4652128Z    |
2019-10-25T17:16:07.4652158Z LL |         #[rustc_symbol_name]
2019-10-25T17:16:07.4652206Z    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4652227Z 
2019-10-25T17:16:07.4652227Z 
2019-10-25T17:16:07.4652396Z error: demangling-alt(impl1::bar::<impl impl1::foo::Foo>::baz)
2019-10-25T17:16:07.4652610Z    |
2019-10-25T17:16:07.4652641Z LL |         #[rustc_symbol_name]
2019-10-25T17:16:07.4652672Z    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4652693Z 
2019-10-25T17:16:07.4652693Z 
2019-10-25T17:16:07.4652865Z error: def-path(bar::<impl foo::Foo>::baz)
2019-10-25T17:16:07.4653078Z    |
2019-10-25T17:16:07.4653119Z LL |         #[rustc_def_path]
2019-10-25T17:16:07.4653150Z    |         ^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4653171Z 
2019-10-25T17:16:07.4653171Z 
2019-10-25T17:16:07.4653500Z error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$_$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17h61b0fcb05ebeeb79E)
2019-10-25T17:16:07.4653734Z    |
2019-10-25T17:16:07.4653779Z LL |             #[rustc_symbol_name]
2019-10-25T17:16:07.4653811Z    |             ^^^^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4653832Z 
2019-10-25T17:16:07.4653832Z 
2019-10-25T17:16:07.4653872Z error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; _] as impl1::main::{{closure}}::Bar>::method::h61b0fcb05ebeeb79)
2019-10-25T17:16:07.4654170Z    |
2019-10-25T17:16:07.4654200Z LL |             #[rustc_symbol_name]
2019-10-25T17:16:07.4654247Z    |             ^^^^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4654268Z 
2019-10-25T17:16:07.4654268Z 
2019-10-25T17:16:07.4654514Z error: demangling-alt(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; _] as impl1::main::{{closure}}::Bar>::method)
2019-10-25T17:16:07.4654729Z    |
2019-10-25T17:16:07.4654759Z LL |             #[rustc_symbol_name]
2019-10-25T17:16:07.4654790Z    |             ^^^^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4654818Z 
2019-10-25T17:16:07.4654818Z 
2019-10-25T17:16:07.4655032Z error: def-path(<[&dyn Foo<Assoc = for<'r> extern "C" fn(&'r u8, ...)> + AutoTrait; _] as main::{{closure}}#1::Bar>::method)
2019-10-25T17:16:07.4655253Z    |
2019-10-25T17:16:07.4655290Z LL |             #[rustc_def_path]
2019-10-25T17:16:07.4655327Z    |             ^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4655348Z 
---
2019-10-25T17:16:07.4655636Z 
2019-10-25T17:16:07.4655804Z ---- [ui] ui/symbol-names/issue-60925.rs#legacy stdout ----
2019-10-25T17:16:07.4655839Z diff of stderr:
2019-10-25T17:16:07.4655870Z 
2019-10-25T17:16:07.4656078Z - error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17h059a991a004536adE)
2019-10-25T17:16:07.4656292Z + error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17hc86312d25b60f6eeE)
2019-10-25T17:16:07.4656462Z 2   --> $DIR/issue-60925.rs:21:9
2019-10-25T17:16:07.4656527Z 4 LL |         #[rustc_symbol_name]
2019-10-25T17:16:07.4656548Z 
2019-10-25T17:16:07.4656593Z 5    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4656689Z 6 
2019-10-25T17:16:07.4656689Z 6 
2019-10-25T17:16:07.4657677Z - error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::h059a991a004536ad)
2019-10-25T17:16:07.4657923Z + error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::hc86312d25b60f6ee)
2019-10-25T17:16:07.4658651Z 8   --> $DIR/issue-60925.rs:21:9
2019-10-25T17:16:07.4658752Z 10 LL |         #[rustc_symbol_name]
2019-10-25T17:16:07.4658793Z 
2019-10-25T17:16:07.4658818Z 
2019-10-25T17:16:07.4658863Z The actual stderr differed from the expected stderr.
2019-10-25T17:16:07.4658863Z The actual stderr differed from the expected stderr.
2019-10-25T17:16:07.4659175Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy/issue-60925.legacy.stderr
2019-10-25T17:16:07.4659435Z To update references, rerun the tests and pass the `--bless` flag
2019-10-25T17:16:07.4659739Z To only update this specific test, also pass `--test-args symbol-names/issue-60925.rs`
2019-10-25T17:16:07.4659779Z 
2019-10-25T17:16:07.4659866Z error in revision `legacy`: 1 errors occurred comparing output.
2019-10-25T17:16:07.4659914Z status: exit code: 1
2019-10-25T17:16:07.4661791Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/issue-60925.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy/auxiliary" "-A" "unused"
2019-10-25T17:16:07.4662127Z ------------------------------------------
2019-10-25T17:16:07.4662167Z 
2019-10-25T17:16:07.4662338Z ------------------------------------------
2019-10-25T17:16:07.4663392Z stderr:
2019-10-25T17:16:07.4663392Z stderr:
2019-10-25T17:16:07.4663672Z ------------------------------------------
2019-10-25T17:16:07.4663897Z error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17hc86312d25b60f6eeE)
2019-10-25T17:16:07.4664086Z   --> /checkout/src/test/ui/symbol-names/issue-60925.rs:21:9
2019-10-25T17:16:07.4664169Z LL |         #[rustc_symbol_name]
2019-10-25T17:16:07.4664204Z    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4664227Z 
2019-10-25T17:16:07.4664227Z 
2019-10-25T17:16:07.4664270Z error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::hc86312d25b60f6ee)
2019-10-25T17:16:07.4664458Z   --> /checkout/src/test/ui/symbol-names/issue-60925.rs:21:9
2019-10-25T17:16:07.4664535Z LL |         #[rustc_symbol_name]
2019-10-25T17:16:07.4664568Z    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4664590Z 
2019-10-25T17:16:07.4664590Z 
2019-10-25T17:16:07.4664781Z error: demangling-alt(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo)
2019-10-25T17:16:07.4664993Z   --> /checkout/src/test/ui/symbol-names/issue-60925.rs:21:9
2019-10-25T17:16:07.4665061Z LL |         #[rustc_symbol_name]
2019-10-25T17:16:07.4665111Z    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T17:16:07.4665133Z 
2019-10-25T17:16:07.4665166Z error: aborting due to 3 previous errors
---
2019-10-25T17:16:07.4666903Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-25T17:16:07.4666945Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-25T17:16:07.4666970Z 
2019-10-25T17:16:07.4666995Z 
2019-10-25T17:16:07.4668926Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-25T17:16:07.4669194Z 
2019-10-25T17:16:07.4669224Z 
2019-10-25T17:16:07.4669318Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-25T17:16:07.4669388Z Build completed unsuccessfully in 1:00:47
2019-10-25T17:16:07.4669388Z Build completed unsuccessfully in 1:00:47
2019-10-25T17:16:07.4669435Z == clock drift check ==
2019-10-25T17:16:07.4669599Z   local time: Fri Oct 25 17:16:07 UTC 2019
2019-10-25T17:16:07.4765971Z   network time: Fri, 25 Oct 2019 17:16:07 GMT
2019-10-25T17:16:07.4766603Z == end clock drift check ==
2019-10-25T17:16:08.7231111Z 
2019-10-25T17:16:08.7323212Z ##[error]Bash exited with code '1'.
2019-10-25T17:16:08.7362808Z ##[section]Starting: Checkout
2019-10-25T17:16:08.7364566Z ==============================================================================
2019-10-25T17:16:08.7364642Z Task         : Get sources
2019-10-25T17:16:08.7364691Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
