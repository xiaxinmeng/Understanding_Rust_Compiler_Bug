plain
2019-07-05T10:11:11.5387472Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-05T10:11:11.5387528Z 
2019-07-05T10:11:11.5387787Z   git checkout -b <new-branch-name>
2019-07-05T10:11:11.5387834Z 
2019-07-05T10:11:11.5388136Z HEAD is now at ada9e88f7 Auto merge of #62399 - Centril:rollup-du8hsoo, r=Centril
2019-07-05T10:11:11.5542140Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-05T10:11:11.5545400Z ==============================================================================
2019-07-05T10:11:11.5545870Z Task         : Bash
2019-07-05T10:11:11.5545952Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-05T10:11:11.8603153Z 
2019-07-05T10:11:11.8603402Z 
2019-07-05T10:11:11.8603443Z 
2019-07-05T10:11:11.8603526Z 
2019-07-05T10:11:11.8604298Z  - #62123 ( Remove needless lifetimes (std))
2019-07-05T10:11:11.8604722Z  - #62150 (Implement mem::{zeroed,uninitialized} in terms of MaybeUninit.)
2019-07-05T10:11:11.8605011Z  - #62169 (Derive which queries to save using the proc macro)
2019-07-05T10:11:11.8605782Z  - #62238 (Fix code block information icon position)
2019-07-05T10:11:11.8606137Z  - #62292 (Move `async || ...` closures into `#![feature(async_closure)]`)
2019-07-05T10:11:11.8606416Z  - #62323 (Clarify unaligned fields in ptr::{read,write}_unaligned)
2019-07-05T10:11:11.8606764Z  - #62324 (Reduce reliance on `await!(...)` macro)
2019-07-05T10:11:11.8607029Z  - #62331 (Fix leak when early returning out of `box` syntax)
2019-07-05T10:11:11.8607344Z  - #62371 (Add tracking issue for Box::into_pin)
2019-07-05T10:11:11.8607618Z  - #62383 (Improve error span for async type inference error)
2019-07-05T10:11:11.8607813Z AGENT_BUILDDIRECTORY=/home/vsts/work/1
2019-07-05T10:11:11.8607933Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-07-05T10:11:11.8608008Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-07-05T10:11:11.8608134Z AGENT_HOMEDIRECTORY=/home/vsts/agents/2.153.2
---
2019-07-05T10:11:11.8614442Z BUILD_SOURCEBRANCH=refs/heads/auto
2019-07-05T10:11:11.8614547Z BUILD_SOURCESDIRECTORY=/home/vsts/work/1/s
2019-07-05T10:11:11.8614616Z BUILD_SOURCEVERSION=ada9e88f7f90b67743adf8d4f80e1c1a0ce97a77
2019-07-05T10:11:11.8614726Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-07-05T10:11:11.8614996Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #62399 - Centril:rollup-du8hsoo, r=Centril
2019-07-05T10:11:11.8615585Z CARGO_HOME=/usr/local/cargo
2019-07-05T10:11:11.8615917Z CHROME_BIN=/usr/bin/google-chrome
2019-07-05T10:11:11.8615995Z COMMON_TESTRESULTSDIRECTORY=/home/vsts/work/1/TestResults
2019-07-05T10:11:11.8616129Z CONDA=/usr/share/miniconda
---
2019-07-05T10:12:59.2590974Z   Downloading https://files.pythonhosted.org/packages/73/fb/00a976f728d0d1fecfe898238ce23f502a721c0ac0ecfedb80e0d88c64e9/six-1.12.0-py2.py3-none-any.whl
2019-07-05T10:12:59.2638105Z Building wheels for collected packages: PyYAML
2019-07-05T10:12:59.2639393Z   Running setup.py bdist_wheel for PyYAML: started
2019-07-05T10:12:59.4574727Z   Running setup.py bdist_wheel for PyYAML: finished with status 'error'
2019-07-05T10:12:59.4576787Z   Complete output from command /usr/bin/python3 -u -c "import setuptools, tokenize;__file__='/tmp/pip-build-ugxy40ro/PyYAML/setup.py';exec(compile(getattr(tokenize, 'open', open)(__file__).read().replace('\r\n', '\n'), __file__, 'exec'))" bdist_wheel -d /tmp/tmpj2y89pwdpip-wheel- --python-tag cp35:
2019-07-05T10:12:59.4585702Z     warnings.warn(msg)
2019-07-05T10:12:59.4586805Z   usage: -c [global_opts] cmd1 [cmd1_opts] [cmd2 [cmd2_opts] ...]
2019-07-05T10:12:59.4587402Z      or: -c --help [cmd1 cmd2 ...]
2019-07-05T10:12:59.4587950Z      or: -c --help-commands
---
2019-07-05T11:45:26.9276829Z test [mir-opt] mir-opt/storage_live_dead_in_statics.rs ... ok
2019-07-05T11:45:26.9277862Z 
2019-07-05T11:45:26.9278160Z failures:
2019-07-05T11:45:26.9281033Z 
2019-07-05T11:45:26.9281718Z ---- [mir-opt] mir-opt/issue-62289.rs stdout ----
2019-07-05T11:45:26.9284635Z thread '[mir-opt] mir-opt/issue-62289.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2019-07-05T11:45:26.9285240Z Current block: None
2019-07-05T11:45:26.9285870Z Actual Line: "        _3 = const <std::option::Option<u32> as std::ops::Try>::into_result(move _4) -> bb1;"
2019-07-05T11:45:26.9286666Z Expected Line: "        _3 = const <std::option::Option<u32> as std::ops::Try>::into_result(move _4) -> [return: bb2, unwind: bb3];"
2019-07-05T11:45:26.9287047Z Test Name: rustc.test.ElaborateDrops.before.mir
2019-07-05T11:45:26.9287182Z Expected:
2019-07-05T11:45:26.9287238Z ... (elided)
2019-07-05T11:45:26.9288048Z fn test() -> std::option::Option<std::boxed::Box<u32>> {
2019-07-05T11:45:26.9288145Z ... (elided)
2019-07-05T11:45:26.9288201Z     bb0: {
2019-07-05T11:45:26.9288275Z         StorageLive(_1);
2019-07-05T11:45:26.9288335Z         StorageLive(_2);
2019-07-05T11:45:26.9288411Z         _2 = Box(u32);
2019-07-05T11:45:26.9288472Z         StorageLive(_3);
2019-07-05T11:45:26.9288569Z         StorageLive(_4);
2019-07-05T11:45:26.9288642Z         _4 = std::option::Option::<u32>::None;
2019-07-05T11:45:26.9288999Z         _3 = const <std::option::Option<u32> as std::ops::Try>::into_result(move _4) -> [return: bb2, unwind: bb3];
2019-07-05T11:45:26.9289483Z     }
2019-07-05T11:45:26.9289570Z     bb1 (cleanup): {
2019-07-05T11:45:26.9289634Z         resume;
2019-07-05T11:45:26.9289768Z     bb2: {
2019-07-05T11:45:26.9289845Z         StorageDead(_4);
2019-07-05T11:45:26.9289845Z         StorageDead(_4);
2019-07-05T11:45:26.9289910Z         _5 = discriminant(_3);
2019-07-05T11:45:26.9290853Z         switchInt(move _5) -> [0isize: bb10, 1isize: bb5, otherwise: bb4];
2019-07-05T11:45:26.9290945Z     }
2019-07-05T11:45:26.9291027Z     bb3 (cleanup): {
2019-07-05T11:45:26.9291260Z         drop(_2) -> bb1;
2019-07-05T11:45:26.9291401Z     bb4: {
2019-07-05T11:45:26.9291477Z         unreachable;
2019-07-05T11:45:26.9291538Z     }
2019-07-05T11:45:26.9291626Z     bb5: {
2019-07-05T11:45:26.9291626Z     bb5: {
2019-07-05T11:45:26.9291685Z         StorageLive(_6);
2019-07-05T11:45:26.9291781Z         _6 = ((_3 as Err).0: std::option::NoneError);
2019-07-05T11:45:26.9291852Z         StorageLive(_8);
2019-07-05T11:45:26.9294209Z         StorageLive(_9);
2019-07-05T11:45:26.9294317Z         _9 = _6;
2019-07-05T11:45:26.9296466Z         _8 = const <std::option::NoneError as std::convert::From<std::option::NoneError>>::from(move _9) -> [return: bb7, unwind: bb3];
2019-07-05T11:45:26.9296650Z     bb6: {
2019-07-05T11:45:26.9296706Z         return;
2019-07-05T11:45:26.9296779Z     }
2019-07-05T11:45:26.9296832Z     bb7: {
2019-07-05T11:45:26.9296832Z     bb7: {
2019-07-05T11:45:26.9296904Z         StorageDead(_9);
2019-07-05T11:45:26.9297237Z         _0 = const <std::option::Option<std::boxed::Box<u32>> as std::ops::Try>::from_error(move _8) -> [return: bb8, unwind: bb3];
2019-07-05T11:45:26.9297393Z     bb8: {
2019-07-05T11:45:26.9297466Z         StorageDead(_8);
2019-07-05T11:45:26.9297704Z         StorageDead(_6);
2019-07-05T11:45:26.9297704Z         StorageDead(_6);
2019-07-05T11:45:26.9297961Z         drop(_2) -> bb9;
2019-07-05T11:45:26.9298102Z     bb9: {
2019-07-05T11:45:26.9298156Z         StorageDead(_2);
2019-07-05T11:45:26.9298232Z         StorageDead(_1);
2019-07-05T11:45:26.9298291Z         StorageDead(_3);
2019-07-05T11:45:26.9298291Z         StorageDead(_3);
2019-07-05T11:45:26.9298511Z         goto -> bb6;
2019-07-05T11:45:26.9298643Z     bb10: {
2019-07-05T11:45:26.9298699Z         StorageLive(_10);
2019-07-05T11:45:26.9298699Z         StorageLive(_10);
2019-07-05T11:45:26.9298778Z         _10 = ((_3 as Ok).0: u32);
2019-07-05T11:45:26.9298841Z         (*_2) = _10;
2019-07-05T11:45:26.9298920Z         StorageDead(_10);
2019-07-05T11:45:26.9298979Z         _1 = move _2;
2019-07-05T11:45:26.9299776Z         drop(_2) -> [return: bb12, unwind: bb11];
2019-07-05T11:45:26.9299851Z     }
2019-07-05T11:45:26.9299930Z     bb11 (cleanup): {
2019-07-05T11:45:26.9300370Z         drop(_1) -> bb1;
2019-07-05T11:45:26.9300526Z     bb12: {
2019-07-05T11:45:26.9300603Z         StorageDead(_2);
2019-07-05T11:45:26.9300603Z         StorageDead(_2);
2019-07-05T11:45:26.9300798Z         _0 = std::option::Option::<std::boxed::Box<u32>>::Some(move _1,);
2019-07-05T11:45:26.9301095Z         drop(_1) -> bb13;
2019-07-05T11:45:26.9301235Z     bb13: {
2019-07-05T11:45:26.9301294Z         StorageDead(_1);
2019-07-05T11:45:26.9301375Z         StorageDead(_3);
2019-07-05T11:45:26.9301375Z         StorageDead(_3);
2019-07-05T11:45:26.9301589Z         goto -> bb6;
2019-07-05T11:45:26.9301728Z }
2019-07-05T11:45:26.9301803Z Actual:
2019-07-05T11:45:26.9301803Z Actual:
2019-07-05T11:45:26.9302057Z fn  test() -> std::option::Option<std::boxed::Box<u32>> {
2019-07-05T11:45:26.9302157Z     let mut _0: std::option::Option<std::boxed::Box<u32>>;
2019-07-05T11:45:26.9302231Z     let mut _1: std::boxed::Box<u32>;
2019-07-05T11:45:26.9302316Z     let mut _2: std::boxed::Box<u32>;
2019-07-05T11:45:26.9302395Z     let mut _3: std::result::Result<u32, std::option::NoneError>;
2019-07-05T11:45:26.9302489Z     let mut _4: std::option::Option<u32>;
2019-07-05T11:45:26.9302570Z     let mut _5: isize;
2019-07-05T11:45:26.9302660Z     let _6: std::option::NoneError;
2019-07-05T11:45:26.9302729Z     let mut _7: !;
2019-07-05T11:45:26.9302813Z     let mut _8: std::option::NoneError;
2019-07-05T11:45:26.9302880Z     let mut _9: std::option::NoneError;
2019-07-05T11:45:26.9302963Z     let _10: u32;
2019-07-05T11:45:26.9303026Z     scope 1 {
2019-07-05T11:45:26.9303104Z         scope 2 {
2019-07-05T11:45:26.9303239Z     }
2019-07-05T11:45:26.9303297Z     scope 3 {
2019-07-05T11:45:26.9303374Z         scope 4 {
2019-07-05T11:45:26.9303436Z         }
2019-07-05T11:45:26.9303436Z         }
2019-07-05T11:45:26.9303509Z     }
2019-07-05T11:45:26.9303565Z     bb0: {
2019-07-05T11:45:26.9303642Z         StorageLive(_1);
2019-07-05T11:45:26.9303706Z         StorageLive(_2);
2019-07-05T11:45:26.9303788Z         _2 = Box(u32);
2019-07-05T11:45:26.9303851Z         StorageLive(_3);
2019-07-05T11:45:26.9303932Z         StorageLive(_4);
2019-07-05T11:45:26.9304009Z         _4 = std::option::Option::<u32>::None;
2019-07-05T11:45:26.9304353Z         _3 = const <std::option::Option<u32> as std::ops::Try>::into_result(move _4) -> bb1;
2019-07-05T11:45:26.9304511Z     bb1: {
2019-07-05T11:45:26.9304570Z         StorageDead(_4);
2019-07-05T11:45:26.9304570Z         StorageDead(_4);
2019-07-05T11:45:26.9305609Z         _5 = discriminant(_3);
2019-07-05T11:45:26.9305972Z         switchInt(move _5) -> [0isize: bb8, 1isize: bb3, otherwise: bb2];
2019-07-05T11:45:26.9306119Z     bb2: {
2019-07-05T11:45:26.9306191Z         unreachable;
2019-07-05T11:45:26.9306248Z     }
2019-07-05T11:45:26.9306318Z     bb3: {
2019-07-05T11:45:26.9306318Z     bb3: {
2019-07-05T11:45:26.9306372Z         StorageLive(_6);
2019-07-05T11:45:26.9306456Z         _6 = ((_3 as Err).0: std::option::NoneError);
2019-07-05T11:45:26.9306522Z         StorageLive(_8);
2019-07-05T11:45:26.9306598Z         StorageLive(_9);
2019-07-05T11:45:26.9307613Z         _9 = _6;
2019-07-05T11:45:26.9308058Z         _8 = const <std::option::NoneError as std::convert::From<std::option::NoneError>>::from(move _9) -> bb5;
2019-07-05T11:45:26.9308388Z     bb4: {
2019-07-05T11:45:26.9308445Z         return;
2019-07-05T11:45:26.9308521Z     }
2019-07-05T11:45:26.9308575Z     bb5: {
2019-07-05T11:45:26.9308575Z     bb5: {
2019-07-05T11:45:26.9308649Z         StorageDead(_9);
2019-07-05T11:45:26.9308991Z         _0 = const <std::option::Option<std::boxed::Box<u32>> as std::ops::Try>::from_error(move _8) -> bb6;
2019-07-05T11:45:26.9309687Z     bb6: {
2019-07-05T11:45:26.9309763Z         StorageDead(_8);
2019-07-05T11:45:26.9309827Z         StorageDead(_6);
2019-07-05T11:45:26.9309827Z         StorageDead(_6);
2019-07-05T11:45:26.9310103Z         drop(_2) -> bb7;
2019-07-05T11:45:26.9310310Z     bb7: {
2019-07-05T11:45:26.9310389Z         StorageDead(_2);
2019-07-05T11:45:26.9310454Z         StorageDead(_1);
2019-07-05T11:45:26.9310535Z         StorageDead(_3);
2019-07-05T11:45:26.9310535Z         StorageDead(_3);
2019-07-05T11:45:26.9310761Z         goto -> bb4;
2019-07-05T11:45:26.9310912Z     bb8: {
2019-07-05T11:45:26.9310991Z         StorageLive(_10);
2019-07-05T11:45:26.9310991Z         StorageLive(_10);
2019-07-05T11:45:26.9311156Z         _10 = ((_3 as Ok).0: u32);
2019-07-05T11:45:26.9311251Z         (*_2) = _10;
2019-07-05T11:45:26.9311314Z         StorageDead(_10);
2019-07-05T11:45:26.9311396Z         _1 = move _2;
2019-07-05T11:45:26.9311642Z         drop(_2) -> bb9;
2019-07-05T11:45:26.9311784Z     bb9: {
2019-07-05T11:45:26.9311842Z         StorageDead(_2);
2019-07-05T11:45:26.9311842Z         StorageDead(_2);
2019-07-05T11:45:26.9311935Z         _0 = std::option::Option::<std::boxed::Box<u32>>::Some(move _1,);
2019-07-05T11:45:26.9312184Z         drop(_1) -> bb10;
2019-07-05T11:45:26.9312323Z     bb10: {
2019-07-05T11:45:26.9312384Z         StorageDead(_1);
2019-07-05T11:45:26.9312464Z         StorageDead(_3);
2019-07-05T11:45:26.9312464Z         StorageDead(_3);
2019-07-05T11:45:26.9312679Z         goto -> bb4;
2019-07-05T11:45:26.9313187Z }', src/tools/compiletest/src/runtest.rs:3197:13
2019-07-05T11:45:26.9313296Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-05T11:45:26.9313359Z 
2019-07-05T11:45:26.9313394Z 
---
2019-07-05T11:45:26.9314099Z 
2019-07-05T11:45:26.9314388Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-05T11:45:26.9314444Z 
2019-07-05T11:45:26.9314477Z 
2019-07-05T11:45:26.9316338Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "mir-opt" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-05T11:45:26.9317063Z 
2019-07-05T11:45:26.9317102Z 
2019-07-05T11:45:26.9318618Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2019-07-05T11:45:26.9318954Z Build completed unsuccessfully in 1:24:05
2019-07-05T11:45:26.9318954Z Build completed unsuccessfully in 1:24:05
2019-07-05T11:45:30.6545870Z ##[error]Bash exited with code '1'.
2019-07-05T11:45:30.6582396Z ##[section]Starting: Upload CPU usage statistics
2019-07-05T11:45:30.6589171Z ==============================================================================
2019-07-05T11:45:30.6589280Z Task         : Bash
2019-07-05T11:45:30.6589349Z Description  : Run a Bash script on macOS, Linux, or Windows
