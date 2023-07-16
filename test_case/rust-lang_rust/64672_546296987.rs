plain
2019-10-25T09:12:18.9198146Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-25T09:12:18.9434835Z ##[command]git config gc.auto 0
2019-10-25T09:12:18.9505340Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-25T09:12:18.9562323Z ##[command]git config --get-all http.proxy
2019-10-25T09:12:18.9734539Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64672/merge:refs/remotes/pull/64672/merge
---
2019-10-25T10:11:29.7761723Z .................................................................................................... 1600/9249
2019-10-25T10:11:35.1844554Z .................................................................................................... 1700/9249
2019-10-25T10:11:47.5496701Z ........................................................i...............i........................... 1800/9249
2019-10-25T10:11:54.9015861Z .................................................................................................... 1900/9249
2019-10-25T10:12:09.3820145Z ..............................................iiiii................................................. 2000/9249
2019-10-25T10:12:19.9953662Z .................................................................................................... 2200/9249
2019-10-25T10:12:22.5311543Z .................................................................................................... 2300/9249
2019-10-25T10:12:26.3704456Z .................................................................................................... 2400/9249
2019-10-25T10:12:49.2817663Z .................................................................................................... 2500/9249
---
2019-10-25T10:15:38.3285185Z ..................................................i...............i................................. 4800/9249
2019-10-25T10:15:47.2801491Z .................................................................................................... 4900/9249
2019-10-25T10:15:55.7468382Z .................................................................................................... 5000/9249
2019-10-25T10:16:01.8495198Z .................................................................................................... 5100/9249
2019-10-25T10:16:12.0831627Z ...................................................ii.ii............................................ 5200/9249
2019-10-25T10:16:21.7593616Z .................................................................................................... 5400/9249
2019-10-25T10:16:30.9241065Z .................................................................................................... 5500/9249
2019-10-25T10:16:38.5474791Z ..................i................................................................................. 5600/9249
2019-10-25T10:16:43.9382397Z .................................................................................................... 5700/9249
2019-10-25T10:16:43.9382397Z .................................................................................................... 5700/9249
2019-10-25T10:16:55.8509790Z .................................................................................................... 5800/9249
2019-10-25T10:17:07.3340006Z ...............ii...i..ii...........i............................................................... 5900/9249
2019-10-25T10:17:28.7205452Z .................................................................................................... 6100/9249
2019-10-25T10:17:33.1485220Z .................................................................................................... 6200/9249
2019-10-25T10:17:33.1485220Z .................................................................................................... 6200/9249
2019-10-25T10:17:45.9862238Z ......................................i..ii......................................................... 6300/9249
2019-10-25T10:18:07.5083845Z .................................................................................................... 6500/9249
2019-10-25T10:18:09.6797304Z ....i............................................................................................... 6600/9249
2019-10-25T10:18:11.9257796Z ...............................................................................i.................... 6700/9249
2019-10-25T10:18:14.6009451Z .................................................................................................... 6800/9249
---
2019-10-25T10:22:17.3117978Z ---- [ui] ui/feature-gates/feature-gate-extern_types.rs stdout ----
2019-10-25T10:22:17.3118027Z 
2019-10-25T10:22:17.3118080Z error: ui test compiled successfully!
2019-10-25T10:22:17.3118127Z status: exit code: 0
2019-10-25T10:22:17.3119113Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-extern_types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-extern_types" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-extern_types/auxiliary" "-A" "unused"
2019-10-25T10:22:17.3119519Z ------------------------------------------
2019-10-25T10:22:17.3119567Z 
2019-10-25T10:22:17.3119788Z ------------------------------------------
2019-10-25T10:22:17.3119852Z stderr:
---
2019-10-25T10:22:17.3120947Z 26 
2019-10-25T10:22:17.3120973Z 
2019-10-25T10:22:17.3120998Z 
2019-10-25T10:22:17.3121059Z The actual stderr differed from the expected stderr.
2019-10-25T10:22:17.3121398Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias/trait-alias-syntax-fail/trait-alias-syntax-fail.stderr
2019-10-25T10:22:17.3121674Z To update references, rerun the tests and pass the `--bless` flag
2019-10-25T10:22:17.3121960Z To only update this specific test, also pass `--test-args traits/trait-alias/trait-alias-syntax-fail.rs`
2019-10-25T10:22:17.3122041Z error: 1 errors occurred comparing output.
2019-10-25T10:22:17.3122110Z status: exit code: 1
2019-10-25T10:22:17.3122110Z status: exit code: 1
2019-10-25T10:22:17.3122866Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-alias/trait-alias-syntax-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias/trait-alias-syntax-fail" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias/trait-alias-syntax-fail/auxiliary" "-A" "unused"
2019-10-25T10:22:17.3123345Z ------------------------------------------
2019-10-25T10:22:17.3123379Z 
2019-10-25T10:22:17.3123610Z ------------------------------------------
2019-10-25T10:22:17.3123654Z stderr:
2019-10-25T10:22:17.3123654Z stderr:
2019-10-25T10:22:17.3123862Z ------------------------------------------
2019-10-25T10:22:17.3123936Z error: trait aliases cannot be `auto`
2019-10-25T10:22:17.3124190Z   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-syntax-fail.rs:4:1
2019-10-25T10:22:17.3124241Z    |
2019-10-25T10:22:17.3124308Z LL | auto trait A = Foo; //~ ERROR trait aliases cannot be `auto`
2019-10-25T10:22:17.3124360Z    | ^^^^^^^^^^^^^^^^^^^ trait aliases cannot be `auto`
2019-10-25T10:22:17.3124432Z error: trait aliases cannot be `unsafe`
2019-10-25T10:22:17.3124700Z   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-syntax-fail.rs:5:1
2019-10-25T10:22:17.3124757Z    |
2019-10-25T10:22:17.3124757Z    |
2019-10-25T10:22:17.3124804Z LL | unsafe trait B = Foo; //~ ERROR trait aliases cannot be `unsafe`
2019-10-25T10:22:17.3124872Z    | ^^^^^^^^^^^^^^^^^^^^^ trait aliases cannot be `unsafe`
2019-10-25T10:22:17.3125180Z error: bounds are not allowed on trait aliases
2019-10-25T10:22:17.3125514Z   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-syntax-fail.rs:7:8
2019-10-25T10:22:17.3125675Z    |
2019-10-25T10:22:17.3125675Z    |
2019-10-25T10:22:17.3125734Z LL | trait C: Ord = Eq; //~ ERROR bounds are not allowed on trait aliases
2019-10-25T10:22:17.3125824Z 
2019-10-25T10:22:17.3125867Z error: bounds are not allowed on trait aliases
2019-10-25T10:22:17.3126154Z   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-syntax-fail.rs:8:8
2019-10-25T10:22:17.3126220Z    |
2019-10-25T10:22:17.3126220Z    |
2019-10-25T10:22:17.3126267Z LL | trait D: = Eq; //~ ERROR bounds are not allowed on trait aliases
2019-10-25T10:22:17.3126341Z 
2019-10-25T10:22:17.3126408Z error: aborting due to 4 previous errors
2019-10-25T10:22:17.3126437Z 
2019-10-25T10:22:17.3126462Z 
---
2019-10-25T10:22:17.3167557Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-25T10:22:17.3167637Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-25T10:22:17.3185445Z 
2019-10-25T10:22:17.3186286Z 
2019-10-25T10:22:17.3191108Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-25T10:22:17.3191625Z 
2019-10-25T10:22:17.3191659Z 
2019-10-25T10:22:17.3199069Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-25T10:22:17.3199144Z Build completed unsuccessfully in 1:03:31
2019-10-25T10:22:17.3199144Z Build completed unsuccessfully in 1:03:31
2019-10-25T10:22:17.3257306Z == clock drift check ==
2019-10-25T10:22:17.3279018Z   local time: Fri Oct 25 10:22:17 UTC 2019
2019-10-25T10:22:17.6174697Z   network time: Fri, 25 Oct 2019 10:22:17 GMT
2019-10-25T10:22:17.6174874Z == end clock drift check ==
2019-10-25T10:22:18.6617940Z 
2019-10-25T10:22:18.6732350Z ##[error]Bash exited with code '1'.
2019-10-25T10:22:18.6768552Z ##[section]Starting: Checkout
2019-10-25T10:22:18.6770322Z ==============================================================================
2019-10-25T10:22:18.6770379Z Task         : Get sources
2019-10-25T10:22:18.6770446Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
