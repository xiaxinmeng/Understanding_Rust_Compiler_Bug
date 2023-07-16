plain
2020-03-21T17:12:34.1393497Z ========================== Starting Command Output ===========================
2020-03-21T17:12:34.1396171Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/07b2fbd6-8b0f-46b8-9035-2674d456d430.sh
2020-03-21T17:12:34.1396495Z 
2020-03-21T17:12:34.1400524Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-21T17:12:34.1420325Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70212/merge to s
2020-03-21T17:12:34.1424026Z Task         : Get sources
2020-03-21T17:12:34.1424310Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-21T17:12:34.1424583Z Version      : 1.0.0
2020-03-21T17:12:34.1424821Z Author       : Microsoft
---
2020-03-21T17:12:35.1336186Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-21T17:12:35.1344171Z ##[command]git config gc.auto 0
2020-03-21T17:12:35.1350831Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-21T17:12:35.1356154Z ##[command]git config --get-all http.proxy
2020-03-21T17:12:35.1364749Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70212/merge:refs/remotes/pull/70212/merge
---
2020-03-21T17:15:15.7618506Z configure: build.locked-deps    := True
2020-03-21T17:15:15.7618802Z configure: llvm.ccache          := sccache
2020-03-21T17:15:15.7619296Z configure: build.cargo-native-static := True
2020-03-21T17:15:15.7619776Z configure: dist.missing-tools   := True
2020-03-21T17:15:15.7620368Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-21T17:15:15.7620938Z configure: writing `config.toml` in current directory
2020-03-21T17:15:15.7621172Z configure: 
2020-03-21T17:15:15.7621594Z configure: run `python /checkout/x.py --help`
2020-03-21T17:15:15.7621820Z configure: 
---
2020-03-21T18:50:31.6185536Z     Finished release [optimized] target(s) in 1m 45s
2020-03-21T18:50:32.2482254Z Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> wasm32-unknown-emscripten)
2020-03-21T18:50:35.2678631Z 
2020-03-21T18:50:35.2679401Z running 9815 tests
2020-03-21T18:52:02.6809024Z ......ii..i.......i......i.......iiii.....................................ii................i......i 100/9815
2020-03-21T18:52:55.2676576Z i.............i.......................i....iiiiiii.iii.............................................. 200/9815
2020-03-21T18:53:55.7057377Z ........................................................................................ii.......... 400/9815
2020-03-21T18:53:55.7057377Z ........................................................................................ii.......... 400/9815
2020-03-21T18:54:27.1846002Z ...............................................................iii.................................. 500/9815
2020-03-21T18:55:00.8601154Z ........i........................................................iii................................ 600/9815
2020-03-21T18:56:34.9111417Z ...............................................i.................................................... 800/9815
2020-03-21T18:56:47.5798095Z .................................................................................................... 900/9815
2020-03-21T18:57:01.6917655Z .................................................................i.................................. 1000/9815
2020-03-21T18:57:47.4581029Z .................................................................................i........i.i....... 1100/9815
2020-03-21T18:57:47.4581029Z .................................................................................i........i.i....... 1100/9815
2020-03-21T18:58:12.6180865Z ......................................i............................................................. 1200/9815
2020-03-21T18:58:22.4238285Z .............................................................................iiiii.................. 1300/9815
2020-03-21T18:59:20.0055033Z .............................................................................i.......F.............. 1500/9815
2020-03-21T18:59:47.1717734Z .................................................................................................... 1600/9815
2020-03-21T19:00:22.7399180Z .............................................................ii.i.................i................. 1700/9815
2020-03-21T19:00:46.5289282Z ................i..................i.i.............................................................. 1800/9815
2020-03-21T19:00:46.5289282Z ................i..................i.i.............................................................. 1800/9815
2020-03-21T19:01:31.3371692Z ..................................................i......i...................i...................ii. 1900/9815
2020-03-21T19:01:58.3296313Z .................................................................................................... 2000/9815
2020-03-21T19:02:31.5572004Z ...................................................................iiiii.....................i...... 2100/9815
2020-03-21T19:03:22.3134247Z .................................................................................................... 2200/9815
2020-03-21T19:03:32.4771665Z .............................iii...i................................................................ 2300/9815
2020-03-21T19:03:34.8520154Z .................................................................................................... 2400/9815
2020-03-21T19:03:38.1056866Z ...........................................................................iiii....................i 2500/9815
2020-03-21T19:04:14.5903757Z ......................................................................................iiiiiiiiiiiiii 2600/9815
2020-03-21T19:04:21.7457921Z i.........i...........................i..........ii................................................. 2700/9815
2020-03-21T19:05:48.0436093Z ................................i....i.............................................................. 2900/9815
2020-03-21T19:06:21.9138746Z .............i............i..i...................................................................... 3000/9815
2020-03-21T19:06:49.3840221Z ....................................i.....................................................i......... 3100/9815
2020-03-21T19:07:15.1080729Z .................................................................................................... 3200/9815
2020-03-21T19:07:15.1080729Z .................................................................................................... 3200/9815
2020-03-21T19:07:42.1237692Z .................................................................................................... 3300/9815
2020-03-21T19:08:05.7755573Z ..............................................................................................ii.... 3400/9815
2020-03-21T19:08:34.9136856Z ................ii.ii...................i...i................................i...................... 3500/9815
2020-03-21T19:09:50.5839415Z ..........................i...................i........................................i.......i.... 3700/9815
2020-03-21T19:10:30.2077543Z ....i............................i.................................................................. 3800/9815
2020-03-21T19:11:00.8166266Z ................................................i................................................... 3900/9815
2020-03-21T19:11:30.9853580Z ..............i...........................................................i......................... 4000/9815
---
2020-03-21T19:24:27.1608992Z ..............i............... 5400/9815
2020-03-21T19:25:00.3606489Z ................................................................................................i... 5500/9815
2020-03-21T19:25:15.9136851Z .......i..............................i............................................................. 5600/9815
2020-03-21T19:25:40.8281198Z ............i.......................................................i...............i............... 5700/9815
2020-03-21T19:26:21.3480783Z .............................ii.................F.................ii................................ 5800/9815
2020-03-21T19:26:57.0023059Z .........................................i...i...................................................... 5900/9815
2020-03-21T19:27:36.9804223Z .................................................................................................... 6000/9815
2020-03-21T19:28:11.1490398Z ......i.....................................................ii...i..ii...........i.................. 6100/9815
2020-03-21T19:28:45.2795707Z ...........i........i.i............................................................................. 6200/9815
2020-03-21T19:29:01.9009644Z .................................................................................................... 6400/9815
2020-03-21T19:29:01.9009644Z .................................................................................................... 6400/9815
2020-03-21T19:29:11.0582465Z ........................................................................i...F..........i..i..ii..... 6500/9815
2020-03-21T19:29:57.2038524Z ...........................i.....ii................................................................. 6600/9815
2020-03-21T19:30:42.1781084Z ...............................................................i..............................ii.... 6700/9815
2020-03-21T19:30:58.8688736Z ................iii...Fii.FFFi.iiiiii....................................................i.......... 6800/9815
2020-03-21T19:31:04.0800917Z .................................................................................................... 7000/9815
2020-03-21T19:31:06.3996144Z ........................i........................................................................... 7100/9815
2020-03-21T19:31:18.4654439Z ......................................................i............................................. 7200/9815
2020-03-21T19:31:34.1156741Z ...............................................................i.................................... 7300/9815
2020-03-21T19:31:34.1156741Z ...............................................................i.................................... 7300/9815
2020-03-21T19:32:59.0762277Z ...........................................................i........................................ 7400/9815
2020-03-21T19:34:04.4905659Z ........................i...i.......i...................iiiiiiii.............................F...... 7500/9815
2020-03-21T19:35:05.3066739Z .................................................................................................... 7700/9815
2020-03-21T19:35:31.7190667Z .................................................................................................... 7800/9815
2020-03-21T19:35:46.3656388Z .................................................................................................... 7900/9815
2020-03-21T19:36:08.1260190Z .............................................................................i...................... 8000/9815
2020-03-21T19:36:08.1260190Z .............................................................................i...................... 8000/9815
2020-03-21T19:37:01.1237214Z ...................................................................i................................ 8100/9815
2020-03-21T19:37:23.0379583Z ..........................iiiiiiiiii.i.............................................................. 8200/9815
2020-03-21T19:37:58.1100848Z ......................................i..............iii.....i......iiiiiiiiiii.i................... 8300/9815
2020-03-21T19:38:32.4320510Z .................................................................................................... 8500/9815
2020-03-21T19:39:46.1372325Z .........i.i.....................................................................i.................. 8600/9815
2020-03-21T19:40:26.8276657Z ...............................................i.................................................... 8700/9815
2020-03-21T19:40:40.2356282Z .................................................................................................... 8800/9815
2020-03-21T19:40:40.2356282Z .................................................................................................... 8800/9815
2020-03-21T19:41:50.0366654Z .........iii......i....................iiii....ii....i.i.....iiiiii.....iiiiiiii.ii...ii.iii..iiii.. 8900/9815
2020-03-21T19:43:22.2745024Z .................................................................................................... 9100/9815
2020-03-21T19:43:48.5835136Z ..............................................i..................................................... 9200/9815
2020-03-21T19:44:04.5911186Z .......................i............................................................................ 9300/9815
2020-03-21T19:44:37.1929349Z .................................................................................................... 9400/9815
---
2020-03-21T19:46:59.0756404Z 
2020-03-21T19:46:59.0756833Z - error: any use of this value will cause an error
2020-03-21T19:46:59.0757342Z -   --> $DIR/const_panic_libcore_main.rs:9:15
2020-03-21T19:46:59.0757710Z -    |
2020-03-21T19:46:59.0758084Z - LL | const Z: () = panic!("cheese");
2020-03-21T19:46:59.0758906Z -    |               |
2020-03-21T19:46:59.0758906Z -    |               |
2020-03-21T19:46:59.0759485Z -    |               the evaluated program panicked at 'cheese', $DIR/const_panic_libcore_main.rs:9:15
2020-03-21T19:46:59.0760359Z -    = note: `#[deny(const_err)]` on by default
2020-03-21T19:46:59.0761002Z -    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-03-21T19:46:59.0761002Z -    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-03-21T19:46:59.0761471Z + error: language item required, but not found: `eh_catch_typeinfo`
2020-03-21T19:46:59.0762986Z - error: any use of this value will cause an error
2020-03-21T19:46:59.0763519Z -   --> $DIR/const_panic_libcore_main.rs:12:15
2020-03-21T19:46:59.0763895Z -    |
2020-03-21T19:46:59.0763895Z -    |
2020-03-21T19:46:59.0764274Z - LL | const Y: () = unreachable!();
2020-03-21T19:46:59.0765127Z -    |               |
2020-03-21T19:46:59.0765859Z -    |               the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic_libcore_main.rs:12:15
2020-03-21T19:46:59.0766827Z -    |
2020-03-21T19:46:59.0767401Z -    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-03-21T19:46:59.0767401Z -    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-03-21T19:46:59.0767919Z - 
2020-03-21T19:46:59.0768631Z - error: any use of this value will cause an error
2020-03-21T19:46:59.0769137Z -   --> $DIR/const_panic_libcore_main.rs:15:15
2020-03-21T19:46:59.0769514Z -    |
2020-03-21T19:46:59.0769914Z - LL | const X: () = unimplemented!();
2020-03-21T19:46:59.0770578Z -    | --------------^^^^^^^^^^^^^^^^-
2020-03-21T19:46:59.0770955Z -    |               |
2020-03-21T19:46:59.0771574Z -    |               the evaluated program panicked at 'not implemented', $DIR/const_panic_libcore_main.rs:15:15
2020-03-21T19:46:59.0773143Z -    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-03-21T19:46:59.0773610Z - 
2020-03-21T19:46:59.0773990Z - error: aborting due to 3 previous errors
2020-03-21T19:46:59.0778028Z + error: aborting due to previous error
2020-03-21T19:46:59.0778028Z + error: aborting due to previous error
2020-03-21T19:46:59.0778242Z 33 
2020-03-21T19:46:59.0778365Z 34 
2020-03-21T19:46:59.0778470Z 
2020-03-21T19:46:59.0778567Z 
2020-03-21T19:46:59.0778773Z The actual stderr differed from the expected stderr.
2020-03-21T19:46:59.0779679Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main/const_panic_libcore_main.stderr
2020-03-21T19:46:59.0780370Z To update references, rerun the tests and pass the `--bless` flag
2020-03-21T19:46:59.0781029Z To only update this specific test, also pass `--test-args consts/const-eval/const_panic_libcore_main.rs`
2020-03-21T19:46:59.0781508Z error: 1 errors occurred comparing output.
2020-03-21T19:46:59.0781747Z status: exit code: 1
2020-03-21T19:46:59.0781747Z status: exit code: 1
2020-03-21T19:46:59.0783948Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main/auxiliary"
2020-03-21T19:46:59.0785597Z ------------------------------------------
2020-03-21T19:46:59.0785767Z 
2020-03-21T19:46:59.0786118Z ------------------------------------------
2020-03-21T19:46:59.0786581Z stderr:
2020-03-21T19:46:59.0786581Z stderr:
2020-03-21T19:46:59.0786958Z ------------------------------------------
2020-03-21T19:46:59.0787285Z error: language item required, but not found: `eh_catch_typeinfo`
2020-03-21T19:46:59.0787942Z error: aborting due to previous error
2020-03-21T19:46:59.0788108Z 
2020-03-21T19:46:59.0788204Z 
2020-03-21T19:46:59.0789316Z ------------------------------------------
---
2020-03-21T19:46:59.0790710Z 40 LL |             write!(f, "{}",)?;
2020-03-21T19:46:59.0794854Z 41    |                        ^^
2020-03-21T19:46:59.0795058Z 42 
2020-03-21T19:46:59.0795575Z - error: aborting due to 7 previous errors
2020-03-21T19:46:59.0795902Z + error: language item required, but not found: `eh_catch_typeinfo`
2020-03-21T19:46:59.0796807Z + error: aborting due to 8 previous errors
2020-03-21T19:46:59.0797004Z 44 
2020-03-21T19:46:59.0829212Z 45 
2020-03-21T19:46:59.0829349Z 
2020-03-21T19:46:59.0829349Z 
2020-03-21T19:46:59.0829453Z 
2020-03-21T19:46:59.0829670Z The actual stderr differed from the expected stderr.
2020-03-21T19:46:59.0830948Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior.core/macro-comma-behavior.core.stderr
2020-03-21T19:46:59.0831623Z To update references, rerun the tests and pass the `--bless` flag
2020-03-21T19:46:59.0832679Z To only update this specific test, also pass `--test-args macros/macro-comma-behavior.rs`
2020-03-21T19:46:59.0833187Z error in revision `core`: 1 errors occurred comparing output.
2020-03-21T19:46:59.0833680Z status: exit code: 1
2020-03-21T19:46:59.0833680Z status: exit code: 1
2020-03-21T19:46:59.0836063Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-behavior.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--cfg" "core" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior.core" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-A" "unused" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior.core/auxiliary"
2020-03-21T19:46:59.0837828Z ------------------------------------------
2020-03-21T19:46:59.0838009Z 
2020-03-21T19:46:59.0838371Z ------------------------------------------
2020-03-21T19:46:59.0838594Z stderr:
2020-03-21T19:46:59.0838594Z stderr:
2020-03-21T19:46:59.0838967Z ------------------------------------------
2020-03-21T19:46:59.0839291Z error: 1 positional argument in format string, but no arguments were given
2020-03-21T19:46:59.0839887Z   --> /checkout/src/test/ui/macros/macro-comma-behavior.rs:20:23
2020-03-21T19:46:59.0840148Z    |
2020-03-21T19:46:59.0840330Z LL |     assert_eq!(1, 1, "{}",);
2020-03-21T19:46:59.0840719Z 
2020-03-21T19:46:59.0840965Z error: 1 positional argument in format string, but no arguments were given
2020-03-21T19:46:59.0841561Z   --> /checkout/src/test/ui/macros/macro-comma-behavior.rs:23:23
2020-03-21T19:46:59.0841811Z    |
2020-03-21T19:46:59.0841811Z    |
2020-03-21T19:46:59.0841992Z LL |     assert_ne!(1, 2, "{}",);
2020-03-21T19:46:59.0842388Z 
2020-03-21T19:46:59.0842806Z error: 1 positional argument in format string, but no arguments were given
2020-03-21T19:46:59.0843358Z   --> /checkout/src/test/ui/macros/macro-comma-behavior.rs:29:29
2020-03-21T19:46:59.0843613Z    |
2020-03-21T19:46:59.0843613Z    |
2020-03-21T19:46:59.0843795Z LL |     debug_assert_eq!(1, 1, "{}",);
2020-03-21T19:46:59.0844183Z 
2020-03-21T19:46:59.0844440Z error: 1 positional argument in format string, but no arguments were given
2020-03-21T19:46:59.0845194Z   --> /checkout/src/test/ui/macros/macro-comma-behavior.rs:32:29
2020-03-21T19:46:59.0845444Z    |
2020-03-21T19:46:59.0845444Z    |
2020-03-21T19:46:59.0845648Z LL |     debug_assert_ne!(1, 2, "{}",);
2020-03-21T19:46:59.0846211Z 
2020-03-21T19:46:59.0846649Z error: 1 positional argument in format string, but no arguments were given
2020-03-21T19:46:59.0847216Z   --> /checkout/src/test/ui/macros/macro-comma-behavior.rs:53:19
2020-03-21T19:46:59.0847470Z    |
2020-03-21T19:46:59.0847470Z    |
2020-03-21T19:46:59.0847659Z LL |     format_args!("{}",);
2020-03-21T19:46:59.0848060Z 
2020-03-21T19:46:59.0848305Z error: 1 positional argument in format string, but no arguments were given
2020-03-21T19:46:59.0849092Z   --> /checkout/src/test/ui/macros/macro-comma-behavior.rs:71:21
2020-03-21T19:46:59.0849702Z    |
---
2020-03-21T19:46:59.0851923Z    |
2020-03-21T19:46:59.0852115Z LL |             write!(f, "{}",)?;
2020-03-21T19:46:59.0852352Z    |                        ^^
2020-03-21T19:46:59.0852669Z 
2020-03-21T19:46:59.0853806Z error: language item required, but not found: `eh_catch_typeinfo`
2020-03-21T19:46:59.0854391Z error: aborting due to 8 previous errors
2020-03-21T19:46:59.0854584Z 
2020-03-21T19:46:59.0854682Z 
2020-03-21T19:46:59.0855161Z ------------------------------------------
2020-03-21T19:46:59.0855161Z ------------------------------------------
2020-03-21T19:46:59.0855343Z 
2020-03-21T19:46:59.0855442Z 
2020-03-21T19:46:59.0855864Z ---- [ui] ui/no_owned_box_lang_item.rs stdout ----
2020-03-21T19:46:59.0856105Z diff of stderr:
2020-03-21T19:46:59.0856233Z 
2020-03-21T19:46:59.0856604Z + error: language item required, but not found: `eh_catch_typeinfo`
2020-03-21T19:46:59.0857077Z 1 error: requires `owned_box` lang_item
2020-03-21T19:46:59.0857272Z 2 
2020-03-21T19:46:59.0857700Z - error: aborting due to previous error
2020-03-21T19:46:59.0857979Z + error: aborting due to 2 previous errors
2020-03-21T19:46:59.0857979Z + error: aborting due to 2 previous errors
2020-03-21T19:46:59.0858179Z 4 
2020-03-21T19:46:59.0858319Z 5 
2020-03-21T19:46:59.0858424Z 
2020-03-21T19:46:59.0858524Z 
2020-03-21T19:46:59.0858736Z The actual stderr differed from the expected stderr.
2020-03-21T19:46:59.0859474Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no_owned_box_lang_item/no_owned_box_lang_item.stderr
2020-03-21T19:46:59.0860144Z To update references, rerun the tests and pass the `--bless` flag
2020-03-21T19:46:59.0860774Z To only update this specific test, also pass `--test-args no_owned_box_lang_item.rs`
2020-03-21T19:46:59.0861229Z error: 1 errors occurred comparing output.
2020-03-21T19:46:59.0861478Z status: exit code: 1
2020-03-21T19:46:59.0861478Z status: exit code: 1
2020-03-21T19:46:59.0863788Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/no_owned_box_lang_item.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no_owned_box_lang_item" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no_owned_box_lang_item/auxiliary"
2020-03-21T19:46:59.0865449Z ------------------------------------------
2020-03-21T19:46:59.0865632Z 
2020-03-21T19:46:59.0866008Z ------------------------------------------
2020-03-21T19:46:59.0866237Z stderr:
2020-03-21T19:46:59.0866237Z stderr:
2020-03-21T19:46:59.0866622Z ------------------------------------------
2020-03-21T19:46:59.0866965Z error: language item required, but not found: `eh_catch_typeinfo`
2020-03-21T19:46:59.0867412Z error: requires `owned_box` lang_item
2020-03-21T19:46:59.0867584Z 
2020-03-21T19:46:59.0868071Z error: aborting due to 2 previous errors
2020-03-21T19:46:59.0868269Z 
---
2020-03-21T19:46:59.0869502Z ---- [ui] ui/panic-runtime/link-to-abort.rs stdout ----
2020-03-21T19:46:59.0870375Z 
2020-03-21T19:46:59.0871332Z error: test compilation failed although it shouldn't!
2020-03-21T19:46:59.0871616Z status: exit code: 1
2020-03-21T19:46:59.0873688Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/link-to-abort.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/auxiliary"
2020-03-21T19:46:59.0875561Z ------------------------------------------
2020-03-21T19:46:59.0877402Z 
2020-03-21T19:46:59.0877870Z ------------------------------------------
2020-03-21T19:46:59.0878237Z stderr:
2020-03-21T19:46:59.0878237Z stderr:
2020-03-21T19:46:59.0878662Z ------------------------------------------
2020-03-21T19:46:59.0878986Z error: linking with `emcc` failed: exit code: 1
2020-03-21T19:46:59.0879219Z    |
2020-03-21T19:46:59.0891116Z    = note: "emcc" "-s" "DISABLE_EXCEPTION_CATCHING=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/a.link_to_abort.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/a.link_to_abort.7rcbfp3g-cgu.1.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/a.js" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/a.4hgw2zitnpvuzihs.rcgu.o" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-90a14715185f1e04.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_abort-c6bc0c8c141f7ae8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-fc74f8b041b9ecca.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-2251c0700d84727f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libbacktrace-3772ab2e3c5d21db.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-1e628ec036d91e60.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-3442107804c1b2f9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-6449a1b3aa986018.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-07cd903545e5136b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-7075ff8fcd9e3c8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-097bf655430f1ab7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-bda184b8b23ac7fa.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-a6af0508d3a0b2e1.rlib" "-l" "c" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
2020-03-21T19:46:59.0904303Z    = note: cache:INFO: generating system library: libc++abi-noexcept.a... (this will be cached in "/root/.emscripten_cache/wasm-obj/libc++abi-noexcept.a" for subsequent builds)
2020-03-21T19:46:59.0905078Z            cache:INFO:  - ok
2020-03-21T19:46:59.0906122Z            wasm-ld: error: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-90a14715185f1e04.rlib(std-90a14715185f1e04.std.559a6xlu-cgu.0.rcgu.o): undefined symbol: rust_eh_catch_typeinfo
2020-03-21T19:46:59.0908093Z            wasm-ld: error: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-90a14715185f1e04.rlib(std-90a14715185f1e04.std.559a6xlu-cgu.0.rcgu.o): undefined symbol: rust_eh_catch_typeinfo
2020-03-21T19:46:59.0919849Z            shared:ERROR: '/emsdk-portable/upstream/bin/wasm-ld -o /tmp/emscripten_temp_2kYRNL/a.wasm --allow-undefined --lto-O0 -L/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/a.link_to_abort.7rcbfp3g-cgu.0.rcgu.o /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/a.link_to_abort.7rcbfp3g-cgu.1.rcgu.o /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/a.4hgw2zitnpvuzihs.rcgu.o -L/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers -L/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/auxiliary -L/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-90a14715185f1e04.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_abort-c6bc0c8c141f7ae8.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-fc74f8b041b9ecca.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-2251c0700d84727f.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libbacktrace-3772ab2e3c5d21db.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-1e628ec036d91e60.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-3442107804c1b2f9.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-6449a1b3aa986018.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-07cd903545e5136b.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-7075ff8fcd9e3c8b.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-097bf655430f1ab7.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-bda184b8b23ac7fa.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-a6af0508d3a0b2e1.rlib -lc --fatal-warnings -L/emsdk-portable/upstream/emscripten/system/local/lib -L/emsdk-portable/upstream/emscripten/system/lib -L/root/.emscripten_cache/wasm-obj /root/.emscripten_cache/wasm-obj/libc.a /root/.emscripten_cache/wasm-obj/libcompiler_rt.a /root/.emscripten_cache/wasm-obj/libc-wasm.a /root/.emscripten_cache/wasm-obj/libc++abi-noexcept.a /root/.emscripten_cache/wasm-obj/libc-extras.a /root/.emscripten_cache/wasm-obj/libdlmalloc.a /root/.emscripten_cache/wasm-obj/libpthread_stub.a /root/.emscripten_cache/wasm-obj/libc_rt_wasm.a --import-memory --import-table -mllvm -combiner-global-alias-analysis=false -mllvm -enable-emscripten-sjlj -mllvm -disable-lsr --export __wasm_call_ctors --export __data_end --export main --export rust_eh_personality --export malloc --export free --export setThrew --export __errno_location --export fflush --export htonl --export htons --export ntohs --export _get_environ --export __cxa_is_pointer_type --export __cxa_can_catch -z stack-size=5242880 --initial-memory=16777216 --no-entry --max-memory=16777216 --global-base=1024' failed (1)
2020-03-21T19:46:59.0926968Z 
2020-03-21T19:46:59.0927171Z error: aborting due to previous error
2020-03-21T19:46:59.0927345Z 
2020-03-21T19:46:59.0927445Z 
2020-03-21T19:46:59.0927445Z 
2020-03-21T19:46:59.0927929Z ------------------------------------------
2020-03-21T19:46:59.0928113Z 
2020-03-21T19:46:59.0928232Z 
2020-03-21T19:46:59.0928679Z ---- [ui] ui/panic-runtime/transitive-link-a-bunch.rs stdout ----
2020-03-21T19:46:59.0928904Z 
2020-03-21T19:46:59.0929315Z error: test compilation failed although it shouldn't!
2020-03-21T19:46:59.0929601Z status: exit code: 1
2020-03-21T19:46:59.0932455Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/transitive-link-a-bunch.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/transitive-link-a-bunch" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/transitive-link-a-bunch/auxiliary"
2020-03-21T19:46:59.0934275Z ------------------------------------------
2020-03-21T19:46:59.0934476Z 
2020-03-21T19:46:59.0934853Z ------------------------------------------
2020-03-21T19:46:59.0935063Z stderr:
2020-03-21T19:46:59.0935063Z stderr:
2020-03-21T19:46:59.0935465Z ------------------------------------------
2020-03-21T19:46:59.0935805Z error: language item required, but not found: `eh_catch_typeinfo`
2020-03-21T19:46:59.0936242Z error: aborting due to previous error
2020-03-21T19:46:59.0936430Z 
2020-03-21T19:46:59.0936529Z 
2020-03-21T19:46:59.0936903Z ------------------------------------------
2020-03-21T19:46:59.0936903Z ------------------------------------------
2020-03-21T19:46:59.0937083Z 
2020-03-21T19:46:59.0937183Z 
2020-03-21T19:46:59.0937630Z ---- [ui] ui/panic-runtime/want-unwind-got-abort.rs stdout ----
2020-03-21T19:46:59.0937848Z 
2020-03-21T19:46:59.0938258Z error: test compilation failed although it shouldn't!
2020-03-21T19:46:59.0938543Z status: exit code: 1
2020-03-21T19:46:59.0940642Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/want-unwind-got-abort.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/want-unwind-got-abort" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/want-unwind-got-abort/auxiliary"
2020-03-21T19:46:59.0942365Z ------------------------------------------
2020-03-21T19:46:59.0942756Z 
2020-03-21T19:46:59.0943137Z ------------------------------------------
2020-03-21T19:46:59.0943339Z stderr:
2020-03-21T19:46:59.0943339Z stderr:
2020-03-21T19:46:59.0943714Z ------------------------------------------
2020-03-21T19:46:59.0944057Z error: language item required, but not found: `eh_catch_typeinfo`
2020-03-21T19:46:59.0944470Z error: aborting due to previous error
2020-03-21T19:46:59.0944635Z 
2020-03-21T19:46:59.0944750Z 
2020-03-21T19:46:59.0945112Z ------------------------------------------
2020-03-21T19:46:59.0945112Z ------------------------------------------
2020-03-21T19:46:59.0945285Z 
2020-03-21T19:46:59.0945559Z 
2020-03-21T19:46:59.0946013Z ---- [ui] ui/panic-runtime/want-unwind-got-abort2.rs stdout ----
2020-03-21T19:46:59.0946239Z 
2020-03-21T19:46:59.0946648Z error: test compilation failed although it shouldn't!
2020-03-21T19:46:59.0946916Z status: exit code: 1
2020-03-21T19:46:59.0950042Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/want-unwind-got-abort2.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/want-unwind-got-abort2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/want-unwind-got-abort2/auxiliary"
2020-03-21T19:46:59.0951777Z ------------------------------------------
2020-03-21T19:46:59.0951958Z 
2020-03-21T19:46:59.0952485Z ------------------------------------------
2020-03-21T19:46:59.0952717Z stderr:
2020-03-21T19:46:59.0952717Z stderr:
2020-03-21T19:46:59.0953103Z ------------------------------------------
2020-03-21T19:46:59.0953441Z error: language item required, but not found: `eh_catch_typeinfo`
2020-03-21T19:46:59.0953888Z error: aborting due to previous error
2020-03-21T19:46:59.0954058Z 
2020-03-21T19:46:59.0954156Z 
2020-03-21T19:46:59.0954547Z ------------------------------------------
2020-03-21T19:46:59.0954547Z ------------------------------------------
2020-03-21T19:46:59.0954726Z 
2020-03-21T19:46:59.0954903Z 
2020-03-21T19:46:59.0955345Z ---- [ui] ui/range/issue-54505-no-std.rs stdout ----
2020-03-21T19:46:59.0955609Z diff of stderr:
2020-03-21T19:46:59.0955737Z 
2020-03-21T19:46:59.0955971Z 1 error: `#[panic_handler]` function required, but not found
2020-03-21T19:46:59.0956206Z 2 
2020-03-21T19:46:59.0956494Z + error: language item required, but not found: `eh_catch_typeinfo`
2020-03-21T19:46:59.0956949Z 3 error[E0308]: mismatched types
2020-03-21T19:46:59.0957440Z 4   --> $DIR/issue-54505-no-std.rs:24:16
2020-03-21T19:46:59.0957663Z 5    |
2020-03-21T19:46:59.0957774Z 
2020-03-21T19:46:59.0957774Z 
2020-03-21T19:46:59.0957983Z 72    = note: expected reference `&_`
2020-03-21T19:46:59.0958333Z 73                  found struct `core::ops::RangeToInclusive<{integer}>`
2020-03-21T19:46:59.0958996Z - error: aborting due to 7 previous errors
2020-03-21T19:46:59.0959294Z + error: aborting due to 8 previous errors
2020-03-21T19:46:59.0959497Z 76 
2020-03-21T19:46:59.0959965Z 77 For more information about this error, try `rustc --explain E0308`.
2020-03-21T19:46:59.0959965Z 77 For more information about this error, try `rustc --explain E0308`.
2020-03-21T19:46:59.0960236Z 78 
2020-03-21T19:46:59.0960342Z 
2020-03-21T19:46:59.0960441Z 
2020-03-21T19:46:59.0960654Z The actual stderr differed from the expected stderr.
2020-03-21T19:46:59.0961526Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/issue-54505-no-std/issue-54505-no-std.stderr
2020-03-21T19:46:59.0962172Z To update references, rerun the tests and pass the `--bless` flag
2020-03-21T19:46:59.0962791Z To only update this specific test, also pass `--test-args range/issue-54505-no-std.rs`
2020-03-21T19:46:59.0963231Z error: 1 errors occurred comparing output.
2020-03-21T19:46:59.0963466Z status: exit code: 1
2020-03-21T19:46:59.0963466Z status: exit code: 1
2020-03-21T19:46:59.0965425Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/range/issue-54505-no-std.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/issue-54505-no-std" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/issue-54505-no-std/auxiliary"
2020-03-21T19:46:59.0967219Z ------------------------------------------
2020-03-21T19:46:59.0967407Z 
2020-03-21T19:46:59.0968032Z ------------------------------------------
2020-03-21T19:46:59.0968268Z stderr:
2020-03-21T19:46:59.0968268Z stderr:
2020-03-21T19:46:59.0968658Z ------------------------------------------
2020-03-21T19:46:59.0968966Z error: `#[panic_handler]` function required, but not found
2020-03-21T19:46:59.0969195Z 
2020-03-21T19:46:59.0969457Z error: language item required, but not found: `eh_catch_typeinfo`
2020-03-21T19:46:59.0969883Z error[E0308]: mismatched types
2020-03-21T19:46:59.0970563Z   --> /checkout/src/test/ui/range/issue-54505-no-std.rs:24:16
2020-03-21T19:46:59.0970809Z    |
2020-03-21T19:46:59.0970977Z LL |     take_range(0..1);
2020-03-21T19:46:59.0970977Z LL |     take_range(0..1);
2020-03-21T19:46:59.0971197Z    |                ^^^^
2020-03-21T19:46:59.0971389Z    |                |
2020-03-21T19:46:59.0971698Z    |                expected reference, found struct `core::ops::Range`
2020-03-21T19:46:59.0972120Z    |                help: consider borrowing here: `&(0..1)`
2020-03-21T19:46:59.0972474Z    |
2020-03-21T19:46:59.0972661Z    = note: expected reference `&_`
2020-03-21T19:46:59.0972981Z                  found struct `core::ops::Range<{integer}>`
2020-03-21T19:46:59.0973373Z error[E0308]: mismatched types
2020-03-21T19:46:59.0973900Z   --> /checkout/src/test/ui/range/issue-54505-no-std.rs:29:16
2020-03-21T19:46:59.0974161Z    |
2020-03-21T19:46:59.0974327Z LL |     take_range(1..);
2020-03-21T19:46:59.0974327Z LL |     take_range(1..);
2020-03-21T19:46:59.0974526Z    |                ^^^
2020-03-21T19:46:59.0974804Z    |                |
2020-03-21T19:46:59.0975126Z    |                expected reference, found struct `core::ops::RangeFrom`
2020-03-21T19:46:59.0975529Z    |                help: consider borrowing here: `&(1..)`
2020-03-21T19:46:59.0975794Z    |
2020-03-21T19:46:59.0975980Z    = note: expected reference `&_`
2020-03-21T19:46:59.0976290Z                  found struct `core::ops::RangeFrom<{integer}>`
2020-03-21T19:46:59.0976887Z error[E0308]: mismatched types
2020-03-21T19:46:59.0977601Z   --> /checkout/src/test/ui/range/issue-54505-no-std.rs:34:16
2020-03-21T19:46:59.0977846Z    |
2020-03-21T19:46:59.0978026Z LL |     take_range(..);
---
2020-03-21T19:46:59.0981685Z LL |     take_range(0..=1);
2020-03-21T19:46:59.0981907Z    |                ^^^^^
2020-03-21T19:46:59.0982103Z    |                |
2020-03-21T19:46:59.0982434Z    |                expected reference, found struct `core::ops::RangeInclusive`
2020-03-21T19:46:59.0982869Z    |                help: consider borrowing here: `&(0..=1)`
2020-03-21T19:46:59.0983309Z    = note: expected reference `&_`
2020-03-21T19:46:59.0983309Z    = note: expected reference `&_`
2020-03-21T19:46:59.0983646Z                  found struct `core::ops::RangeInclusive<{integer}>`
2020-03-21T19:46:59.0984054Z error[E0308]: mismatched types
2020-03-21T19:46:59.0984578Z   --> /checkout/src/test/ui/range/issue-54505-no-std.rs:44:16
2020-03-21T19:46:59.0984823Z    |
2020-03-21T19:46:59.0984990Z LL |     take_range(..5);
2020-03-21T19:46:59.0984990Z LL |     take_range(..5);
2020-03-21T19:46:59.0985188Z    |                ^^^
2020-03-21T19:46:59.0985397Z    |                |
2020-03-21T19:46:59.0985710Z    |                expected reference, found struct `core::ops::RangeTo`
2020-03-21T19:46:59.0986110Z    |                help: consider borrowing here: `&(..5)`
2020-03-21T19:46:59.0986546Z    |
2020-03-21T19:46:59.0986743Z    = note: expected reference `&_`
2020-03-21T19:46:59.0987060Z                  found struct `core::ops::RangeTo<{integer}>`
2020-03-21T19:46:59.0987485Z error[E0308]: mismatched types
2020-03-21T19:46:59.0988283Z   --> /checkout/src/test/ui/range/issue-54505-no-std.rs:49:16
2020-03-21T19:46:59.0988538Z    |
2020-03-21T19:46:59.0988733Z LL |     take_range(..=42);
2020-03-21T19:46:59.0988733Z LL |     take_range(..=42);
2020-03-21T19:46:59.0988946Z    |                ^^^^^
2020-03-21T19:46:59.0989148Z    |                |
2020-03-21T19:46:59.0989510Z    |                expected reference, found struct `core::ops::RangeToInclusive`
2020-03-21T19:46:59.0990087Z    |                help: consider borrowing here: `&(..=42)`
2020-03-21T19:46:59.0990540Z    = note: expected reference `&_`
2020-03-21T19:46:59.0990540Z    = note: expected reference `&_`
2020-03-21T19:46:59.0990862Z                  found struct `core::ops::RangeToInclusive<{integer}>`
2020-03-21T19:46:59.0991302Z error: aborting due to 8 previous errors
2020-03-21T19:46:59.0991588Z 
2020-03-21T19:46:59.0992229Z For more information about this error, try `rustc --explain E0308`.
2020-03-21T19:46:59.0992456Z 
---
2020-03-21T19:46:59.0998733Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-21T19:46:59.0999164Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-21T19:46:59.0999423Z 
2020-03-21T19:46:59.0999523Z 
2020-03-21T19:46:59.1003748Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-emscripten" "--mode" "ui" "--target" "wasm32-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/12.9.1_64bit/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.1-rust-1.44.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-21T19:46:59.1006776Z 
2020-03-21T19:46:59.1006878Z 
2020-03-21T19:46:59.1007823Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-emscripten --exclude src/libcore --exclude src/liballoc --exclude src/libproc_macro --exclude src/libstd --exclude src/libterm --exclude src/libtest
2020-03-21T19:46:59.1008477Z Build completed unsuccessfully in 2:29:52
2020-03-21T19:46:59.1008477Z Build completed unsuccessfully in 2:29:52
2020-03-21T19:46:59.1008736Z == clock drift check ==
2020-03-21T19:46:59.1009009Z   local time: Sat Mar 21 19:46:59 UTC 2020
2020-03-21T19:46:59.3823495Z   network time: Sat, 21 Mar 2020 19:46:59 GMT
2020-03-21T19:46:59.3827139Z == end clock drift check ==
2020-03-21T19:46:59.7856986Z 
2020-03-21T19:46:59.7913622Z ##[error]Bash exited with code '1'.
2020-03-21T19:46:59.7929025Z ##[section]Finishing: Run build
2020-03-21T19:46:59.7984089Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70212/merge to s
2020-03-21T19:46:59.7990937Z Task         : Get sources
2020-03-21T19:46:59.7991290Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-21T19:46:59.7991783Z Version      : 1.0.0
2020-03-21T19:46:59.7991998Z Author       : Microsoft
2020-03-21T19:46:59.7991998Z Author       : Microsoft
2020-03-21T19:46:59.7992351Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-21T19:46:59.7992742Z ==============================================================================
2020-03-21T19:47:00.1434597Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-21T19:47:00.1484039Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70212/merge to s
2020-03-21T19:47:00.1579791Z Cleaning up task key
2020-03-21T19:47:00.1581085Z Start cleaning up orphan processes.
2020-03-21T19:47:00.1782267Z Terminate orphan process: pid (3461) (python)
2020-03-21T19:47:00.2054730Z ##[section]Finishing: Finalize Job
