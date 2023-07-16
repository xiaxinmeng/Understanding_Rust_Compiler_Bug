plain
2019-08-10T05:15:57.8813902Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-10T05:15:57.9493293Z ##[command]git config gc.auto 0
2019-08-10T05:15:57.9578076Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-10T05:15:57.9636820Z ##[command]git config --get-all http.proxy
2019-08-10T05:15:57.9805012Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63427/merge:refs/remotes/pull/63427/merge
---
2019-08-10T05:16:34.8941010Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-10T05:16:34.8941046Z 
2019-08-10T05:16:34.8941274Z   git checkout -b <new-branch-name>
2019-08-10T05:16:34.8941307Z 
2019-08-10T05:16:34.8941362Z HEAD is now at cb739d7fc Merge 2dfa3a9d458ab3940538507fac33754843b08ea9 into be8bbb06976c8065425b18e9cbe24a6d1d4e7515
2019-08-10T05:16:34.9135049Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-10T05:16:34.9138037Z ==============================================================================
2019-08-10T05:16:34.9138085Z Task         : Bash
2019-08-10T05:16:34.9138157Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-10T06:18:23.7162349Z .................................................................................................... 1300/8868
2019-08-10T06:18:30.3249950Z .................................................................................................... 1400/8868
2019-08-10T06:18:36.7360418Z .................................................................................................... 1500/8868
2019-08-10T06:18:47.5135186Z ....................................................................................i............... 1600/8868
2019-08-10T06:18:55.3271330Z i................................................................................................... 1700/8868
2019-08-10T06:19:02.0581053Z ...........................................................................iiiii.................... 1800/8868
2019-08-10T06:19:24.4908544Z .................................................................................................... 2000/8868
2019-08-10T06:19:27.0162700Z .................................................................................................... 2100/8868
2019-08-10T06:19:29.8150885Z .................................................................................................... 2200/8868
2019-08-10T06:19:37.7146529Z .................................................................................................... 2300/8868
---
2019-08-10T06:23:26.4214660Z .................................................................................................... 5200/8868
2019-08-10T06:23:37.1628445Z .................................................................................................... 5300/8868
2019-08-10T06:23:44.8147799Z ....i............................................................................................... 5400/8868
2019-08-10T06:23:50.2324208Z .................................................................................................... 5500/8868
2019-08-10T06:24:02.7427049Z ...................................................................................................i 5600/8868
2019-08-10T06:24:17.0351558Z i...i..ii...........i............................................................................... 5700/8868
2019-08-10T06:24:34.3453637Z .................................................................................................... 5900/8868
2019-08-10T06:24:39.1374220Z .................................................................................................... 6000/8868
2019-08-10T06:24:39.1374220Z .................................................................................................... 6000/8868
2019-08-10T06:24:53.8149813Z i..ii............................................................................................... 6100/8868
2019-08-10T06:25:15.6063237Z ...........................................i........................................................ 6300/8868
2019-08-10T06:25:18.5381070Z .................................................................................................... 6400/8868
2019-08-10T06:25:21.9179525Z ...............i.................................................................................... 6500/8868
2019-08-10T06:25:27.8380699Z .................................................................................................... 6600/8868
---
2019-08-10T06:29:37.8045477Z 
2019-08-10T06:29:37.8046022Z ---- [ui] ui/imports/glob-shadowing.rs stdout ----
2019-08-10T06:29:37.8046097Z diff of stderr:
2019-08-10T06:29:37.8046148Z 
2019-08-10T06:29:37.8046192Z 4 LL |         let x = env!("PATH");
2019-08-10T06:29:37.8046237Z 5    |                 ^^^ ambiguous name
2019-08-10T06:29:37.8046294Z 6    |
2019-08-10T06:29:37.8046533Z - note: `env` could refer to the macro imported here
2019-08-10T06:29:37.8046583Z +    = note: `env` could refer to a macro from prelude
2019-08-10T06:29:37.8046639Z + note: `env` could also refer to the macro imported here
2019-08-10T06:29:37.8046918Z 9    |
2019-08-10T06:29:37.8046959Z 10 LL |     use m::*;
2019-08-10T06:29:37.8047002Z 
2019-08-10T06:29:37.8047041Z 11    |         ^^^^
2019-08-10T06:29:37.8047041Z 11    |         ^^^^
2019-08-10T06:29:37.8047088Z 12    = help: consider adding an explicit import of `env` to disambiguate
2019-08-10T06:29:37.8047154Z 13    = help: or use `self::env` to refer to this macro unambiguously
2019-08-10T06:29:37.8047378Z - note: `env` could also refer to the macro defined here
2019-08-10T06:29:37.8047596Z -   --> $SRC_DIR/libstd/prelude/v1.rs:LL:COL
2019-08-10T06:29:37.8047967Z - LL |     env,
2019-08-10T06:29:37.8048140Z -    |     ^^^
2019-08-10T06:29:37.8048181Z 19 
2019-08-10T06:29:37.8048181Z 19 
2019-08-10T06:29:37.8048247Z 20 error[E0659]: `env` is ambiguous (glob import vs any other name from outer scope during import/macro resolution)
2019-08-10T06:29:37.8048498Z 
2019-08-10T06:29:37.8048498Z 
2019-08-10T06:29:37.8048554Z 23 LL |             let x = env!("PATH");
2019-08-10T06:29:37.8048598Z 24    |                     ^^^ ambiguous name
2019-08-10T06:29:37.8048638Z 25    |
2019-08-10T06:29:37.8048851Z - note: `env` could refer to the macro imported here
2019-08-10T06:29:37.8048914Z +    = note: `env` could refer to a macro from prelude
2019-08-10T06:29:37.8048961Z + note: `env` could also refer to the macro imported here
2019-08-10T06:29:37.8049614Z 28    |
2019-08-10T06:29:37.8049840Z 29 LL |         use m::*;
2019-08-10T06:29:37.8049881Z 
2019-08-10T06:29:37.8049922Z 30    |             ^^^^
2019-08-10T06:29:37.8049922Z 30    |             ^^^^
2019-08-10T06:29:37.8049988Z 31    = help: consider adding an explicit import of `env` to disambiguate
2019-08-10T06:29:37.8050315Z - note: `env` could also refer to the macro defined here
2019-08-10T06:29:37.8050534Z -   --> $SRC_DIR/libstd/prelude/v1.rs:LL:COL
2019-08-10T06:29:37.8050925Z - LL |     env,
2019-08-10T06:29:37.8051103Z -    |     ^^^
2019-08-10T06:29:37.8051161Z 37 
2019-08-10T06:29:37.8051161Z 37 
2019-08-10T06:29:37.8051213Z 38 error[E0659]: `fenv` is ambiguous (glob import vs any other name from outer scope during import/macro resolution)
2019-08-10T06:29:37.8051458Z 
2019-08-10T06:29:37.8051498Z 
2019-08-10T06:29:37.8051541Z The actual stderr differed from the expected stderr.
2019-08-10T06:29:37.8051841Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/glob-shadowing/glob-shadowing.stderr
2019-08-10T06:29:37.8051841Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/glob-shadowing/glob-shadowing.stderr
2019-08-10T06:29:37.8052099Z To update references, rerun the tests and pass the `--bless` flag
2019-08-10T06:29:37.8052349Z To only update this specific test, also pass `--test-args imports/glob-shadowing.rs`
2019-08-10T06:29:37.8052427Z error: 1 errors occurred comparing output.
2019-08-10T06:29:37.8052485Z status: exit code: 1
2019-08-10T06:29:37.8052485Z status: exit code: 1
2019-08-10T06:29:37.8053280Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/glob-shadowing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/glob-shadowing" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/glob-shadowing/auxiliary" "-A" "unused"
2019-08-10T06:29:37.8053739Z ------------------------------------------
2019-08-10T06:29:37.8053772Z 
2019-08-10T06:29:37.8053996Z ------------------------------------------
2019-08-10T06:29:37.8054038Z stderr:
2019-08-10T06:29:37.8054038Z stderr:
2019-08-10T06:29:37.8054234Z ------------------------------------------
2019-08-10T06:29:37.8054304Z error[E0659]: `env` is ambiguous (glob import vs any other name from outer scope during import/macro resolution)
2019-08-10T06:29:37.8054585Z    |
2019-08-10T06:29:37.8054585Z    |
2019-08-10T06:29:37.8054646Z LL |         let x = env!("PATH"); //~ ERROR `env` is ambiguous
2019-08-10T06:29:37.8054690Z    |                 ^^^ ambiguous name
2019-08-10T06:29:37.8054728Z    |
2019-08-10T06:29:37.8054787Z    = note: `env` could refer to a macro from prelude
2019-08-10T06:29:37.8054834Z note: `env` could also refer to the macro imported here
2019-08-10T06:29:37.8055099Z    |
2019-08-10T06:29:37.8055160Z LL |     use m::*;
2019-08-10T06:29:37.8055201Z    |         ^^^^
2019-08-10T06:29:37.8055201Z    |         ^^^^
2019-08-10T06:29:37.8055246Z    = help: consider adding an explicit import of `env` to disambiguate
2019-08-10T06:29:37.8055312Z    = help: or use `self::env` to refer to this macro unambiguously
2019-08-10T06:29:37.8055343Z 
2019-08-10T06:29:37.8055392Z error[E0659]: `env` is ambiguous (glob import vs any other name from outer scope during import/macro resolution)
2019-08-10T06:29:37.8055691Z    |
2019-08-10T06:29:37.8055691Z    |
2019-08-10T06:29:37.8055735Z LL |             let x = env!("PATH"); //~ ERROR `env` is ambiguous
2019-08-10T06:29:37.8055781Z    |                     ^^^ ambiguous name
2019-08-10T06:29:37.8055838Z    |
2019-08-10T06:29:37.8055880Z    = note: `env` could refer to a macro from prelude
2019-08-10T06:29:37.8055926Z note: `env` could also refer to the macro imported here
2019-08-10T06:29:37.8056294Z    |
2019-08-10T06:29:37.8056333Z LL |         use m::*;
2019-08-10T06:29:37.8056389Z    |             ^^^^
2019-08-10T06:29:37.8056389Z    |             ^^^^
2019-08-10T06:29:37.8056436Z    = help: consider adding an explicit import of `env` to disambiguate
2019-08-10T06:29:37.8056466Z 
2019-08-10T06:29:37.8056514Z error[E0659]: `fenv` is ambiguous (glob import vs any other name from outer scope during import/macro resolution)
2019-08-10T06:29:37.8056840Z    |
2019-08-10T06:29:37.8056840Z    |
2019-08-10T06:29:37.8056883Z LL |             let x = fenv!(); //~ ERROR `fenv` is ambiguous
2019-08-10T06:29:37.8056944Z    |                     ^^^^ ambiguous name
2019-08-10T06:29:37.8056984Z    |
2019-08-10T06:29:37.8057027Z note: `fenv` could refer to the macro imported here
2019-08-10T06:29:37.8057307Z    |
2019-08-10T06:29:37.8057345Z LL |         use m::*;
2019-08-10T06:29:37.8057410Z    |             ^^^^
2019-08-10T06:29:37.8057410Z    |             ^^^^
2019-08-10T06:29:37.8057456Z    = help: consider adding an explicit import of `fenv` to disambiguate
2019-08-10T06:29:37.8057503Z note: `fenv` could also refer to the macro defined here
2019-08-10T06:29:37.8057781Z    |
2019-08-10T06:29:37.8057781Z    |
2019-08-10T06:29:37.8057822Z LL |     pub macro fenv($e: expr) { $e }
2019-08-10T06:29:37.8057945Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-10T06:29:37.8058006Z    = help: use `self::fenv` to refer to this macro unambiguously
2019-08-10T06:29:37.8058076Z error: aborting due to 3 previous errors
2019-08-10T06:29:37.8058103Z 
2019-08-10T06:29:37.8058372Z For more information about this error, try `rustc --explain E0659`.
2019-08-10T06:29:37.8058406Z 
2019-08-10T06:29:37.8058406Z 
2019-08-10T06:29:37.8058601Z ------------------------------------------
2019-08-10T06:29:37.8058631Z 
2019-08-10T06:29:37.8058670Z 
2019-08-10T06:29:37.8058898Z ---- [ui] ui/imports/local-modularized-tricky-fail-1.rs stdout ----
2019-08-10T06:29:37.8058945Z diff of stderr:
2019-08-10T06:29:37.8058971Z 
2019-08-10T06:29:37.8059143Z 27 LL | include!();
2019-08-10T06:29:37.8059186Z 28    | ^^^^^^^ ambiguous name
2019-08-10T06:29:37.8059226Z 29    |
2019-08-10T06:29:37.8059739Z - note: `include` could refer to the macro defined here
2019-08-10T06:29:37.8059799Z +    = note: `include` could refer to a macro from prelude
2019-08-10T06:29:37.8059858Z + note: `include` could also refer to the macro defined here
2019-08-10T06:29:37.8060093Z 31   --> $DIR/local-modularized-tricky-fail-1.rs:17:5
2019-08-10T06:29:37.8060196Z 33 LL | /     macro_rules! include {
2019-08-10T06:29:37.8060226Z 
2019-08-10T06:29:37.8060284Z 38 LL |       define_include!();
2019-08-10T06:29:37.8060507Z 39    |       ------------------ in this macro invocation
2019-08-10T06:29:37.8060507Z 39    |       ------------------ in this macro invocation
2019-08-10T06:29:37.8060560Z 40    = help: use `crate::include` to refer to this macro unambiguously
2019-08-10T06:29:37.8060807Z - note: `include` could also refer to the macro defined here
2019-08-10T06:29:37.8061024Z -   --> $SRC_DIR/libstd/prelude/v1.rs:LL:COL
2019-08-10T06:29:37.8061382Z - LL |     include,
2019-08-10T06:29:37.8061581Z -    |     ^^^^^^^
2019-08-10T06:29:37.8061622Z 46 
2019-08-10T06:29:37.8061622Z 46 
2019-08-10T06:29:37.8061919Z 47 error[E0659]: `panic` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
2019-08-10T06:29:37.8062176Z 48   --> $DIR/local-modularized-tricky-fail-1.rs:35:5
2019-08-10T06:29:37.8062233Z 
2019-08-10T06:29:37.8062276Z The actual stderr differed from the expected stderr.
2019-08-10T06:29:37.8062276Z The actual stderr differed from the expected stderr.
2019-08-10T06:29:37.8062716Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/local-modularized-tricky-fail-1/local-modularized-tricky-fail-1.stderr
2019-08-10T06:29:37.8062953Z To update references, rerun the tests and pass the `--bless` flag
2019-08-10T06:29:37.8063334Z To only update this specific test, also pass `--test-args imports/local-modularized-tricky-fail-1.rs`
2019-08-10T06:29:37.8063424Z error: 1 errors occurred comparing output.
2019-08-10T06:29:37.8063466Z status: exit code: 1
2019-08-10T06:29:37.8063466Z status: exit code: 1
2019-08-10T06:29:37.8064235Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/local-modularized-tricky-fail-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/local-modularized-tricky-fail-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/local-modularized-tricky-fail-1/auxiliary" "-A" "unused"
2019-08-10T06:29:37.8064563Z ------------------------------------------
2019-08-10T06:29:37.8064596Z 
2019-08-10T06:29:37.8064804Z ------------------------------------------
2019-08-10T06:29:37.8064862Z stderr:
2019-08-10T06:29:37.8064862Z stderr:
2019-08-10T06:29:37.8065062Z ------------------------------------------
2019-08-10T06:29:37.8065337Z error[E0659]: `exported` is ambiguous (glob import vs macro-expanded name in the same module during import/macro resolution)
2019-08-10T06:29:37.8065626Z   --> /checkout/src/test/ui/imports/local-modularized-tricky-fail-1.rs:28:1
2019-08-10T06:29:37.8065770Z    |
2019-08-10T06:29:37.8065815Z LL | exported!(); //~ ERROR `exported` is ambiguous
2019-08-10T06:29:37.8065876Z    | ^^^^^^^^ ambiguous name
2019-08-10T06:29:37.8065919Z    |
2019-08-10T06:29:37.8065963Z note: `exported` could refer to the macro defined here
2019-08-10T06:29:37.8066272Z   --> /checkout/src/test/ui/imports/local-modularized-tricky-fail-1.rs:5:5
2019-08-10T06:29:37.8066364Z LL | /     macro_rules! exported {
2019-08-10T06:29:37.8066409Z LL | |         () => ()
2019-08-10T06:29:37.8066467Z LL | |     }
2019-08-10T06:29:37.8066518Z    | |_____^
2019-08-10T06:29:37.8066518Z    | |_____^
2019-08-10T06:29:37.8066558Z ...
2019-08-10T06:29:37.8066617Z LL |       define_exported!();
2019-08-10T06:29:37.8066856Z    |       ------------------- in this macro invocation
2019-08-10T06:29:37.8066909Z note: `exported` could also refer to the macro imported here
2019-08-10T06:29:37.8067180Z   --> /checkout/src/test/ui/imports/local-modularized-tricky-fail-1.rs:22:5
2019-08-10T06:29:37.8067280Z LL | use inner1::*;
2019-08-10T06:29:37.8067323Z    |     ^^^^^^^^^
2019-08-10T06:29:37.8067323Z    |     ^^^^^^^^^
2019-08-10T06:29:37.8067388Z    = help: consider adding an explicit import of `exported` to disambiguate
2019-08-10T06:29:37.8067421Z 
2019-08-10T06:29:37.8067731Z error[E0659]: `include` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
2019-08-10T06:29:37.8068010Z   --> /checkout/src/test/ui/imports/local-modularized-tricky-fail-1.rs:46:1
2019-08-10T06:29:37.8068058Z    |
2019-08-10T06:29:37.8068110Z LL | include!(); //~ ERROR `include` is ambiguous
2019-08-10T06:29:37.8068173Z    | ^^^^^^^ ambiguous name
2019-08-10T06:29:37.8068215Z    |
2019-08-10T06:29:37.8068261Z    = note: `include` could refer to a macro from prelude
2019-08-10T06:29:37.8068309Z note: `include` could also refer to the macro defined here
2019-08-10T06:29:37.8068588Z   --> /checkout/src/test/ui/imports/local-modularized-tricky-fail-1.rs:17:5
2019-08-10T06:29:37.8068688Z LL | /     macro_rules! include {
2019-08-10T06:29:37.8068746Z LL | |         () => ()
2019-08-10T06:29:37.8068788Z LL | |     }
2019-08-10T06:29:37.8068829Z    | |_____^
2019-08-10T06:29:37.8068829Z    | |_____^
2019-08-10T06:29:37.8068886Z ...
2019-08-10T06:29:37.8068928Z LL |       define_include!();
2019-08-10T06:29:37.8069285Z    |       ------------------ in this macro invocation
2019-08-10T06:29:37.8069538Z    = help: use `crate::include` to refer to this macro unambiguously
2019-08-10T06:29:37.8069599Z 
2019-08-10T06:29:37.8070088Z error[E0659]: `panic` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
2019-08-10T06:29:37.8070389Z   --> /checkout/src/test/ui/imports/local-modularized-tricky-fail-1.rs:35:5
2019-08-10T06:29:37.8070456Z    |
2019-08-10T06:29:37.8070501Z LL |     panic!(); //~ ERROR `panic` is ambiguous
2019-08-10T06:29:37.8070546Z    |     ^^^^^ ambiguous name
2019-08-10T06:29:37.8070605Z    |
2019-08-10T06:29:37.8070649Z    = note: `panic` could refer to a macro from prelude
2019-08-10T06:29:37.8070707Z note: `panic` could also refer to the macro defined here
2019-08-10T06:29:37.8070976Z   --> /checkout/src/test/ui/imports/local-modularized-tricky-fail-1.rs:11:5
2019-08-10T06:29:37.8071065Z LL | /     macro_rules! panic {
2019-08-10T06:29:37.8071107Z LL | |         () => ()
2019-08-10T06:29:37.8071165Z LL | |     }
2019-08-10T06:29:37.8071204Z    | |_____^
2019-08-10T06:29:37.8071204Z    | |_____^
2019-08-10T06:29:37.8071243Z ...
2019-08-10T06:29:37.8071301Z LL |       define_panic!();
2019-08-10T06:29:37.8071531Z    |       ---------------- in this macro invocation
2019-08-10T06:29:37.8071584Z    = help: use `crate::panic` to refer to this macro unambiguously
2019-08-10T06:29:37.8071674Z error: aborting due to 3 previous errors
2019-08-10T06:29:37.8071702Z 
2019-08-10T06:29:37.8071935Z For more information about this error, try `rustc --explain E0659`.
2019-08-10T06:29:37.8071969Z 
---
2019-08-10T06:29:37.8080245Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-10T06:29:37.8080347Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-10T06:29:37.8097261Z 
2019-08-10T06:29:37.8098314Z 
2019-08-10T06:29:37.8102356Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-10T06:29:37.8102786Z 
2019-08-10T06:29:37.8102832Z 
2019-08-10T06:29:37.8106138Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-10T06:29:37.8106202Z Build completed unsuccessfully in 1:06:50
2019-08-10T06:29:37.8106202Z Build completed unsuccessfully in 1:06:50
2019-08-10T06:29:38.5846744Z ##[error]Bash exited with code '1'.
2019-08-10T06:29:38.5887447Z ##[section]Starting: Checkout
2019-08-10T06:29:38.5889230Z ==============================================================================
2019-08-10T06:29:38.5889279Z Task         : Get sources
2019-08-10T06:29:38.5889320Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
