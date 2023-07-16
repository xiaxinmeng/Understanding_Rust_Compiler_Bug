plain
2019-11-26T20:26:51.6870866Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-26T20:26:51.6903354Z ##[command]git config gc.auto 0
2019-11-26T20:26:51.6908846Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-26T20:26:51.6913595Z ##[command]git config --get-all http.proxy
2019-11-26T20:26:51.6919731Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66654/merge:refs/remotes/pull/66654/merge
---
2019-11-26T21:25:38.2143962Z .................................................................................................... 1600/9295
2019-11-26T21:25:42.8018807Z .................................................................................................... 1700/9295
2019-11-26T21:25:55.6811304Z ................................i................................................................... 1800/9295
2019-11-26T21:26:02.5562769Z .................................................................................................... 1900/9295
2019-11-26T21:26:16.3348453Z .................iiiii.............................................................................. 2000/9295
2019-11-26T21:26:26.0703150Z .................................................................................................... 2200/9295
2019-11-26T21:26:28.5175957Z ..........F......................................................................................... 2300/9295
2019-11-26T21:26:33.5075819Z .................................................................................................... 2400/9295
2019-11-26T21:26:54.6075939Z .................................................................................................... 2500/9295
---
2019-11-26T21:29:31.3576682Z ..................i...............i................................................................. 4800/9295
2019-11-26T21:29:41.3974313Z .................................................................................................... 4900/9295
2019-11-26T21:29:47.2218832Z .................................................................................................... 5000/9295
2019-11-26T21:29:56.5631465Z .................................................................................................... 5100/9295
2019-11-26T21:30:02.7862513Z .......................ii.ii...........i............................................................ 5200/9295
2019-11-26T21:30:11.7894951Z .................................................................................................... 5400/9295
2019-11-26T21:30:22.4643240Z .................................................................................................... 5500/9295
2019-11-26T21:30:29.3904765Z .....i.............................................................................................. 5600/9295
2019-11-26T21:30:35.4539607Z .................................................................................................... 5700/9295
2019-11-26T21:30:35.4539607Z .................................................................................................... 5700/9295
2019-11-26T21:30:45.9135146Z ...........................................................................................ii...i..i 5800/9295
2019-11-26T21:30:58.2742645Z i...........i....................................................................................... 5900/9295
2019-11-26T21:31:16.0966157Z .................................................................................................... 6100/9295
2019-11-26T21:31:22.2939656Z .................................................................................................... 6200/9295
2019-11-26T21:31:22.2939656Z .................................................................................................... 6200/9295
2019-11-26T21:31:35.6874917Z ..............i..ii................................................................................. 6300/9295
2019-11-26T21:31:54.8735439Z ..................................................................................i................. 6500/9295
2019-11-26T21:31:57.1056972Z .................................................................................................... 6600/9295
2019-11-26T21:31:59.3626232Z .........................................................................i.......................... 6700/9295
2019-11-26T21:32:02.1179085Z .................................................................................................... 6800/9295
---
2019-11-26T21:36:43.5513322Z 
2019-11-26T21:36:43.5514067Z ---- [ui] ui/error-codes/E0017.rs stdout ----
2019-11-26T21:36:43.5514324Z diff of stderr:
2019-11-26T21:36:43.5514574Z 
2019-11-26T21:36:43.5515019Z 4 LL | const CR: &'static mut i32 = &mut C;
2019-11-26T21:36:43.5515245Z 5    |                              ^^^^^^ constants require immutable values
2019-11-26T21:36:43.5516188Z - error[E0017]: references in statics may only refer to immutable values
2019-11-26T21:36:43.5516639Z -   --> $DIR/E0017.rs:5:39
2019-11-26T21:36:43.5517086Z -    |
2019-11-26T21:36:43.5517086Z -    |
2019-11-26T21:36:43.5517510Z - LL | static STATIC_REF: &'static mut i32 = &mut X;
2019-11-26T21:36:43.5517986Z -    |                                       ^^^^^^ statics require immutable values
2019-11-26T21:36:43.5518607Z 13 error[E0596]: cannot borrow immutable static item `X` as mutable
2019-11-26T21:36:43.5518999Z 14   --> $DIR/E0017.rs:5:39
2019-11-26T21:36:43.5519201Z 15    |
2019-11-26T21:36:43.5519343Z 
---
2019-11-26T21:36:43.5521491Z 28 For more information about an error, try `rustc --explain E0017`.
2019-11-26T21:36:43.5522063Z 
2019-11-26T21:36:43.5524433Z 
2019-11-26T21:36:43.5524808Z The actual stderr differed from the expected stderr.
2019-11-26T21:36:43.5525381Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0017/E0017.stderr
2019-11-26T21:36:43.5526288Z To update references, rerun the tests and pass the `--bless` flag
2019-11-26T21:36:43.5526798Z To only update this specific test, also pass `--test-args error-codes/E0017.rs`
2019-11-26T21:36:43.5527181Z error: 1 errors occurred comparing output.
2019-11-26T21:36:43.5527345Z status: exit code: 1
2019-11-26T21:36:43.5527345Z status: exit code: 1
2019-11-26T21:36:43.5528262Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0017.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0017" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0017/auxiliary" "-A" "unused"
2019-11-26T21:36:43.5528962Z ------------------------------------------
2019-11-26T21:36:43.5529154Z 
2019-11-26T21:36:43.5529541Z ------------------------------------------
2019-11-26T21:36:43.5529764Z stderr:
2019-11-26T21:36:43.5529764Z stderr:
2019-11-26T21:36:43.5530140Z ------------------------------------------
2019-11-26T21:36:43.5530373Z error[E0017]: references in constants may only refer to immutable values
2019-11-26T21:36:43.5530796Z   --> /checkout/src/test/ui/error-codes/E0017.rs:4:30
2019-11-26T21:36:43.5531008Z    |
2019-11-26T21:36:43.5531404Z LL | const CR: &'static mut i32 = &mut C; //~ ERROR E0017
2019-11-26T21:36:43.5531644Z    |                              ^^^^^^ constants require immutable values
2019-11-26T21:36:43.5532002Z error[E0596]: cannot borrow immutable static item `X` as mutable
2019-11-26T21:36:43.5532549Z   --> /checkout/src/test/ui/error-codes/E0017.rs:5:39
2019-11-26T21:36:43.5535121Z    |
2019-11-26T21:36:43.5535121Z    |
2019-11-26T21:36:43.5538170Z LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0017
2019-11-26T21:36:43.5538722Z 
2019-11-26T21:36:43.5538915Z error[E0017]: references in statics may only refer to immutable values
2019-11-26T21:36:43.5539353Z   --> /checkout/src/test/ui/error-codes/E0017.rs:7:38
2019-11-26T21:36:43.5545169Z    |
2019-11-26T21:36:43.5545169Z    |
2019-11-26T21:36:43.5546568Z LL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR E0017
2019-11-26T21:36:43.5546956Z    |                                      ^^^^^^ statics require immutable values
2019-11-26T21:36:43.5547379Z error: aborting due to 3 previous errors
2019-11-26T21:36:43.5547541Z 
2019-11-26T21:36:43.5547726Z Some errors have detailed explanations: E0017, E0596.
2019-11-26T21:36:43.5548303Z For more information about an error, try `rustc --explain E0017`.
2019-11-26T21:36:43.5548303Z For more information about an error, try `rustc --explain E0017`.
2019-11-26T21:36:43.5548549Z 
2019-11-26T21:36:43.5549005Z ------------------------------------------
2019-11-26T21:36:43.5549217Z 
2019-11-26T21:36:43.5549378Z 
2019-11-26T21:36:43.5549812Z ---- [ui] ui/error-codes/E0388.rs stdout ----
2019-11-26T21:36:43.5550074Z diff of stderr:
2019-11-26T21:36:43.5550259Z 
2019-11-26T21:36:43.5550700Z 4 LL | const CR: &'static mut i32 = &mut C;
2019-11-26T21:36:43.5550948Z 5    |                              ^^^^^^ constants require immutable values
2019-11-26T21:36:43.5551597Z - error[E0017]: references in statics may only refer to immutable values
2019-11-26T21:36:43.5552089Z -   --> $DIR/E0388.rs:5:39
2019-11-26T21:36:43.5552536Z -    |
2019-11-26T21:36:43.5552536Z -    |
2019-11-26T21:36:43.5553016Z - LL | static STATIC_REF: &'static mut i32 = &mut X;
2019-11-26T21:36:43.5553535Z -    |                                       ^^^^^^ statics require immutable values
2019-11-26T21:36:43.5554500Z 13 error[E0596]: cannot borrow immutable static item `X` as mutable
2019-11-26T21:36:43.5554946Z 14   --> $DIR/E0388.rs:5:39
2019-11-26T21:36:43.5555207Z 15    |
2019-11-26T21:36:43.5555365Z 
---
2019-11-26T21:36:43.5558348Z 28 For more information about an error, try `rustc --explain E0017`.
2019-11-26T21:36:43.5558587Z 
2019-11-26T21:36:43.5558757Z 
2019-11-26T21:36:43.5558939Z The actual stderr differed from the expected stderr.
2019-11-26T21:36:43.5559489Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388/E0388.stderr
2019-11-26T21:36:43.5560152Z To update references, rerun the tests and pass the `--bless` flag
2019-11-26T21:36:43.5560716Z To only update this specific test, also pass `--test-args error-codes/E0388.rs`
2019-11-26T21:36:43.5561130Z error: 1 errors occurred comparing output.
2019-11-26T21:36:43.5561311Z status: exit code: 1
2019-11-26T21:36:43.5561311Z status: exit code: 1
2019-11-26T21:36:43.5562364Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0388.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388/auxiliary" "-A" "unused"
2019-11-26T21:36:43.5563100Z ------------------------------------------
2019-11-26T21:36:43.5563471Z 
2019-11-26T21:36:43.5564075Z ------------------------------------------
2019-11-26T21:36:43.5564328Z stderr:
2019-11-26T21:36:43.5564328Z stderr:
2019-11-26T21:36:43.5564826Z ------------------------------------------
2019-11-26T21:36:43.5565084Z error[E0017]: references in constants may only refer to immutable values
2019-11-26T21:36:43.5570187Z   --> /checkout/src/test/ui/error-codes/E0388.rs:4:30
2019-11-26T21:36:43.5570550Z    |
2019-11-26T21:36:43.5571798Z LL | const CR: &'static mut i32 = &mut C; //~ ERROR E0017
2019-11-26T21:36:43.5575935Z    |                              ^^^^^^ constants require immutable values
2019-11-26T21:36:43.5576072Z error[E0596]: cannot borrow immutable static item `X` as mutable
2019-11-26T21:36:43.5576499Z   --> /checkout/src/test/ui/error-codes/E0388.rs:5:39
2019-11-26T21:36:43.5576556Z    |
2019-11-26T21:36:43.5576556Z    |
2019-11-26T21:36:43.5576870Z LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0017
2019-11-26T21:36:43.5576982Z 
2019-11-26T21:36:43.5577034Z error[E0017]: references in statics may only refer to immutable values
2019-11-26T21:36:43.5577315Z   --> /checkout/src/test/ui/error-codes/E0388.rs:7:38
2019-11-26T21:36:43.5577368Z    |
2019-11-26T21:36:43.5577368Z    |
2019-11-26T21:36:43.5577637Z LL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR E0017
2019-11-26T21:36:43.5577717Z    |                                      ^^^^^^ statics require immutable values
2019-11-26T21:36:43.5577802Z error: aborting due to 3 previous errors
2019-11-26T21:36:43.5577835Z 
2019-11-26T21:36:43.5577899Z Some errors have detailed explanations: E0017, E0596.
2019-11-26T21:36:43.5578174Z For more information about an error, try `rustc --explain E0017`.
---
2019-11-26T21:36:43.5580015Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-26T21:36:43.5580079Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-26T21:36:43.5580116Z 
2019-11-26T21:36:43.5580160Z 
2019-11-26T21:36:43.5581853Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-26T21:36:43.5582134Z 
2019-11-26T21:36:43.5582166Z 
2019-11-26T21:36:43.5582318Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-26T21:36:43.5582385Z Build completed unsuccessfully in 1:04:00
2019-11-26T21:36:43.5582385Z Build completed unsuccessfully in 1:04:00
2019-11-26T21:36:43.5604435Z == clock drift check ==
2019-11-26T21:36:43.5621323Z   local time: Tue Nov 26 21:36:43 UTC 2019
2019-11-26T21:36:44.0965552Z   network time: Tue, 26 Nov 2019 21:36:44 GMT
2019-11-26T21:36:44.0974798Z == end clock drift check ==
2019-11-26T21:36:44.9331703Z 
2019-11-26T21:36:44.9471412Z ##[error]Bash exited with code '1'.
2019-11-26T21:36:44.9526744Z ##[section]Starting: Checkout
2019-11-26T21:36:44.9528990Z ==============================================================================
2019-11-26T21:36:44.9529077Z Task         : Get sources
2019-11-26T21:36:44.9529133Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
