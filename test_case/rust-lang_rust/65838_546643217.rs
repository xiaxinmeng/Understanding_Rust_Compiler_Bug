plain
2019-10-26T20:48:39.1880666Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-26T20:48:39.2113769Z ##[command]git config gc.auto 0
2019-10-26T20:48:39.2203105Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-26T20:48:39.2267759Z ##[command]git config --get-all http.proxy
2019-10-26T20:48:39.2449097Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65838/merge:refs/remotes/pull/65838/merge
---
2019-10-26T21:52:37.5851608Z .................................................................................................... 1600/9254
2019-10-26T21:52:43.5960270Z .................................................................................................... 1700/9254
2019-10-26T21:52:57.2116789Z ........................................................i...............i........................... 1800/9254
2019-10-26T21:53:05.3207148Z .................................................................................................... 1900/9254
2019-10-26T21:53:21.1553415Z ..............................................iiiii................................................. 2000/9254
2019-10-26T21:53:32.8533688Z .................................................................................................... 2200/9254
2019-10-26T21:53:35.6076887Z .................................................................................................... 2300/9254
2019-10-26T21:53:39.9061107Z .................................................................................................... 2400/9254
2019-10-26T21:54:04.7059552Z .................................................................................................... 2500/9254
---
2019-10-26T21:57:08.7975372Z .................................................i...............i.................................. 4800/9254
2019-10-26T21:57:18.6208195Z .................................................................................................... 4900/9254
2019-10-26T21:57:27.6460205Z .................................................................................................... 5000/9254
2019-10-26T21:57:34.4693948Z .................................................................................................... 5100/9254
2019-10-26T21:57:45.7603757Z ..................................................ii.ii...........i................................. 5200/9254
2019-10-26T21:57:56.3885650Z .................................................................................................... 5400/9254
2019-10-26T21:58:06.2949798Z .................................................................................................... 5500/9254
2019-10-26T21:58:14.8047426Z ....................i............................................................................... 5600/9254
2019-10-26T21:58:20.8243714Z .................................................................................................... 5700/9254
2019-10-26T21:58:20.8243714Z .................................................................................................... 5700/9254
2019-10-26T21:58:33.8953426Z .................................................................................................... 5800/9254
2019-10-26T21:58:46.0190381Z .................ii...i..ii...........i............................................................. 5900/9254
2019-10-26T21:59:09.2685870Z .................................................................................................... 6100/9254
2019-10-26T21:59:18.4445058Z .................................................................................................... 6200/9254
2019-10-26T21:59:18.4445058Z .................................................................................................... 6200/9254
2019-10-26T21:59:33.5750550Z ........................................i..ii....................................................... 6300/9254
2019-10-26T21:59:57.4298138Z .................................................................................................... 6500/9254
2019-10-26T21:59:59.8193364Z ......i............................................................................................. 6600/9254
2019-10-26T22:00:02.2607516Z ................................................................................F...i............... 6700/9254
2019-10-26T22:00:05.1446369Z .................................................................................................... 6800/9254
---
2019-10-26T22:04:19.7981578Z ..........................................................i......................................... 9200/9254
2019-10-26T22:04:31.3004696Z ......................................................
2019-10-26T22:04:31.3006073Z failures:
2019-10-26T22:04:31.3052587Z 
2019-10-26T22:04:31.3053477Z ---- [ui] ui/parser/mismatched-braces/missing-close-brace-in-struct.rs stdout ----
2019-10-26T22:04:31.3054109Z 
2019-10-26T22:04:31.3054352Z 12    |
2019-10-26T22:04:31.3054352Z 12    |
2019-10-26T22:04:31.3054585Z 13 LL | trait T {
2019-10-26T22:04:31.3055291Z +    |
2019-10-26T22:04:31.3055507Z 15 help: you can escape reserved keywords to use them as identifiers
2019-10-26T22:04:31.3055907Z 16    |
2019-10-26T22:04:31.3055907Z 16    |
2019-10-26T22:04:31.3056110Z 17 LL | r#trait T {
2019-10-26T22:04:31.3056499Z 
2019-10-26T22:04:31.3056724Z The actual stderr differed from the expected stderr.
2019-10-26T22:04:31.3056724Z The actual stderr differed from the expected stderr.
2019-10-26T22:04:31.3057314Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mismatched-braces/missing-close-brace-in-struct/missing-close-brace-in-struct.stderr
2019-10-26T22:04:31.3057903Z To update references, rerun the tests and pass the `--bless` flag
2019-10-26T22:04:31.3058525Z To only update this specific test, also pass `--test-args parser/mismatched-braces/missing-close-brace-in-struct.rs`
2019-10-26T22:04:31.3059050Z error: 1 errors occurred comparing output.
2019-10-26T22:04:31.3059269Z status: exit code: 1
2019-10-26T22:04:31.3059269Z status: exit code: 1
2019-10-26T22:04:31.3060629Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mismatched-braces/missing-close-brace-in-struct" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mismatched-braces/missing-close-brace-in-struct/auxiliary" "-A" "unused"
2019-10-26T22:04:31.3062150Z ------------------------------------------
2019-10-26T22:04:31.3062431Z 
2019-10-26T22:04:31.3062940Z ------------------------------------------
2019-10-26T22:04:31.3063519Z stderr:
2019-10-26T22:04:31.3063519Z stderr:
2019-10-26T22:04:31.3064042Z ------------------------------------------
2019-10-26T22:04:31.3064751Z error: this file contains an un-closed delimiter
2019-10-26T22:04:31.3065297Z   --> /checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-struct.rs:14:66
2019-10-26T22:04:31.3065567Z    |
2019-10-26T22:04:31.3065813Z LL | pub(crate) struct Bar<T> { //~ ERROR `main` function not found
2019-10-26T22:04:31.3066258Z    |                          - un-closed delimiter
2019-10-26T22:04:31.3067001Z LL | fn main() {} //~ ERROR this file contains an un-closed delimiter
2019-10-26T22:04:31.3067290Z    |                                                                  ^
2019-10-26T22:04:31.3067490Z 
2019-10-26T22:04:31.3067737Z error: expected identifier, found keyword `trait`
2019-10-26T22:04:31.3067737Z error: expected identifier, found keyword `trait`
2019-10-26T22:04:31.3068224Z   --> /checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-struct.rs:4:1
2019-10-26T22:04:31.3068490Z    |
2019-10-26T22:04:31.3068733Z LL | trait T { //~ ERROR expected identifier, found keyword `trait`
2019-10-26T22:04:31.3069704Z    |
2019-10-26T22:04:31.3070091Z help: you can escape reserved keywords to use them as identifiers
2019-10-26T22:04:31.3070307Z    |
2019-10-26T22:04:31.3070307Z    |
2019-10-26T22:04:31.3071129Z LL | r#trait T { //~ ERROR expected identifier, found keyword `trait`
2019-10-26T22:04:31.3071602Z 
2019-10-26T22:04:31.3071602Z 
2019-10-26T22:04:31.3071835Z error: expected `:`, found `T`
2019-10-26T22:04:31.3072423Z   --> /checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-struct.rs:4:7
2019-10-26T22:04:31.3072883Z    |
2019-10-26T22:04:31.3073201Z LL | trait T { //~ ERROR expected identifier, found keyword `trait`
2019-10-26T22:04:31.3073449Z    |       ^ expected `:`
2019-10-26T22:04:31.3073790Z 
2019-10-26T22:04:31.3074198Z error[E0601]: `main` function not found in crate `missing_close_brace_in_struct`
2019-10-26T22:04:31.3074734Z   --> /checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-struct.rs:1:1
2019-10-26T22:04:31.3075012Z    |
2019-10-26T22:04:31.3075424Z LL | / pub(crate) struct Bar<T> { //~ ERROR `main` function not found
2019-10-26T22:04:31.3075652Z LL | |   foo: T,
2019-10-26T22:04:31.3075864Z LL | |
2019-10-26T22:04:31.3076109Z LL | | trait T { //~ ERROR expected identifier, found keyword `trait`
2019-10-26T22:04:31.3076721Z LL | |
2019-10-26T22:04:31.3077762Z LL | | fn main() {} //~ ERROR this file contains an un-closed delimiter
2019-10-26T22:04:31.3077762Z LL | | fn main() {} //~ ERROR this file contains an un-closed delimiter
2019-10-26T22:04:31.3079677Z    | |_________________________________________________________________^ consider adding a `main` function to `/checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-struct.rs`
2019-10-26T22:04:31.3081754Z error: aborting due to 4 previous errors
2019-10-26T22:04:31.3081796Z 
2019-10-26T22:04:31.3082158Z For more information about this error, try `rustc --explain E0601`.
2019-10-26T22:04:31.3082216Z 
2019-10-26T22:04:31.3082216Z 
2019-10-26T22:04:31.3082453Z ------------------------------------------
2019-10-26T22:04:31.3082484Z 
2019-10-26T22:04:31.3082510Z 
2019-10-26T22:04:31.3082536Z 
2019-10-26T22:04:31.3082594Z failures:
2019-10-26T22:04:31.3082852Z     [ui] ui/parser/mismatched-braces/missing-close-brace-in-struct.rs
2019-10-26T22:04:31.3083190Z test result: FAILED. 9212 passed; 1 failed; 41 ignored; 0 measured; 0 filtered out
2019-10-26T22:04:31.3083226Z 
2019-10-26T22:04:31.3091918Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-26T22:04:31.3092018Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-26T22:04:31.3092018Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-26T22:04:31.3117618Z 
2019-10-26T22:04:31.3117722Z 
2019-10-26T22:04:31.3119320Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-26T22:04:31.3119605Z 
2019-10-26T22:04:31.3119632Z 
2019-10-26T22:04:31.3146656Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-26T22:04:31.3146739Z Build completed unsuccessfully in 1:08:41
2019-10-26T22:04:31.3146739Z Build completed unsuccessfully in 1:08:41
2019-10-26T22:04:31.3176661Z == clock drift check ==
2019-10-26T22:04:31.3191613Z   local time: Sat Oct 26 22:04:31 UTC 2019
2019-10-26T22:04:31.4047747Z   network time: Sat, 26 Oct 2019 22:04:31 GMT
2019-10-26T22:04:31.4052533Z == end clock drift check ==
2019-10-26T22:04:32.3885449Z 
2019-10-26T22:04:32.3973728Z ##[error]Bash exited with code '1'.
2019-10-26T22:04:32.4028008Z ##[section]Starting: Checkout
2019-10-26T22:04:32.4032434Z ==============================================================================
2019-10-26T22:04:32.4032498Z Task         : Get sources
2019-10-26T22:04:32.4032564Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
