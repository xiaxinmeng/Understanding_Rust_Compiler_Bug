plain
2020-02-28T04:20:32.9420485Z ========================== Starting Command Output ===========================
2020-02-28T04:20:32.9423011Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/825674db-a38f-4460-a7c9-a5ba905049c8.sh
2020-02-28T04:20:32.9423265Z 
2020-02-28T04:20:32.9428802Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-28T04:20:32.9451856Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69537/merge to s
2020-02-28T04:20:32.9455356Z Task         : Get sources
2020-02-28T04:20:32.9455780Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-28T04:20:32.9456059Z Version      : 1.0.0
2020-02-28T04:20:32.9456251Z Author       : Microsoft
---
2020-02-28T04:20:35.2782942Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-28T04:20:35.2793179Z ##[command]git config gc.auto 0
2020-02-28T04:20:35.2797180Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-28T04:20:35.2801096Z ##[command]git config --get-all http.proxy
2020-02-28T04:20:35.2809836Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69537/merge:refs/remotes/pull/69537/merge
---
2020-02-28T05:25:58.3328567Z .................................................................................................... 1700/9735
2020-02-28T05:26:03.2188148Z .................................................................................................... 1800/9735
2020-02-28T05:26:15.4754787Z ......................................................................i............................. 1900/9735
2020-02-28T05:26:22.5514717Z .................................................................................................... 2000/9735
2020-02-28T05:26:38.7682271Z ............................................................iiiii................................... 2100/9735
2020-02-28T05:26:50.0850312Z .................................................................................................... 2300/9735
2020-02-28T05:26:52.5310525Z .................................................................................................... 2400/9735
2020-02-28T05:26:55.8763949Z .................................................................................................... 2500/9735
2020-02-28T05:27:17.0091387Z .................................................................................................... 2600/9735
---
2020-02-28T05:30:01.9114594Z ....................i...............i............................................................... 5000/9735
2020-02-28T05:30:11.7468713Z .................................................................................................... 5100/9735
2020-02-28T05:30:17.4950113Z ...............................................................i.................................... 5200/9735
2020-02-28T05:30:23.9578515Z .............................................F...................................................... 5300/9735
2020-02-28T05:30:33.0714806Z .........................................ii.ii........i...i......................................... 5400/9735
2020-02-28T05:30:42.1456160Z .................................................................................................... 5600/9735
2020-02-28T05:30:51.8711315Z .................................................................................................... 5700/9735
2020-02-28T05:30:58.7964968Z ................................i................................................................... 5800/9735
2020-02-28T05:31:05.2628161Z .................................................................................................... 5900/9735
2020-02-28T05:31:05.2628161Z .................................................................................................... 5900/9735
2020-02-28T05:31:16.3152832Z .................................................................................................... 6000/9735
2020-02-28T05:31:26.0140769Z .......................ii...i..ii...........i....................................................... 6100/9735
2020-02-28T05:31:43.3113866Z .................................................................................................... 6300/9735
2020-02-28T05:31:50.5179275Z .................................................................................................... 6400/9735
2020-02-28T05:31:50.5179275Z .................................................................................................... 6400/9735
2020-02-28T05:32:02.5651498Z ......................................................i..ii......................................... 6500/9735
2020-02-28T05:32:30.1088197Z .................................................................................................... 6700/9735
2020-02-28T05:32:32.6783649Z ..............................................i..................................................... 6800/9735
2020-02-28T05:32:34.8321311Z .................................................................................................... 6900/9735
2020-02-28T05:32:37.0663962Z ............................................................................i....................... 7000/9735
---
2020-02-28T05:34:18.5935466Z .................................................................................................... 7700/9735
2020-02-28T05:34:23.5369114Z .................................................................................................... 7800/9735
2020-02-28T05:34:28.9971543Z .................................................................................................... 7900/9735
2020-02-28T05:34:37.4995136Z .....................i.............................................................................. 8000/9735
2020-02-28T05:34:46.4021690Z ......................................................................iiiiiii..i.................... 8100/9735
2020-02-28T05:35:02.5111576Z ...........i......i................................................................................. 8300/9735
2020-02-28T05:35:08.3518309Z .................................................................................................... 8400/9735
2020-02-28T05:35:22.0679816Z .................................................................................................... 8500/9735
2020-02-28T05:35:31.8933386Z .................................................................................................... 8600/9735
---
2020-02-28T05:37:31.7964981Z normalized stderr:
2020-02-28T05:37:31.7965760Z error[E0428]: the name `A` is defined multiple times
2020-02-28T05:37:31.7966614Z   --> $DIR/issue-69396-const-no-type-in-macro.rs:4:13
2020-02-28T05:37:31.7966813Z    |
2020-02-28T05:37:31.7966989Z LL |               const A = "A".$fn();
2020-02-28T05:37:31.7967370Z    |               |
2020-02-28T05:37:31.7967370Z    |               |
2020-02-28T05:37:31.7967561Z    |               `A` redefined here
2020-02-28T05:37:31.7967792Z    |               previous definition of the value `A` here
2020-02-28T05:37:31.7967972Z ...
2020-02-28T05:37:31.7968110Z LL | / suite! {
2020-02-28T05:37:31.7968246Z LL | |     len;
2020-02-28T05:37:31.7968407Z LL | |     is_empty;
2020-02-28T05:37:31.7968879Z    | |_- in this macro invocation
2020-02-28T05:37:31.7969031Z    |
2020-02-28T05:37:31.7969248Z    = note: `A` must be defined only once in the value namespace of this module
2020-02-28T05:37:31.7970511Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-28T05:37:31.7970511Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-28T05:37:31.7970774Z 
2020-02-28T05:37:31.7971116Z error: missing type for `const` item
2020-02-28T05:37:31.7971917Z   --> $DIR/issue-69396-const-no-type-in-macro.rs:4:19
2020-02-28T05:37:31.7972148Z    |
2020-02-28T05:37:31.7972330Z LL |               const A = "A".$fn();
2020-02-28T05:37:31.7972680Z    |                     ^ help: provide a type for the item: `A: usize`
2020-02-28T05:37:31.7972944Z ...
2020-02-28T05:37:31.7973248Z LL | / suite! {
2020-02-28T05:37:31.7973422Z LL | |     len;
2020-02-28T05:37:31.7973584Z LL | |     is_empty;
2020-02-28T05:37:31.7974296Z    | |_- in this macro invocation
2020-02-28T05:37:31.7974465Z    |
2020-02-28T05:37:31.7975336Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-28T05:37:31.7975576Z 
2020-02-28T05:37:31.7975576Z 
2020-02-28T05:37:31.7975744Z error: missing type for `const` item
2020-02-28T05:37:31.7976343Z   --> $DIR/issue-69396-const-no-type-in-macro.rs:4:19
2020-02-28T05:37:31.7976545Z    |
2020-02-28T05:37:31.7976724Z LL |               const A = "A".$fn();
2020-02-28T05:37:31.7977007Z    |                     ^ help: provide a type for the item: `A: bool`
2020-02-28T05:37:31.7977237Z ...
2020-02-28T05:37:31.7977377Z LL | / suite! {
2020-02-28T05:37:31.7977515Z LL | |     len;
2020-02-28T05:37:31.7977662Z LL | |     is_empty;
2020-02-28T05:37:31.7978127Z    | |_- in this macro invocation
2020-02-28T05:37:31.7978283Z    |
2020-02-28T05:37:31.7978757Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-28T05:37:31.7979031Z 
---
2020-02-28T05:37:31.7980504Z 
2020-02-28T05:37:31.7980580Z 
2020-02-28T05:37:31.7980748Z The actual stderr differed from the expected stderr.
2020-02-28T05:37:31.7981904Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69396-const-no-type-in-macro/issue-69396-const-no-type-in-macro.stderr
2020-02-28T05:37:31.7982565Z To update references, rerun the tests and pass the `--bless` flag
2020-02-28T05:37:31.7983168Z To only update this specific test, also pass `--test-args issues/issue-69396-const-no-type-in-macro.rs`
2020-02-28T05:37:31.7983626Z error: 1 errors occurred comparing output.
2020-02-28T05:37:31.7983843Z status: exit code: 1
2020-02-28T05:37:31.7983843Z status: exit code: 1
2020-02-28T05:37:31.7986984Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-69396-const-no-type-in-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69396-const-no-type-in-macro" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69396-const-no-type-in-macro/auxiliary"
2020-02-28T05:37:31.7988714Z ------------------------------------------
2020-02-28T05:37:31.7988881Z 
2020-02-28T05:37:31.7989390Z ------------------------------------------
2020-02-28T05:37:31.7989779Z stderr:
2020-02-28T05:37:31.7989779Z stderr:
2020-02-28T05:37:31.7990305Z ------------------------------------------
2020-02-28T05:37:31.7990738Z error[E0428]: the name `A` is defined multiple times
2020-02-28T05:37:31.7991697Z   --> /checkout/src/test/ui/issues/issue-69396-const-no-type-in-macro.rs:4:13
2020-02-28T05:37:31.7991969Z    |
2020-02-28T05:37:31.7992388Z LL |               const A = "A".$fn();
2020-02-28T05:37:31.7993084Z    |               |
2020-02-28T05:37:31.7993084Z    |               |
2020-02-28T05:37:31.7993303Z    |               `A` redefined here
2020-02-28T05:37:31.7993639Z    |               previous definition of the value `A` here
2020-02-28T05:37:31.7993868Z ...
2020-02-28T05:37:31.7994180Z LL | / suite! {
2020-02-28T05:37:31.7994373Z LL | |     len;
2020-02-28T05:37:31.7994918Z LL | |     is_empty;
2020-02-28T05:37:31.7996269Z    | |_- in this macro invocation
2020-02-28T05:37:31.7996449Z    |
2020-02-28T05:37:31.7996671Z    = note: `A` must be defined only once in the value namespace of this module
2020-02-28T05:37:31.7997261Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-28T05:37:31.7997261Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-28T05:37:31.7997535Z 
2020-02-28T05:37:31.7997690Z error: missing type for `const` item
2020-02-28T05:37:31.7998164Z   --> /checkout/src/test/ui/issues/issue-69396-const-no-type-in-macro.rs:4:19
2020-02-28T05:37:31.7998585Z    |
2020-02-28T05:37:31.7998741Z LL |               const A = "A".$fn();
2020-02-28T05:37:31.7999018Z    |                     ^ help: provide a type for the item: `A: usize`
2020-02-28T05:37:31.7999259Z ...
2020-02-28T05:37:31.7999377Z LL | / suite! {
2020-02-28T05:37:31.7999511Z LL | |     len;
2020-02-28T05:37:31.7999687Z LL | |     is_empty;
2020-02-28T05:37:31.8000124Z    | |_- in this macro invocation
2020-02-28T05:37:31.8000275Z    |
2020-02-28T05:37:31.8001125Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-28T05:37:31.8001409Z 
2020-02-28T05:37:31.8001409Z 
2020-02-28T05:37:31.8001629Z error: missing type for `const` item
2020-02-28T05:37:31.8002224Z   --> /checkout/src/test/ui/issues/issue-69396-const-no-type-in-macro.rs:4:19
2020-02-28T05:37:31.8002493Z    |
2020-02-28T05:37:31.8002676Z LL |               const A = "A".$fn();
2020-02-28T05:37:31.8003369Z    |                     ^ help: provide a type for the item: `A: bool`
2020-02-28T05:37:31.8003824Z ...
2020-02-28T05:37:31.8004272Z LL | / suite! {
2020-02-28T05:37:31.8004711Z LL | |     len;
2020-02-28T05:37:31.8005227Z LL | |     is_empty;
2020-02-28T05:37:31.8005901Z    | |_- in this macro invocation
2020-02-28T05:37:31.8006355Z    |
2020-02-28T05:37:31.8006873Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-28T05:37:31.8007144Z 
---
2020-02-28T05:37:31.8019524Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-28T05:37:31.8019974Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-28T05:37:31.8038101Z 
2020-02-28T05:37:31.8038291Z 
2020-02-28T05:37:31.8042167Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-28T05:37:31.8044978Z 
2020-02-28T05:37:31.8045067Z 
2020-02-28T05:37:31.8048182Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-28T05:37:31.8048673Z Build completed unsuccessfully in 1:09:44
2020-02-28T05:37:31.8048673Z Build completed unsuccessfully in 1:09:44
2020-02-28T05:37:31.8099475Z == clock drift check ==
2020-02-28T05:37:31.8120247Z   local time: Fri Feb 28 05:37:31 UTC 2020
2020-02-28T05:37:32.3621253Z   network time: Fri, 28 Feb 2020 05:37:32 GMT
2020-02-28T05:37:32.3626603Z == end clock drift check ==
2020-02-28T05:37:32.7878500Z 
2020-02-28T05:37:32.7958330Z ##[error]Bash exited with code '1'.
2020-02-28T05:37:32.7974348Z ##[section]Finishing: Run build
2020-02-28T05:37:32.8025744Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69537/merge to s
2020-02-28T05:37:32.8030986Z Task         : Get sources
2020-02-28T05:37:32.8031350Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-28T05:37:32.8031724Z Version      : 1.0.0
2020-02-28T05:37:32.8031977Z Author       : Microsoft
2020-02-28T05:37:32.8031977Z Author       : Microsoft
2020-02-28T05:37:32.8032343Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-28T05:37:32.8032796Z ==============================================================================
2020-02-28T05:37:33.1609195Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-28T05:37:33.1652506Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69537/merge to s
2020-02-28T05:37:33.1740428Z Cleaning up task key
2020-02-28T05:37:33.1741871Z Start cleaning up orphan processes.
2020-02-28T05:37:33.1916899Z Terminate orphan process: pid (3695) (python)
2020-02-28T05:37:33.2145636Z ##[section]Finishing: Finalize Job
