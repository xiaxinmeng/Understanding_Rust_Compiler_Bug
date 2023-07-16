plain
2019-07-29T06:50:07.7542262Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-29T06:50:07.7744975Z ##[command]git config gc.auto 0
2019-07-29T06:50:07.7812282Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-29T06:50:07.7872177Z ##[command]git config --get-all http.proxy
2019-07-29T06:50:07.8014157Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63008/merge:refs/remotes/pull/63008/merge
---
2019-07-29T06:50:41.1333511Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-29T06:50:41.1333542Z 
2019-07-29T06:50:41.1333784Z   git checkout -b <new-branch-name>
2019-07-29T06:50:41.1333814Z 
2019-07-29T06:50:41.1333862Z HEAD is now at fea0d021b Merge 93666a47cd6d45be96cb56ecfa58c9383c528759 into 8b94e9e9188b65df38a5f1ae723617dc2dfb3155
2019-07-29T06:50:41.1462832Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-29T06:50:41.1465644Z ==============================================================================
2019-07-29T06:50:41.1465722Z Task         : Bash
2019-07-29T06:50:41.1465771Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-29T07:53:21.9131427Z .................................................................................................... 1400/8804
2019-07-29T07:53:27.9882229Z .................................................................................................... 1500/8804
2019-07-29T07:53:40.6191990Z ................................................................i...............i................... 1600/8804
2019-07-29T07:53:48.4195652Z .................................................................................................... 1700/8804
2019-07-29T07:54:03.5714613Z ..................................................iiiii............................................. 1800/8804
2019-07-29T07:54:15.0471064Z .................................................................................................... 2000/8804
2019-07-29T07:54:17.6816851Z .................................................................................................... 2100/8804
2019-07-29T07:54:21.7162997Z .................................................................................................... 2200/8804
2019-07-29T07:54:28.3163754Z .................................................................................................... 2300/8804
---
2019-07-29T07:58:18.2155740Z .................................................................................................... 5200/8804
2019-07-29T07:58:29.1367787Z .................................................................................................... 5300/8804
2019-07-29T07:58:36.8959977Z ..i................................................................................................. 5400/8804
2019-07-29T07:58:42.3318897Z .................................................................................................... 5500/8804
2019-07-29T07:58:54.5743298Z ................................................................................................ii.. 5600/8804
2019-07-29T07:59:09.3897312Z .i..ii...........i.................................................................................. 5700/8804
2019-07-29T07:59:25.7620652Z .................................................................................................... 5900/8804
2019-07-29T07:59:30.6084741Z ................................................................................................i..i 6000/8804
2019-07-29T07:59:45.1412275Z i................................................................................................... 6100/8804
2019-07-29T08:00:01.7412339Z .................................................................................................... 6200/8804
---
2019-07-29T08:04:13.6429413Z 
2019-07-29T08:04:13.6429942Z ---- [ui] ui/parser/match-vec-invalid.rs stdout ----
2019-07-29T08:04:13.6430003Z diff of stderr:
2019-07-29T08:04:13.6430033Z 
2019-07-29T08:04:13.6430080Z 10 LL |         [1, tail @ .., tail @ ..] => {},
2019-07-29T08:04:13.6430228Z 12    |
2019-07-29T08:04:13.6430228Z 12    |
2019-07-29T08:04:13.6430616Z -    = note: for more information, see ***/issues/62254
2019-07-29T08:04:13.6430956Z +    = note: for more information, see issue #62254 <***/issues/62254>
2019-07-29T08:04:13.6431018Z 14    = help: add `#![feature(slice_patterns)]` to the crate attributes to enable
2019-07-29T08:04:13.6431127Z 16 error[E0658]: subslice patterns are unstable
2019-07-29T08:04:13.6431157Z 
2019-07-29T08:04:13.6431157Z 
2019-07-29T08:04:13.6431203Z 19 LL |         [1, tail @ .., tail @ ..] => {},
2019-07-29T08:04:13.6431309Z 21    |
2019-07-29T08:04:13.6431309Z 21    |
2019-07-29T08:04:13.6431593Z -    = note: for more information, see ***/issues/62254
2019-07-29T08:04:13.6431953Z +    = note: for more information, see issue #62254 <***/issues/62254>
2019-07-29T08:04:13.6432016Z 23    = help: add `#![feature(slice_patterns)]` to the crate attributes to enable
2019-07-29T08:04:13.6432082Z 24 
2019-07-29T08:04:13.6432313Z 25 error: `..` can only be used once per slice pattern
2019-07-29T08:04:13.6432479Z 
2019-07-29T08:04:13.6432672Z The actual stderr differed from the expected stderr.
2019-07-29T08:04:13.6433500Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/match-vec-invalid/match-vec-invalid.stderr
2019-07-29T08:04:13.6433500Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/match-vec-invalid/match-vec-invalid.stderr
2019-07-29T08:04:13.6434095Z To update references, rerun the tests and pass the `--bless` flag
2019-07-29T08:04:13.6434388Z To only update this specific test, also pass `--test-args parser/match-vec-invalid.rs`
2019-07-29T08:04:13.6434471Z error: 1 errors occurred comparing output.
2019-07-29T08:04:13.6434517Z status: exit code: 1
2019-07-29T08:04:13.6434517Z status: exit code: 1
2019-07-29T08:04:13.6435264Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/match-vec-invalid.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/match-vec-invalid" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/match-vec-invalid/auxiliary" "-A" "unused"
2019-07-29T08:04:13.6435599Z ------------------------------------------
2019-07-29T08:04:13.6435634Z 
2019-07-29T08:04:13.6435845Z ------------------------------------------
2019-07-29T08:04:13.6435906Z stderr:
2019-07-29T08:04:13.6435906Z stderr:
2019-07-29T08:04:13.6436697Z ------------------------------------------
2019-07-29T08:04:13.6436756Z error[E0416]: identifier `tail` is bound more than once in the same pattern
2019-07-29T08:04:13.6437073Z    |
2019-07-29T08:04:13.6437073Z    |
2019-07-29T08:04:13.6437119Z LL |         [1, tail @ .., tail @ ..] => {},
2019-07-29T08:04:13.6437188Z    |                        ^^^^ used in a pattern more than once
2019-07-29T08:04:13.6437283Z error[E0658]: subslice patterns are unstable
2019-07-29T08:04:13.6437522Z   --> /checkout/src/test/ui/parser/match-vec-invalid.rs:4:13
2019-07-29T08:04:13.6437585Z    |
2019-07-29T08:04:13.6437585Z    |
2019-07-29T08:04:13.6437630Z LL |         [1, tail @ .., tail @ ..] => {},
2019-07-29T08:04:13.6437732Z    |
2019-07-29T08:04:13.6437732Z    |
2019-07-29T08:04:13.6438045Z    = note: for more information, see issue #62254 <***/issues/62254>
2019-07-29T08:04:13.6438104Z    = help: add `#![feature(slice_patterns)]` to the crate attributes to enable
2019-07-29T08:04:13.6438196Z error[E0658]: subslice patterns are unstable
2019-07-29T08:04:13.6438442Z   --> /checkout/src/test/ui/parser/match-vec-invalid.rs:4:24
2019-07-29T08:04:13.6438504Z    |
2019-07-29T08:04:13.6438504Z    |
2019-07-29T08:04:13.6438548Z LL |         [1, tail @ .., tail @ ..] => {},
2019-07-29T08:04:13.6438638Z    |
2019-07-29T08:04:13.6438638Z    |
2019-07-29T08:04:13.6438950Z    = note: for more information, see issue #62254 <***/issues/62254>
2019-07-29T08:04:13.6439015Z    = help: add `#![feature(slice_patterns)]` to the crate attributes to enable
2019-07-29T08:04:13.6439047Z 
2019-07-29T08:04:13.6439109Z error: `..` can only be used once per slice pattern
2019-07-29T08:04:13.6439401Z    |
2019-07-29T08:04:13.6439401Z    |
2019-07-29T08:04:13.6439461Z LL |         [1, tail @ .., tail @ ..] => {},
2019-07-29T08:04:13.6439709Z    |                    --         ^^ can only be used once per slice pattern
2019-07-29T08:04:13.6439818Z    |                    previously used here
2019-07-29T08:04:13.6439849Z 
2019-07-29T08:04:13.6439892Z error[E0308]: mismatched types
2019-07-29T08:04:13.6440368Z   --> /checkout/src/test/ui/parser/match-vec-invalid.rs:13:30
2019-07-29T08:04:13.6440368Z   --> /checkout/src/test/ui/parser/match-vec-invalid.rs:13:30
2019-07-29T08:04:13.6440430Z    |
2019-07-29T08:04:13.6440604Z LL | const RECOVERY_WITNESS: () = 0; //~ ERROR mismatched types
2019-07-29T08:04:13.6440665Z    |                              ^ expected (), found integer
2019-07-29T08:04:13.6440829Z    = note: expected type `()`
2019-07-29T08:04:13.6440829Z    = note: expected type `()`
2019-07-29T08:04:13.6440874Z               found type `{integer}`
2019-07-29T08:04:13.6440963Z error: aborting due to 5 previous errors
2019-07-29T08:04:13.6440993Z 
2019-07-29T08:04:13.6441038Z Some errors have detailed explanations: E0308, E0416, E0658.
2019-07-29T08:04:13.6441341Z For more information about an error, try `rustc --explain E0308`.
2019-07-29T08:04:13.6441341Z For more information about an error, try `rustc --explain E0308`.
2019-07-29T08:04:13.6441376Z 
2019-07-29T08:04:13.6441587Z ------------------------------------------
2019-07-29T08:04:13.6441618Z 
2019-07-29T08:04:13.6441644Z 
2019-07-29T08:04:13.6441879Z ---- [ui] ui/parser/pat-lt-bracket-6.rs stdout ----
2019-07-29T08:04:13.6441928Z diff of stderr:
2019-07-29T08:04:13.6441955Z 
2019-07-29T08:04:13.6441997Z 10 LL |     let Test(&desc[..]) = x;
2019-07-29T08:04:13.6442110Z 12    |
2019-07-29T08:04:13.6442110Z 12    |
2019-07-29T08:04:13.6442419Z -    = note: for more information, see ***/issues/62254
2019-07-29T08:04:13.6442722Z +    = note: for more information, see issue #62254 <***/issues/62254>
2019-07-29T08:04:13.6442780Z 14    = help: add `#![feature(slice_patterns)]` to the crate attributes to enable
2019-07-29T08:04:13.6442886Z 16 error[E0308]: mismatched types
2019-07-29T08:04:13.6442915Z 
2019-07-29T08:04:13.6442940Z 
2019-07-29T08:04:13.6443001Z The actual stderr differed from the expected stderr.
2019-07-29T08:04:13.6443001Z The actual stderr differed from the expected stderr.
2019-07-29T08:04:13.6443586Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-lt-bracket-6/pat-lt-bracket-6.stderr
2019-07-29T08:04:13.6443843Z To update references, rerun the tests and pass the `--bless` flag
2019-07-29T08:04:13.6444124Z To only update this specific test, also pass `--test-args parser/pat-lt-bracket-6.rs`
2019-07-29T08:04:13.6444214Z error: 1 errors occurred comparing output.
2019-07-29T08:04:13.6444275Z status: exit code: 1
2019-07-29T08:04:13.6444275Z status: exit code: 1
2019-07-29T08:04:13.6444997Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/pat-lt-bracket-6.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-lt-bracket-6" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-lt-bracket-6/auxiliary" "-A" "unused"
2019-07-29T08:04:13.6445314Z ------------------------------------------
2019-07-29T08:04:13.6445347Z 
2019-07-29T08:04:13.6445558Z ------------------------------------------
2019-07-29T08:04:13.6445620Z stderr:
2019-07-29T08:04:13.6445620Z stderr:
2019-07-29T08:04:13.6445830Z ------------------------------------------
2019-07-29T08:04:13.6445889Z error: expected one of `)`, `,`, or `@`, found `[`
2019-07-29T08:04:13.6446144Z   --> /checkout/src/test/ui/parser/pat-lt-bracket-6.rs:5:19
2019-07-29T08:04:13.6446193Z    |
2019-07-29T08:04:13.6446245Z LL |     let Test(&desc[..]) = x; //~ ERROR: expected one of `)`, `,`, or `@`, found `[`
2019-07-29T08:04:13.6446316Z    |                   ^ expected one of `)`, `,`, or `@` here
2019-07-29T08:04:13.6446519Z error[E0658]: subslice patterns are unstable
2019-07-29T08:04:13.6446762Z   --> /checkout/src/test/ui/parser/pat-lt-bracket-6.rs:5:20
2019-07-29T08:04:13.6446806Z    |
2019-07-29T08:04:13.6446806Z    |
2019-07-29T08:04:13.6446853Z LL |     let Test(&desc[..]) = x; //~ ERROR: expected one of `)`, `,`, or `@`, found `[`
2019-07-29T08:04:13.6447079Z    |
2019-07-29T08:04:13.6447079Z    |
2019-07-29T08:04:13.6447381Z    = note: for more information, see issue #62254 <***/issues/62254>
2019-07-29T08:04:13.6447454Z    = help: add `#![feature(slice_patterns)]` to the crate attributes to enable
2019-07-29T08:04:13.6447649Z error[E0308]: mismatched types
2019-07-29T08:04:13.6448012Z   --> /checkout/src/test/ui/parser/pat-lt-bracket-6.rs:9:30
2019-07-29T08:04:13.6448077Z    |
2019-07-29T08:04:13.6448077Z    |
2019-07-29T08:04:13.6448124Z LL | const RECOVERY_WITNESS: () = 0; //~ ERROR mismatched types
2019-07-29T08:04:13.6448174Z    |                              ^ expected (), found integer
2019-07-29T08:04:13.6448276Z    = note: expected type `()`
2019-07-29T08:04:13.6448276Z    = note: expected type `()`
2019-07-29T08:04:13.6448321Z               found type `{integer}`
2019-07-29T08:04:13.6448410Z error: aborting due to 3 previous errors
2019-07-29T08:04:13.6448438Z 
2019-07-29T08:04:13.6448483Z Some errors have detailed explanations: E0308, E0658.
2019-07-29T08:04:13.6448742Z For more information about an error, try `rustc --explain E0308`.
2019-07-29T08:04:13.6448742Z For more information about an error, try `rustc --explain E0308`.
2019-07-29T08:04:13.6448775Z 
2019-07-29T08:04:13.6448986Z ------------------------------------------
2019-07-29T08:04:13.6449016Z 
2019-07-29T08:04:13.6449041Z 
2019-07-29T08:04:13.6449280Z ---- [ui] ui/parser/pat-tuple-5.rs stdout ----
2019-07-29T08:04:13.6449334Z diff of stderr:
2019-07-29T08:04:13.6449361Z 
2019-07-29T08:04:13.6449401Z 10 LL |         (PAT ..) => {}
2019-07-29T08:04:13.6449504Z 12    |
2019-07-29T08:04:13.6449504Z 12    |
2019-07-29T08:04:13.6449804Z -    = note: for more information, see ***/issues/37854
2019-07-29T08:04:13.6450107Z +    = note: for more information, see issue #37854 <***/issues/37854>
2019-07-29T08:04:13.6450167Z 14    = help: add `#![feature(exclusive_range_pattern)]` to the crate attributes to enable
2019-07-29T08:04:13.6450272Z 16 error[E0308]: mismatched types
2019-07-29T08:04:13.6450301Z 
2019-07-29T08:04:13.6450327Z 
2019-07-29T08:04:13.6450387Z The actual stderr differed from the expected stderr.
2019-07-29T08:04:13.6450387Z The actual stderr differed from the expected stderr.
2019-07-29T08:04:13.6450803Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-tuple-5/pat-tuple-5.stderr
2019-07-29T08:04:13.6451047Z To update references, rerun the tests and pass the `--bless` flag
2019-07-29T08:04:13.6451317Z To only update this specific test, also pass `--test-args parser/pat-tuple-5.rs`
2019-07-29T08:04:13.6451393Z error: 1 errors occurred comparing output.
2019-07-29T08:04:13.6451450Z status: exit code: 1
2019-07-29T08:04:13.6451450Z status: exit code: 1
2019-07-29T08:04:13.6452255Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/pat-tuple-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-tuple-5" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-tuple-5/auxiliary" "-A" "unused"
2019-07-29T08:04:13.6452569Z ------------------------------------------
2019-07-29T08:04:13.6452609Z 
2019-07-29T08:04:13.6452822Z ------------------------------------------
2019-07-29T08:04:13.6452889Z stderr:
2019-07-29T08:04:13.6452889Z stderr:
2019-07-29T08:04:13.6453330Z ------------------------------------------
2019-07-29T08:04:13.6453391Z error: `X..` range patterns are not supported
2019-07-29T08:04:13.6453694Z   --> /checkout/src/test/ui/parser/pat-tuple-5.rs:5:10
2019-07-29T08:04:13.6453742Z    |
2019-07-29T08:04:13.6453784Z LL |         (PAT ..) => {}
2019-07-29T08:04:13.6453852Z    |          ^^^^^^ help: try using the maximum value for the type: `PAT..MAX`
2019-07-29T08:04:13.6453888Z 
2019-07-29T08:04:13.6453936Z error[E0658]: exclusive range pattern syntax is experimental
2019-07-29T08:04:13.6454167Z   --> /checkout/src/test/ui/parser/pat-tuple-5.rs:5:10
2019-07-29T08:04:13.6454230Z    |
2019-07-29T08:04:13.6454273Z LL |         (PAT ..) => {}
2019-07-29T08:04:13.6454373Z    |
2019-07-29T08:04:13.6454373Z    |
2019-07-29T08:04:13.6454673Z    = note: for more information, see issue #37854 <***/issues/37854>
2019-07-29T08:04:13.6454845Z    = help: add `#![feature(exclusive_range_pattern)]` to the crate attributes to enable
2019-07-29T08:04:13.6454996Z error[E0308]: mismatched types
2019-07-29T08:04:13.6455327Z   --> /checkout/src/test/ui/parser/pat-tuple-5.rs:5:10
2019-07-29T08:04:13.6455393Z    |
2019-07-29T08:04:13.6455435Z LL |     match (0, 1) {
2019-07-29T08:04:13.6455435Z LL |     match (0, 1) {
2019-07-29T08:04:13.6455685Z    |           ------ this match expression has type `({integer}, {integer})`
2019-07-29T08:04:13.6455735Z LL |         (PAT ..) => {}
2019-07-29T08:04:13.6455799Z    |          ^^^^^^ expected tuple, found u8
2019-07-29T08:04:13.6455841Z    |
2019-07-29T08:04:13.6455885Z    = note: expected type `({integer}, {integer})`
2019-07-29T08:04:13.6455947Z               found type `u8`
2019-07-29T08:04:13.6456019Z error: aborting due to 3 previous errors
2019-07-29T08:04:13.6456048Z 
2019-07-29T08:04:13.6456108Z Some errors have detailed explanations: E0308, E0658.
2019-07-29T08:04:13.6456358Z For more information about an error, try `rustc --explain E0308`.
---
2019-07-29T08:04:13.6456989Z 
2019-07-29T08:04:13.6457030Z 10 LL |         (.. PAT) => {}
2019-07-29T08:04:13.6457073Z 11    |          ^^^^^^
2019-07-29T08:04:13.6457114Z 12    |
2019-07-29T08:04:13.6457412Z -    = note: for more information, see ***/issues/37854
2019-07-29T08:04:13.6457712Z +    = note: for more information, see issue #37854 <***/issues/37854>
2019-07-29T08:04:13.6457787Z 14    = help: add `#![feature(exclusive_range_pattern)]` to the crate attributes to enable
2019-07-29T08:04:13.6457879Z 16 error[E0308]: mismatched types
2019-07-29T08:04:13.6457924Z 
2019-07-29T08:04:13.6457949Z 
2019-07-29T08:04:13.6458002Z The actual stderr differed from the expected stderr.
2019-07-29T08:04:13.6458002Z The actual stderr differed from the expected stderr.
2019-07-29T08:04:13.6458305Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-tuple-4/pat-tuple-4.stderr
2019-07-29T08:04:13.6458567Z To update references, rerun the tests and pass the `--bless` flag
2019-07-29T08:04:13.6458850Z To only update this specific test, also pass `--test-args parser/pat-tuple-4.rs`
2019-07-29T08:04:13.6458949Z error: 1 errors occurred comparing output.
2019-07-29T08:04:13.6458997Z status: exit code: 1
2019-07-29T08:04:13.6458997Z status: exit code: 1
2019-07-29T08:04:13.6459720Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/pat-tuple-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-tuple-4" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-tuple-4/auxiliary" "-A" "unused"
2019-07-29T08:04:13.6460069Z ------------------------------------------
2019-07-29T08:04:13.6460119Z 
2019-07-29T08:04:13.6462569Z ------------------------------------------
2019-07-29T08:04:13.6463507Z stderr:
2019-07-29T08:04:13.6463507Z stderr:
2019-07-29T08:04:13.6463843Z ------------------------------------------
2019-07-29T08:04:13.6463920Z error: `..X` range patterns are not supported
2019-07-29T08:04:13.6464159Z   --> /checkout/src/test/ui/parser/pat-tuple-4.rs:5:10
2019-07-29T08:04:13.6464272Z LL |         (.. PAT) => {}
2019-07-29T08:04:13.6464272Z LL |         (.. PAT) => {}
2019-07-29T08:04:13.6464326Z    |          ^^^^^^ help: try using the minimum value for the type: `MIN..PAT`
2019-07-29T08:04:13.6464365Z 
2019-07-29T08:04:13.6464422Z error[E0658]: exclusive range pattern syntax is experimental
2019-07-29T08:04:13.6464703Z   --> /checkout/src/test/ui/parser/pat-tuple-4.rs:5:10
2019-07-29T08:04:13.6464928Z LL |         (.. PAT) => {}
2019-07-29T08:04:13.6465060Z    |          ^^^^^^
2019-07-29T08:04:13.6465106Z    |
2019-07-29T08:04:13.6465106Z    |
2019-07-29T08:04:13.6465498Z    = note: for more information, see issue #37854 <***/issues/37854>
2019-07-29T08:04:13.6465577Z    = help: add `#![feature(exclusive_range_pattern)]` to the crate attributes to enable
2019-07-29T08:04:13.6465657Z error[E0308]: mismatched types
2019-07-29T08:04:13.6465919Z   --> /checkout/src/test/ui/parser/pat-tuple-4.rs:11:30
2019-07-29T08:04:13.6465986Z    |
2019-07-29T08:04:13.6465986Z    |
2019-07-29T08:04:13.6466034Z LL | const RECOVERY_WITNESS: () = 0; //~ ERROR mismatched types
2019-07-29T08:04:13.6466087Z    |                              ^ expected (), found integer
2019-07-29T08:04:13.6466194Z    = note: expected type `()`
2019-07-29T08:04:13.6466194Z    = note: expected type `()`
2019-07-29T08:04:13.6466284Z               found type `{integer}`
2019-07-29T08:04:13.6466377Z error: aborting due to 3 previous errors
2019-07-29T08:04:13.6466418Z 
2019-07-29T08:04:13.6466473Z Some errors have detailed explanations: E0308, E0658.
2019-07-29T08:04:13.6466761Z For more information about an error, try `rustc --explain E0308`.
---
2019-07-29T08:04:13.6470144Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:533:22
2019-07-29T08:04:13.6470243Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-29T08:04:13.6484133Z 
2019-07-29T08:04:13.6484206Z 
2019-07-29T08:04:13.6485869Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-29T08:04:13.6486137Z 
2019-07-29T08:04:13.6486165Z 
2019-07-29T08:04:13.6489390Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-29T08:04:13.6489676Z Build completed unsuccessfully in 1:06:48
2019-07-29T08:04:13.6489676Z Build completed unsuccessfully in 1:06:48
2019-07-29T08:04:14.3284154Z ##[error]Bash exited with code '1'.
2019-07-29T08:04:14.3321647Z ##[section]Starting: Checkout
2019-07-29T08:04:14.3324156Z ==============================================================================
2019-07-29T08:04:14.3324216Z Task         : Get sources
2019-07-29T08:04:14.3324441Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
