plain
2019-09-12T15:37:54.4868528Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-12T15:37:54.5074588Z ##[command]git config gc.auto 0
2019-09-12T15:37:54.5161955Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-12T15:37:54.5221226Z ##[command]git config --get-all http.proxy
2019-09-12T15:37:54.5361953Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64406/merge:refs/remotes/pull/64406/merge
---
2019-09-12T16:40:12.3344307Z .................................................................................................... 1500/9011
2019-09-12T16:40:18.1821218Z .................................................................................................... 1600/9011
2019-09-12T16:40:30.6836642Z .........................................................i...............i.......................... 1700/9011
2019-09-12T16:40:38.4467790Z .................................................................................................... 1800/9011
2019-09-12T16:40:53.2363778Z ................................................iiiii............................................... 1900/9011
2019-09-12T16:41:04.3107376Z .................................................................................................... 2100/9011
2019-09-12T16:41:06.8320059Z .................................................................................................... 2200/9011
2019-09-12T16:41:10.4397874Z .................................................................................................... 2300/9011
2019-09-12T16:41:18.3685836Z .................................................................................................... 2400/9011
---
2019-09-12T16:44:16.2803990Z ....................................i...............i............................................... 4700/9011
2019-09-12T16:44:27.5090917Z .................................................................................................... 4800/9011
2019-09-12T16:44:34.2540162Z .................................................................................................... 4900/9011
2019-09-12T16:44:44.9935277Z .................................................................................................... 5000/9011
2019-09-12T16:44:51.1949333Z ...................ii.ii............................................................................ 5100/9011
2019-09-12T16:45:01.7068098Z .................................................................................................... 5300/9011
2019-09-12T16:45:12.0862092Z ..................................................................................i................. 5400/9011
2019-09-12T16:45:19.9707805Z .................................................................................................... 5500/9011
2019-09-12T16:45:25.6659181Z .................................................................................................... 5600/9011
2019-09-12T16:45:25.6659181Z .................................................................................................... 5600/9011
2019-09-12T16:45:35.9310903Z .............................................................................ii...i..ii...........i. 5700/9011
2019-09-12T16:46:01.3767194Z .................................................................................................... 5900/9011
2019-09-12T16:46:11.1342223Z .................................................................................................... 6000/9011
2019-09-12T16:46:11.1342223Z .................................................................................................... 6000/9011
2019-09-12T16:46:17.2824096Z ...............................................................................i..ii................ 6100/9011
2019-09-12T16:46:46.9681811Z .................................................................................................... 6300/9011
2019-09-12T16:46:49.1627886Z ......................................i............................................................. 6400/9011
2019-09-12T16:46:51.3365635Z .................................................................................................... 6500/9011
2019-09-12T16:46:53.9060285Z ..........i......................................................................................... 6600/9011
---
2019-09-12T16:50:58.1398923Z 
2019-09-12T16:50:58.1400106Z ---- [ui] ui/feature-gates/feature-gate-abi.rs stdout ----
2019-09-12T16:50:58.1400743Z diff of stderr:
2019-09-12T16:50:58.1401106Z 
2019-09-12T16:50:58.1401738Z 544    = note: for more information, see ***/issues/51575
2019-09-12T16:50:58.1402232Z 545    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-09-12T16:50:58.1403006Z - error: aborting due to 63 previous errors
2019-09-12T16:50:58.1403006Z - error: aborting due to 63 previous errors
2019-09-12T16:50:58.1403692Z + error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
2019-09-12T16:50:58.1404187Z +   --> $DIR/feature-gate-abi.rs:24:32
2019-09-12T16:50:58.1404448Z +    |
2019-09-12T16:50:58.1404977Z + LL |     extern "rust-intrinsic" fn m1();
2019-09-12T16:50:58.1405529Z + 
2019-09-12T16:50:58.1405529Z + 
2019-09-12T16:50:58.1406146Z + error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
2019-09-12T16:50:58.1406858Z +   --> $DIR/feature-gate-abi.rs:25:36
2019-09-12T16:50:58.1407312Z +    |
2019-09-12T16:50:58.1408138Z + LL |     extern "platform-intrinsic" fn m2();
2019-09-12T16:50:58.1408747Z + 
2019-09-12T16:50:58.1408747Z + 
2019-09-12T16:50:58.1409241Z + error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
2019-09-12T16:50:58.1409789Z +   --> $DIR/feature-gate-abi.rs:34:32
2019-09-12T16:50:58.1410075Z +    |
2019-09-12T16:50:58.1410536Z + LL |     extern "rust-intrinsic" fn dm1() {}
2019-09-12T16:50:58.1411244Z + 
2019-09-12T16:50:58.1411244Z + 
2019-09-12T16:50:58.1412044Z + error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
2019-09-12T16:50:58.1412498Z +   --> $DIR/feature-gate-abi.rs:35:36
2019-09-12T16:50:58.1412752Z +    |
2019-09-12T16:50:58.1414192Z + LL |     extern "platform-intrinsic" fn dm2() {}
2019-09-12T16:50:58.1414606Z + 
2019-09-12T16:50:58.1414606Z + 
2019-09-12T16:50:58.1415058Z + error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
2019-09-12T16:50:58.1415590Z +   --> $DIR/feature-gate-abi.rs:12:33
2019-09-12T16:50:58.1416170Z +    |
2019-09-12T16:50:58.1416590Z + LL | extern "rust-intrinsic" fn f1() {}
2019-09-12T16:50:58.1416936Z + 
2019-09-12T16:50:58.1416936Z + 
2019-09-12T16:50:58.1417332Z + error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
2019-09-12T16:50:58.1417707Z +   --> $DIR/feature-gate-abi.rs:13:37
2019-09-12T16:50:58.1417898Z +    |
2019-09-12T16:50:58.1418271Z + LL | extern "platform-intrinsic" fn f2() {}
2019-09-12T16:50:58.1418622Z + 
2019-09-12T16:50:58.1418622Z + 
2019-09-12T16:50:58.1418990Z + error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
2019-09-12T16:50:58.1419626Z +   --> $DIR/feature-gate-abi.rs:34:38
2019-09-12T16:50:58.1419805Z +    |
2019-09-12T16:50:58.1420120Z + LL |     extern "rust-intrinsic" fn dm1() {}
2019-09-12T16:50:58.1420433Z + 
2019-09-12T16:50:58.1420433Z + 
2019-09-12T16:50:58.1420763Z + error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
2019-09-12T16:50:58.1421275Z +   --> $DIR/feature-gate-abi.rs:35:42
2019-09-12T16:50:58.1421625Z +    |
2019-09-12T16:50:58.1421954Z + LL |     extern "platform-intrinsic" fn dm2() {}
2019-09-12T16:50:58.1422277Z + 
2019-09-12T16:50:58.1422277Z + 
2019-09-12T16:50:58.1422621Z + error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
2019-09-12T16:50:58.1422958Z +   --> $DIR/feature-gate-abi.rs:49:37
2019-09-12T16:50:58.1423137Z +    |
2019-09-12T16:50:58.1423451Z + LL |     extern "rust-intrinsic" fn m1() {}
2019-09-12T16:50:58.1423765Z + 
2019-09-12T16:50:58.1423765Z + 
2019-09-12T16:50:58.1424095Z + error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
2019-09-12T16:50:58.1424447Z +   --> $DIR/feature-gate-abi.rs:50:41
2019-09-12T16:50:58.1424734Z +    |
2019-09-12T16:50:58.1425385Z + LL |     extern "platform-intrinsic" fn m2() {}
2019-09-12T16:50:58.1426348Z + 
2019-09-12T16:50:58.1426348Z + 
2019-09-12T16:50:58.1426956Z + error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
2019-09-12T16:50:58.1429544Z +   --> $DIR/feature-gate-abi.rs:62:38
2019-09-12T16:50:58.1430026Z +    |
2019-09-12T16:50:58.1430442Z + LL |     extern "rust-intrinsic" fn im1() {}
2019-09-12T16:50:58.1430993Z + 
2019-09-12T16:50:58.1430993Z + 
2019-09-12T16:50:58.1431549Z + error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
2019-09-12T16:50:58.1431935Z +   --> $DIR/feature-gate-abi.rs:63:42
2019-09-12T16:50:58.1432101Z +    |
2019-09-12T16:50:58.1432446Z + LL |     extern "platform-intrinsic" fn im2() {}
2019-09-12T16:50:58.1432749Z + 
2019-09-12T16:50:58.1432900Z + error: aborting due to 75 previous errors
2019-09-12T16:50:58.1433037Z 548 
2019-09-12T16:50:58.1433544Z 549 For more information about this error, try `rustc --explain E0658`.
2019-09-12T16:50:58.1433544Z 549 For more information about this error, try `rustc --explain E0658`.
2019-09-12T16:50:58.1433734Z 550 
2019-09-12T16:50:58.1433849Z 
2019-09-12T16:50:58.1433962Z 
2019-09-12T16:50:58.1434107Z The actual stderr differed from the expected stderr.
2019-09-12T16:50:58.1434538Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi/feature-gate-abi.stderr
2019-09-12T16:50:58.1434924Z To update references, rerun the tests and pass the `--bless` flag
2019-09-12T16:50:58.1435378Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-abi.rs`
2019-09-12T16:50:58.1435689Z error: 1 errors occurred comparing output.
2019-09-12T16:50:58.1436243Z status: exit code: 1
2019-09-12T16:50:58.1436243Z status: exit code: 1
2019-09-12T16:50:58.1437196Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-abi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi/auxiliary" "-A" "unused"
2019-09-12T16:50:58.1437821Z ------------------------------------------
2019-09-12T16:50:58.1438001Z 
2019-09-12T16:50:58.1438354Z ------------------------------------------
2019-09-12T16:50:58.1438530Z stderr:
2019-09-12T16:50:58.1438530Z stderr:
2019-09-12T16:50:58.1438999Z ------------------------------------------
2019-09-12T16:50:58.1439189Z error[E0658]: intrinsics are subject to change
2019-09-12T16:50:58.1439702Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:12:1
2019-09-12T16:50:58.1439889Z    |
2019-09-12T16:50:58.1440244Z LL | extern "rust-intrinsic" fn f1() {} //~ ERROR intrinsics are subject to change
2019-09-12T16:50:58.1440557Z    |
2019-09-12T16:50:58.1440557Z    |
2019-09-12T16:50:58.1440695Z    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-09-12T16:50:58.1441008Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-12T16:50:58.1441379Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:13:1
2019-09-12T16:50:58.1441560Z    |
2019-09-12T16:50:58.1441560Z    |
2019-09-12T16:50:58.1441941Z LL | extern "platform-intrinsic" fn f2() {} //~ ERROR platform intrinsics are experimental
2019-09-12T16:50:58.1442289Z    |
2019-09-12T16:50:58.1442289Z    |
2019-09-12T16:50:58.1442932Z    = note: for more information, see ***/issues/27731
2019-09-12T16:50:58.1443138Z    = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
2019-09-12T16:50:58.1443419Z error[E0658]: vectorcall is experimental and subject to change
2019-09-12T16:50:58.1443791Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:14:1
2019-09-12T16:50:58.1447790Z    |
2019-09-12T16:50:58.1447790Z    |
2019-09-12T16:50:58.1451198Z LL | extern "vectorcall" fn f3() {} //~ ERROR vectorcall is experimental and subject to change
2019-09-12T16:50:58.1451332Z    |
2019-09-12T16:50:58.1451376Z    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-12T16:50:58.1451476Z 
2019-09-12T16:50:58.1452103Z error[E0658]: rust-call ABI is subject to change
2019-09-12T16:50:58.1452103Z error[E0658]: rust-call ABI is subject to change
2019-09-12T16:50:58.1457839Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:15:1
2019-09-12T16:50:58.1457940Z    |
2019-09-12T16:50:58.1458266Z LL | extern "rust-call" fn f4() {} //~ ERROR rust-call ABI is subject to change
2019-09-12T16:50:58.1458366Z    |
2019-09-12T16:50:58.1458366Z    |
2019-09-12T16:50:58.1458733Z    = note: for more information, see ***/issues/29625
2019-09-12T16:50:58.1458794Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-09-12T16:50:58.1458987Z 
2019-09-12T16:50:58.1459232Z error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-09-12T16:50:58.1459548Z    |
2019-09-12T16:50:58.1459548Z    |
2019-09-12T16:50:58.1459789Z LL | extern "msp430-interrupt" fn f5() {} //~ ERROR msp430-interrupt ABI is experimental
2019-09-12T16:50:58.1459910Z    |
2019-09-12T16:50:58.1459910Z    |
2019-09-12T16:50:58.1460171Z    = note: for more information, see ***/issues/38487
2019-09-12T16:50:58.1460245Z    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-09-12T16:50:58.1460277Z 
2019-09-12T16:50:58.1460320Z error[E0658]: PTX ABIs are experimental and subject to change
2019-09-12T16:50:58.1460698Z    |
2019-09-12T16:50:58.1460698Z    |
2019-09-12T16:50:58.1460939Z LL | extern "ptx-kernel" fn f6() {} //~ ERROR PTX ABIs are experimental and subject to change
2019-09-12T16:50:58.1461046Z    |
2019-09-12T16:50:58.1461046Z    |
2019-09-12T16:50:58.1461305Z    = note: for more information, see ***/issues/38788
2019-09-12T16:50:58.1461378Z    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-09-12T16:50:58.1461416Z 
2019-09-12T16:50:58.1461652Z error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-09-12T16:50:58.1461939Z    |
2019-09-12T16:50:58.1461939Z    |
2019-09-12T16:50:58.1462173Z LL | extern "x86-interrupt" fn f7() {} //~ ERROR x86-interrupt ABI is experimental
2019-09-12T16:50:58.1462280Z    |
2019-09-12T16:50:58.1462280Z    |
2019-09-12T16:50:58.1462530Z    = note: for more information, see ***/issues/40180
2019-09-12T16:50:58.1462583Z    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-09-12T16:50:58.1462676Z error[E0658]: thiscall is experimental and subject to change
2019-09-12T16:50:58.1462916Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:19:1
2019-09-12T16:50:58.1462979Z    |
2019-09-12T16:50:58.1462979Z    |
2019-09-12T16:50:58.1463026Z LL | extern "thiscall" fn f8() {} //~ ERROR thiscall is experimental and subject to change
2019-09-12T16:50:58.1463138Z    |
2019-09-12T16:50:58.1463184Z    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-09-12T16:50:58.1463214Z 
2019-09-12T16:50:58.1463214Z 
2019-09-12T16:50:58.1463436Z error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-09-12T16:50:58.1463719Z    |
2019-09-12T16:50:58.1463719Z    |
2019-09-12T16:50:58.1463969Z LL | extern "amdgpu-kernel" fn f9() {} //~ ERROR amdgpu-kernel ABI is experimental and subject to change
2019-09-12T16:50:58.1464075Z    |
2019-09-12T16:50:58.1464075Z    |
2019-09-12T16:50:58.1464327Z    = note: for more information, see ***/issues/51575
2019-09-12T16:50:58.1464606Z    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-09-12T16:50:58.1464693Z error[E0658]: intrinsics are subject to change
2019-09-12T16:50:58.1465073Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:24:5
2019-09-12T16:50:58.1465117Z    |
2019-09-12T16:50:58.1465117Z    |
2019-09-12T16:50:58.1465351Z LL |     extern "rust-intrinsic" fn m1(); //~ ERROR intrinsics are subject to change
2019-09-12T16:50:58.1465458Z    |
2019-09-12T16:50:58.1465458Z    |
2019-09-12T16:50:58.1465501Z    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-09-12T16:50:58.1465594Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-12T16:50:58.1466461Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:25:5
2019-09-12T16:50:58.1466514Z    |
2019-09-12T16:50:58.1466514Z    |
2019-09-12T16:50:58.1466810Z LL |     extern "platform-intrinsic" fn m2(); //~ ERROR platform intrinsics are experimental
2019-09-12T16:50:58.1466919Z    |
2019-09-12T16:50:58.1466919Z    |
2019-09-12T16:50:58.1467230Z    = note: for more information, see ***/issues/27731
2019-09-12T16:50:58.1467298Z    = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
2019-09-12T16:50:58.1467377Z error[E0658]: vectorcall is experimental and subject to change
2019-09-12T16:50:58.1467652Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:26:5
2019-09-12T16:50:58.1467700Z    |
2019-09-12T16:50:58.1467700Z    |
2019-09-12T16:50:58.1467750Z LL |     extern "vectorcall" fn m3(); //~ ERROR vectorcall is experimental and subject to change
2019-09-12T16:50:58.1467863Z    |
2019-09-12T16:50:58.1467911Z    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-12T16:50:58.1467962Z 
2019-09-12T16:50:58.1468185Z error[E0658]: rust-call ABI is subject to change
2019-09-12T16:50:58.1468185Z error[E0658]: rust-call ABI is subject to change
2019-09-12T16:50:58.1468432Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:27:5
2019-09-12T16:50:58.1468478Z    |
2019-09-12T16:50:58.1468748Z LL |     extern "rust-call" fn m4(); //~ ERROR rust-call ABI is subject to change
2019-09-12T16:50:58.1468850Z    |
2019-09-12T16:50:58.1468850Z    |
2019-09-12T16:50:58.1469140Z    = note: for more information, see ***/issues/29625
2019-09-12T16:50:58.1469198Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-09-12T16:50:58.1469231Z 
2019-09-12T16:50:58.1469642Z error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-09-12T16:50:58.1469898Z    |
2019-09-12T16:50:58.1469898Z    |
2019-09-12T16:50:58.1470146Z LL |     extern "msp430-interrupt" fn m5(); //~ ERROR msp430-interrupt ABI is experimental
2019-09-12T16:50:58.1470233Z    |
2019-09-12T16:50:58.1470233Z    |
2019-09-12T16:50:58.1470499Z    = note: for more information, see ***/issues/38487
2019-09-12T16:50:58.1470552Z    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-09-12T16:50:58.1470581Z 
2019-09-12T16:50:58.1470646Z error[E0658]: PTX ABIs are experimental and subject to change
2019-09-12T16:50:58.1470912Z    |
2019-09-12T16:50:58.1470912Z    |
2019-09-12T16:50:58.1471162Z LL |     extern "ptx-kernel" fn m6(); //~ ERROR PTX ABIs are experimental and subject to change
2019-09-12T16:50:58.1471248Z    |
2019-09-12T16:50:58.1471248Z    |
2019-09-12T16:50:58.1471512Z    = note: for more information, see ***/issues/38788
2019-09-12T16:50:58.1471563Z    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-09-12T16:50:58.1471592Z 
2019-09-12T16:50:58.1471815Z error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-09-12T16:50:58.1472193Z    |
2019-09-12T16:50:58.1472193Z    |
2019-09-12T16:50:58.1472457Z LL |     extern "x86-interrupt" fn m7(); //~ ERROR x86-interrupt ABI is experimental
2019-09-12T16:50:58.1472644Z    |
2019-09-12T16:50:58.1472644Z    |
2019-09-12T16:50:58.1472912Z    = note: for more information, see ***/issues/40180
2019-09-12T16:50:58.1472980Z    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-09-12T16:50:58.1473051Z error[E0658]: thiscall is experimental and subject to change
2019-09-12T16:50:58.1473297Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:31:5
2019-09-12T16:50:58.1473339Z    |
2019-09-12T16:50:58.1473339Z    |
2019-09-12T16:50:58.1473384Z LL |     extern "thiscall" fn m8(); //~ ERROR thiscall is experimental and subject to change
2019-09-12T16:50:58.1473485Z    |
2019-09-12T16:50:58.1473528Z    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-09-12T16:50:58.1473566Z 
2019-09-12T16:50:58.1473566Z 
2019-09-12T16:50:58.1473800Z error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-09-12T16:50:58.1474066Z    |
2019-09-12T16:50:58.1474066Z    |
2019-09-12T16:50:58.1474326Z LL |     extern "amdgpu-kernel" fn m9(); //~ ERROR amdgpu-kernel ABI is experimental and subject to change
2019-09-12T16:50:58.1474413Z    |
2019-09-12T16:50:58.1474413Z    |
2019-09-12T16:50:58.1474677Z    = note: for more information, see ***/issues/51575
2019-09-12T16:50:58.1474728Z    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-09-12T16:50:58.1474867Z error[E0658]: intrinsics are subject to change
2019-09-12T16:50:58.1475094Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:34:5
2019-09-12T16:50:58.1475136Z    |
2019-09-12T16:50:58.1475136Z    |
2019-09-12T16:50:58.1475369Z LL |     extern "rust-intrinsic" fn dm1() {} //~ ERROR intrinsics are subject to change
2019-09-12T16:50:58.1475473Z    |
2019-09-12T16:50:58.1475473Z    |
2019-09-12T16:50:58.1475693Z    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-09-12T16:50:58.1475793Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-12T16:50:58.1480475Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:35:5
2019-09-12T16:50:58.1480573Z    |
2019-09-12T16:50:58.1480573Z    |
2019-09-12T16:50:58.1480910Z LL |     extern "platform-intrinsic" fn dm2() {} //~ ERROR platform intrinsics are experimental
2019-09-12T16:50:58.1481019Z    |
2019-09-12T16:50:58.1481019Z    |
2019-09-12T16:50:58.1481304Z    = note: for more information, see ***/issues/27731
2019-09-12T16:50:58.1481358Z    = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
2019-09-12T16:50:58.1481465Z error[E0658]: vectorcall is experimental and subject to change
2019-09-12T16:50:58.1481702Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:36:5
2019-09-12T16:50:58.1481745Z    |
2019-09-12T16:50:58.1481745Z    |
2019-09-12T16:50:58.1481818Z LL |     extern "vectorcall" fn dm3() {} //~ ERROR vectorcall is experimental and subject to change
2019-09-12T16:50:58.1481901Z    |
2019-09-12T16:50:58.1481962Z    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-12T16:50:58.1481991Z 
2019-09-12T16:50:58.1482192Z error[E0658]: rust-call ABI is subject to change
2019-09-12T16:50:58.1482192Z error[E0658]: rust-call ABI is subject to change
2019-09-12T16:50:58.1482425Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:37:5
2019-09-12T16:50:58.1482470Z    |
2019-09-12T16:50:58.1482697Z LL |     extern "rust-call" fn dm4() {} //~ ERROR rust-call ABI is subject to change
2019-09-12T16:50:58.1482800Z    |
2019-09-12T16:50:58.1482800Z    |
2019-09-12T16:50:58.1490114Z    = note: for more information, see ***/issues/29625
2019-09-12T16:50:58.1490236Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-09-12T16:50:58.1490270Z 
2019-09-12T16:50:58.1490685Z error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-09-12T16:50:58.1490965Z    |
2019-09-12T16:50:58.1490965Z    |
2019-09-12T16:50:58.1491195Z LL |     extern "msp430-interrupt" fn dm5() {} //~ ERROR msp430-interrupt ABI is experimental
2019-09-12T16:50:58.1491299Z    |
2019-09-12T16:50:58.1491299Z    |
2019-09-12T16:50:58.1491551Z    = note: for more information, see ***/issues/38487
2019-09-12T16:50:58.1491604Z    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-09-12T16:50:58.1491650Z 
2019-09-12T16:50:58.1491689Z error[E0658]: PTX ABIs are experimental and subject to change
2019-09-12T16:50:58.1491979Z    |
2019-09-12T16:50:58.1491979Z    |
2019-09-12T16:50:58.1492212Z LL |     extern "ptx-kernel" fn dm6() {} //~ ERROR PTX ABIs are experimental and subject to change
2019-09-12T16:50:58.1492302Z    |
2019-09-12T16:50:58.1492302Z    |
2019-09-12T16:50:58.1492560Z    = note: for more information, see ***/issues/38788
2019-09-12T16:50:58.1492610Z    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-09-12T16:50:58.1492638Z 
2019-09-12T16:50:58.1493153Z error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-09-12T16:50:58.1493456Z    |
2019-09-12T16:50:58.1493456Z    |
2019-09-12T16:50:58.1493888Z LL |     extern "x86-interrupt" fn dm7() {} //~ ERROR x86-interrupt ABI is experimental
2019-09-12T16:50:58.1493981Z    |
2019-09-12T16:50:58.1493981Z    |
2019-09-12T16:50:58.1494279Z    = note: for more information, see ***/issues/40180
2019-09-12T16:50:58.1494351Z    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-09-12T16:50:58.1494454Z error[E0658]: thiscall is experimental and subject to change
2019-09-12T16:50:58.1494741Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:41:5
2019-09-12T16:50:58.1494789Z    |
2019-09-12T16:50:58.1494789Z    |
2019-09-12T16:50:58.1494858Z LL |     extern "thiscall" fn dm8() {} //~ ERROR thiscall is experimental and subject to change
2019-09-12T16:50:58.1494953Z    |
2019-09-12T16:50:58.1495018Z    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-09-12T16:50:58.1495054Z 
2019-09-12T16:50:58.1495054Z 
2019-09-12T16:50:58.1495311Z error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-09-12T16:50:58.1496056Z    |
2019-09-12T16:50:58.1496056Z    |
2019-09-12T16:50:58.1496349Z LL |     extern "amdgpu-kernel" fn dm9() {} //~ ERROR amdgpu-kernel ABI is experimental and subject to change
2019-09-12T16:50:58.1496464Z    |
2019-09-12T16:50:58.1496464Z    |
2019-09-12T16:50:58.1496759Z    = note: for more information, see ***/issues/51575
2019-09-12T16:50:58.1496820Z    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-09-12T16:50:58.1496920Z error[E0658]: intrinsics are subject to change
2019-09-12T16:50:58.1497195Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:49:5
2019-09-12T16:50:58.1497263Z    |
2019-09-12T16:50:58.1497263Z    |
2019-09-12T16:50:58.1497536Z LL |     extern "rust-intrinsic" fn m1() {} //~ ERROR intrinsics are subject to change
2019-09-12T16:50:58.1497637Z    |
2019-09-12T16:50:58.1497637Z    |
2019-09-12T16:50:58.1497705Z    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-09-12T16:50:58.1497938Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-12T16:50:58.1498268Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:50:5
2019-09-12T16:50:58.1498318Z    |
2019-09-12T16:50:58.1498318Z    |
2019-09-12T16:50:58.1498715Z LL |     extern "platform-intrinsic" fn m2() {} //~ ERROR platform intrinsics are experimental
2019-09-12T16:50:58.1498836Z    |
2019-09-12T16:50:58.1498836Z    |
2019-09-12T16:50:58.1499275Z    = note: for more information, see ***/issues/27731
2019-09-12T16:50:58.1499346Z    = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
2019-09-12T16:50:58.1499418Z error[E0658]: vectorcall is experimental and subject to change
2019-09-12T16:50:58.1499644Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:51:5
2019-09-12T16:50:58.1499707Z    |
2019-09-12T16:50:58.1499707Z    |
2019-09-12T16:50:58.1499753Z LL |     extern "vectorcall" fn m3() {} //~ ERROR vectorcall is experimental and subject to change
2019-09-12T16:50:58.1499863Z    |
2019-09-12T16:50:58.1499906Z    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-12T16:50:58.1499942Z 
2019-09-12T16:50:58.1500165Z error[E0658]: rust-call ABI is subject to change
2019-09-12T16:50:58.1500165Z error[E0658]: rust-call ABI is subject to change
2019-09-12T16:50:58.1500384Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:52:5
2019-09-12T16:50:58.1500427Z    |
2019-09-12T16:50:58.1500649Z LL |     extern "rust-call" fn m4() {} //~ ERROR rust-call ABI is subject to change
2019-09-12T16:50:58.1500751Z    |
2019-09-12T16:50:58.1500751Z    |
2019-09-12T16:50:58.1501018Z    = note: for more information, see ***/issues/29625
2019-09-12T16:50:58.1501072Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-09-12T16:50:58.1501103Z 
2019-09-12T16:50:58.1501350Z error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-09-12T16:50:58.1501655Z    |
2019-09-12T16:50:58.1501655Z    |
2019-09-12T16:50:58.1501906Z LL |     extern "msp430-interrupt" fn m5() {} //~ ERROR msp430-interrupt ABI is experimental
2019-09-12T16:50:58.1502022Z    |
2019-09-12T16:50:58.1502022Z    |
2019-09-12T16:50:58.1502284Z    = note: for more information, see ***/issues/38487
2019-09-12T16:50:58.1502355Z    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-09-12T16:50:58.1502386Z 
2019-09-12T16:50:58.1502430Z error[E0658]: PTX ABIs are experimental and subject to change
2019-09-12T16:50:58.1502732Z    |
2019-09-12T16:50:58.1502732Z    |
2019-09-12T16:50:58.1502983Z LL |     extern "ptx-kernel" fn m6() {} //~ ERROR PTX ABIs are experimental and subject to change
2019-09-12T16:50:58.1503090Z    |
2019-09-12T16:50:58.1503090Z    |
2019-09-12T16:50:58.1503358Z    = note: for more information, see ***/issues/38788
2019-09-12T16:50:58.1503429Z    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-09-12T16:50:58.1503459Z 
2019-09-12T16:50:58.1503707Z error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-09-12T16:50:58.1504000Z    |
2019-09-12T16:50:58.1504000Z    |
2019-09-12T16:50:58.1504241Z LL |     extern "x86-interrupt" fn m7() {} //~ ERROR x86-interrupt ABI is experimental
2019-09-12T16:50:58.1504349Z    |
2019-09-12T16:50:58.1504349Z    |
2019-09-12T16:50:58.1504609Z    = note: for more information, see ***/issues/40180
2019-09-12T16:50:58.1504662Z    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-09-12T16:50:58.1504993Z error[E0658]: thiscall is experimental and subject to change
2019-09-12T16:50:58.1505312Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:56:5
2019-09-12T16:50:58.1505852Z    |
2019-09-12T16:50:58.1505852Z    |
2019-09-12T16:50:58.1505932Z LL |     extern "thiscall" fn m8() {} //~ ERROR thiscall is experimental and subject to change
2019-09-12T16:50:58.1506105Z    |
2019-09-12T16:50:58.1506175Z    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-09-12T16:50:58.1506211Z 
2019-09-12T16:50:58.1506211Z 
2019-09-12T16:50:58.1506530Z error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-09-12T16:50:58.1507882Z    |
2019-09-12T16:50:58.1507882Z    |
2019-09-12T16:50:58.1513434Z LL |     extern "amdgpu-kernel" fn m9() {} //~ ERROR amdgpu-kernel ABI is experimental and subject to change
2019-09-12T16:50:58.1513575Z    |
2019-09-12T16:50:58.1513575Z    |
2019-09-12T16:50:58.1513874Z    = note: for more information, see ***/issues/51575
2019-09-12T16:50:58.1513966Z    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-09-12T16:50:58.1514043Z error[E0658]: intrinsics are subject to change
2019-09-12T16:50:58.1514975Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:62:5
2019-09-12T16:50:58.1515062Z    |
2019-09-12T16:50:58.1515062Z    |
2019-09-12T16:50:58.1515337Z LL |     extern "rust-intrinsic" fn im1() {} //~ ERROR intrinsics are subject to change
2019-09-12T16:50:58.1515449Z    |
2019-09-12T16:50:58.1515449Z    |
2019-09-12T16:50:58.1515495Z    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-09-12T16:50:58.1515590Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-12T16:50:58.1516325Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:63:5
2019-09-12T16:50:58.1516376Z    |
2019-09-12T16:50:58.1516376Z    |
2019-09-12T16:50:58.1516638Z LL |     extern "platform-intrinsic" fn im2() {} //~ ERROR platform intrinsics are experimental
2019-09-12T16:50:58.1516766Z    |
2019-09-12T16:50:58.1516766Z    |
2019-09-12T16:50:58.1517085Z    = note: for more information, see ***/issues/27731
2019-09-12T16:50:58.1517147Z    = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
2019-09-12T16:50:58.1517241Z error[E0658]: vectorcall is experimental and subject to change
2019-09-12T16:50:58.1517533Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:64:5
2019-09-12T16:50:58.1517584Z    |
2019-09-12T16:50:58.1517584Z    |
2019-09-12T16:50:58.1517637Z LL |     extern "vectorcall" fn im3() {} //~ ERROR vectorcall is experimental and subject to change
2019-09-12T16:50:58.1517751Z    |
2019-09-12T16:50:58.1517802Z    = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable
2019-09-12T16:50:58.1517854Z 
2019-09-12T16:50:58.1518096Z error[E0658]: rust-call ABI is subject to change
2019-09-12T16:50:58.1518096Z error[E0658]: rust-call ABI is subject to change
2019-09-12T16:50:58.1518364Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:65:5
2019-09-12T16:50:58.1518413Z    |
2019-09-12T16:50:58.1518700Z LL |     extern "rust-call" fn im4() {} //~ ERROR rust-call ABI is subject to change
2019-09-12T16:50:58.1518807Z    |
2019-09-12T16:50:58.1518807Z    |
2019-09-12T16:50:58.1519119Z    = note: for more information, see ***/issues/29625
2019-09-12T16:50:58.1519180Z    = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
2019-09-12T16:50:58.1519215Z 
2019-09-12T16:50:58.1520255Z error[E0658]: msp430-interrupt ABI is experimental and subject to change
2019-09-12T16:50:58.1520822Z    |
2019-09-12T16:50:58.1520822Z    |
2019-09-12T16:50:58.1521416Z LL |     extern "msp430-interrupt" fn im5() {} //~ ERROR msp430-interrupt ABI is experimental
2019-09-12T16:50:58.1521504Z    |
2019-09-12T16:50:58.1521504Z    |
2019-09-12T16:50:58.1521964Z    = note: for more information, see ***/issues/38487
2019-09-12T16:50:58.1522032Z    = help: add `#![feature(abi_msp430_interrupt)]` to the crate attributes to enable
2019-09-12T16:50:58.1522064Z 
2019-09-12T16:50:58.1522190Z error[E0658]: PTX ABIs are experimental and subject to change
2019-09-12T16:50:58.1522503Z    |
2019-09-12T16:50:58.1522503Z    |
2019-09-12T16:50:58.1522738Z LL |     extern "ptx-kernel" fn im6() {} //~ ERROR PTX ABIs are experimental and subject to change
2019-09-12T16:50:58.1522845Z    |
2019-09-12T16:50:58.1522845Z    |
2019-09-12T16:50:58.1523118Z    = note: for more information, see ***/issues/38788
2019-09-12T16:50:58.1523175Z    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
2019-09-12T16:50:58.1523205Z 
2019-09-12T16:50:58.1523447Z error[E0658]: x86-interrupt ABI is experimental and subject to change
2019-09-12T16:50:58.1523754Z    |
2019-09-12T16:50:58.1523754Z    |
2019-09-12T16:50:58.1525747Z LL |     extern "x86-interrupt" fn im7() {} //~ ERROR x86-interrupt ABI is experimental
2019-09-12T16:50:58.1526422Z    |
2019-09-12T16:50:58.1526422Z    |
2019-09-12T16:50:58.1526829Z    = note: for more information, see ***/issues/40180
2019-09-12T16:50:58.1526910Z    = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable
2019-09-12T16:50:58.1526992Z error[E0658]: thiscall is experimental and subject to change
2019-09-12T16:50:58.1528126Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:69:5
2019-09-12T16:50:58.1528199Z    |
2019-09-12T16:50:58.1528199Z    |
2019-09-12T16:50:58.1528250Z LL |     extern "thiscall" fn im8() {} //~ ERROR thiscall is experimental and subject to change
2019-09-12T16:50:58.1528361Z    |
2019-09-12T16:50:58.1528427Z    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
2019-09-12T16:50:58.1528466Z 
2019-09-12T16:50:58.1528466Z 
2019-09-12T16:50:58.1529039Z error[E0658]: amdgpu-kernel ABI is experimental and subject to change
2019-09-12T16:50:58.1529412Z    |
2019-09-12T16:50:58.1529412Z    |
2019-09-12T16:50:58.1529863Z LL |     extern "amdgpu-kernel" fn im9() {} //~ ERROR amdgpu-kernel ABI is experimental and subject to change
2019-09-12T16:50:58.1529952Z    |
2019-09-12T16:50:58.1529952Z    |
2019-09-12T16:50:58.1530466Z    = note: for more information, see ***/issues/51575
2019-09-12T16:50:58.1530528Z    = help: add `#![feature(abi_amdgpu_kernel)]` to the crate attributes to enable
2019-09-12T16:50:58.1530598Z error[E0658]: intrinsics are subject to change
2019-09-12T16:50:58.1530857Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:74:11
2019-09-12T16:50:58.1530901Z    |
2019-09-12T16:50:58.1530901Z    |
2019-09-12T16:50:58.1531315Z LL | type A1 = extern "rust-intrinsic" fn(); //~ ERROR intrinsics are subject to change
2019-09-12T16:50:58.1531438Z    |
2019-09-12T16:50:58.1531438Z    |
2019-09-12T16:50:58.1531487Z    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-09-12T16:50:58.1531574Z error[E0658]: platform intrinsics are experimental and possibly buggy
2019-09-12T16:50:58.1531840Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:75:11
2019-09-12T16:50:58.1531900Z    |
---
2019-09-12T16:50:58.1588625Z 22 LL |     fn bar();
2019-09-12T16:50:58.1588672Z 23    |     ^^^^^^^^^ unrecognized intrinsic
2019-09-12T16:50:58.1588717Z 24 
2019-09-12T16:50:58.1588949Z - error: aborting due to 3 previous errors
2019-09-12T16:50:58.1589192Z + error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
2019-09-12T16:50:58.1589755Z +   --> $DIR/feature-gate-intrinsics.rs:5:34
2019-09-12T16:50:58.1589816Z +    |
2019-09-12T16:50:58.1590090Z + LL | extern "rust-intrinsic" fn baz() {}
2019-09-12T16:50:58.1590276Z + 
2019-09-12T16:50:58.1590336Z + error: aborting due to 4 previous errors
2019-09-12T16:50:58.1590376Z 26 
2019-09-12T16:50:58.1590420Z 27 Some errors have detailed explanations: E0093, E0658.
2019-09-12T16:50:58.1590420Z 27 Some errors have detailed explanations: E0093, E0658.
2019-09-12T16:50:58.1590695Z 28 For more information about an error, try `rustc --explain E0093`.
2019-09-12T16:50:58.1590730Z 
2019-09-12T16:50:58.1590755Z 
2019-09-12T16:50:58.1590800Z The actual stderr differed from the expected stderr.
2019-09-12T16:50:58.1591125Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-intrinsics/feature-gate-intrinsics.stderr
2019-09-12T16:50:58.1591368Z To update references, rerun the tests and pass the `--bless` flag
2019-09-12T16:50:58.1591643Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-intrinsics.rs`
2019-09-12T16:50:58.1591738Z error: 1 errors occurred comparing output.
2019-09-12T16:50:58.1591788Z status: exit code: 1
2019-09-12T16:50:58.1591788Z status: exit code: 1
2019-09-12T16:50:58.1592900Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-intrinsics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-intrinsics" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-intrinsics/auxiliary" "-A" "unused"
2019-09-12T16:50:58.1593268Z ------------------------------------------
2019-09-12T16:50:58.1593302Z 
2019-09-12T16:50:58.1593522Z ------------------------------------------
2019-09-12T16:50:58.1593584Z stderr:
2019-09-12T16:50:58.1593584Z stderr:
2019-09-12T16:50:58.1593789Z ------------------------------------------
2019-09-12T16:50:58.1593846Z error[E0658]: intrinsics are subject to change
2019-09-12T16:50:58.1594081Z   --> /checkout/src/test/ui/feature-gates/feature-gate-intrinsics.rs:1:1
2019-09-12T16:50:58.1594148Z    |
2019-09-12T16:50:58.1594390Z LL | / extern "rust-intrinsic" {   //~ ERROR intrinsics are subject to change
2019-09-12T16:50:58.1594443Z LL | |     fn bar(); //~ ERROR unrecognized intrinsic function: `bar`
2019-09-12T16:50:58.1594544Z    | |_^
2019-09-12T16:50:58.1594582Z    |
2019-09-12T16:50:58.1594582Z    |
2019-09-12T16:50:58.1594645Z    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-09-12T16:50:58.1594777Z error[E0658]: intrinsics are subject to change
2019-09-12T16:50:58.1595023Z   --> /checkout/src/test/ui/feature-gates/feature-gate-intrinsics.rs:5:1
2019-09-12T16:50:58.1595095Z    |
2019-09-12T16:50:58.1595095Z    |
2019-09-12T16:50:58.1595340Z LL | extern "rust-intrinsic" fn baz() {} //~ ERROR intrinsics are subject to change
2019-09-12T16:50:58.1596032Z    |
2019-09-12T16:50:58.1596032Z    |
2019-09-12T16:50:58.1596090Z    = help: add `#![feature(intrinsics)]` to the crate attributes to enable
2019-09-12T16:50:58.1596186Z error[E0093]: unrecognized intrinsic function: `bar`
2019-09-12T16:50:58.1596474Z   --> /checkout/src/test/ui/feature-gates/feature-gate-intrinsics.rs:2:5
2019-09-12T16:50:58.1596523Z    |
2019-09-12T16:50:58.1596523Z    |
2019-09-12T16:50:58.1596569Z LL |     fn bar(); //~ ERROR unrecognized intrinsic function: `bar`
2019-09-12T16:50:58.1596637Z    |     ^^^^^^^^^ unrecognized intrinsic
2019-09-12T16:50:58.1596665Z 
2019-09-12T16:50:58.1596903Z error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
2019-09-12T16:50:58.1597214Z    |
2019-09-12T16:50:58.1597214Z    |
2019-09-12T16:50:58.1597591Z LL | extern "rust-intrinsic" fn baz() {} //~ ERROR intrinsics are subject to change
2019-09-12T16:50:58.1597774Z 
2019-09-12T16:50:58.1597818Z error: aborting due to 4 previous errors
2019-09-12T16:50:58.1597848Z 
2019-09-12T16:50:58.1597910Z Some errors have detailed explanations: E0093, E0658.
---
2019-09-12T16:50:58.1599801Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-12T16:50:58.1599856Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-12T16:50:58.1599896Z 
2019-09-12T16:50:58.1599938Z 
2019-09-12T16:50:58.1601690Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-12T16:50:58.1601966Z 
2019-09-12T16:50:58.1601995Z 
2019-09-12T16:50:58.1602101Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-12T16:50:58.1602167Z Build completed unsuccessfully in 1:05:47
2019-09-12T16:50:58.1602167Z Build completed unsuccessfully in 1:05:47
2019-09-12T16:50:58.1602211Z == clock drift check ==
2019-09-12T16:50:58.1602262Z   local time: Thu Sep 12 16:50:58 UTC 2019
2019-09-12T16:50:58.2384355Z   network time: Thu, 12 Sep 2019 16:50:58 GMT
2019-09-12T16:50:58.2388564Z == end clock drift check ==
2019-09-12T16:50:59.0638292Z ##[error]Bash exited with code '1'.
2019-09-12T16:50:59.0703326Z ##[section]Starting: Checkout
2019-09-12T16:50:59.0704964Z ==============================================================================
2019-09-12T16:50:59.0705018Z Task         : Get sources
2019-09-12T16:50:59.0705069Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
