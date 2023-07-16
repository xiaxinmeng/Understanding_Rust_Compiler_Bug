plain
2019-08-09T20:10:53.4065286Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-09T20:10:53.4319030Z ##[command]git config gc.auto 0
2019-08-09T20:10:53.4416192Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-09T20:10:53.4483932Z ##[command]git config --get-all http.proxy
2019-08-09T20:10:53.4643016Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63406/merge:refs/remotes/pull/63406/merge
---
2019-08-09T20:11:28.4000183Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-09T20:11:28.4000476Z 
2019-08-09T20:11:28.4000930Z   git checkout -b <new-branch-name>
2019-08-09T20:11:28.4001182Z 
2019-08-09T20:11:28.4001439Z HEAD is now at 5151d5c64 Merge 5491d7b83e49da8308474b2d58b9e413dbec0d8d into 534b42394d743511db1335d5ed08d507ab7c6e73
2019-08-09T20:11:28.4155599Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-09T20:11:28.4158933Z ==============================================================================
2019-08-09T20:11:28.4158994Z Task         : Bash
2019-08-09T20:11:28.4159043Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-09T21:13:45.7005972Z .................................................................................................... 1300/8860
2019-08-09T21:13:52.6339626Z .................................................................................................... 1400/8860
2019-08-09T21:13:59.3106454Z .................................................................................................... 1500/8860
2019-08-09T21:14:10.6168886Z ....................................................................................i............... 1600/8860
2019-08-09T21:14:18.7411858Z i................................................................................................... 1700/8860
2019-08-09T21:14:26.2219924Z ...........................................................................iiiii.................... 1800/8860
2019-08-09T21:14:48.9149149Z .................................................................................................... 2000/8860
2019-08-09T21:14:51.4945413Z .................................................................................................... 2100/8860
2019-08-09T21:14:54.3490143Z .................................................................................................... 2200/8860
2019-08-09T21:15:02.4552448Z .................................................................................................... 2300/8860
---
2019-08-09T21:18:59.7165942Z .................................................................................................... 5200/8860
2019-08-09T21:19:11.1640154Z .................................................................................................... 5300/8860
2019-08-09T21:19:19.3432083Z .i.................................................................................................. 5400/8860
2019-08-09T21:19:24.7903423Z .................................................................................................... 5500/8860
2019-08-09T21:19:37.4560512Z ...............................................................................................ii... 5600/8860
2019-08-09T21:19:52.4280938Z i..ii...........i................................................................................... 5700/8860
2019-08-09T21:20:09.9921623Z .................................................................................................... 5900/8860
2019-08-09T21:20:14.9951057Z ................................................................................................i..i 6000/8860
2019-08-09T21:20:29.9759170Z i................................................................................................... 6100/8860
2019-08-09T21:20:47.0090672Z .................................................................................................... 6200/8860
---
2019-08-09T21:25:09.3335311Z 
2019-08-09T21:25:09.3336213Z ---- [ui] ui/resolve/resolve-inconsistent-names.rs stdout ----
2019-08-09T21:25:09.3336767Z diff of stderr:
2019-08-09T21:25:09.3337386Z 
2019-08-09T21:25:09.3337899Z 23    |          |       pattern doesn't bind `A`
2019-08-09T21:25:09.3338179Z 24    |          variable not in all patterns
2019-08-09T21:25:09.3338393Z 25    |
2019-08-09T21:25:09.3338899Z - help: if you meant to match on a variant or a const, consider making the path in the pattern qualified: `?::A`
2019-08-09T21:25:09.3339933Z + help: if you meant to match on a variant or a `const` item, consider making the path in the pattern qualified: `?::A`
2019-08-09T21:25:09.3340918Z 28    |
2019-08-09T21:25:09.3340918Z 28    |
2019-08-09T21:25:09.3341627Z 29 LL |         (A, B) | (ref B, c) | (c, A) => ()
2019-08-09T21:25:09.3342018Z 63    |          |
2019-08-09T21:25:09.3342198Z 64    |          variable not in all patterns
2019-08-09T21:25:09.3342396Z 65    |
2019-08-09T21:25:09.3342396Z 65    |
2019-08-09T21:25:09.3342835Z - help: if you meant to match on a variant or a const, consider making the path in the pattern qualified: `?::CONST1`
2019-08-09T21:25:09.3343555Z + help: if you meant to match on a variant or a `const` item, consider making the path in the pattern qualified: `?::CONST1`
2019-08-09T21:25:09.3344406Z 68    |
2019-08-09T21:25:09.3344406Z 68    |
2019-08-09T21:25:09.3344640Z 69 LL |         (CONST1, _) | (_, Const2) => ()
2019-08-09T21:25:09.3345102Z 77    |         |
2019-08-09T21:25:09.3345102Z 77    |         |
2019-08-09T21:25:09.3345563Z 78    |         pattern doesn't bind `Const2`
2019-08-09T21:25:09.3345871Z 79    |
2019-08-09T21:25:09.3346400Z - help: if you meant to match on a variant or a const, consider making the path in the pattern qualified: `?::Const2`
2019-08-09T21:25:09.3346859Z + help: if you meant to match on a variant or a `const` item, consider making the path in the pattern qualified: `?::Const2`
2019-08-09T21:25:09.3347721Z 82    |
2019-08-09T21:25:09.3347721Z 82    |
2019-08-09T21:25:09.3347942Z 83 LL |         (CONST1, _) | (_, Const2) => ()
2019-08-09T21:25:09.3348294Z 
2019-08-09T21:25:09.3348481Z The actual stderr differed from the expected stderr.
2019-08-09T21:25:09.3349153Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-inconsistent-names/resolve-inconsistent-names.stderr
2019-08-09T21:25:09.3349153Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-inconsistent-names/resolve-inconsistent-names.stderr
2019-08-09T21:25:09.3349792Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T21:25:09.3350289Z To only update this specific test, also pass `--test-args resolve/resolve-inconsistent-names.rs`
2019-08-09T21:25:09.3350706Z error: 1 errors occurred comparing output.
2019-08-09T21:25:09.3350915Z status: exit code: 1
2019-08-09T21:25:09.3350915Z status: exit code: 1
2019-08-09T21:25:09.3351908Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/resolve-inconsistent-names.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-inconsistent-names" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-inconsistent-names/auxiliary" "-A" "unused"
2019-08-09T21:25:09.3353228Z ------------------------------------------
2019-08-09T21:25:09.3353729Z 
2019-08-09T21:25:09.3354469Z ------------------------------------------
2019-08-09T21:25:09.3354758Z stderr:
2019-08-09T21:25:09.3354758Z stderr:
2019-08-09T21:25:09.3355226Z ------------------------------------------
2019-08-09T21:25:09.3355544Z error[E0408]: variable `a` is not bound in all patterns
2019-08-09T21:25:09.3356922Z    |
2019-08-09T21:25:09.3356922Z    |
2019-08-09T21:25:09.3357341Z LL |        a | b => {} //~  ERROR variable `a` is not bound in all patterns
2019-08-09T21:25:09.3357923Z    |        -   ^ pattern doesn't bind `a`
2019-08-09T21:25:09.3358875Z    |        variable not in all patterns
2019-08-09T21:25:09.3358984Z 
2019-08-09T21:25:09.3358984Z 
2019-08-09T21:25:09.3359107Z error[E0408]: variable `b` is not bound in all patterns
2019-08-09T21:25:09.3359655Z    |
2019-08-09T21:25:09.3359655Z    |
2019-08-09T21:25:09.3359786Z LL |        a | b => {} //~  ERROR variable `a` is not bound in all patterns
2019-08-09T21:25:09.3360114Z    |        ^   - variable not in all patterns
2019-08-09T21:25:09.3360270Z    |        |
2019-08-09T21:25:09.3360571Z    |        pattern doesn't bind `b`
2019-08-09T21:25:09.3360731Z 
2019-08-09T21:25:09.3360859Z error[E0408]: variable `A` is not bound in all patterns
2019-08-09T21:25:09.3361387Z    |
2019-08-09T21:25:09.3361387Z    |
2019-08-09T21:25:09.3361523Z LL |         (A, B) | (ref B, c) | (c, A) => ()
2019-08-09T21:25:09.3361857Z    |          -       ^^^^^^^^^^       - variable not in all patterns
2019-08-09T21:25:09.3362040Z    |          |       |
2019-08-09T21:25:09.3362343Z    |          |       pattern doesn't bind `A`
2019-08-09T21:25:09.3362668Z    |          variable not in all patterns
2019-08-09T21:25:09.3362807Z    |
2019-08-09T21:25:09.3362933Z help: if you meant to match on a variant or a `const` item, consider making the path in the pattern qualified: `?::A`
2019-08-09T21:25:09.3364107Z    |
2019-08-09T21:25:09.3364107Z    |
2019-08-09T21:25:09.3364254Z LL |         (A, B) | (ref B, c) | (c, A) => ()
2019-08-09T21:25:09.3364543Z 
2019-08-09T21:25:09.3364543Z 
2019-08-09T21:25:09.3364838Z error[E0408]: variable `B` is not bound in all patterns
2019-08-09T21:25:09.3365465Z    |
2019-08-09T21:25:09.3365465Z    |
2019-08-09T21:25:09.3365620Z LL |         (A, B) | (ref B, c) | (c, A) => ()
2019-08-09T21:25:09.3366025Z    |             -         -       ^^^^^^ pattern doesn't bind `B`
2019-08-09T21:25:09.3366450Z    |             |         variable not in all patterns
2019-08-09T21:25:09.3366629Z    |             variable not in all patterns
2019-08-09T21:25:09.3366755Z 
2019-08-09T21:25:09.3367058Z error[E0408]: variable `c` is not bound in all patterns
2019-08-09T21:25:09.3367058Z error[E0408]: variable `c` is not bound in all patterns
2019-08-09T21:25:09.3367464Z   --> /checkout/src/test/ui/resolve/resolve-inconsistent-names.rs:19:9
2019-08-09T21:25:09.3367644Z    |
2019-08-09T21:25:09.3367800Z LL |         (A, B) | (ref B, c) | (c, A) => ()
2019-08-09T21:25:09.3368715Z    |         ^^^^^^           -     - variable not in all patterns
2019-08-09T21:25:09.3425460Z    |         |                variable not in all patterns
2019-08-09T21:25:09.3425460Z    |         |                variable not in all patterns
2019-08-09T21:25:09.3426193Z    |         pattern doesn't bind `c`
2019-08-09T21:25:09.3426421Z 
2019-08-09T21:25:09.3426608Z error[E0409]: variable `B` is bound in inconsistent ways within the same match arm
2019-08-09T21:25:09.3427404Z    |
2019-08-09T21:25:09.3427404Z    |
2019-08-09T21:25:09.3427893Z LL |         (A, B) | (ref B, c) | (c, A) => ()
2019-08-09T21:25:09.3428390Z    |             -         ^ bound in different ways
2019-08-09T21:25:09.3429063Z    |             first binding
2019-08-09T21:25:09.3429186Z 
2019-08-09T21:25:09.3429186Z 
2019-08-09T21:25:09.3429377Z error[E0408]: variable `CONST1` is not bound in all patterns
2019-08-09T21:25:09.3431015Z    |
2019-08-09T21:25:09.3431015Z    |
2019-08-09T21:25:09.3431094Z LL |         (CONST1, _) | (_, Const2) => ()
2019-08-09T21:25:09.3431542Z    |          ------       ^^^^^^^^^^^ pattern doesn't bind `CONST1`
2019-08-09T21:25:09.3431787Z    |          variable not in all patterns
2019-08-09T21:25:09.3431837Z    |
2019-08-09T21:25:09.3431837Z    |
2019-08-09T21:25:09.3431884Z help: if you meant to match on a variant or a `const` item, consider making the path in the pattern qualified: `?::CONST1`
2019-08-09T21:25:09.3432516Z    |
2019-08-09T21:25:09.3432516Z    |
2019-08-09T21:25:09.3432723Z LL |         (CONST1, _) | (_, Const2) => ()
2019-08-09T21:25:09.3432801Z 
2019-08-09T21:25:09.3432801Z 
2019-08-09T21:25:09.3432837Z error[E0408]: variable `Const2` is not bound in all patterns
2019-08-09T21:25:09.3433146Z    |
2019-08-09T21:25:09.3433146Z    |
2019-08-09T21:25:09.3433181Z LL |         (CONST1, _) | (_, Const2) => ()
2019-08-09T21:25:09.3433939Z    |         ^^^^^^^^^^^       ------ variable not in all patterns
2019-08-09T21:25:09.3434016Z    |         |
2019-08-09T21:25:09.3434268Z    |         pattern doesn't bind `Const2`
2019-08-09T21:25:09.3434317Z    |
2019-08-09T21:25:09.3434372Z help: if you meant to match on a variant or a `const` item, consider making the path in the pattern qualified: `?::Const2`
2019-08-09T21:25:09.3434695Z    |
2019-08-09T21:25:09.3434695Z    |
2019-08-09T21:25:09.3434740Z LL |         (CONST1, _) | (_, Const2) => ()
2019-08-09T21:25:09.3434837Z 
2019-08-09T21:25:09.3434880Z error[E0308]: mismatched types
2019-08-09T21:25:09.3435143Z   --> /checkout/src/test/ui/resolve/resolve-inconsistent-names.rs:19:19
2019-08-09T21:25:09.3435194Z    |
2019-08-09T21:25:09.3435194Z    |
2019-08-09T21:25:09.3435240Z LL |         (A, B) | (ref B, c) | (c, A) => ()
2019-08-09T21:25:09.3435290Z    |                   ^^^^^ expected enum `E`, found &E
2019-08-09T21:25:09.3435530Z    = note: expected type `E`
2019-08-09T21:25:09.3435575Z               found type `&E`
2019-08-09T21:25:09.3435621Z 
2019-08-09T21:25:09.3435665Z error: aborting due to 9 previous errors
---
2019-08-09T21:25:09.3437219Z test result: FAILED. 8826 passed; 1 failed; 33 ignored; 0 measured; 0 filtered out
2019-08-09T21:25:09.3437257Z 
2019-08-09T21:25:09.3437288Z 
2019-08-09T21:25:09.3437311Z 
2019-08-09T21:25:09.3439342Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-09T21:25:09.3439552Z 
2019-08-09T21:25:09.3439592Z 
2019-08-09T21:25:09.3439840Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-09T21:25:09.3439889Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-09T21:25:09.3439889Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-09T21:25:09.3439949Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-09T21:25:09.3439987Z Build completed unsuccessfully in 1:07:20
2019-08-09T21:25:10.1836092Z ##[error]Bash exited with code '1'.
2019-08-09T21:25:10.1878047Z ##[section]Starting: Checkout
2019-08-09T21:25:10.1879687Z ==============================================================================
2019-08-09T21:25:10.1879739Z Task         : Get sources
2019-08-09T21:25:10.1879803Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
