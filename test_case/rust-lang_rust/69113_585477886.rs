plain
2020-02-12T22:55:47.7437919Z ========================== Starting Command Output ===========================
2020-02-12T22:55:47.7450349Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/516ee814-f421-4c94-9c65-ec7e7a79bd9d.sh
2020-02-12T22:55:47.7616335Z 
2020-02-12T22:55:47.7655393Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-12T22:55:47.7659632Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69113/merge to s
2020-02-12T22:55:47.7661053Z Task         : Get sources
2020-02-12T22:55:47.7661079Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-12T22:55:47.7661103Z Version      : 1.0.0
2020-02-12T22:55:47.7661163Z Author       : Microsoft
---
2020-02-12T22:55:48.5314011Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-12T22:55:48.5331440Z ##[command]git config gc.auto 0
2020-02-12T22:55:48.5338376Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-12T22:55:48.5341128Z ##[command]git config --get-all http.proxy
2020-02-12T22:55:48.5350068Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69113/merge:refs/remotes/pull/69113/merge
---
2020-02-12T23:44:18.2581283Z ......................................................FF............................................ 1700/9631
2020-02-12T23:44:21.8410515Z .................................................................................................... 1800/9631
2020-02-12T23:44:30.8650851Z .............................i...................................................................... 1900/9631
2020-02-12T23:44:36.3021395Z .................................................................................................... 2000/9631
2020-02-12T23:44:46.7776630Z ...................iiiii............................................................................ 2100/9631
2020-02-12T23:44:53.8701572Z .................................................................................................... 2300/9631
2020-02-12T23:44:55.6160308Z .................................................................................................... 2400/9631
2020-02-12T23:44:59.1932556Z .................................................................................................... 2500/9631
2020-02-12T23:45:14.2212305Z .................................................................................................... 2600/9631
---
2020-02-12T23:47:28.0051041Z .......................................................................i...............i............ 4900/9631
2020-02-12T23:47:35.4986071Z .................................................................................................... 5000/9631
2020-02-12T23:47:43.0746399Z .................................................................................................... 5100/9631
2020-02-12T23:47:47.7685085Z .............i...................................................................................... 5200/9631
2020-02-12T23:47:58.9337236Z .......................................................................................ii.ii........ 5300/9631
2020-02-12T23:48:02.8358443Z i...i............................................................................................... 5400/9631
2020-02-12T23:48:14.6850934Z .................................................................................................... 5600/9631
2020-02-12T23:48:22.9670330Z ............................................................................i....................... 5700/9631
2020-02-12T23:48:30.5234775Z .................................................................................................... 5800/9631
2020-02-12T23:48:36.7540411Z ..........................................................................F......................... 5900/9631
2020-02-12T23:48:36.7540411Z ..........................................................................F......................... 5900/9631
2020-02-12T23:48:46.2710200Z ....................................................................ii...i..ii...........i.......... 6000/9631
2020-02-12T23:49:07.2246022Z .................................................................................................... 6200/9631
2020-02-12T23:49:11.3219527Z .................................................................................................... 6300/9631
2020-02-12T23:49:15.3469828Z ................................................................................................i..i 6400/9631
2020-02-12T23:49:27.9087212Z i................................................................................................... 6500/9631
---
2020-02-12T23:51:10.1552158Z .................................................................................................... 7600/9631
2020-02-12T23:51:13.7840806Z .................................................................................................... 7700/9631
2020-02-12T23:51:18.6625668Z .................................................................................................... 7800/9631
2020-02-12T23:51:25.2481519Z .................................................................................................... 7900/9631
2020-02-12T23:51:32.0637423Z .........................................................................iiiiiii.i.................. 8000/9631
2020-02-12T23:51:44.6709995Z .............i......i............................................................................... 8200/9631
2020-02-12T23:51:49.4454619Z .................................................................................................... 8300/9631
2020-02-12T23:52:01.0980311Z .................................................................................................... 8400/9631
2020-02-12T23:52:09.7913905Z .................................................................................................... 8500/9631
---
2020-02-12T23:53:46.0022798Z 
2020-02-12T23:53:46.0023113Z 1 error[E0493]: destructors cannot be evaluated at compile-time
2020-02-12T23:53:46.0023422Z +   --> $DIR/const_let.rs:13:33
2020-02-12T23:53:46.0023555Z +    |
2020-02-12T23:53:46.0023689Z + LL | const X2: FakeNeedsDrop = { let x; x = FakeNeedsDrop; x };
2020-02-12T23:53:46.0023834Z +    |                                 ^ constants cannot evaluate destructors
2020-02-12T23:53:46.0024262Z + error[E0493]: destructors cannot be evaluated at compile-time
2020-02-12T23:53:46.0024545Z 2   --> $DIR/const_let.rs:16:32
2020-02-12T23:53:46.0024671Z 3    |
2020-02-12T23:53:46.0024671Z 3    |
2020-02-12T23:53:46.0024805Z 4 LL | const Y: FakeNeedsDrop = { let mut x = FakeNeedsDrop; x = FakeNeedsDrop; x };
2020-02-12T23:53:46.0025032Z 5    |                                ^^^^^ constants cannot evaluate destructors
2020-02-12T23:53:46.0025168Z 6 
2020-02-12T23:53:46.0025479Z 7 error[E0493]: destructors cannot be evaluated at compile-time
2020-02-12T23:53:46.0025757Z +   --> $DIR/const_let.rs:16:32
2020-02-12T23:53:46.0025757Z +   --> $DIR/const_let.rs:16:32
2020-02-12T23:53:46.0025897Z +    |
2020-02-12T23:53:46.0026017Z + LL | const Y: FakeNeedsDrop = { let mut x = FakeNeedsDrop; x = FakeNeedsDrop; x };
2020-02-12T23:53:46.0026395Z + 
2020-02-12T23:53:46.0026703Z + error[E0493]: destructors cannot be evaluated at compile-time
2020-02-12T23:53:46.0026998Z 8   --> $DIR/const_let.rs:20:33
2020-02-12T23:53:46.0027122Z 9    |
2020-02-12T23:53:46.0027122Z 9    |
2020-02-12T23:53:46.0027244Z 10 LL | const Y2: FakeNeedsDrop = { let mut x; x = FakeNeedsDrop; x = FakeNeedsDrop; x };
2020-02-12T23:53:46.0027479Z 11    |                                 ^^^^^ constants cannot evaluate destructors
2020-02-12T23:53:46.0027595Z 12 
2020-02-12T23:53:46.0027914Z 13 error[E0493]: destructors cannot be evaluated at compile-time
2020-02-12T23:53:46.0028195Z +   --> $DIR/const_let.rs:20:33
2020-02-12T23:53:46.0028195Z +   --> $DIR/const_let.rs:20:33
2020-02-12T23:53:46.0028320Z +    |
2020-02-12T23:53:46.0028456Z + LL | const Y2: FakeNeedsDrop = { let mut x; x = FakeNeedsDrop; x = FakeNeedsDrop; x };
2020-02-12T23:53:46.0028692Z + 
2020-02-12T23:53:46.0028998Z + error[E0493]: destructors cannot be evaluated at compile-time
2020-02-12T23:53:46.0029286Z 14   --> $DIR/const_let.rs:24:21
2020-02-12T23:53:46.0029431Z 15    |
2020-02-12T23:53:46.0029431Z 15    |
2020-02-12T23:53:46.0029549Z 16 LL | const Z: () = { let mut x = None; x = Some(FakeNeedsDrop); };
2020-02-12T23:53:46.0029650Z 
2020-02-12T23:53:46.0029766Z 22 LL | const Z2: () = { let mut x; x = None; x = Some(FakeNeedsDrop); };
2020-02-12T23:53:46.0030016Z 24 
2020-02-12T23:53:46.0030314Z - error: aborting due to 4 previous errors
2020-02-12T23:53:46.0030621Z + error[E0493]: destructors cannot be evaluated at compile-time
2020-02-12T23:53:46.0030898Z +   --> $DIR/const_let.rs:28:22
2020-02-12T23:53:46.0030898Z +   --> $DIR/const_let.rs:28:22
2020-02-12T23:53:46.0031045Z +    |
2020-02-12T23:53:46.0031162Z + LL | const Z2: () = { let mut x; x = None; x = Some(FakeNeedsDrop); };
2020-02-12T23:53:46.0031411Z + 
2020-02-12T23:53:46.0031525Z + error: aborting due to 8 previous errors
2020-02-12T23:53:46.0031637Z 26 
2020-02-12T23:53:46.0032109Z 27 For more information about this error, try `rustc --explain E0493`.
2020-02-12T23:53:46.0032109Z 27 For more information about this error, try `rustc --explain E0493`.
2020-02-12T23:53:46.0032254Z 28 
2020-02-12T23:53:46.0032353Z 
2020-02-12T23:53:46.0032468Z 
2020-02-12T23:53:46.0032587Z The actual stderr differed from the expected stderr.
2020-02-12T23:53:46.0032940Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_let/const_let.stderr
2020-02-12T23:53:46.0033372Z To update references, rerun the tests and pass the `--bless` flag
2020-02-12T23:53:46.0033703Z To only update this specific test, also pass `--test-args consts/const-eval/const_let.rs`
2020-02-12T23:53:46.0033972Z error: 1 errors occurred comparing output.
2020-02-12T23:53:46.0034089Z status: exit code: 1
2020-02-12T23:53:46.0034089Z status: exit code: 1
2020-02-12T23:53:46.0034869Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_let.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_let" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_let/auxiliary" "-A" "unused"
2020-02-12T23:53:46.0035334Z ------------------------------------------
2020-02-12T23:53:46.0035450Z 
2020-02-12T23:53:46.0035725Z ------------------------------------------
2020-02-12T23:53:46.0035868Z stderr:
2020-02-12T23:53:46.0035868Z stderr:
2020-02-12T23:53:46.0036141Z ------------------------------------------
2020-02-12T23:53:46.0036446Z error[E0493]: destructors cannot be evaluated at compile-time
2020-02-12T23:53:46.0036810Z   --> /checkout/src/test/ui/consts/const-eval/const_let.rs:13:33
2020-02-12T23:53:46.0037262Z    |
2020-02-12T23:53:46.0037569Z LL | const X2: FakeNeedsDrop = { let x; x = FakeNeedsDrop; x };
2020-02-12T23:53:46.0037618Z    |                                 ^ constants cannot evaluate destructors
2020-02-12T23:53:46.0037930Z error[E0493]: destructors cannot be evaluated at compile-time
2020-02-12T23:53:46.0038254Z   --> /checkout/src/test/ui/consts/const-eval/const_let.rs:16:32
2020-02-12T23:53:46.0038387Z    |
2020-02-12T23:53:46.0038387Z    |
2020-02-12T23:53:46.0038486Z LL | const Y: FakeNeedsDrop = { let mut x = FakeNeedsDrop; x = FakeNeedsDrop; x };
2020-02-12T23:53:46.0038608Z 
2020-02-12T23:53:46.0038843Z error[E0493]: destructors cannot be evaluated at compile-time
2020-02-12T23:53:46.0039052Z   --> /checkout/src/test/ui/consts/const-eval/const_let.rs:16:32
2020-02-12T23:53:46.0039092Z    |
2020-02-12T23:53:46.0039092Z    |
2020-02-12T23:53:46.0039149Z LL | const Y: FakeNeedsDrop = { let mut x = FakeNeedsDrop; x = FakeNeedsDrop; x };
2020-02-12T23:53:46.0039338Z 
2020-02-12T23:53:46.0039567Z error[E0493]: destructors cannot be evaluated at compile-time
2020-02-12T23:53:46.0039880Z   --> /checkout/src/test/ui/consts/const-eval/const_let.rs:20:33
2020-02-12T23:53:46.0040007Z    |
2020-02-12T23:53:46.0040007Z    |
2020-02-12T23:53:46.0040099Z LL | const Y2: FakeNeedsDrop = { let mut x; x = FakeNeedsDrop; x = FakeNeedsDrop; x };
2020-02-12T23:53:46.0040264Z 
2020-02-12T23:53:46.0040592Z error[E0493]: destructors cannot be evaluated at compile-time
2020-02-12T23:53:46.0040802Z   --> /checkout/src/test/ui/consts/const-eval/const_let.rs:20:33
2020-02-12T23:53:46.0040931Z    |
2020-02-12T23:53:46.0040931Z    |
2020-02-12T23:53:46.0041024Z LL | const Y2: FakeNeedsDrop = { let mut x; x = FakeNeedsDrop; x = FakeNeedsDrop; x };
2020-02-12T23:53:46.0041192Z 
2020-02-12T23:53:46.0041525Z error[E0493]: destructors cannot be evaluated at compile-time
2020-02-12T23:53:46.0041834Z   --> /checkout/src/test/ui/consts/const-eval/const_let.rs:24:21
2020-02-12T23:53:46.0041961Z    |
2020-02-12T23:53:46.0041961Z    |
2020-02-12T23:53:46.0042042Z LL | const Z: () = { let mut x = None; x = Some(FakeNeedsDrop); };
2020-02-12T23:53:46.0042173Z 
2020-02-12T23:53:46.0042499Z error[E0493]: destructors cannot be evaluated at compile-time
2020-02-12T23:53:46.0042809Z   --> /checkout/src/test/ui/consts/const-eval/const_let.rs:28:22
2020-02-12T23:53:46.0042939Z    |
2020-02-12T23:53:46.0042939Z    |
2020-02-12T23:53:46.0043020Z LL | const Z2: () = { let mut x; x = None; x = Some(FakeNeedsDrop); };
2020-02-12T23:53:46.0043198Z 
2020-02-12T23:53:46.0043431Z error[E0493]: destructors cannot be evaluated at compile-time
2020-02-12T23:53:46.0043748Z   --> /checkout/src/test/ui/consts/const-eval/const_let.rs:28:22
2020-02-12T23:53:46.0043878Z    |
2020-02-12T23:53:46.0043878Z    |
2020-02-12T23:53:46.0043956Z LL | const Z2: () = { let mut x; x = None; x = Some(FakeNeedsDrop); };
2020-02-12T23:53:46.0044121Z 
2020-02-12T23:53:46.0044231Z error: aborting due to 8 previous errors
2020-02-12T23:53:46.0044256Z 
2020-02-12T23:53:46.0044508Z For more information about this error, try `rustc --explain E0493`.
---
2020-02-12T23:53:46.0050891Z 28 
2020-02-12T23:53:46.0050948Z 
2020-02-12T23:53:46.0050987Z 
2020-02-12T23:53:46.0051048Z The actual stderr differed from the expected stderr.
2020-02-12T23:53:46.0051341Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/drop-failure/drop-failure.stderr
2020-02-12T23:53:46.0051736Z To update references, rerun the tests and pass the `--bless` flag
2020-02-12T23:53:46.0052071Z To only update this specific test, also pass `--test-args consts/control-flow/drop-failure.rs`
2020-02-12T23:53:46.0052279Z error: 1 errors occurred comparing output.
2020-02-12T23:53:46.0052335Z status: exit code: 1
2020-02-12T23:53:46.0052335Z status: exit code: 1
2020-02-12T23:53:46.0053135Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/control-flow/drop-failure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/drop-failure" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/drop-failure/auxiliary" "-A" "unused"
2020-02-12T23:53:46.0053523Z ------------------------------------------
2020-02-12T23:53:46.0053637Z 
2020-02-12T23:53:46.0053888Z ------------------------------------------
2020-02-12T23:53:46.0053942Z stderr:
---
2020-02-12T23:53:46.0055254Z 
2020-02-12T23:53:46.0055488Z error[E0493]: destructors cannot be evaluated at compile-time
2020-02-12T23:53:46.0055701Z   --> /checkout/src/test/ui/consts/control-flow/drop-failure.rs:20:9
2020-02-12T23:53:46.0055742Z    |
2020-02-12T23:53:46.0055786Z LL |     let vec_tuple = (Vec::new(),);
2020-02-12T23:53:46.0055871Z 
2020-02-12T23:53:46.0056073Z error[E0493]: destructors cannot be evaluated at compile-time
2020-02-12T23:53:46.0056297Z   --> /checkout/src/test/ui/consts/control-flow/drop-failure.rs:28:9
2020-02-12T23:53:46.0056427Z    |
---
2020-02-12T23:53:46.0062661Z ---- [ui] ui/consts/control-flow/drop-success.rs stdout ----
2020-02-12T23:53:46.0062773Z 
2020-02-12T23:53:46.0063028Z error: test compilation failed although it shouldn't!
2020-02-12T23:53:46.0063070Z status: exit code: 1
2020-02-12T23:53:46.0063812Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/control-flow/drop-success.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/drop-success/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/drop-success/auxiliary"
2020-02-12T23:53:46.0111150Z ------------------------------------------
2020-02-12T23:53:46.0115271Z 
2020-02-12T23:53:46.0115695Z ------------------------------------------
2020-02-12T23:53:46.0115744Z stderr:
---
2020-02-12T23:53:46.0120929Z 
2020-02-12T23:53:46.0121099Z ---- [ui] ui/mir-dataflow/indirect-mutation-offset.rs stdout ----
2020-02-12T23:53:46.0121135Z diff of stderr:
2020-02-12T23:53:46.0121175Z 
2020-02-12T23:53:46.0121206Z 1 error: rustc_peek: bit not set
2020-02-12T23:53:46.0121536Z +   --> $DIR/indirect-mutation-offset.rs:41:14
2020-02-12T23:53:46.0121586Z 3    |
2020-02-12T23:53:46.0121586Z 3    |
2020-02-12T23:53:46.0121619Z 4 LL |     unsafe { rustc_peek(x) };
2020-02-12T23:53:46.0121690Z 
2020-02-12T23:53:46.0121709Z 
2020-02-12T23:53:46.0121742Z The actual stderr differed from the expected stderr.
2020-02-12T23:53:46.0121975Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/indirect-mutation-offset/indirect-mutation-offset.stderr
2020-02-12T23:53:46.0121975Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/indirect-mutation-offset/indirect-mutation-offset.stderr
2020-02-12T23:53:46.0122178Z To update references, rerun the tests and pass the `--bless` flag
2020-02-12T23:53:46.0122380Z To only update this specific test, also pass `--test-args mir-dataflow/indirect-mutation-offset.rs`
2020-02-12T23:53:46.0122457Z error: 1 errors occurred comparing output.
2020-02-12T23:53:46.0122491Z status: exit code: 1
2020-02-12T23:53:46.0122491Z status: exit code: 1
2020-02-12T23:53:46.0123194Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir-dataflow/indirect-mutation-offset.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/indirect-mutation-offset" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/indirect-mutation-offset/auxiliary" "-A" "unused"
2020-02-12T23:53:46.0123443Z ------------------------------------------
2020-02-12T23:53:46.0123485Z 
2020-02-12T23:53:46.0123641Z ------------------------------------------
2020-02-12T23:53:46.0123673Z stderr:
2020-02-12T23:53:46.0123673Z stderr:
2020-02-12T23:53:46.0123845Z ------------------------------------------
2020-02-12T23:53:46.0123882Z error: rustc_peek: bit not set
2020-02-12T23:53:46.0124230Z    |
2020-02-12T23:53:46.0124230Z    |
2020-02-12T23:53:46.0124267Z LL |     unsafe { rustc_peek(x) }; //~ ERROR rustc_peek: bit not set
2020-02-12T23:53:46.0124324Z 
2020-02-12T23:53:46.0124324Z 
2020-02-12T23:53:46.0124372Z error: stop_after_dataflow ended compilation
2020-02-12T23:53:46.0124425Z error: aborting due to 2 previous errors
2020-02-12T23:53:46.0124485Z 
2020-02-12T23:53:46.0124519Z 
2020-02-12T23:53:46.0124687Z ------------------------------------------
---
2020-02-12T23:53:46.0125911Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-12T23:53:46.0125953Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-12T23:53:46.0125978Z 
2020-02-12T23:53:46.0126019Z 
2020-02-12T23:53:46.0127051Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-12T23:53:46.0127222Z 
2020-02-12T23:53:46.0127241Z 
2020-02-12T23:53:46.0127275Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-12T23:53:46.0127326Z Build completed unsuccessfully in 0:52:07
2020-02-12T23:53:46.0127326Z Build completed unsuccessfully in 0:52:07
2020-02-12T23:53:46.0137157Z == clock drift check ==
2020-02-12T23:53:46.0154563Z   local time: Wed Feb 12 23:53:46 UTC 2020
2020-02-12T23:53:46.3107794Z   network time: Wed, 12 Feb 2020 23:53:46 GMT
2020-02-12T23:53:46.3110919Z == end clock drift check ==
2020-02-12T23:53:46.7782872Z 
2020-02-12T23:53:46.7877994Z ##[error]Bash exited with code '1'.
2020-02-12T23:53:46.7890060Z ##[section]Finishing: Run build
2020-02-12T23:53:46.7909086Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69113/merge to s
2020-02-12T23:53:46.7910995Z Task         : Get sources
2020-02-12T23:53:46.7911063Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-12T23:53:46.7911111Z Version      : 1.0.0
2020-02-12T23:53:46.7911155Z Author       : Microsoft
2020-02-12T23:53:46.7911155Z Author       : Microsoft
2020-02-12T23:53:46.7911222Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-12T23:53:46.7911275Z ==============================================================================
2020-02-12T23:53:47.2046665Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-12T23:53:47.2082897Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69113/merge to s
2020-02-12T23:53:47.2181794Z Cleaning up task key
2020-02-12T23:53:47.2182560Z Start cleaning up orphan processes.
2020-02-12T23:53:47.2280046Z Terminate orphan process: pid (4325) (python)
2020-02-12T23:53:47.2461895Z ##[section]Finishing: Finalize Job
