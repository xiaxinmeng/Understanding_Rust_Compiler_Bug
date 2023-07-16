plain
2019-07-31T20:39:01.3877161Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-31T20:39:01.4062705Z ##[command]git config gc.auto 0
2019-07-31T20:39:01.4130694Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-31T20:39:01.4181920Z ##[command]git config --get-all http.proxy
2019-07-31T20:39:01.4327277Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63174/merge:refs/remotes/pull/63174/merge
---
2019-07-31T20:39:38.4692191Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-31T20:39:38.4692237Z 
2019-07-31T20:39:38.4692404Z   git checkout -b <new-branch-name>
2019-07-31T20:39:38.4692428Z 
2019-07-31T20:39:38.4692466Z HEAD is now at 68b53628c Merge dff0ba1228c90471713f00fb46e3d79666f20d96 into e3976fff44e6ce14c2f92252e6a807800b9aa7c0
2019-07-31T20:39:38.4842667Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-31T20:39:38.4845009Z ==============================================================================
2019-07-31T20:39:38.4845055Z Task         : Bash
2019-07-31T20:39:38.4845092Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-31T21:37:10.1884736Z .................................................................................................... 1400/8820
2019-07-31T21:37:15.5485685Z ..............................................................F..................................... 1500/8820
2019-07-31T21:37:27.4023434Z .................................................................i...............i.................. 1600/8820
2019-07-31T21:37:34.1634914Z .................................................................................................... 1700/8820
2019-07-31T21:37:47.8374916Z ...................................................iiiii............................................ 1800/8820
2019-07-31T21:37:58.1497053Z .................................................................................................... 2000/8820
2019-07-31T21:38:00.4413097Z .................................................................................................... 2100/8820
2019-07-31T21:38:03.7227598Z .................................................................................................... 2200/8820
2019-07-31T21:38:10.4954023Z .................................................................................................... 2300/8820
---
2019-07-31T21:41:47.3508264Z .................................................................................................... 5300/8820
2019-07-31T21:41:54.3855482Z ..............i..................................................................................... 5400/8820
2019-07-31T21:41:59.6714496Z .................................................................................................... 5500/8820
2019-07-31T21:42:11.2891411Z .................................................................................................... 5600/8820
2019-07-31T21:42:24.1769965Z ........ii...i..ii...........i...................................................................... 5700/8820
2019-07-31T21:42:37.2939813Z .................................................................................................... 5900/8820
2019-07-31T21:42:41.7340413Z .................................................................................................... 6000/8820
2019-07-31T21:42:41.7340413Z .................................................................................................... 6000/8820
2019-07-31T21:42:54.7217057Z ........i..ii....................................................................................... 6100/8820
2019-07-31T21:43:12.4665135Z ...................................................i................................................ 6300/8820
2019-07-31T21:43:14.4152165Z .................................................................................................... 6400/8820
2019-07-31T21:43:16.6383584Z ......................i............................................................................. 6500/8820
2019-07-31T21:43:20.8156536Z .................................................................................................... 6600/8820
---
2019-07-31T21:46:59.9379415Z 
2019-07-31T21:46:59.9379811Z ---- [ui] ui/consts/issue-51559.rs stdout ----
2019-07-31T21:46:59.9379856Z diff of stderr:
2019-07-31T21:46:59.9379881Z 
2019-07-31T21:46:59.9380102Z - error[E0080]: it is undefined behavior to use this value
2019-07-31T21:46:59.9380299Z -   --> $DIR/issue-51559.rs:4:1
2019-07-31T21:46:59.9380341Z + error: any use of this value will cause an error
2019-07-31T21:46:59.9380503Z +   --> $DIR/issue-51559.rs:4:33
2019-07-31T21:46:59.9380561Z 3    |
2019-07-31T21:46:59.9380598Z 4 LL | pub const FOO: usize = unsafe { BAR as usize };
2019-07-31T21:46:59.9380848Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected initialized plain (non-pointer) bytes
2019-07-31T21:46:59.9381061Z +    | --------------------------------^^^^^^^^^^^^---
2019-07-31T21:46:59.9381103Z +    |                                 |
2019-07-31T21:46:59.9381326Z +    |                                 "pointer-to-integer cast" needs an rfc before being allowed inside constants
2019-07-31T21:46:59.9381385Z 6    |
2019-07-31T21:46:59.9381689Z -    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-07-31T21:46:59.9381972Z 8 
2019-07-31T21:46:59.9382007Z 9 error: aborting due to previous error
2019-07-31T21:46:59.9382040Z 10 
2019-07-31T21:46:59.9382063Z 
2019-07-31T21:46:59.9382063Z 
2019-07-31T21:46:59.9382323Z - For more information about this error, try `rustc --explain E0080`.
2019-07-31T21:46:59.9382362Z 12 
2019-07-31T21:46:59.9382384Z 
2019-07-31T21:46:59.9382404Z 
2019-07-31T21:46:59.9382552Z The actual stderr differed from the expected stderr.
2019-07-31T21:46:59.9382825Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-51559/issue-51559.stderr
2019-07-31T21:46:59.9383016Z To update references, rerun the tests and pass the `--bless` flag
2019-07-31T21:46:59.9383239Z To only update this specific test, also pass `--test-args consts/issue-51559.rs`
2019-07-31T21:46:59.9383313Z error: 1 errors occurred comparing output.
2019-07-31T21:46:59.9383368Z status: exit code: 1
2019-07-31T21:46:59.9383368Z status: exit code: 1
2019-07-31T21:46:59.9383939Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-51559.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-51559" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-51559/auxiliary" "-A" "unused"
2019-07-31T21:46:59.9384204Z ------------------------------------------
2019-07-31T21:46:59.9384232Z 
2019-07-31T21:46:59.9384399Z ------------------------------------------
2019-07-31T21:46:59.9384455Z stderr:
2019-07-31T21:46:59.9384455Z stderr:
2019-07-31T21:46:59.9384619Z ------------------------------------------
2019-07-31T21:46:59.9384669Z error: any use of this value will cause an error
2019-07-31T21:46:59.9384870Z   --> /checkout/src/test/ui/consts/issue-51559.rs:4:33
2019-07-31T21:46:59.9384909Z    |
2019-07-31T21:46:59.9384946Z LL | pub const FOO: usize = unsafe { BAR as usize };
2019-07-31T21:46:59.9385142Z    | --------------------------------^^^^^^^^^^^^---
2019-07-31T21:46:59.9385183Z    |                                 |
2019-07-31T21:46:59.9385688Z    |                                 "pointer-to-integer cast" needs an rfc before being allowed inside constants
2019-07-31T21:46:59.9385827Z    = note: `#[deny(const_err)]` on by default
2019-07-31T21:46:59.9385857Z 
2019-07-31T21:46:59.9385900Z error: aborting due to previous error
2019-07-31T21:46:59.9385947Z 
---
2019-07-31T21:46:59.9416719Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:533:22
2019-07-31T21:46:59.9416832Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-31T21:46:59.9430642Z 
2019-07-31T21:46:59.9430711Z 
2019-07-31T21:46:59.9436638Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-31T21:46:59.9436994Z 
2019-07-31T21:46:59.9437027Z 
2019-07-31T21:46:59.9448182Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-31T21:46:59.9448279Z Build completed unsuccessfully in 1:00:59
2019-07-31T21:46:59.9448279Z Build completed unsuccessfully in 1:00:59
2019-07-31T21:47:00.6159807Z ##[error]Bash exited with code '1'.
2019-07-31T21:47:00.6192056Z ##[section]Starting: Checkout
2019-07-31T21:47:00.6193547Z ==============================================================================
2019-07-31T21:47:00.6193590Z Task         : Get sources
2019-07-31T21:47:00.6193646Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
