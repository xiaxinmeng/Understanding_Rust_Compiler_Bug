plain
2019-08-24T17:34:28.1553650Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-24T17:34:28.1752260Z ##[command]git config gc.auto 0
2019-08-24T17:34:28.6986811Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-24T17:34:28.6993040Z ##[command]git config --get-all http.proxy
2019-08-24T17:34:28.7000168Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63855/merge:refs/remotes/pull/63855/merge
---
2019-08-24T17:35:02.6410439Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-24T17:35:02.6410627Z 
2019-08-24T17:35:02.6410896Z   git checkout -b <new-branch-name>
2019-08-24T17:35:02.6410953Z 
2019-08-24T17:35:02.6411043Z HEAD is now at fd7bf489d Merge 6ffc834dee1423d02613794406241280af10719c into 478464570e60523adc6d303577d1782229ca1f93
2019-08-24T17:35:02.6566663Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-24T17:35:02.6569143Z ==============================================================================
2019-08-24T17:35:02.6569212Z Task         : Bash
2019-08-24T17:35:02.6569250Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-24T18:35:34.4803134Z .................................................................................................... 1500/8950
2019-08-24T18:35:40.0546749Z .................................................................................................... 1600/8950
2019-08-24T18:35:52.9644933Z ...........................................i...............i........................................ 1700/8950
2019-08-24T18:36:01.1742862Z .................................................................................................... 1800/8950
2019-08-24T18:36:15.7678238Z ...................................iiiii............................................................ 1900/8950
2019-08-24T18:36:23.9855422Z ...........................................F......................................................F. 2000/8950
2019-08-24T18:36:29.1811040Z .................................................................................................... 2200/8950
2019-08-24T18:36:33.7155150Z ..................................................F................................................. 2300/8950
2019-08-24T18:36:40.9648050Z .................................................................................................... 2400/8950
2019-08-24T18:36:43.6342885Z .................................................................................................... 2500/8950
---
2019-08-24T18:39:37.6322950Z .......................i...............i............................................................ 4700/8950
2019-08-24T18:39:49.5029411Z .................................................................................................... 4800/8950
2019-08-24T18:39:55.3689633Z .................................................................................................... 4900/8950
2019-08-24T18:40:06.0353027Z .................................................................................................... 5000/8950
2019-08-24T18:40:11.0959170Z ....ii.ii........................................................................................... 5100/8950
2019-08-24T18:40:25.1343429Z .................................................................................................... 5300/8950
2019-08-24T18:40:31.5062375Z ............................................................i....................................... 5400/8950
2019-08-24T18:40:38.0646081Z .................................................................................................... 5500/8950
2019-08-24T18:40:45.5074131Z .................................................................................................... 5600/8950
2019-08-24T18:40:45.5074131Z .................................................................................................... 5600/8950
2019-08-24T18:40:55.5886605Z ......................................................ii...i..ii...........i........................ 5700/8950
2019-08-24T18:41:15.4780695Z .................................................................................................... 5900/8950
2019-08-24T18:41:20.0314679Z .................................................................................................... 6000/8950
2019-08-24T18:41:20.0314679Z .................................................................................................... 6000/8950
2019-08-24T18:41:27.1075973Z .......................................................i..ii........................................ 6100/8950
2019-08-24T18:41:52.6850098Z .................................................................................................... 6300/8950
2019-08-24T18:41:54.7033211Z .i.................................................................................................. 6400/8950
2019-08-24T18:41:56.7638144Z .........................................................................i.......................... 6500/8950
2019-08-24T18:41:59.5435900Z .................................................................................................... 6600/8950
2019-08-24T18:41:59.5435900Z .................................................................................................... 6600/8950
2019-08-24T18:42:05.2390535Z .................................................................................................... 6700/8950
2019-08-24T18:42:29.3663888Z ................F................................................................................... 6800/8950
2019-08-24T18:42:40.8896800Z .................................................................................................... 6900/8950
2019-08-24T18:42:45.8373187Z .................................................................................................... 7000/8950
2019-08-24T18:42:50.9056811Z .................................................................................................... 7100/8950
2019-08-24T18:42:55.6916568Z .................................................................................................... 7200/8950
2019-08-24T18:43:01.6461362Z .................................................................................................... 7300/8950
2019-08-24T18:43:10.6980720Z ..................................................F..FF..F.F.FF.FFF......F.F........................ 7400/8950
2019-08-24T18:43:26.9892719Z .....................ii......i...................................................................... 7600/8950
2019-08-24T18:43:32.1611270Z .................................................................................................... 7700/8950
2019-08-24T18:43:44.9136818Z .................................................................................................... 7800/8950
2019-08-24T18:43:55.0628465Z .................................................................................................... 7900/8950
---
2019-08-24T18:45:42.3378509Z failures:
2019-08-24T18:45:42.3436457Z 
2019-08-24T18:45:42.3440682Z ---- [ui] ui/editions/edition-feature-ok.rs stdout ----
2019-08-24T18:45:42.3440767Z normalized stderr:
2019-08-24T18:45:42.3440844Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-24T18:45:42.3441992Z    |
2019-08-24T18:45:42.3441992Z    |
2019-08-24T18:45:42.3442057Z LL | #![feature(rust_2018_preview)]
2019-08-24T18:45:42.3442135Z 
2019-08-24T18:45:42.3442162Z 
2019-08-24T18:45:42.3442211Z 
2019-08-24T18:45:42.3442253Z 
2019-08-24T18:45:42.3442253Z 
2019-08-24T18:45:42.3442299Z The actual stderr differed from the expected stderr.
2019-08-24T18:45:42.3442624Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-feature-ok/edition-feature-ok.stderr
2019-08-24T18:45:42.3442897Z To update references, rerun the tests and pass the `--bless` flag
2019-08-24T18:45:42.3443381Z To only update this specific test, also pass `--test-args editions/edition-feature-ok.rs`
2019-08-24T18:45:42.3443480Z error: 1 errors occurred comparing output.
2019-08-24T18:45:42.3443549Z status: exit code: 0
2019-08-24T18:45:42.3443549Z status: exit code: 0
2019-08-24T18:45:42.3444326Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/editions/edition-feature-ok.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-feature-ok" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-feature-ok/auxiliary" "-A" "unused"
2019-08-24T18:45:42.3444816Z ------------------------------------------
2019-08-24T18:45:42.3444846Z 
2019-08-24T18:45:42.3445046Z ------------------------------------------
2019-08-24T18:45:42.3445084Z stderr:
2019-08-24T18:45:42.3445084Z stderr:
2019-08-24T18:45:42.3445251Z ------------------------------------------
2019-08-24T18:45:42.3445313Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-24T18:45:42.3445551Z    |
2019-08-24T18:45:42.3445551Z    |
2019-08-24T18:45:42.3445603Z LL | #![feature(rust_2018_preview)]
2019-08-24T18:45:42.3445664Z 
2019-08-24T18:45:42.3445684Z 
2019-08-24T18:45:42.3445871Z ------------------------------------------
2019-08-24T18:45:42.3446024Z 
2019-08-24T18:45:42.3446024Z 
2019-08-24T18:45:42.3446044Z 
2019-08-24T18:45:42.3446244Z ---- [ui] ui/epoch-gate-feature.rs stdout ----
2019-08-24T18:45:42.3446301Z normalized stderr:
2019-08-24T18:45:42.3446342Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-24T18:45:42.3446524Z   --> $DIR/epoch-gate-feature.rs:8:12
2019-08-24T18:45:42.3446580Z    |
2019-08-24T18:45:42.3446616Z LL | #![feature(rust_2018_preview)]
2019-08-24T18:45:42.3446676Z 
2019-08-24T18:45:42.3446696Z 
2019-08-24T18:45:42.3446733Z 
2019-08-24T18:45:42.3446754Z 
2019-08-24T18:45:42.3446754Z 
2019-08-24T18:45:42.3446789Z The actual stderr differed from the expected stderr.
2019-08-24T18:45:42.3447030Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/epoch-gate-feature/epoch-gate-feature.stderr
2019-08-24T18:45:42.3447246Z To update references, rerun the tests and pass the `--bless` flag
2019-08-24T18:45:42.3447460Z To only update this specific test, also pass `--test-args epoch-gate-feature.rs`
2019-08-24T18:45:42.3447721Z error: 1 errors occurred comparing output.
2019-08-24T18:45:42.3447758Z status: exit code: 0
2019-08-24T18:45:42.3447758Z status: exit code: 0
2019-08-24T18:45:42.3448917Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/epoch-gate-feature.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/epoch-gate-feature/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/epoch-gate-feature/auxiliary" "-A" "unused"
2019-08-24T18:45:42.3449237Z ------------------------------------------
2019-08-24T18:45:42.3449283Z 
2019-08-24T18:45:42.3449463Z ------------------------------------------
2019-08-24T18:45:42.3449509Z stderr:
2019-08-24T18:45:42.3449509Z stderr:
2019-08-24T18:45:42.3449698Z ------------------------------------------
2019-08-24T18:45:42.3449744Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-24T18:45:42.3449995Z    |
2019-08-24T18:45:42.3449995Z    |
2019-08-24T18:45:42.3450127Z LL | #![feature(rust_2018_preview)]
2019-08-24T18:45:42.3450197Z 
2019-08-24T18:45:42.3450234Z 
2019-08-24T18:45:42.3450435Z ------------------------------------------
2019-08-24T18:45:42.3450462Z 
2019-08-24T18:45:42.3450462Z 
2019-08-24T18:45:42.3450483Z 
2019-08-24T18:45:42.3450660Z ---- [ui] ui/error-codes/E0705.rs stdout ----
2019-08-24T18:45:42.3450719Z diff of stderr:
2019-08-24T18:45:42.3450742Z 
2019-08-24T18:45:42.3451366Z - warning[E0705]: the feature `test_2018_feature` is included in the Rust 2018 edition
2019-08-24T18:45:42.3451611Z -   --> $DIR/E0705.rs:6:12
2019-08-24T18:45:42.3451679Z + warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-24T18:45:42.3451886Z +   --> $DIR/E0705.rs:8:12
2019-08-24T18:45:42.3452163Z - LL | #![feature(test_2018_feature)]
2019-08-24T18:45:42.3452163Z - LL | #![feature(test_2018_feature)]
2019-08-24T18:45:42.3452212Z + LL | #![feature(rust_2018_preview)]
2019-08-24T18:45:42.3452327Z 6 
2019-08-24T18:45:42.3452369Z 7 
2019-08-24T18:45:42.3452396Z 
2019-08-24T18:45:42.3452422Z 
2019-08-24T18:45:42.3452422Z 
2019-08-24T18:45:42.3452485Z The actual stderr differed from the expected stderr.
2019-08-24T18:45:42.3452780Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0705/E0705.stderr
2019-08-24T18:45:42.3453021Z To update references, rerun the tests and pass the `--bless` flag
2019-08-24T18:45:42.3453292Z To only update this specific test, also pass `--test-args error-codes/E0705.rs`
2019-08-24T18:45:42.3453372Z error: 1 errors occurred comparing output.
2019-08-24T18:45:42.3453530Z status: exit code: 0
2019-08-24T18:45:42.3453530Z status: exit code: 0
2019-08-24T18:45:42.3454295Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0705.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0705" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0705/auxiliary" "-A" "unused"
2019-08-24T18:45:42.3454910Z ------------------------------------------
2019-08-24T18:45:42.3454938Z 
2019-08-24T18:45:42.3455286Z ------------------------------------------
2019-08-24T18:45:42.3455338Z stderr:
2019-08-24T18:45:42.3455338Z stderr:
2019-08-24T18:45:42.3455505Z ------------------------------------------
2019-08-24T18:45:42.3455547Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-24T18:45:42.3455803Z    |
2019-08-24T18:45:42.3455803Z    |
2019-08-24T18:45:42.3455838Z LL | #![feature(rust_2018_preview)]
2019-08-24T18:45:42.3455913Z 
2019-08-24T18:45:42.3455933Z 
2019-08-24T18:45:42.3456110Z ------------------------------------------
2019-08-24T18:45:42.3456137Z 
2019-08-24T18:45:42.3456137Z 
2019-08-24T18:45:42.3456177Z 
2019-08-24T18:45:42.3456366Z ---- [ui] ui/proc-macro/custom-attr-only-one-derive.rs stdout ----
2019-08-24T18:45:42.3456405Z normalized stderr:
2019-08-24T18:45:42.3456462Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-24T18:45:42.3456676Z    |
2019-08-24T18:45:42.3456676Z    |
2019-08-24T18:45:42.3456711Z LL | #![feature(rust_2018_preview)]
2019-08-24T18:45:42.3456786Z 
2019-08-24T18:45:42.3456814Z 
2019-08-24T18:45:42.3456834Z 
2019-08-24T18:45:42.3456869Z 
2019-08-24T18:45:42.3456869Z 
2019-08-24T18:45:42.3456905Z The actual stderr differed from the expected stderr.
2019-08-24T18:45:42.3457166Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/custom-attr-only-one-derive/custom-attr-only-one-derive.stderr
2019-08-24T18:45:42.3457452Z To update references, rerun the tests and pass the `--bless` flag
2019-08-24T18:45:42.3457706Z To only update this specific test, also pass `--test-args proc-macro/custom-attr-only-one-derive.rs`
2019-08-24T18:45:42.3457772Z error: 1 errors occurred comparing output.
2019-08-24T18:45:42.3457824Z status: exit code: 0
2019-08-24T18:45:42.3457824Z status: exit code: 0
2019-08-24T18:45:42.3458436Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/custom-attr-only-one-derive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/custom-attr-only-one-derive/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/custom-attr-only-one-derive/auxiliary" "-A" "unused"
2019-08-24T18:45:42.3458896Z ------------------------------------------
2019-08-24T18:45:42.3458926Z 
2019-08-24T18:45:42.3459123Z ------------------------------------------
2019-08-24T18:45:42.3459160Z stderr:
2019-08-24T18:45:42.3459160Z stderr:
2019-08-24T18:45:42.3459332Z ------------------------------------------
2019-08-24T18:45:42.3459395Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-24T18:45:42.3459645Z    |
2019-08-24T18:45:42.3459645Z    |
2019-08-24T18:45:42.3459698Z LL | #![feature(rust_2018_preview)]
2019-08-24T18:45:42.3459832Z 
2019-08-24T18:45:42.3459853Z 
2019-08-24T18:45:42.3460064Z ------------------------------------------
2019-08-24T18:45:42.3460092Z 
2019-08-24T18:45:42.3460092Z 
2019-08-24T18:45:42.3460112Z 
2019-08-24T18:45:42.3460311Z ---- [ui] ui/rust-2018/edition-lint-fully-qualified-paths.rs stdout ----
2019-08-24T18:45:42.3460367Z diff of stderr:
2019-08-24T18:45:42.3460391Z 
2019-08-24T18:45:42.3460719Z + warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-24T18:45:42.3461985Z +   --> $DIR/edition-lint-fully-qualified-paths.rs:3:12
2019-08-24T18:45:42.3462071Z +    |
2019-08-24T18:45:42.3462119Z + LL | #![feature(rust_2018_preview, crate_visibility_modifier)]
2019-08-24T18:45:42.3462502Z + 
2019-08-24T18:45:42.3462502Z + 
2019-08-24T18:45:42.3462557Z 1 error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-24T18:45:42.3462978Z 3    |
2019-08-24T18:45:42.3463007Z 
2019-08-24T18:45:42.3463045Z 23 
2019-08-24T18:45:42.3463092Z 24 error: aborting due to 2 previous errors
2019-08-24T18:45:42.3463092Z 24 error: aborting due to 2 previous errors
2019-08-24T18:45:42.3463152Z 25 
2019-08-24T18:45:42.3463400Z + For more information about this error, try `rustc --explain E0705`.
2019-08-24T18:45:42.3463447Z 26 
2019-08-24T18:45:42.3463494Z 
2019-08-24T18:45:42.3463520Z 
2019-08-24T18:45:42.3463575Z The actual stderr differed from the expected stderr.
2019-08-24T18:45:42.3463915Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-fully-qualified-paths/edition-lint-fully-qualified-paths.stderr
2019-08-24T18:45:42.3464186Z To update references, rerun the tests and pass the `--bless` flag
2019-08-24T18:45:42.3464944Z To only update this specific test, also pass `--test-args rust-2018/edition-lint-fully-qualified-paths.rs`
2019-08-24T18:45:42.3465034Z error: 1 errors occurred comparing output.
2019-08-24T18:45:42.3465081Z status: exit code: 1
2019-08-24T18:45:42.3465081Z status: exit code: 1
2019-08-24T18:45:42.3466039Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/edition-lint-fully-qualified-paths.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-fully-qualified-paths" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-fully-qualified-paths/auxiliary" "-A" "unused"
2019-08-24T18:45:42.3466702Z ------------------------------------------
2019-08-24T18:45:42.3467042Z 
2019-08-24T18:45:42.3467312Z ------------------------------------------
2019-08-24T18:45:42.3467353Z stderr:
2019-08-24T18:45:42.3467353Z stderr:
2019-08-24T18:45:42.3468155Z ------------------------------------------
2019-08-24T18:45:42.3468242Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-24T18:45:42.3468605Z    |
2019-08-24T18:45:42.3468605Z    |
2019-08-24T18:45:42.3468645Z LL | #![feature(rust_2018_preview, crate_visibility_modifier)]
2019-08-24T18:45:42.3468718Z 
2019-08-24T18:45:42.3468718Z 
2019-08-24T18:45:42.3468778Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-24T18:45:42.3469046Z    |
2019-08-24T18:45:42.3469046Z    |
2019-08-24T18:45:42.3469099Z LL |     let _: <foo::Baz as ::foo::Foo>::Bar = ();
2019-08-24T18:45:42.3469141Z    |                         ^^^^^^^^^^ help: use `crate`: `crate::foo::Foo`
2019-08-24T18:45:42.3469228Z note: lint level defined here
2019-08-24T18:45:42.3469438Z   --> /checkout/src/test/ui/rust-2018/edition-lint-fully-qualified-paths.rs:4:9
2019-08-24T18:45:42.3469589Z    |
2019-08-24T18:45:42.3469589Z    |
2019-08-24T18:45:42.3469643Z LL | #![deny(absolute_paths_not_starting_with_crate)]
2019-08-24T18:45:42.3469683Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-24T18:45:42.3469736Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-24T18:45:42.3470110Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-24T18:45:42.3470142Z 
2019-08-24T18:45:42.3470186Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-24T18:45:42.3470476Z    |
2019-08-24T18:45:42.3470476Z    |
2019-08-24T18:45:42.3470513Z LL |     let _: <::foo::Baz as foo::Foo>::Bar = ();
2019-08-24T18:45:42.3470570Z    |             ^^^^^^^^^^ help: use `crate`: `crate::foo::Baz`
2019-08-24T18:45:42.3470615Z    |
2019-08-24T18:45:42.3470663Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-24T18:45:42.3470940Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-24T18:45:42.3471006Z error: aborting due to 2 previous errors
2019-08-24T18:45:42.3471037Z 
2019-08-24T18:45:42.3471732Z For more information about this error, try `rustc --explain E0705`.
2019-08-24T18:45:42.3471772Z 
2019-08-24T18:45:42.3471772Z 
2019-08-24T18:45:42.3471992Z ------------------------------------------
2019-08-24T18:45:42.3472025Z 
2019-08-24T18:45:42.3472072Z 
2019-08-24T18:45:42.3472312Z ---- [ui] ui/rust-2018/edition-lint-nested-empty-paths.rs stdout ----
2019-08-24T18:45:42.3472361Z diff of stderr:
2019-08-24T18:45:42.3472388Z 
2019-08-24T18:45:42.3472456Z + warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-24T18:45:42.3472686Z +   --> $DIR/edition-lint-nested-empty-paths.rs:3:12
2019-08-24T18:45:42.3472743Z +    |
2019-08-24T18:45:42.3472808Z + LL | #![feature(rust_2018_preview, crate_visibility_modifier)]
2019-08-24T18:45:42.3472899Z + 
2019-08-24T18:45:42.3472899Z + 
2019-08-24T18:45:42.3472968Z 1 error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-24T18:45:42.3473368Z 3    |
2019-08-24T18:45:42.3479418Z 
2019-08-24T18:45:42.3479524Z 32 
2019-08-24T18:45:42.3479565Z 33 error: aborting due to 3 previous errors
2019-08-24T18:45:42.3479565Z 33 error: aborting due to 3 previous errors
2019-08-24T18:45:42.3479601Z 34 
2019-08-24T18:45:42.3480027Z + For more information about this error, try `rustc --explain E0705`.
2019-08-24T18:45:42.3480072Z 35 
2019-08-24T18:45:42.3480095Z 
2019-08-24T18:45:42.3480116Z 
2019-08-24T18:45:42.3480169Z The actual stderr differed from the expected stderr.
2019-08-24T18:45:42.3480450Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-nested-empty-paths/edition-lint-nested-empty-paths.stderr
2019-08-24T18:45:42.3480673Z To update references, rerun the tests and pass the `--bless` flag
2019-08-24T18:45:42.3480920Z To only update this specific test, also pass `--test-args rust-2018/edition-lint-nested-empty-paths.rs`
2019-08-24T18:45:42.3480996Z error: 1 errors occurred comparing output.
2019-08-24T18:45:42.3481050Z status: exit code: 1
2019-08-24T18:45:42.3481050Z status: exit code: 1
2019-08-24T18:45:42.3482515Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/edition-lint-nested-empty-paths.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-nested-empty-paths" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-nested-empty-paths/auxiliary" "-A" "unused"
2019-08-24T18:45:42.3483081Z ------------------------------------------
2019-08-24T18:45:42.3483117Z 
2019-08-24T18:45:42.3483355Z ------------------------------------------
2019-08-24T18:45:42.3483403Z stderr:
2019-08-24T18:45:42.3483403Z stderr:
2019-08-24T18:45:42.3483626Z ------------------------------------------
2019-08-24T18:45:42.3483683Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-24T18:45:42.3484021Z    |
2019-08-24T18:45:42.3484021Z    |
2019-08-24T18:45:42.3484070Z LL | #![feature(rust_2018_preview, crate_visibility_modifier)]
2019-08-24T18:45:42.3484167Z 
2019-08-24T18:45:42.3484167Z 
2019-08-24T18:45:42.3484266Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-24T18:45:42.3484623Z    |
2019-08-24T18:45:42.3484623Z    |
2019-08-24T18:45:42.3484667Z LL | use foo::{bar::{baz::{}}};
2019-08-24T18:45:42.3484736Z    |     ^^^^^^^^^^^^^^^^^^^^^ help: use `crate`: `crate::foo::{bar::{baz::{}}}`
2019-08-24T18:45:42.3484991Z note: lint level defined here
2019-08-24T18:45:42.3485384Z   --> /checkout/src/test/ui/rust-2018/edition-lint-nested-empty-paths.rs:4:9
2019-08-24T18:45:42.3485424Z    |
2019-08-24T18:45:42.3485424Z    |
2019-08-24T18:45:42.3485460Z LL | #![deny(absolute_paths_not_starting_with_crate)]
2019-08-24T18:45:42.3485516Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-24T18:45:42.3485565Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-24T18:45:42.3485853Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-24T18:45:42.3485898Z 
2019-08-24T18:45:42.3485950Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-24T18:45:42.3486234Z    |
2019-08-24T18:45:42.3486234Z    |
2019-08-24T18:45:42.3486270Z LL | use foo::{bar::{XX, baz::{}}};
2019-08-24T18:45:42.3486404Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `crate`: `crate::foo::{bar::{XX, baz::{}}}`
2019-08-24T18:45:42.3486467Z    |
2019-08-24T18:45:42.3486512Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-24T18:45:42.3486792Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-24T18:45:42.3486840Z 
2019-08-24T18:45:42.3486883Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-24T18:45:42.3487163Z    |
2019-08-24T18:45:42.3487163Z    |
2019-08-24T18:45:42.3487199Z LL | use foo::{bar::{baz::{}, baz1::{}}};
2019-08-24T18:45:42.3487242Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `crate`: `crate::foo::{bar::{baz::{}, baz1::{}}}`
2019-08-24T18:45:42.3487280Z    |
2019-08-24T18:45:42.3487347Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-24T18:45:42.3487595Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-24T18:45:42.3487678Z error: aborting due to 3 previous errors
2019-08-24T18:45:42.3487701Z 
2019-08-24T18:45:42.3487899Z For more information about this error, try `rustc --explain E0705`.
2019-08-24T18:45:42.3487941Z 
2019-08-24T18:45:42.3487941Z 
2019-08-24T18:45:42.3488111Z ------------------------------------------
2019-08-24T18:45:42.3488137Z 
2019-08-24T18:45:42.3488158Z 
2019-08-24T18:45:42.3488611Z ---- [ui] ui/rust-2018/edition-lint-nested-paths.rs stdout ----
2019-08-24T18:45:42.3488678Z diff of stderr:
2019-08-24T18:45:42.3488813Z 
2019-08-24T18:45:42.3488852Z + warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-24T18:45:42.3489085Z +   --> $DIR/edition-lint-nested-paths.rs:3:12
2019-08-24T18:45:42.3489124Z +    |
2019-08-24T18:45:42.3489162Z + LL | #![feature(rust_2018_preview, crate_visibility_modifier)]
2019-08-24T18:45:42.3489258Z + 
2019-08-24T18:45:42.3489258Z + 
2019-08-24T18:45:42.3489302Z 1 error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-24T18:45:42.3489545Z 3    |
2019-08-24T18:45:42.3489566Z 
2019-08-24T18:45:42.3489597Z 23 
2019-08-24T18:45:42.3489653Z 24 error: aborting due to 2 previous errors
2019-08-24T18:45:42.3489653Z 24 error: aborting due to 2 previous errors
2019-08-24T18:45:42.3489686Z 25 
2019-08-24T18:45:42.3489879Z + For more information about this error, try `rustc --explain E0705`.
2019-08-24T18:45:42.3489916Z 26 
2019-08-24T18:45:42.3489963Z 
2019-08-24T18:45:42.3489984Z 
2019-08-24T18:45:42.3490020Z The actual stderr differed from the expected stderr.
2019-08-24T18:45:42.3490277Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-nested-paths/edition-lint-nested-paths.stderr
2019-08-24T18:45:42.3490509Z To update references, rerun the tests and pass the `--bless` flag
2019-08-24T18:45:42.3491174Z To only update this specific test, also pass `--test-args rust-2018/edition-lint-nested-paths.rs`
2019-08-24T18:45:42.3491283Z error: 1 errors occurred comparing output.
2019-08-24T18:45:42.3491328Z status: exit code: 1
2019-08-24T18:45:42.3491328Z status: exit code: 1
2019-08-24T18:45:42.3492097Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/edition-lint-nested-paths.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-nested-paths" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-nested-paths/auxiliary" "-A" "unused"
2019-08-24T18:45:42.3492548Z ------------------------------------------
2019-08-24T18:45:42.3492611Z 
2019-08-24T18:45:42.3492859Z ------------------------------------------
2019-08-24T18:45:42.3492906Z stderr:
2019-08-24T18:45:42.3492906Z stderr:
2019-08-24T18:45:42.3493133Z ------------------------------------------
2019-08-24T18:45:42.3493191Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-24T18:45:42.3493510Z    |
2019-08-24T18:45:42.3493510Z    |
2019-08-24T18:45:42.3493558Z LL | #![feature(rust_2018_preview, crate_visibility_modifier)]
2019-08-24T18:45:42.3493647Z 
2019-08-24T18:45:42.3493647Z 
2019-08-24T18:45:42.3493718Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-24T18:45:42.3494027Z    |
2019-08-24T18:45:42.3494027Z    |
2019-08-24T18:45:42.3494269Z LL | use foo::{a, b};
2019-08-24T18:45:42.3494477Z    |     ^^^^^^^^^^^ help: use `crate`: `crate::foo::{a, b}`
2019-08-24T18:45:42.3494736Z note: lint level defined here
2019-08-24T18:45:42.3494939Z   --> /checkout/src/test/ui/rust-2018/edition-lint-nested-paths.rs:4:9
2019-08-24T18:45:42.3494978Z    |
2019-08-24T18:45:42.3494978Z    |
2019-08-24T18:45:42.3495030Z LL | #![deny(absolute_paths_not_starting_with_crate)]
2019-08-24T18:45:42.3495069Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-24T18:45:42.3495115Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-24T18:45:42.3495394Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-24T18:45:42.3495505Z 
2019-08-24T18:45:42.3495548Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-24T18:45:42.3495843Z    |
2019-08-24T18:45:42.3495843Z    |
2019-08-24T18:45:42.3495885Z LL |         use foo::{self as x, c};
2019-08-24T18:45:42.3495943Z    |             ^^^^^^^^^^^^^^^^^^^ help: use `crate`: `crate::foo::{self as x, c}`
2019-08-24T18:45:42.3495980Z    |
2019-08-24T18:45:42.3496022Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-24T18:45:42.3496291Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-24T18:45:42.3496356Z error: aborting due to 2 previous errors
2019-08-24T18:45:42.3496379Z 
2019-08-24T18:45:42.3496593Z For more information about this error, try `rustc --explain E0705`.
2019-08-24T18:45:42.3496629Z 
2019-08-24T18:45:42.3496629Z 
2019-08-24T18:45:42.3496796Z ------------------------------------------
2019-08-24T18:45:42.3496822Z 
2019-08-24T18:45:42.3496858Z 
2019-08-24T18:45:42.3497039Z ---- [ui] ui/rust-2018/edition-lint-paths.rs stdout ----
2019-08-24T18:45:42.3497078Z diff of stderr:
2019-08-24T18:45:42.3497101Z 
2019-08-24T18:45:42.3497163Z + warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-24T18:45:42.3497337Z +   --> $DIR/edition-lint-paths.rs:4:12
2019-08-24T18:45:42.3497374Z +    |
2019-08-24T18:45:42.3497425Z + LL | #![feature(rust_2018_preview)]
2019-08-24T18:45:42.3497494Z + 
2019-08-24T18:45:42.3497494Z + 
2019-08-24T18:45:42.3497537Z 1 error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-24T18:45:42.3497766Z 3    |
2019-08-24T18:45:42.3497796Z 
2019-08-24T18:45:42.3497842Z 68 
2019-08-24T18:45:42.3497878Z 69 error: aborting due to 7 previous errors
2019-08-24T18:45:42.3497878Z 69 error: aborting due to 7 previous errors
2019-08-24T18:45:42.3497912Z 70 
2019-08-24T18:45:42.3498105Z + For more information about this error, try `rustc --explain E0705`.
2019-08-24T18:45:42.3498159Z 71 
2019-08-24T18:45:42.3498180Z 
2019-08-24T18:45:42.3498200Z 
2019-08-24T18:45:42.3498306Z The actual stderr differed from the expected stderr.
2019-08-24T18:45:42.3498595Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-paths/edition-lint-paths.stderr
2019-08-24T18:45:42.3498790Z To update references, rerun the tests and pass the `--bless` flag
2019-08-24T18:45:42.3498998Z To only update this specific test, also pass `--test-args rust-2018/edition-lint-paths.rs`
2019-08-24T18:45:42.3499081Z error: 1 errors occurred comparing output.
2019-08-24T18:45:42.3499116Z status: exit code: 1
2019-08-24T18:45:42.3499116Z status: exit code: 1
2019-08-24T18:45:42.3499731Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/edition-lint-paths.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-paths" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-paths/auxiliary" "-A" "unused"
2019-08-24T18:45:42.3500004Z ------------------------------------------
2019-08-24T18:45:42.3500031Z 
2019-08-24T18:45:42.3500201Z ------------------------------------------
2019-08-24T18:45:42.3500253Z stderr:
2019-08-24T18:45:42.3500253Z stderr:
2019-08-24T18:45:42.3500419Z ------------------------------------------
2019-08-24T18:45:42.3500463Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-24T18:45:42.3500782Z    |
2019-08-24T18:45:42.3500782Z    |
2019-08-24T18:45:42.3500818Z LL | #![feature(rust_2018_preview)]
2019-08-24T18:45:42.3500894Z 
2019-08-24T18:45:42.3500894Z 
2019-08-24T18:45:42.3500936Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-24T18:45:42.3501967Z    |
2019-08-24T18:45:42.3501967Z    |
2019-08-24T18:45:42.3502010Z LL |     use ::bar::Bar;
2019-08-24T18:45:42.3502058Z    |         ^^^^^^^^^^ help: use `crate`: `crate::bar::Bar`
2019-08-24T18:45:42.3502162Z note: lint level defined here
2019-08-24T18:45:42.3502446Z   --> /checkout/src/test/ui/rust-2018/edition-lint-paths.rs:5:9
2019-08-24T18:45:42.3502511Z    |
2019-08-24T18:45:42.3502511Z    |
2019-08-24T18:45:42.3502558Z LL | #![deny(absolute_paths_not_starting_with_crate)]
2019-08-24T18:45:42.3502607Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-24T18:45:42.3502677Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-24T18:45:42.3503022Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-24T18:45:42.3503060Z 
2019-08-24T18:45:42.3503138Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-24T18:45:42.3503443Z    |
2019-08-24T18:45:42.3503485Z LL |     use bar;
2019-08-24T18:45:42.3503548Z    |         ^^^ help: use `crate`: `crate::bar`
2019-08-24T18:45:42.3503592Z    |
2019-08-24T18:45:42.3503592Z    |
2019-08-24T18:45:42.3503646Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-24T18:45:42.3503964Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-24T18:45:42.3503999Z 
2019-08-24T18:45:42.3504063Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-24T18:45:42.3504383Z    |
2019-08-24T18:45:42.3504383Z    |
2019-08-24T18:45:42.3504427Z LL |     use {Bar as SomethingElse, main};
2019-08-24T18:45:42.3504806Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `crate`: `crate::{Bar as SomethingElse, main}`
2019-08-24T18:45:42.3505036Z    |
2019-08-24T18:45:42.3505078Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-24T18:45:42.3505368Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-24T18:45:42.3505397Z 
2019-08-24T18:45:42.3505439Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-24T18:45:42.3505706Z    |
2019-08-24T18:45:42.3505739Z LL | use bar::Bar;
2019-08-24T18:45:42.3505739Z LL | use bar::Bar;
2019-08-24T18:45:42.3505792Z    |     ^^^^^^^^ help: use `crate`: `crate::bar::Bar`
2019-08-24T18:45:42.3505826Z    |
2019-08-24T18:45:42.3505869Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-24T18:45:42.3506136Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-24T18:45:42.3506165Z 
2019-08-24T18:45:42.3506207Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-24T18:45:42.3506464Z    |
2019-08-24T18:45:42.3506497Z LL |     use *;
2019-08-24T18:45:42.3506497Z LL |     use *;
2019-08-24T18:45:42.3506550Z    |         ^ help: use `crate`: `crate::*`
2019-08-24T18:45:42.3506584Z    |
2019-08-24T18:45:42.3506627Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-24T18:45:42.3506989Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-24T18:45:42.3507018Z 
2019-08-24T18:45:42.3507061Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-24T18:45:42.3507327Z    |
2019-08-24T18:45:42.3507327Z    |
2019-08-24T18:45:42.3507362Z LL | impl ::foo::SomeTrait for u32 { }
2019-08-24T18:45:42.3507401Z    |      ^^^^^^^^^^^^^^^^ help: use `crate`: `crate::foo::SomeTrait`
2019-08-24T18:45:42.3507453Z    |
2019-08-24T18:45:42.3507497Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-24T18:45:42.3507751Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-24T18:45:42.3507780Z 
2019-08-24T18:45:42.3507822Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-24T18:45:42.3508089Z    |
2019-08-24T18:45:42.3508123Z LL |     let x = ::bar::Bar;
2019-08-24T18:45:42.3508123Z LL |     let x = ::bar::Bar;
2019-08-24T18:45:42.3508162Z    |             ^^^^^^^^^^ help: use `crate`: `crate::bar::Bar`
2019-08-24T18:45:42.3508214Z    |
2019-08-24T18:45:42.3508265Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-24T18:45:42.3508519Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-24T18:45:42.3508757Z error: aborting due to 7 previous errors
2019-08-24T18:45:42.3508780Z 
2019-08-24T18:45:42.3508997Z For more information about this error, try `rustc --explain E0705`.
2019-08-24T18:45:42.3509207Z 
2019-08-24T18:45:42.3509207Z 
2019-08-24T18:45:42.3509386Z ------------------------------------------
2019-08-24T18:45:42.3509413Z 
2019-08-24T18:45:42.3509435Z 
2019-08-24T18:45:42.3509644Z ---- [ui] ui/rust-2018/extern-crate-idiomatic.rs stdout ----
2019-08-24T18:45:42.3509695Z normalized stderr:
2019-08-24T18:45:42.3509738Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-24T18:45:42.3509979Z    |
2019-08-24T18:45:42.3509979Z    |
2019-08-24T18:45:42.3510016Z LL | #![feature(rust_2018_preview)]
2019-08-24T18:45:42.3510174Z 
2019-08-24T18:45:42.3510195Z 
2019-08-24T18:45:42.3510216Z 
2019-08-24T18:45:42.3510237Z 
2019-08-24T18:45:42.3510237Z 
2019-08-24T18:45:42.3510294Z The actual stderr differed from the expected stderr.
2019-08-24T18:45:42.3510584Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic/extern-crate-idiomatic.stderr
2019-08-24T18:45:42.3510797Z To update references, rerun the tests and pass the `--bless` flag
2019-08-24T18:45:42.3511049Z To only update this specific test, also pass `--test-args rust-2018/extern-crate-idiomatic.rs`
2019-08-24T18:45:42.3511493Z error: 1 errors occurred comparing output.
2019-08-24T18:45:42.3511559Z status: exit code: 0
2019-08-24T18:45:42.3511559Z status: exit code: 0
2019-08-24T18:45:42.3512407Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/extern-crate-idiomatic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern" "edition_lint_paths" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic/auxiliary" "-A" "unused"
2019-08-24T18:45:42.3512743Z ------------------------------------------
---
2019-08-24T18:45:42.3571619Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-24T18:45:42.3571680Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-24T18:45:42.3571731Z 
2019-08-24T18:45:42.3571757Z 
2019-08-24T18:45:42.3573556Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-24T18:45:42.3573841Z 
2019-08-24T18:45:42.3573873Z 
2019-08-24T18:45:42.3573931Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-24T18:45:42.3574001Z Build completed unsuccessfully in 1:04:20
2019-08-24T18:45:42.3574001Z Build completed unsuccessfully in 1:04:20
2019-08-24T18:45:42.3574050Z == clock drift check ==
2019-08-24T18:45:42.3574099Z   local time: Sat Aug 24 18:45:42 UTC 2019
2019-08-24T18:45:42.5038850Z   network time: Sat, 24 Aug 2019 18:45:42 GMT
2019-08-24T18:45:42.5042821Z == end clock drift check ==
2019-08-24T18:45:43.4698348Z ##[error]Bash exited with code '1'.
2019-08-24T18:45:43.4755565Z ##[section]Starting: Checkout
2019-08-24T18:45:43.4757007Z ==============================================================================
2019-08-24T18:45:43.4757050Z Task         : Get sources
2019-08-24T18:45:43.4757106Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
