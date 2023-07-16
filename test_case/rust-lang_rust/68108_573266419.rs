plain
2020-01-11T00:35:41.0384373Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-11T00:35:41.0458725Z ##[command]git config gc.auto 0
2020-01-11T00:35:41.0529990Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-11T00:35:41.0587798Z ##[command]git config --get-all http.proxy
2020-01-11T00:35:41.0725563Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68108/merge:refs/remotes/pull/68108/merge
---
2020-01-11T01:31:03.3544563Z ........................................i...............i........................................... 4900/9513
2020-01-11T01:31:11.8924077Z .................................................................................................... 5000/9513
2020-01-11T01:31:18.1885732Z .....................................................................................i.............. 5100/9513
2020-01-11T01:31:23.4400401Z .................................................................................................... 5200/9513
2020-01-11T01:31:33.0224304Z ....................................................ii.ii...........i............................... 5300/9513
2020-01-11T01:31:41.4534588Z .................................................................................................... 5500/9513
2020-01-11T01:31:50.8356105Z .................................................................................................... 5600/9513
2020-01-11T01:31:57.2595466Z ....................................i............................................................... 5700/9513
2020-01-11T01:32:03.1987368Z .................................................................................................... 5800/9513
2020-01-11T01:32:03.1987368Z .................................................................................................... 5800/9513
2020-01-11T01:32:13.4073369Z .................................................................................................... 5900/9513
2020-01-11T01:32:23.0729432Z ...........................i.i..i..ii...........i................................................... 6000/9513
2020-01-11T01:32:39.9108823Z .................................................................................................... 6200/9513
2020-01-11T01:32:47.3277683Z .................................................................................................... 6300/9513
2020-01-11T01:32:47.3277683Z .................................................................................................... 6300/9513
2020-01-11T01:32:58.7138462Z ......................................................i..ii......................................... 6400/9513
2020-01-11T01:33:24.0543483Z .................................................................................................... 6600/9513
2020-01-11T01:33:25.9038376Z ..............................i..................................................................... 6700/9513
2020-01-11T01:33:27.8776982Z .................................................................................................... 6800/9513
2020-01-11T01:33:30.1895837Z ..............................i..................................................................... 6900/9513
---
2020-01-11T01:35:01.7764640Z .................................................................................................... 7500/9513
2020-01-11T01:35:05.5658135Z .................................................................................................... 7600/9513
2020-01-11T01:35:11.1643188Z .................................................................................................... 7700/9513
2020-01-11T01:35:18.1660143Z .................................................................................................... 7800/9513
2020-01-11T01:35:26.9180028Z ...............................................................................iiii................. 7900/9513
2020-01-11T01:35:42.4793769Z .............i......i............................................................................... 8100/9513
2020-01-11T01:35:47.2104826Z .................................................................................................... 8200/9513
2020-01-11T01:36:00.0220503Z .................................................................................................... 8300/9513
2020-01-11T01:36:09.4553406Z .................................................................................................... 8400/9513
---
2020-01-11T01:38:01.3688770Z 5    |                    ^^^^^
2020-01-11T01:38:01.3688929Z 6    |
2020-01-11T01:38:01.3689264Z + help: split the comparison into two...
2020-01-11T01:38:01.3689866Z +    |
2020-01-11T01:38:01.3690109Z + LL |     (0..13).collect < Vec && Vec <i32>>();
2020-01-11T01:38:01.3691265Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-11T01:38:01.3691588Z + help: ...or parenthesize one of the comparisons
2020-01-11T01:38:01.3691803Z +    |
2020-01-11T01:38:01.3692027Z + LL |     ((0..13).collect < Vec) <i32>>();
2020-01-11T01:38:01.3692272Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-11T01:38:01.3692492Z 7 help: use `::<...>` instead of `<...>` to specify type arguments
2020-01-11T01:38:01.3692938Z 9 LL |     (0..13).collect::<Vec<i32>>();
2020-01-11T01:38:01.3693486Z 
2020-01-11T01:38:01.3693486Z 
2020-01-11T01:38:01.3693699Z 26 LL |     (0..13).collect<Vec<i32>();
2020-01-11T01:38:01.3694038Z 28    |
2020-01-11T01:38:01.3694221Z + help: split the comparison into two...
2020-01-11T01:38:01.3694389Z +    |
2020-01-11T01:38:01.3694389Z +    |
2020-01-11T01:38:01.3694554Z + LL |     (0..13).collect < Vec && Vec <i32>();
2020-01-11T01:38:01.3696127Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-11T01:38:01.3696581Z + help: ...or parenthesize one of the comparisons
2020-01-11T01:38:01.3696766Z +    |
2020-01-11T01:38:01.3696947Z + LL |     ((0..13).collect < Vec) <i32>();
2020-01-11T01:38:01.3697249Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-11T01:38:01.3697415Z 29 help: use `::<...>` instead of `<...>` to specify type arguments
2020-01-11T01:38:01.3697774Z 31 LL |     (0..13).collect::<Vec<i32>();
2020-01-11T01:38:01.3697912Z 
2020-01-11T01:38:01.3698081Z 
2020-01-11T01:38:01.3698404Z The actual stderr differed from the expected stderr.
2020-01-11T01:38:01.3698404Z The actual stderr differed from the expected stderr.
2020-01-11T01:38:01.3698995Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-40396/issue-40396.stderr
2020-01-11T01:38:01.3700086Z To update references, rerun the tests and pass the `--bless` flag
2020-01-11T01:38:01.3700714Z To only update this specific test, also pass `--test-args did_you_mean/issue-40396.rs`
2020-01-11T01:38:01.3701208Z error: 1 errors occurred comparing output.
2020-01-11T01:38:01.3701421Z status: exit code: 1
2020-01-11T01:38:01.3701421Z status: exit code: 1
2020-01-11T01:38:01.3702525Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-40396.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-40396" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-40396/auxiliary" "-A" "unused"
2020-01-11T01:38:01.3703628Z ------------------------------------------
2020-01-11T01:38:01.3703826Z 
2020-01-11T01:38:01.3704187Z ------------------------------------------
2020-01-11T01:38:01.3704415Z stderr:
---
2020-01-11T01:38:01.3706416Z    |                    ^^^^^
2020-01-11T01:38:01.3706521Z    |
2020-01-11T01:38:01.3706648Z help: split the comparison into two...
2020-01-11T01:38:01.3706755Z    |
2020-01-11T01:38:01.3706868Z LL |     (0..13).collect < Vec && Vec <i32>>();
2020-01-11T01:38:01.3707107Z help: ...or parenthesize one of the comparisons
2020-01-11T01:38:01.3707251Z    |
2020-01-11T01:38:01.3707251Z    |
2020-01-11T01:38:01.3707359Z LL |     ((0..13).collect < Vec) <i32>>();
2020-01-11T01:38:01.3707468Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-11T01:38:01.3707595Z help: use `::<...>` instead of `<...>` to specify type arguments
2020-01-11T01:38:01.3707815Z LL |     (0..13).collect::<Vec<i32>>();
2020-01-11T01:38:01.3707941Z    |                    ^^
2020-01-11T01:38:01.3708032Z 
2020-01-11T01:38:01.3708140Z error: comparison operators may not be chained
2020-01-11T01:38:01.3708140Z error: comparison operators may not be chained
2020-01-11T01:38:01.3708564Z   --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:4:8
2020-01-11T01:38:01.3708717Z    |
2020-01-11T01:38:01.3708823Z LL |     Vec<i32>::new();
2020-01-11T01:38:01.3708947Z    |        ^^^^^
2020-01-11T01:38:01.3709049Z    |
2020-01-11T01:38:01.3709344Z help: use `::<...>` instead of `<...>` to specify type arguments
2020-01-11T01:38:01.3710001Z LL |     Vec::<i32>::new();
2020-01-11T01:38:01.3710146Z    |        ^^
2020-01-11T01:38:01.3710293Z 
2020-01-11T01:38:01.3710436Z error: comparison operators may not be chained
2020-01-11T01:38:01.3710436Z error: comparison operators may not be chained
2020-01-11T01:38:01.3710864Z   --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:6:20
2020-01-11T01:38:01.3711071Z    |
2020-01-11T01:38:01.3711211Z LL |     (0..13).collect<Vec<i32>();
2020-01-11T01:38:01.3711520Z    |
2020-01-11T01:38:01.3711661Z help: split the comparison into two...
2020-01-11T01:38:01.3711804Z    |
2020-01-11T01:38:01.3711804Z    |
2020-01-11T01:38:01.3711966Z LL |     (0..13).collect < Vec && Vec <i32>();
2020-01-11T01:38:01.3712292Z help: ...or parenthesize one of the comparisons
2020-01-11T01:38:01.3712442Z    |
2020-01-11T01:38:01.3712442Z    |
2020-01-11T01:38:01.3712582Z LL |     ((0..13).collect < Vec) <i32>();
2020-01-11T01:38:01.3712746Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-11T01:38:01.3713181Z help: use `::<...>` instead of `<...>` to specify type arguments
2020-01-11T01:38:01.3713697Z LL |     (0..13).collect::<Vec<i32>();
2020-01-11T01:38:01.3713823Z    |                    ^^
2020-01-11T01:38:01.3713925Z 
2020-01-11T01:38:01.3714064Z error: aborting due to 3 previous errors
---
2020-01-11T01:38:01.3714878Z 
2020-01-11T01:38:01.3715369Z ---- [ui] ui/parser/require-parens-for-chained-comparison.rs stdout ----
2020-01-11T01:38:01.3715536Z diff of stderr:
2020-01-11T01:38:01.3715643Z 
2020-01-11T01:38:01.3715924Z 27 LL |     f<Result<Option<X>, Option<Option<X>>>(1, 2);
2020-01-11T01:38:01.3716616Z 29    |
2020-01-11T01:38:01.3716723Z + help: split the comparison into two...
2020-01-11T01:38:01.3716828Z +    |
2020-01-11T01:38:01.3716828Z +    |
2020-01-11T01:38:01.3716964Z + LL |     f < Result && Result <Option<X>, Option<Option<X>>>(1, 2);
2020-01-11T01:38:01.3717078Z +    |     ^^^^^^^^^^^^^^^^^^^^^^
2020-01-11T01:38:01.3717439Z + help: ...or parenthesize one of the comparisons
2020-01-11T01:38:01.3717629Z +    |
2020-01-11T01:38:01.3717747Z + LL |     (f < Result) <Option<X>, Option<Option<X>>>(1, 2);
2020-01-11T01:38:01.3717876Z +    |     ^^^^^^^^^^^^^^
2020-01-11T01:38:01.3717990Z 30 help: use `::<...>` instead of `<...>` to specify type arguments
2020-01-11T01:38:01.3718098Z 31    |
2020-01-11T01:38:01.3718225Z 32 LL |     f::<Result<Option<X>, Option<Option<X>>>(1, 2);
2020-01-11T01:38:01.3718412Z 
2020-01-11T01:38:01.3718534Z The actual stderr differed from the expected stderr.
2020-01-11T01:38:01.3718967Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/require-parens-for-chained-comparison/require-parens-for-chained-comparison.stderr
2020-01-11T01:38:01.3718967Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/require-parens-for-chained-comparison/require-parens-for-chained-comparison.stderr
2020-01-11T01:38:01.3720030Z To update references, rerun the tests and pass the `--bless` flag
2020-01-11T01:38:01.3720690Z To only update this specific test, also pass `--test-args parser/require-parens-for-chained-comparison.rs`
2020-01-11T01:38:01.3721028Z error: 1 errors occurred comparing output.
2020-01-11T01:38:01.3721205Z status: exit code: 1
2020-01-11T01:38:01.3721205Z status: exit code: 1
2020-01-11T01:38:01.3722290Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/require-parens-for-chained-comparison.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/require-parens-for-chained-comparison" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/require-parens-for-chained-comparison/auxiliary" "-A" "unused"
2020-01-11T01:38:01.3722965Z ------------------------------------------
2020-01-11T01:38:01.3723158Z 
2020-01-11T01:38:01.3723636Z ------------------------------------------
2020-01-11T01:38:01.3723767Z stderr:
2020-01-11T01:38:01.3723767Z stderr:
2020-01-11T01:38:01.3724049Z ------------------------------------------
2020-01-11T01:38:01.3724185Z error: comparison operators may not be chained
2020-01-11T01:38:01.3724478Z   --> /checkout/src/test/ui/parser/require-parens-for-chained-comparison.rs:5:11
2020-01-11T01:38:01.3724636Z    |
2020-01-11T01:38:01.3724747Z LL |     false == false == false;
2020-01-11T01:38:01.3724961Z 
2020-01-11T01:38:01.3725070Z error: comparison operators may not be chained
2020-01-11T01:38:01.3725402Z   --> /checkout/src/test/ui/parser/require-parens-for-chained-comparison.rs:8:11
2020-01-11T01:38:01.3725538Z    |
2020-01-11T01:38:01.3725538Z    |
2020-01-11T01:38:01.3725645Z LL |     false == 0 < 2;
2020-01-11T01:38:01.3725766Z    |           ^^^^^^
2020-01-11T01:38:01.3725962Z 
2020-01-11T01:38:01.3726110Z error: comparison operators may not be chained
2020-01-11T01:38:01.3726457Z   --> /checkout/src/test/ui/parser/require-parens-for-chained-comparison.rs:13:6
2020-01-11T01:38:01.3726595Z    |
2020-01-11T01:38:01.3726701Z LL |     f<X>();
2020-01-11T01:38:01.3726950Z    |
2020-01-11T01:38:01.3726950Z    |
2020-01-11T01:38:01.3727058Z help: use `::<...>` instead of `<...>` to specify type arguments
2020-01-11T01:38:01.3727181Z    |
2020-01-11T01:38:01.3727286Z LL |     f::<X>();
2020-01-11T01:38:01.3727510Z 
2020-01-11T01:38:01.3727620Z error: comparison operators may not be chained
2020-01-11T01:38:01.3727926Z   --> /checkout/src/test/ui/parser/require-parens-for-chained-comparison.rs:17:6
2020-01-11T01:38:01.3728196Z    |
2020-01-11T01:38:01.3728196Z    |
2020-01-11T01:38:01.3728310Z LL |     f<Result<Option<X>, Option<Option<X>>>(1, 2);
2020-01-11T01:38:01.3728539Z    |
2020-01-11T01:38:01.3728653Z help: split the comparison into two...
2020-01-11T01:38:01.3728760Z    |
2020-01-11T01:38:01.3728760Z    |
2020-01-11T01:38:01.3728886Z LL |     f < Result && Result <Option<X>, Option<Option<X>>>(1, 2);
2020-01-11T01:38:01.3729278Z help: ...or parenthesize one of the comparisons
2020-01-11T01:38:01.3729871Z    |
2020-01-11T01:38:01.3729871Z    |
2020-01-11T01:38:01.3730079Z LL |     (f < Result) <Option<X>, Option<Option<X>>>(1, 2);
2020-01-11T01:38:01.3730239Z    |     ^^^^^^^^^^^^^^
2020-01-11T01:38:01.3730411Z help: use `::<...>` instead of `<...>` to specify type arguments
2020-01-11T01:38:01.3730561Z    |
2020-01-11T01:38:01.3730738Z LL |     f::<Result<Option<X>, Option<Option<X>>>(1, 2);
2020-01-11T01:38:01.3731023Z 
2020-01-11T01:38:01.3731197Z error: comparison operators may not be chained
2020-01-11T01:38:01.3731654Z   --> /checkout/src/test/ui/parser/require-parens-for-chained-comparison.rs:22:21
2020-01-11T01:38:01.3731838Z    |
2020-01-11T01:38:01.3731838Z    |
2020-01-11T01:38:01.3732026Z LL |     let _ = identity<u8>;
2020-01-11T01:38:01.3732176Z    |                     ^^^^
2020-01-11T01:38:01.3732318Z    |
2020-01-11T01:38:01.3732483Z    = help: use `::<...>` instead of `<...>` to specify type arguments
2020-01-11T01:38:01.3732660Z    = help: or use `(...)` if you meant to specify fn arguments
2020-01-11T01:38:01.3733106Z error[E0308]: mismatched types
2020-01-11T01:38:01.3733634Z   --> /checkout/src/test/ui/parser/require-parens-for-chained-comparison.rs:8:14
2020-01-11T01:38:01.3733786Z    |
2020-01-11T01:38:01.3733921Z LL |     false == 0 < 2;
---
2020-01-11T01:38:01.3746767Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:386:22
2020-01-11T01:38:01.3747361Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-11T01:38:01.3765208Z 
2020-01-11T01:38:01.3765464Z 
2020-01-11T01:38:01.3767004Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-11T01:38:01.3767803Z 
2020-01-11T01:38:01.3767908Z 
2020-01-11T01:38:01.3774673Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-11T01:38:01.3774730Z Build completed unsuccessfully in 0:57:07
2020-01-11T01:38:01.3774730Z Build completed unsuccessfully in 0:57:07
2020-01-11T01:38:01.3821482Z == clock drift check ==
2020-01-11T01:38:01.3841200Z   local time: Sat Jan 11 01:38:01 UTC 2020
2020-01-11T01:38:01.5460272Z   network time: Sat, 11 Jan 2020 01:38:01 GMT
2020-01-11T01:38:01.5464264Z == end clock drift check ==
2020-01-11T01:38:01.9811260Z 
2020-01-11T01:38:01.9920952Z ##[error]Bash exited with code '1'.
2020-01-11T01:38:01.9960005Z ##[section]Starting: Checkout
2020-01-11T01:38:01.9961655Z ==============================================================================
2020-01-11T01:38:01.9961697Z Task         : Get sources
2020-01-11T01:38:01.9961734Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
