plain
2019-10-28T20:43:50.2391150Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-28T20:43:50.2595845Z ##[command]git config gc.auto 0
2019-10-28T20:43:50.2656430Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-28T20:43:50.2718670Z ##[command]git config --get-all http.proxy
2019-10-28T20:43:50.2860992Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65785/merge:refs/remotes/pull/65785/merge
---
2019-10-28T21:43:25.5466775Z .................................................................................................... 1600/9258
2019-10-28T21:43:31.2731574Z .................................................................................................... 1700/9258
2019-10-28T21:43:43.5513364Z ..........................................................i...............i......................... 1800/9258
2019-10-28T21:43:51.0611738Z .................................................................................................... 1900/9258
2019-10-28T21:44:05.6869393Z ................................................iiiii............................................... 2000/9258
2019-10-28T21:44:16.3939851Z .................................................................................................... 2200/9258
2019-10-28T21:44:18.9184080Z .................................................................................................... 2300/9258
2019-10-28T21:44:22.6538907Z .................................................................................................... 2400/9258
2019-10-28T21:44:45.7551501Z .................................................................................................... 2500/9258
---
2019-10-28T21:47:35.7540606Z ................................................i...............i................................... 4800/9258
2019-10-28T21:47:44.5915154Z .................................................................................................... 4900/9258
2019-10-28T21:47:53.0454174Z .................................................................................................... 5000/9258
2019-10-28T21:47:59.5118267Z .................................................................................................... 5100/9258
2019-10-28T21:48:09.6048007Z .................................................ii.ii...........i.................................. 5200/9258
2019-10-28T21:48:19.1685392Z .................................................................................................... 5400/9258
2019-10-28T21:48:28.4755648Z .................................................................................................... 5500/9258
2019-10-28T21:48:36.2063484Z ...................i................................................................................ 5600/9258
2019-10-28T21:48:42.1477403Z .................................................................................................... 5700/9258
2019-10-28T21:48:42.1477403Z .................................................................................................... 5700/9258
2019-10-28T21:48:53.6889866Z .................................................................................................... 5800/9258
2019-10-28T21:49:06.0029247Z ....ii...i..ii...........i.......................................................................... 5900/9258
2019-10-28T21:49:27.1934072Z .................................................................................................... 6100/9258
2019-10-28T21:49:34.5311761Z .................................................................................................... 6200/9258
2019-10-28T21:49:34.5311761Z .................................................................................................... 6200/9258
2019-10-28T21:49:48.7410077Z .......................i..ii........................................................................ 6300/9258
2019-10-28T21:50:08.6221228Z .........................................................................................i.......... 6500/9258
2019-10-28T21:50:10.8776014Z .................................................................................................... 6600/9258
2019-10-28T21:50:13.2180759Z ................................................................i................................... 6700/9258
2019-10-28T21:50:16.1638983Z .................................................................................................... 6800/9258
---
2019-10-28T21:54:19.1941921Z 12 
2019-10-28T21:54:19.1941946Z 
2019-10-28T21:54:19.1941969Z 
2019-10-28T21:54:19.1942029Z The actual stderr differed from the expected stderr.
2019-10-28T21:54:19.1942396Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-57979-nested-impl-trait-in-assoc-proj/issue-57979-nested-impl-trait-in-assoc-proj.stderr
2019-10-28T21:54:19.1942674Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T21:54:19.1943025Z To only update this specific test, also pass `--test-args impl-trait/issues/issue-57979-nested-impl-trait-in-assoc-proj.rs`
2019-10-28T21:54:19.1943109Z error: 1 errors occurred comparing output.
2019-10-28T21:54:19.1943171Z status: exit code: 1
2019-10-28T21:54:19.1943171Z status: exit code: 1
2019-10-28T21:54:19.1944012Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/issue-57979-nested-impl-trait-in-assoc-proj.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-57979-nested-impl-trait-in-assoc-proj" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-57979-nested-impl-trait-in-assoc-proj/auxiliary" "-A" "unused"
2019-10-28T21:54:19.1944389Z ------------------------------------------
2019-10-28T21:54:19.1944440Z 
2019-10-28T21:54:19.1944687Z ------------------------------------------
2019-10-28T21:54:19.1944734Z stderr:
2019-10-28T21:54:19.1944734Z stderr:
2019-10-28T21:54:19.1945082Z ------------------------------------------
2019-10-28T21:54:19.1945153Z error[E0666]: nested `impl Trait` is not allowed
2019-10-28T21:54:19.1945426Z   --> /checkout/src/test/ui/impl-trait/issues/issue-57979-nested-impl-trait-in-assoc-proj.rs:9:41
2019-10-28T21:54:19.1945662Z    |
2019-10-28T21:54:19.1945728Z LL | pub fn demo(_: impl Quux<Assoc=impl Foo<impl Bar>>) { }
2019-10-28T21:54:19.1946057Z    |                                |        |
2019-10-28T21:54:19.1946119Z    |                                |        nested `impl Trait` here
2019-10-28T21:54:19.1946280Z    |                                outer `impl Trait`
2019-10-28T21:54:19.1946310Z 
---
2019-10-28T21:54:19.1947669Z - error[E0423]: expected function, found struct `S`
2019-10-28T21:54:19.1947726Z + error[E0423]: expected function, tuple struct or tuple variant, found struct `S`
2019-10-28T21:54:19.1948090Z 2   --> $DIR/legacy-ctor-visibility.rs:9:13
2019-10-28T21:54:19.1948138Z 3    |
2019-10-28T21:54:19.1948356Z - LL |             S(10);
2019-10-28T21:54:19.1949023Z -    |             ^ help: a function with a similar name exists: `f`
2019-10-28T21:54:19.1949087Z + LL | /         fn f() {
2019-10-28T21:54:19.1949130Z + LL | |             S(10);
2019-10-28T21:54:19.1949178Z +    | |             ^ help: a function with a similar name exists: `f`
2019-10-28T21:54:19.1949242Z + LL | |
2019-10-28T21:54:19.1949284Z + LL | |         }
2019-10-28T21:54:19.1950029Z +    | |_________- similarly named function `f` defined here
2019-10-28T21:54:19.1950455Z 7 error: aborting due to previous error
2019-10-28T21:54:19.1950567Z 8 
2019-10-28T21:54:19.1950594Z 
2019-10-28T21:54:19.1950638Z 
2019-10-28T21:54:19.1950638Z 
2019-10-28T21:54:19.1950683Z The actual stderr differed from the expected stderr.
2019-10-28T21:54:19.1951098Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/legacy-ctor-visibility/legacy-ctor-visibility.stderr
2019-10-28T21:54:19.1951387Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T21:54:19.1952147Z To only update this specific test, also pass `--test-args privacy/legacy-ctor-visibility.rs`
2019-10-28T21:54:19.1952386Z error: 1 errors occurred comparing output.
2019-10-28T21:54:19.1952428Z status: exit code: 1
2019-10-28T21:54:19.1952428Z status: exit code: 1
2019-10-28T21:54:19.1953318Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/legacy-ctor-visibility.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/legacy-ctor-visibility" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/legacy-ctor-visibility/auxiliary" "-A" "unused"
2019-10-28T21:54:19.1953679Z ------------------------------------------
2019-10-28T21:54:19.1953714Z 
2019-10-28T21:54:19.1953967Z ------------------------------------------
2019-10-28T21:54:19.1954014Z stderr:
2019-10-28T21:54:19.1954014Z stderr:
2019-10-28T21:54:19.1954362Z ------------------------------------------
2019-10-28T21:54:19.1954432Z error[E0423]: expected function, tuple struct or tuple variant, found struct `S`
2019-10-28T21:54:19.1954687Z   --> /checkout/src/test/ui/privacy/legacy-ctor-visibility.rs:9:13
2019-10-28T21:54:19.1954737Z    |
2019-10-28T21:54:19.1954794Z LL | /         fn f() {
2019-10-28T21:54:19.1954834Z LL | |             S(10);
2019-10-28T21:54:19.1954887Z    | |             ^ help: a function with a similar name exists: `f`
2019-10-28T21:54:19.1955089Z LL | |             //~^ ERROR expected function, found struct `S`
2019-10-28T21:54:19.1955133Z LL | |         }
2019-10-28T21:54:19.1955415Z    | |_________- similarly named function `f` defined here
2019-10-28T21:54:19.1955508Z error: aborting due to previous error
2019-10-28T21:54:19.1955535Z 
2019-10-28T21:54:19.1955785Z For more information about this error, try `rustc --explain E0423`.
2019-10-28T21:54:19.1955818Z 
2019-10-28T21:54:19.1955818Z 
2019-10-28T21:54:19.1956179Z ------------------------------------------
2019-10-28T21:54:19.1956209Z 
2019-10-28T21:54:19.1956231Z 
2019-10-28T21:54:19.1956465Z ---- [ui] ui/suggestions/let-binding-init-expr-as-ty.rs stdout ----
2019-10-28T21:54:19.1956526Z diff of stderr:
2019-10-28T21:54:19.1956551Z 
2019-10-28T21:54:19.1956589Z 6    |            |
2019-10-28T21:54:19.1956633Z 7    |            help: use `=` if you meant to assign
2019-10-28T21:54:19.1957290Z - error: parenthesized type parameters may only be used with a `Fn` trait
2019-10-28T21:54:19.1957290Z - error: parenthesized type parameters may only be used with a `Fn` trait
2019-10-28T21:54:19.1957361Z + error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
2019-10-28T21:54:19.1957682Z 11    |
2019-10-28T21:54:19.1957682Z 11    |
2019-10-28T21:54:19.1957724Z 12 LL |     let foo: i32::from_be(num);
2019-10-28T21:54:19.1957810Z 13    |                   ^^^^^^^^^^^^
2019-10-28T21:54:19.1957854Z +    |                   |
2019-10-28T21:54:19.1957854Z +    |                   |
2019-10-28T21:54:19.1957901Z +    |                   only `Fn` traits may use parentheses
2019-10-28T21:54:19.1957967Z +    |                   help: use angle brackets instead: `from_be<num>`
2019-10-28T21:54:19.1958055Z + error[E0109]: type arguments are not allowed for this type
2019-10-28T21:54:19.1958762Z +   --> $DIR/let-binding-init-expr-as-ty.rs:2:27
2019-10-28T21:54:19.1958822Z 14    |
2019-10-28T21:54:19.1959120Z -    = note: `#[deny(parenthesized_params_in_types_and_modules)]` on by default
2019-10-28T21:54:19.1959120Z -    = note: `#[deny(parenthesized_params_in_types_and_modules)]` on by default
2019-10-28T21:54:19.1959480Z -    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2019-10-28T21:54:19.1959913Z -    = note: for more information, see issue #42238 <***/issues/42238>
2019-10-28T21:54:19.1959986Z + LL |     let foo: i32::from_be(num);
2019-10-28T21:54:19.1960410Z +    |                           ^^^ type argument not allowed
2019-10-28T21:54:19.1960533Z 19 error[E0223]: ambiguous associated type
2019-10-28T21:54:19.1961196Z 20   --> $DIR/let-binding-init-expr-as-ty.rs:2:14
2019-10-28T21:54:19.1961254Z 
2019-10-28T21:54:19.1961254Z 
2019-10-28T21:54:19.1961299Z 22 LL |     let foo: i32::from_be(num);
2019-10-28T21:54:19.1961807Z 23    |              ^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<i32 as Trait>::from_be`
2019-10-28T21:54:19.1962089Z - error: aborting due to 3 previous errors
2019-10-28T21:54:19.1962151Z + error: aborting due to 4 previous errors
2019-10-28T21:54:19.1962197Z 26 
2019-10-28T21:54:19.1962432Z - Some errors have detailed explanations: E0223, E0573.
2019-10-28T21:54:19.1962432Z - Some errors have detailed explanations: E0223, E0573.
2019-10-28T21:54:19.1962676Z - For more information about an error, try `rustc --explain E0223`.
2019-10-28T21:54:19.1962744Z + Some errors have detailed explanations: E0109, E0214, E0223, E0573.
2019-10-28T21:54:19.1962988Z + For more information about an error, try `rustc --explain E0109`.
2019-10-28T21:54:19.1963032Z 29 
2019-10-28T21:54:19.1963072Z 
2019-10-28T21:54:19.1963095Z 
2019-10-28T21:54:19.1963136Z The actual stderr differed from the expected stderr.
2019-10-28T21:54:19.1963454Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/let-binding-init-expr-as-ty/let-binding-init-expr-as-ty.stderr
2019-10-28T21:54:19.1963719Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T21:54:19.1964024Z To only update this specific test, also pass `--test-args suggestions/let-binding-init-expr-as-ty.rs`
2019-10-28T21:54:19.1964258Z error: 1 errors occurred comparing output.
2019-10-28T21:54:19.1964300Z status: exit code: 1
2019-10-28T21:54:19.1964300Z status: exit code: 1
2019-10-28T21:54:19.1965075Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/let-binding-init-expr-as-ty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/let-binding-init-expr-as-ty" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/let-binding-init-expr-as-ty/auxiliary" "-A" "unused"
2019-10-28T21:54:19.1965425Z ------------------------------------------
2019-10-28T21:54:19.1965478Z 
2019-10-28T21:54:19.1967643Z ------------------------------------------
2019-10-28T21:54:19.1967721Z stderr:
2019-10-28T21:54:19.1967721Z stderr:
2019-10-28T21:54:19.1968025Z ------------------------------------------
2019-10-28T21:54:19.1968079Z error[E0573]: expected type, found local variable `num`
2019-10-28T21:54:19.1968797Z   --> /checkout/src/test/ui/suggestions/let-binding-init-expr-as-ty.rs:2:27
2019-10-28T21:54:19.1968884Z    |
2019-10-28T21:54:19.1968928Z LL |     let foo: i32::from_be(num);
2019-10-28T21:54:19.1969271Z    |            |
2019-10-28T21:54:19.1969271Z    |            |
2019-10-28T21:54:19.1969318Z    |            help: use `=` if you meant to assign
2019-10-28T21:54:19.1969350Z 
2019-10-28T21:54:19.1969397Z error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
2019-10-28T21:54:19.1969737Z    |
2019-10-28T21:54:19.1969737Z    |
2019-10-28T21:54:19.1969779Z LL |     let foo: i32::from_be(num);
2019-10-28T21:54:19.1969893Z    |                   |
2019-10-28T21:54:19.1969893Z    |                   |
2019-10-28T21:54:19.1969947Z    |                   only `Fn` traits may use parentheses
2019-10-28T21:54:19.1970014Z    |                   help: use angle brackets instead: `from_be<num>`
2019-10-28T21:54:19.1970094Z error[E0109]: type arguments are not allowed for this type
2019-10-28T21:54:19.1970395Z   --> /checkout/src/test/ui/suggestions/let-binding-init-expr-as-ty.rs:2:27
2019-10-28T21:54:19.1970464Z    |
2019-10-28T21:54:19.1970464Z    |
2019-10-28T21:54:19.1970508Z LL |     let foo: i32::from_be(num);
2019-10-28T21:54:19.1970558Z    |                           ^^^ type argument not allowed
2019-10-28T21:54:19.1970651Z error[E0223]: ambiguous associated type
2019-10-28T21:54:19.1971403Z   --> /checkout/src/test/ui/suggestions/let-binding-init-expr-as-ty.rs:2:14
2019-10-28T21:54:19.1971466Z    |
2019-10-28T21:54:19.1971466Z    |
2019-10-28T21:54:19.1971530Z LL |     let foo: i32::from_be(num);
2019-10-28T21:54:19.1972256Z    |              ^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<i32 as Trait>::from_be`
2019-10-28T21:54:19.1972374Z error: aborting due to 4 previous errors
2019-10-28T21:54:19.1972403Z 
2019-10-28T21:54:19.1972449Z Some errors have detailed explanations: E0109, E0214, E0223, E0573.
2019-10-28T21:54:19.1972846Z For more information about an error, try `rustc --explain E0109`.
---
2019-10-28T21:54:19.1992434Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-28T21:54:19.1992520Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-28T21:54:19.2002825Z 
2019-10-28T21:54:19.2002897Z 
2019-10-28T21:54:19.2011681Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-28T21:54:19.2012033Z 
2019-10-28T21:54:19.2012068Z 
2019-10-28T21:54:19.2057654Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-28T21:54:19.2057908Z Build completed unsuccessfully in 1:04:09
2019-10-28T21:54:19.2057908Z Build completed unsuccessfully in 1:04:09
2019-10-28T21:54:19.2064292Z == clock drift check ==
2019-10-28T21:54:19.2075710Z   local time: Mon Oct 28 21:54:19 UTC 2019
2019-10-28T21:54:19.4855815Z   network time: Mon, 28 Oct 2019 21:54:19 GMT
2019-10-28T21:54:19.4864819Z == end clock drift check ==
2019-10-28T21:54:20.3766840Z 
2019-10-28T21:54:20.3853249Z ##[error]Bash exited with code '1'.
2019-10-28T21:54:20.3916401Z ##[section]Starting: Checkout
2019-10-28T21:54:20.3918166Z ==============================================================================
2019-10-28T21:54:20.3918220Z Task         : Get sources
2019-10-28T21:54:20.3918279Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
