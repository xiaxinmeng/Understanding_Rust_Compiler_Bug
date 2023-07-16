plain
2020-02-15T19:57:55.8295444Z ========================== Starting Command Output ===========================
2020-02-15T19:57:55.8298316Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b9ddd63c-0587-487d-9207-f6b7b42da044.sh
2020-02-15T19:57:55.8298493Z 
2020-02-15T19:57:55.8304526Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-15T19:57:55.8311173Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69194/merge to s
2020-02-15T19:57:55.8312896Z Task         : Get sources
2020-02-15T19:57:55.8312921Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-15T19:57:55.8312980Z Version      : 1.0.0
2020-02-15T19:57:55.8313005Z Author       : Microsoft
---
2020-02-15T19:57:58.4405256Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-15T19:57:58.4635342Z ##[command]git config gc.auto 0
2020-02-15T19:57:58.4676510Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-15T19:57:58.4729904Z ##[command]git config --get-all http.proxy
2020-02-15T19:57:58.4867124Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69194/merge:refs/remotes/pull/69194/merge
---
2020-02-15T20:58:46.6929674Z .................................................................................................... 1700/9663
2020-02-15T20:58:51.4458020Z .................................................................................................... 1800/9663
2020-02-15T20:59:03.3830961Z ..................................i................................................................. 1900/9663
2020-02-15T20:59:10.9432016Z .................................................................................................... 2000/9663
2020-02-15T20:59:25.4826691Z ........................iiiii....................................................................... 2100/9663
2020-02-15T20:59:35.2928535Z .................................................................................................... 2300/9663
2020-02-15T20:59:37.6748316Z .................................................................................................... 2400/9663
2020-02-15T20:59:42.2281162Z .................................................................................................... 2500/9663
2020-02-15T21:00:03.8631079Z .................................................................................................... 2600/9663
---
2020-02-15T21:03:25.3910207Z .................................................................................................... 5600/9663
2020-02-15T21:03:36.3225963Z .......................................................................................i............ 5700/9663
2020-02-15T21:03:43.7892386Z .................................................................................................... 5800/9663
2020-02-15T21:03:48.9868700Z .....................................................................................i.............. 5900/9663
2020-02-15T21:03:58.7520769Z ...............................................................................ii...i..ii........... 6000/9663
2020-02-15T21:04:11.1906034Z i................................................................................................... 6100/9663
2020-02-15T21:04:27.7440337Z .................................................................................................... 6300/9663
2020-02-15T21:04:34.6318851Z .................................................................................................... 6400/9663
2020-02-15T21:04:34.6318851Z .................................................................................................... 6400/9663
2020-02-15T21:04:47.7439249Z .......i..ii........................................................................................ 6500/9663
2020-02-15T21:05:07.8226246Z ...................................................................................................i 6700/9663
2020-02-15T21:05:10.0763347Z .................................................................................................... 6800/9663
2020-02-15T21:05:12.3121945Z .................................................................................................... 6900/9663
2020-02-15T21:05:14.6981176Z ...................i................................................................................ 7000/9663
---
2020-02-15T21:06:57.0375443Z .................................................................................................... 7700/9663
2020-02-15T21:07:02.8966922Z .................................................................................................... 7800/9663
2020-02-15T21:07:09.2383334Z .................................................................................................... 7900/9663
2020-02-15T21:07:19.7629286Z .................................................................................................... 8000/9663
2020-02-15T21:07:25.5489890Z iiiiiii.i........................................................................................... 8100/9663
2020-02-15T21:07:40.2894569Z .................................................................................................... 8300/9663
2020-02-15T21:07:51.4995389Z .................................................................................................... 8400/9663
2020-02-15T21:08:03.7246881Z .................................................................................................... 8500/9663
2020-02-15T21:08:09.7880819Z .................................................................................................... 8600/9663
---
2020-02-15T21:10:32.3689417Z  finished in 7.501
2020-02-15T21:10:32.3859366Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-15T21:10:32.5944577Z 
2020-02-15T21:10:32.5944971Z running 178 tests
2020-02-15T21:10:35.5326052Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/178
2020-02-15T21:10:37.8002457Z ...i.i.i...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-02-15T21:10:37.8005133Z 
2020-02-15T21:10:37.8010838Z  finished in 5.415
2020-02-15T21:10:37.8185601Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-15T21:10:37.9867103Z 
---
2020-02-15T21:10:39.9234190Z  finished in 2.104
2020-02-15T21:10:39.9402503Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-15T21:10:40.0958228Z 
2020-02-15T21:10:40.0958536Z running 9 tests
2020-02-15T21:10:40.0960751Z iiiiiiiii
2020-02-15T21:10:40.0962685Z 
2020-02-15T21:10:40.0962891Z  finished in 0.156
2020-02-15T21:10:40.1137375Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-15T21:10:40.3192069Z 
---
2020-02-15T21:11:00.0095950Z  finished in 19.895
2020-02-15T21:11:00.0329656Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-15T21:11:00.2282266Z 
2020-02-15T21:11:00.2282951Z running 116 tests
2020-02-15T21:11:13.4537688Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-02-15T21:11:15.2958786Z ....iiii.....ii.
2020-02-15T21:11:15.2960012Z 
2020-02-15T21:11:15.2964710Z  finished in 15.263
2020-02-15T21:11:15.2969923Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-15T21:11:15.2970714Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-02-15T21:18:42.7418486Z 
2020-02-15T21:18:42.7419080Z error: pretty-printed source does not match expected source
2020-02-15T21:18:42.7419256Z expected:
2020-02-15T21:18:42.7419556Z ------------------------------------------
2020-02-15T21:18:42.7419878Z // pp-exact
2020-02-15T21:18:42.7420119Z fn main() { }
2020-02-15T21:18:42.7420228Z 
2020-02-15T21:18:42.7420334Z struct C {
2020-02-15T21:18:42.7420440Z     field: u8,
---
2020-02-15T21:18:42.7421009Z     C{
2020-02-15T21:18:42.7421115Z       #[cfg(debug_assertions)]
2020-02-15T21:18:42.7421250Z       field: 0,
2020-02-15T21:18:42.7421343Z 
2020-02-15T21:18:42.7421453Z       #[cfg(not (debug_assertions))]
2020-02-15T21:18:42.7421561Z       field: 1,};
2020-02-15T21:18:42.7421966Z ------------------------------------------
2020-02-15T21:18:42.7422112Z actual:
2020-02-15T21:18:42.7422423Z ------------------------------------------
2020-02-15T21:18:42.7422423Z ------------------------------------------
2020-02-15T21:18:42.7422717Z // pp-exact
2020-02-15T21:18:42.7422967Z fn main() { }
2020-02-15T21:18:42.7423059Z 
2020-02-15T21:18:42.7423163Z struct C {
2020-02-15T21:18:42.7423282Z     field: u8,
2020-02-15T21:18:42.7423282Z     field: u8,
2020-02-15T21:18:42.7423388Z }
2020-02-15T21:18:42.7423476Z 
2020-02-15T21:18:42.7423580Z #[allow()]
2020-02-15T21:18:42.7423704Z const  C: C =
2020-02-15T21:18:42.7423808Z     C{
2020-02-15T21:18:42.7424101Z       #[cfg(debug_assertions)]
2020-02-15T21:18:42.7424293Z       field: 0,
2020-02-15T21:18:42.7424388Z 
2020-02-15T21:18:42.7424497Z       #[cfg(not (debug_assertions))]
2020-02-15T21:18:42.7424775Z       field: 1,};
2020-02-15T21:18:42.7425777Z ------------------------------------------
2020-02-15T21:18:42.7425965Z 
2020-02-15T21:18:42.7426080Z 
2020-02-15T21:18:42.7426080Z 
2020-02-15T21:18:42.7427167Z [ERROR compiletest::runtest] fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n// pp-exact\n\nfn main() { }\n\nstruct C {\n    field: u8,\n}\n\n#[allow()]\nconst C: C =\n    C{\n      #[cfg(debug_assertions)]\n      field: 0,\n\n      #[cfg(not (debug_assertions))]\n      field: 1,};\n\n------------------------------------------\nactual:\n------------------------------------------\n// pp-exact\n\nfn main() { }\n\nstruct C {\n    field: u8,\n}\n\n#[allow()]\nconst  C: C =\n    C{\n      #[cfg(debug_assertions)]\n      field: 0,\n\n      #[cfg(not (debug_assertions))]\n      field: 1,};\n\n------------------------------------------\n\n"
2020-02-15T21:18:42.7427753Z thread '[pretty] pretty/issue-68710-field-attr-proc-mac-lost.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2124:9
2020-02-15T21:18:42.7428104Z 
2020-02-15T21:18:42.7428468Z ---- [pretty] pretty/raw-address-of.rs stdout ----
2020-02-15T21:18:42.7428646Z 
2020-02-15T21:18:42.7429284Z error: pretty-printed source does not match expected source
2020-02-15T21:18:42.7429284Z error: pretty-printed source does not match expected source
2020-02-15T21:18:42.7429431Z expected:
2020-02-15T21:18:42.7429731Z ------------------------------------------
2020-02-15T21:18:42.7430019Z // pp-exact
2020-02-15T21:18:42.7430159Z #![feature(raw_ref_op)]
2020-02-15T21:18:42.7430266Z 
2020-02-15T21:18:42.7430374Z const C_PTR: () = { let a = 1; &raw const a; };
2020-02-15T21:18:42.7430483Z static S_PTR: () = { let b = false; &raw const b; };
2020-02-15T21:18:42.7430802Z fn main() {
2020-02-15T21:18:42.7431269Z     let x = 123;
2020-02-15T21:18:42.7432016Z     let mut y = 345;
2020-02-15T21:18:42.7432016Z     let mut y = 345;
2020-02-15T21:18:42.7432870Z     let c_p = &raw const x;
2020-02-15T21:18:42.7433104Z     let parens = unsafe { *(&raw mut (y)) };
2020-02-15T21:18:42.7433208Z 
2020-02-15T21:18:42.7433741Z ------------------------------------------
2020-02-15T21:18:42.7433811Z actual:
2020-02-15T21:18:42.7434279Z ------------------------------------------
2020-02-15T21:18:42.7434279Z ------------------------------------------
2020-02-15T21:18:42.7436292Z // pp-exact
2020-02-15T21:18:42.7436378Z #![feature(raw_ref_op)]
2020-02-15T21:18:42.7436410Z 
2020-02-15T21:18:42.7437332Z const  C_PTR: () = { let a = 1; &raw const a; };
2020-02-15T21:18:42.7437415Z static  S_PTR: () = { let b = false; &raw const b; };
2020-02-15T21:18:42.7437489Z fn main() {
2020-02-15T21:18:42.7437553Z     let x = 123;
2020-02-15T21:18:42.7437598Z     let mut y = 345;
2020-02-15T21:18:42.7437598Z     let mut y = 345;
2020-02-15T21:18:42.7437643Z     let c_p = &raw const x;
2020-02-15T21:18:42.7437725Z     let parens = unsafe { *(&raw mut (y)) };
2020-02-15T21:18:42.7437799Z 
2020-02-15T21:18:42.7438644Z ------------------------------------------
2020-02-15T21:18:42.7439278Z 
2020-02-15T21:18:42.7439312Z 
2020-02-15T21:18:42.7439312Z 
2020-02-15T21:18:42.7440752Z [ERROR compiletest::runtest] fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n// pp-exact\n#![feature(raw_ref_op)]\n\nconst C_PTR: () = { let a = 1; &raw const a; };\nstatic S_PTR: () = { let b = false; &raw const b; };\n\nfn main() {\n    let x = 123;\n    let mut y = 345;\n    let c_p = &raw const x;\n    let parens = unsafe { *(&raw mut (y)) };\n}\n\n------------------------------------------\nactual:\n------------------------------------------\n// pp-exact\n#![feature(raw_ref_op)]\n\nconst  C_PTR: () = { let a = 1; &raw const a; };\nstatic  S_PTR: () = { let b = false; &raw const b; };\n\nfn main() {\n    let x = 123;\n    let mut y = 345;\n    let c_p = &raw const x;\n    let parens = unsafe { *(&raw mut (y)) };\n}\n\n------------------------------------------\n\n"
2020-02-15T21:18:42.7441135Z thread '[pretty] pretty/raw-address-of.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2124:9
2020-02-15T21:18:42.7441308Z 
2020-02-15T21:18:42.7441343Z failures:
2020-02-15T21:18:42.7441568Z     [pretty] pretty/issue-68710-field-attr-proc-mac-lost.rs
2020-02-15T21:18:42.7441991Z     [pretty] pretty/raw-address-of.rs
2020-02-15T21:18:42.7441991Z     [pretty] pretty/raw-address-of.rs
2020-02-15T21:18:42.7442024Z 
2020-02-15T21:18:42.7443868Z test result: FAILED. 59 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
2020-02-15T21:18:42.7444051Z 
2020-02-15T21:18:42.7446043Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-15T21:18:42.7449429Z 
2020-02-15T21:18:42.7450688Z 
2020-02-15T21:18:42.7456521Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/pretty" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "pretty" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-15T21:18:42.7457078Z 
2020-02-15T21:18:42.7457155Z 
2020-02-15T21:18:42.7468353Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-15T21:18:42.7468453Z Build completed unsuccessfully in 1:14:06
2020-02-15T21:18:42.7468453Z Build completed unsuccessfully in 1:14:06
2020-02-15T21:18:42.7519593Z == clock drift check ==
2020-02-15T21:18:42.7540977Z   local time: Sat Feb 15 21:18:42 UTC 2020
2020-02-15T21:18:43.0449764Z   network time: Sat, 15 Feb 2020 21:18:43 GMT
2020-02-15T21:18:43.0452703Z == end clock drift check ==
2020-02-15T21:18:44.3466022Z 
2020-02-15T21:18:44.3564615Z ##[error]Bash exited with code '1'.
2020-02-15T21:18:44.3577464Z ##[section]Finishing: Run build
2020-02-15T21:18:44.3600320Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69194/merge to s
2020-02-15T21:18:44.3602184Z Task         : Get sources
2020-02-15T21:18:44.3602252Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-15T21:18:44.3602320Z Version      : 1.0.0
2020-02-15T21:18:44.3602364Z Author       : Microsoft
2020-02-15T21:18:44.3602364Z Author       : Microsoft
2020-02-15T21:18:44.3602433Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-15T21:18:44.3602484Z ==============================================================================
2020-02-15T21:18:44.8046562Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-15T21:18:44.8050360Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69194/merge to s
2020-02-15T21:18:44.8180232Z Cleaning up task key
2020-02-15T21:18:44.8181068Z Start cleaning up orphan processes.
2020-02-15T21:18:44.8288708Z Terminate orphan process: pid (4135) (python)
2020-02-15T21:18:44.8822021Z ##[section]Finishing: Finalize Job
