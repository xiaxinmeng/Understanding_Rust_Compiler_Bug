plain
2020-01-05T01:11:39.9717466Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-05T01:11:39.9946950Z ##[command]git config gc.auto 0
2020-01-05T01:11:40.0010464Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-05T01:11:40.0066739Z ##[command]git config --get-all http.proxy
2020-01-05T01:11:40.0227826Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67820/merge:refs/remotes/pull/67820/merge
---
2020-01-05T02:08:03.3125833Z .................................................................................................... 1500/9488
2020-01-05T02:08:09.3965771Z .................................................................................................... 1600/9488
2020-01-05T02:08:14.5338683Z .................................................................................................... 1700/9488
2020-01-05T02:08:24.1964652Z .................................................................................................... 1800/9488
2020-01-05T02:08:32.6713553Z i................................................................................................... 1900/9488
2020-01-05T02:08:39.6250766Z ........................................................................................iiiii....... 2000/9488
2020-01-05T02:09:02.2044401Z .................................................................................................... 2200/9488
2020-01-05T02:09:04.6937308Z .................................................................................................... 2300/9488
2020-01-05T02:09:07.2158065Z .................................................................................................... 2400/9488
2020-01-05T02:09:13.3609296Z .................................................................................................... 2500/9488
---
2020-01-05T02:12:19.5691733Z ....................i...............i............................................................... 4900/9488
2020-01-05T02:12:29.8349445Z .................................................................................................... 5000/9488
2020-01-05T02:12:35.9229954Z .................................................................i.................................. 5100/9488
2020-01-05T02:12:44.3449387Z .................................................................................................... 5200/9488
2020-01-05T02:12:52.4343035Z ................................ii.ii...........i................................................... 5300/9488
2020-01-05T02:13:02.4343372Z .................................................................................................... 5500/9488
2020-01-05T02:13:12.7410862Z .................................................................................................... 5600/9488
2020-01-05T02:13:20.1809557Z ................i................................................................................... 5700/9488
2020-01-05T02:13:26.4976124Z ...........................................F........................................................ 5800/9488
2020-01-05T02:13:26.4976124Z ...........................................F........................................................ 5800/9488
2020-01-05T02:13:38.0395845Z .................................................................................................... 5900/9488
2020-01-05T02:13:49.9851275Z .....ii...i..ii...........i......................................................................... 6000/9488
2020-01-05T02:14:07.9245646Z .................................................................................................... 6200/9488
2020-01-05T02:14:15.9052841Z .................................................................................................... 6300/9488
2020-01-05T02:14:15.9052841Z .................................................................................................... 6300/9488
2020-01-05T02:14:32.7203560Z ................................i..ii............................................................... 6400/9488
2020-01-05T02:14:40.0778393Z ............................................FF...................................................... 6500/9488
2020-01-05T02:14:55.9496474Z .......i............................................................................................ 6700/9488
2020-01-05T02:14:58.3146633Z .................................................................................................... 6800/9488
2020-01-05T02:15:00.9615180Z .......i............................................................................................ 6900/9488
2020-01-05T02:15:04.6945267Z .................................................................................................... 7000/9488
---
2020-01-05T02:16:42.4562915Z .................................................................................................... 7500/9488
2020-01-05T02:16:46.7756701Z .................................................................................................... 7600/9488
2020-01-05T02:16:52.2464864Z .................................................................................................... 7700/9488
2020-01-05T02:17:02.0062863Z .................................................................................................... 7800/9488
2020-01-05T02:17:11.5274896Z ........................................................iiii........................................ 7900/9488
2020-01-05T02:17:22.0081963Z ..........................................................................................i......i.. 8000/9488
2020-01-05T02:17:32.4544638Z .................................................................................................... 8200/9488
2020-01-05T02:17:48.6085891Z .................................................................................................... 8300/9488
2020-01-05T02:17:57.0653118Z .................................................................................................... 8400/9488
2020-01-05T02:18:05.4830737Z .................................................................................................... 8500/9488
---
2020-01-05T02:19:58.7123723Z ---- [ui] ui/marker_trait_attr/marker-attribute-with-values.rs stdout ----
2020-01-05T02:19:58.7124032Z 
2020-01-05T02:19:58.7124258Z error: ui test compiled successfully!
2020-01-05T02:19:58.7124459Z status: exit code: 0
2020-01-05T02:19:58.7125542Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/marker_trait_attr/marker-attribute-with-values.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/marker_trait_attr/marker-attribute-with-values" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/marker_trait_attr/marker-attribute-with-values/auxiliary" "-A" "unused"
2020-01-05T02:19:58.7126520Z ------------------------------------------
2020-01-05T02:19:58.7126804Z 
2020-01-05T02:19:58.7127259Z ------------------------------------------
2020-01-05T02:19:58.7127513Z stderr:
---
2020-01-05T02:19:58.7133166Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-05T02:19:58.7133601Z -    |
2020-01-05T02:19:58.7134059Z - help: the following are the possible correct uses
2020-01-05T02:19:58.7134472Z -    |
2020-01-05T02:19:58.7134989Z - LL | #[rustc_on_unimplemented(/*opt*/ message = "...", /*opt*/ label = "...", /*opt*/ note = "...")]
2020-01-05T02:19:58.7135414Z -    |
2020-01-05T02:19:58.7135865Z - LL | #[rustc_on_unimplemented = "message"]
2020-01-05T02:19:58.7139238Z - 
2020-01-05T02:19:58.7139238Z - 
2020-01-05T02:19:58.7139757Z 14 error[E0230]: there is no parameter `C` on trait `BadAnnotation2`
2020-01-05T02:19:58.7140538Z 16    |
2020-01-05T02:19:58.7140733Z 
2020-01-05T02:19:58.7140975Z 77    |
2020-01-05T02:19:58.7140975Z 77    |
2020-01-05T02:19:58.7141193Z 78    = note: eg `#[rustc_on_unimplemented(message="foo")]`
2020-01-05T02:19:58.7142143Z - error: aborting due to 10 previous errors
2020-01-05T02:19:58.7142437Z + error: aborting due to 9 previous errors
2020-01-05T02:19:58.7143262Z 81 
2020-01-05T02:19:58.7143640Z 82 Some errors have detailed explanations: E0230, E0231, E0232.
2020-01-05T02:19:58.7143640Z 82 Some errors have detailed explanations: E0230, E0231, E0232.
2020-01-05T02:19:58.7144402Z 83 For more information about an error, try `rustc --explain E0230`.
2020-01-05T02:19:58.7144777Z 
2020-01-05T02:19:58.7145012Z 
2020-01-05T02:19:58.7145265Z The actual stderr differed from the expected stderr.
2020-01-05T02:19:58.7145911Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/on-unimplemented/bad-annotation/bad-annotation.stderr
2020-01-05T02:19:58.7146587Z To update references, rerun the tests and pass the `--bless` flag
2020-01-05T02:19:58.7147289Z To only update this specific test, also pass `--test-args on-unimplemented/bad-annotation.rs`
2020-01-05T02:19:58.7147925Z error: 1 errors occurred comparing output.
2020-01-05T02:19:58.7148238Z status: exit code: 1
2020-01-05T02:19:58.7148238Z status: exit code: 1
2020-01-05T02:19:58.7149778Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/on-unimplemented/bad-annotation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/on-unimplemented/bad-annotation" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/on-unimplemented/bad-annotation/auxiliary" "-A" "unused"
2020-01-05T02:19:58.7157027Z ------------------------------------------
2020-01-05T02:19:58.7157073Z 
2020-01-05T02:19:58.7157361Z ------------------------------------------
2020-01-05T02:19:58.7157414Z stderr:
2020-01-05T02:19:58.7157414Z stderr:
2020-01-05T02:19:58.7157676Z ------------------------------------------
2020-01-05T02:19:58.7157837Z error[E0230]: there is no parameter `C` on trait `BadAnnotation2`
2020-01-05T02:19:58.7158201Z    |
2020-01-05T02:19:58.7158201Z    |
2020-01-05T02:19:58.7158277Z LL | #[rustc_on_unimplemented = "Unimplemented trait error on `{Self}` with params `<{A},{B},{C}>`"]
2020-01-05T02:19:58.7158383Z 
2020-01-05T02:19:58.7158451Z error[E0231]: only named substitution parameters are allowed
2020-01-05T02:19:58.7158758Z   --> /checkout/src/test/ui/on-unimplemented/bad-annotation.rs:27:1
2020-01-05T02:19:58.7158812Z    |
2020-01-05T02:19:58.7158812Z    |
2020-01-05T02:19:58.7158889Z LL | #[rustc_on_unimplemented = "Unimplemented trait error on `{Self}` with params `<{A},{B},{}>`"]
2020-01-05T02:19:58.7158998Z 
2020-01-05T02:19:58.7159048Z error[E0232]: this attribute must have a valid value
2020-01-05T02:19:58.7159357Z   --> /checkout/src/test/ui/on-unimplemented/bad-annotation.rs:32:26
2020-01-05T02:19:58.7159411Z    |
2020-01-05T02:19:58.7159411Z    |
2020-01-05T02:19:58.7159458Z LL | #[rustc_on_unimplemented(lorem="")]
2020-01-05T02:19:58.7159530Z    |                          ^^^^^^^^ expected value here
2020-01-05T02:19:58.7159877Z    |
2020-01-05T02:19:58.7160009Z    = note: eg `#[rustc_on_unimplemented(message="foo")]`
2020-01-05T02:19:58.7160112Z error[E0232]: this attribute must have a valid value
2020-01-05T02:19:58.7160683Z   --> /checkout/src/test/ui/on-unimplemented/bad-annotation.rs:36:26
2020-01-05T02:19:58.7160739Z    |
2020-01-05T02:19:58.7160739Z    |
2020-01-05T02:19:58.7160810Z LL | #[rustc_on_unimplemented(lorem(ipsum(dolor)))]
2020-01-05T02:19:58.7160866Z    |                          ^^^^^^^^^^^^^^^^^^^ expected value here
2020-01-05T02:19:58.7160914Z    |
2020-01-05T02:19:58.7161085Z    = note: eg `#[rustc_on_unimplemented(message="foo")]`
2020-01-05T02:19:58.7161179Z error[E0232]: this attribute must have a valid value
2020-01-05T02:19:58.7161503Z   --> /checkout/src/test/ui/on-unimplemented/bad-annotation.rs:40:39
2020-01-05T02:19:58.7161579Z    |
2020-01-05T02:19:58.7161579Z    |
2020-01-05T02:19:58.7161628Z LL | #[rustc_on_unimplemented(message="x", message="y")]
2020-01-05T02:19:58.7161685Z    |                                       ^^^^^^^^^^^ expected value here
2020-01-05T02:19:58.7161751Z    |
2020-01-05T02:19:58.7161799Z    = note: eg `#[rustc_on_unimplemented(message="foo")]`
2020-01-05T02:19:58.7161894Z error[E0232]: this attribute must have a valid value
2020-01-05T02:19:58.7162201Z   --> /checkout/src/test/ui/on-unimplemented/bad-annotation.rs:44:39
2020-01-05T02:19:58.7162254Z    |
2020-01-05T02:19:58.7162254Z    |
2020-01-05T02:19:58.7162305Z LL | #[rustc_on_unimplemented(message="x", on(desugared, message="y"))]
2020-01-05T02:19:58.7162392Z    |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected value here
2020-01-05T02:19:58.7162444Z    |
2020-01-05T02:19:58.7162493Z    = note: eg `#[rustc_on_unimplemented(message="foo")]`
2020-01-05T02:19:58.7162544Z 
2020-01-05T02:19:58.7163304Z error[E0232]: empty `on`-clause in `#[rustc_on_unimplemented]`
2020-01-05T02:19:58.7164057Z    |
2020-01-05T02:19:58.7164057Z    |
2020-01-05T02:19:58.7164106Z LL | #[rustc_on_unimplemented(on(), message="y")]
2020-01-05T02:19:58.7164417Z    |                          ^^^^ empty on-clause here
2020-01-05T02:19:58.7164537Z error[E0232]: this attribute must have a valid value
2020-01-05T02:19:58.7164822Z   --> /checkout/src/test/ui/on-unimplemented/bad-annotation.rs:52:26
2020-01-05T02:19:58.7164874Z    |
2020-01-05T02:19:58.7164874Z    |
2020-01-05T02:19:58.7164942Z LL | #[rustc_on_unimplemented(on="x", message="y")]
2020-01-05T02:19:58.7164994Z    |                          ^^^^^^ expected value here
2020-01-05T02:19:58.7165051Z    |
2020-01-05T02:19:58.7165101Z    = note: eg `#[rustc_on_unimplemented(message="foo")]`
2020-01-05T02:19:58.7165199Z error[E0232]: this attribute must have a valid value
2020-01-05T02:19:58.7165483Z   --> /checkout/src/test/ui/on-unimplemented/bad-annotation.rs:59:40
2020-01-05T02:19:58.7165555Z    |
2020-01-05T02:19:58.7165555Z    |
2020-01-05T02:19:58.7165608Z LL | #[rustc_on_unimplemented(on(desugared, on(desugared, message="x")), message="y")]
2020-01-05T02:19:58.7165669Z    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected value here
2020-01-05T02:19:58.7165738Z    |
2020-01-05T02:19:58.7165796Z    = note: eg `#[rustc_on_unimplemented(message="foo")]`
2020-01-05T02:19:58.7165874Z error: aborting due to 9 previous errors
2020-01-05T02:19:58.7165924Z 
2020-01-05T02:19:58.7165974Z Some errors have detailed explanations: E0230, E0231, E0232.
2020-01-05T02:19:58.7167097Z For more information about an error, try `rustc --explain E0230`.
---
2020-01-05T02:19:58.7167775Z ---- [ui] ui/on-unimplemented/expected-comma-found-token.rs stdout ----
2020-01-05T02:19:58.7167831Z 
2020-01-05T02:19:58.7167880Z error: ui test compiled successfully!
2020-01-05T02:19:58.7167929Z status: exit code: 0
2020-01-05T02:19:58.7169105Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/on-unimplemented/expected-comma-found-token.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/on-unimplemented/expected-comma-found-token" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/on-unimplemented/expected-comma-found-token/auxiliary" "-A" "unused"
2020-01-05T02:19:58.7169968Z ------------------------------------------
2020-01-05T02:19:58.7170018Z 
2020-01-05T02:19:58.7170281Z ------------------------------------------
2020-01-05T02:19:58.7170355Z stderr:
---
2020-01-05T02:19:58.7172608Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2020-01-05T02:19:58.7172692Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-05T02:19:58.7172728Z 
2020-01-05T02:19:58.7172756Z 
2020-01-05T02:19:58.7174617Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-05T02:19:58.7174892Z 
2020-01-05T02:19:58.7174924Z 
2020-01-05T02:19:58.7176371Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-05T02:19:58.7183080Z Build completed unsuccessfully in 1:01:42
2020-01-05T02:19:58.7183080Z Build completed unsuccessfully in 1:01:42
2020-01-05T02:19:58.7226006Z == clock drift check ==
2020-01-05T02:19:58.7247220Z   local time: Sun Jan  5 02:19:58 UTC 2020
2020-01-05T02:19:59.2736339Z   network time: Sun, 05 Jan 2020 02:19:59 GMT
2020-01-05T02:19:59.2740406Z == end clock drift check ==
2020-01-05T02:20:00.3240827Z 
2020-01-05T02:20:00.3368924Z ##[error]Bash exited with code '1'.
2020-01-05T02:20:00.3416729Z ##[section]Starting: Checkout
2020-01-05T02:20:00.3418568Z ==============================================================================
2020-01-05T02:20:00.3418619Z Task         : Get sources
2020-01-05T02:20:00.3418852Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
