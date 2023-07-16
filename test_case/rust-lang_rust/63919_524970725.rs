plain
2019-08-26T17:08:07.4071455Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-26T17:08:07.4254141Z ##[command]git config gc.auto 0
2019-08-26T17:08:07.4331705Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-26T17:08:07.4376033Z ##[command]git config --get-all http.proxy
2019-08-26T17:08:07.4512138Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63919/merge:refs/remotes/pull/63919/merge
---
2019-08-26T17:08:43.1351965Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-26T17:08:43.1352001Z 
2019-08-26T17:08:43.1352244Z   git checkout -b <new-branch-name>
2019-08-26T17:08:43.1352275Z 
2019-08-26T17:08:43.1352322Z HEAD is now at 7668717f3 Merge 183ae29ccd221bf9109f1eda6040f37f041c029d into 555d7a2fd6165b614cfc01136d8e3f5c465a1582
2019-08-26T17:08:43.1530032Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-26T17:08:43.1534639Z ==============================================================================
2019-08-26T17:08:43.1534696Z Task         : Bash
2019-08-26T17:08:43.1534762Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-26T18:10:07.7124963Z .................................................................................................... 1500/8956
2019-08-26T18:10:13.2851489Z .................................................................................................... 1600/8956
2019-08-26T18:10:25.7681455Z .............................................i...............i...................................... 1700/8956
2019-08-26T18:10:33.8155479Z .................................................................................................... 1800/8956
2019-08-26T18:10:48.0232103Z .....................................iiiii.......................................................... 1900/8956
2019-08-26T18:10:58.3806845Z .................................................................................................... 2100/8956
2019-08-26T18:11:00.7958780Z .................................................................................................... 2200/8956
2019-08-26T18:11:05.0063363Z .................................................................................................... 2300/8956
2019-08-26T18:11:12.1560697Z .................................................................................................... 2400/8956
---
2019-08-26T18:14:06.8916878Z ..........................i...............i......................................................... 4700/8956
2019-08-26T18:14:18.5377175Z .................................................................................................... 4800/8956
2019-08-26T18:14:24.5413784Z .................................................................................................... 4900/8956
2019-08-26T18:14:35.1343089Z .................................................................................................... 5000/8956
2019-08-26T18:14:40.6551640Z .......ii.ii........................................................................................ 5100/8956
2019-08-26T18:14:54.6551330Z .................................................................................................... 5300/8956
2019-08-26T18:15:01.6637759Z ...............................................................i.................................... 5400/8956
2019-08-26T18:15:08.6523185Z .................................................................................................... 5500/8956
2019-08-26T18:15:16.0124417Z .................................................................................................... 5600/8956
2019-08-26T18:15:16.0124417Z .................................................................................................... 5600/8956
2019-08-26T18:15:26.0564787Z .........................................................ii...i..ii...........i..................... 5700/8956
2019-08-26T18:15:47.2089207Z .................................................................................................... 5900/8956
2019-08-26T18:15:51.9930491Z .................................................................................................... 6000/8956
2019-08-26T18:15:51.9930491Z .................................................................................................... 6000/8956
2019-08-26T18:15:58.8906930Z ..........................................................i..ii..................................... 6100/8956
2019-08-26T18:16:26.0733711Z .................................................................................................... 6300/8956
2019-08-26T18:16:28.2052802Z ....i............................................................................................... 6400/8956
2019-08-26T18:16:30.3508136Z ............................................................................i....................... 6500/8956
2019-08-26T18:16:32.9480492Z .................................................................................................... 6600/8956
---
2019-08-26T18:20:27.0058234Z -    |
2019-08-26T18:20:27.0058462Z - note: lint level defined here
2019-08-26T18:20:27.0059018Z -   --> $DIR/extern-crate-used.rs:6:9
2019-08-26T18:20:27.0059217Z -    |
2019-08-26T18:20:27.0059431Z - LL | #![deny(unused_extern_crates)]
2019-08-26T18:20:27.0059714Z 12 
2019-08-26T18:20:27.0059757Z 13 error: aborting due to previous error
2019-08-26T18:20:27.0059817Z 14 
2019-08-26T18:20:27.0059846Z 
2019-08-26T18:20:27.0059846Z 
2019-08-26T18:20:27.0060072Z 
2019-08-26T18:20:27.0060130Z The actual stderr differed from the expected stderr.
2019-08-26T18:20:27.0060514Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/extern-crate-used/extern-crate-used.stderr
2019-08-26T18:20:27.0060773Z To update references, rerun the tests and pass the `--bless` flag
2019-08-26T18:20:27.0061078Z To only update this specific test, also pass `--test-args imports/extern-crate-used.rs`
2019-08-26T18:20:27.0061185Z error: 1 errors occurred comparing output.
2019-08-26T18:20:27.0061233Z status: exit code: 1
2019-08-26T18:20:27.0061233Z status: exit code: 1
2019-08-26T18:20:27.0062050Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/extern-crate-used.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/extern-crate-used" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/extern-crate-used/auxiliary" "-A" "unused"
2019-08-26T18:20:27.0062410Z ------------------------------------------
2019-08-26T18:20:27.0062448Z 
2019-08-26T18:20:27.0062689Z ------------------------------------------
2019-08-26T18:20:27.0062737Z stderr:
2019-08-26T18:20:27.0062737Z stderr:
2019-08-26T18:20:27.0062995Z ------------------------------------------
2019-08-26T18:20:27.0063045Z error: unused extern crate
2019-08-26T18:20:27.0063306Z   --> /checkout/src/test/ui/imports/extern-crate-used.rs:18:1
2019-08-26T18:20:27.0063505Z    |
2019-08-26T18:20:27.0063556Z LL | extern crate core; //~ ERROR unused extern crate
2019-08-26T18:20:27.0063660Z 
2019-08-26T18:20:27.0063707Z error: aborting due to previous error
2019-08-26T18:20:27.0063737Z 
2019-08-26T18:20:27.0063765Z 
---
2019-08-26T18:20:27.0064852Z -    |
2019-08-26T18:20:27.0065077Z - note: lint level defined here
2019-08-26T18:20:27.0065317Z -   --> $DIR/removing-extern-crate.rs:6:9
2019-08-26T18:20:27.0065556Z -    |
2019-08-26T18:20:27.0065787Z - LL | #![warn(rust_2018_idioms)]
2019-08-26T18:20:27.0066456Z -    |         ^^^^^^^^^^^^^^^^
2019-08-26T18:20:27.0066776Z -    = note: `#[warn(unused_extern_crates)]` implied by `#[warn(rust_2018_idioms)]`
2019-08-26T18:20:27.0066893Z 14 warning: unused extern crate
2019-08-26T18:20:27.0067163Z 15   --> $DIR/removing-extern-crate.rs:10:1
2019-08-26T18:20:27.0067198Z 
2019-08-26T18:20:27.0067226Z 
2019-08-26T18:20:27.0067226Z 
2019-08-26T18:20:27.0067273Z The actual stderr differed from the expected stderr.
2019-08-26T18:20:27.0067619Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/removing-extern-crate/removing-extern-crate.stderr
2019-08-26T18:20:27.0067890Z To update references, rerun the tests and pass the `--bless` flag
2019-08-26T18:20:27.0068173Z To only update this specific test, also pass `--test-args removing-extern-crate.rs`
2019-08-26T18:20:27.0068286Z error: 1 errors occurred comparing output.
2019-08-26T18:20:27.0068333Z status: exit code: 0
2019-08-26T18:20:27.0068333Z status: exit code: 0
2019-08-26T18:20:27.0069469Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/removing-extern-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/removing-extern-crate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/removing-extern-crate/auxiliary" "-A" "unused"
2019-08-26T18:20:27.0069874Z ------------------------------------------
2019-08-26T18:20:27.0069910Z 
2019-08-26T18:20:27.0070135Z ------------------------------------------
2019-08-26T18:20:27.0070181Z stderr:
---
2019-08-26T18:20:27.0073789Z -    |
2019-08-26T18:20:27.0073994Z - note: lint level defined here
2019-08-26T18:20:27.0074232Z -   --> $DIR/extern-crate-idiomatic-in-2018.rs:9:9
2019-08-26T18:20:27.0074413Z -    |
2019-08-26T18:20:27.0074631Z - LL | #![deny(rust_2018_idioms)]
2019-08-26T18:20:27.0074837Z -    |         ^^^^^^^^^^^^^^^^
2019-08-26T18:20:27.0075120Z -    = note: `#[deny(unused_extern_crates)]` implied by `#[deny(rust_2018_idioms)]`
2019-08-26T18:20:27.0075226Z 14 error: aborting due to previous error
2019-08-26T18:20:27.0075287Z 15 
2019-08-26T18:20:27.0075313Z 
2019-08-26T18:20:27.0075337Z 
2019-08-26T18:20:27.0075337Z 
2019-08-26T18:20:27.0075381Z The actual stderr differed from the expected stderr.
2019-08-26T18:20:27.0075730Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic-in-2018/extern-crate-idiomatic-in-2018.stderr
2019-08-26T18:20:27.0075978Z To update references, rerun the tests and pass the `--bless` flag
2019-08-26T18:20:27.0076301Z To only update this specific test, also pass `--test-args rust-2018/extern-crate-idiomatic-in-2018.rs`
2019-08-26T18:20:27.0076394Z error: 1 errors occurred comparing output.
2019-08-26T18:20:27.0076440Z status: exit code: 1
2019-08-26T18:20:27.0076440Z status: exit code: 1
2019-08-26T18:20:27.0077556Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic-in-2018" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern" "edition_lint_paths" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic-in-2018/auxiliary" "-A" "unused"
2019-08-26T18:20:27.0077935Z ------------------------------------------
2019-08-26T18:20:27.0077970Z 
2019-08-26T18:20:27.0078205Z ------------------------------------------
2019-08-26T18:20:27.0078272Z stderr:
---
2019-08-26T18:20:27.0080784Z -    |
2019-08-26T18:20:27.0080995Z - note: lint level defined here
2019-08-26T18:20:27.0081260Z -   --> $DIR/issue-54400-unused-extern-crate-attr-span.rs:6:9
2019-08-26T18:20:27.0081458Z -    |
2019-08-26T18:20:27.0081673Z - LL | #![deny(rust_2018_idioms)]
2019-08-26T18:20:27.0081906Z -    |         ^^^^^^^^^^^^^^^^
2019-08-26T18:20:27.0082172Z -    = note: `#[deny(unused_extern_crates)]` implied by `#[deny(rust_2018_idioms)]`
2019-08-26T18:20:27.0082284Z 17 error: aborting due to previous error
2019-08-26T18:20:27.0082325Z 18 
2019-08-26T18:20:27.0082353Z 
2019-08-26T18:20:27.0082379Z 
2019-08-26T18:20:27.0082379Z 
2019-08-26T18:20:27.0082423Z The actual stderr differed from the expected stderr.
2019-08-26T18:20:27.0082804Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/issue-54400-unused-extern-crate-attr-span/issue-54400-unused-extern-crate-attr-span.stderr
2019-08-26T18:20:27.0083078Z To update references, rerun the tests and pass the `--bless` flag
2019-08-26T18:20:27.0083574Z To only update this specific test, also pass `--test-args rust-2018/issue-54400-unused-extern-crate-attr-span.rs`
2019-08-26T18:20:27.0083663Z error: 1 errors occurred comparing output.
2019-08-26T18:20:27.0083706Z status: exit code: 1
2019-08-26T18:20:27.0083706Z status: exit code: 1
2019-08-26T18:20:27.0084591Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/issue-54400-unused-extern-crate-attr-span.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/issue-54400-unused-extern-crate-attr-span" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern" "edition_lint_paths" "--cfg" "blandiloquence" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/issue-54400-unused-extern-crate-attr-span/auxiliary" "-A" "unused"
2019-08-26T18:20:27.0084939Z ------------------------------------------
2019-08-26T18:20:27.0084972Z 
2019-08-26T18:20:27.0085287Z ------------------------------------------
2019-08-26T18:20:27.0085339Z stderr:
2019-08-26T18:20:27.0085339Z stderr:
2019-08-26T18:20:27.0085574Z ------------------------------------------
2019-08-26T18:20:27.0085621Z error: unused extern crate
2019-08-26T18:20:27.0085905Z   --> /checkout/src/test/ui/rust-2018/issue-54400-unused-extern-crate-attr-span.rs:12:1
2019-08-26T18:20:27.0085956Z    |
2019-08-26T18:20:27.0086001Z LL | / #[cfg(blandiloquence)] //~ HELP remove it
2019-08-26T18:20:27.0086067Z LL | | extern crate edition_lint_paths;
2019-08-26T18:20:27.0086281Z    | | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-
2019-08-26T18:20:27.0086404Z    |                                  help: remove it
2019-08-26T18:20:27.0086434Z 
2019-08-26T18:20:27.0086475Z error: aborting due to previous error
2019-08-26T18:20:27.0086503Z 
---
2019-08-26T18:20:27.0087476Z -    |
2019-08-26T18:20:27.0087677Z - note: lint level defined here
2019-08-26T18:20:27.0087911Z -   --> $DIR/remove-extern-crate.rs:7:9
2019-08-26T18:20:27.0088092Z -    |
2019-08-26T18:20:27.0088690Z - LL | #![warn(rust_2018_idioms)]
2019-08-26T18:20:27.0088937Z -    |         ^^^^^^^^^^^^^^^^
2019-08-26T18:20:27.0089323Z -    = note: `#[warn(unused_extern_crates)]` implied by `#[warn(rust_2018_idioms)]`
2019-08-26T18:20:27.0089373Z 13 
2019-08-26T18:20:27.0089441Z 14 warning: `extern crate` is not idiomatic in the new edition
2019-08-26T18:20:27.0089701Z 
2019-08-26T18:20:27.0089728Z 
2019-08-26T18:20:27.0089802Z The actual stderr differed from the expected stderr.
2019-08-26T18:20:27.0090127Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/remove-extern-crate/remove-extern-crate.stderr
2019-08-26T18:20:27.0090127Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/remove-extern-crate/remove-extern-crate.stderr
2019-08-26T18:20:27.0090381Z To update references, rerun the tests and pass the `--bless` flag
2019-08-26T18:20:27.0090705Z To only update this specific test, also pass `--test-args rust-2018/remove-extern-crate.rs`
2019-08-26T18:20:27.0090789Z error: 1 errors occurred comparing output.
2019-08-26T18:20:27.0090836Z status: exit code: 0
2019-08-26T18:20:27.0090836Z status: exit code: 0
2019-08-26T18:20:27.0091696Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/remove-extern-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/remove-extern-crate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--extern" "remove_extern_crate" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/remove-extern-crate/auxiliary" "-A" "unused"
2019-08-26T18:20:27.0092060Z ------------------------------------------
2019-08-26T18:20:27.0092097Z 
2019-08-26T18:20:27.0092339Z ------------------------------------------
2019-08-26T18:20:27.0092407Z stderr:
2019-08-26T18:20:27.0092407Z stderr:
2019-08-26T18:20:27.0092643Z ------------------------------------------
2019-08-26T18:20:27.0092692Z warning: unused extern crate
2019-08-26T18:20:27.0092992Z   --> /checkout/src/test/ui/rust-2018/remove-extern-crate.rs:9:1
2019-08-26T18:20:27.0093044Z    |
2019-08-26T18:20:27.0093091Z LL | extern crate core;
2019-08-26T18:20:27.0093158Z    | ^^^^^^^^^^^^^^^^^^ help: remove it
2019-08-26T18:20:27.0093190Z 
2019-08-26T18:20:27.0093238Z warning: `extern crate` is not idiomatic in the new edition
2019-08-26T18:20:27.0093667Z    |
2019-08-26T18:20:27.0093712Z LL |     extern crate core;
2019-08-26T18:20:27.0093762Z    |     ^^^^^^^^^^^^^^^^^^ help: convert it to a `use`
2019-08-26T18:20:27.0093814Z 
---
2019-08-26T18:20:27.0094884Z -    |
2019-08-26T18:20:27.0095126Z - note: lint level defined here
2019-08-26T18:20:27.0095365Z -   --> $DIR/unnecessary-extern-crate.rs:3:9
2019-08-26T18:20:27.0095568Z -    |
2019-08-26T18:20:27.0095979Z - LL | #![deny(unused_extern_crates)]
2019-08-26T18:20:27.0096282Z 12 
2019-08-26T18:20:27.0096342Z 13 error: unused extern crate
2019-08-26T18:20:27.0096572Z 14   --> $DIR/unnecessary-extern-crate.rs:9:1
2019-08-26T18:20:27.0096605Z 
2019-08-26T18:20:27.0096605Z 
2019-08-26T18:20:27.0096630Z 
2019-08-26T18:20:27.0096674Z The actual stderr differed from the expected stderr.
2019-08-26T18:20:27.0097000Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unnecessary-extern-crate/unnecessary-extern-crate.stderr
2019-08-26T18:20:27.0097246Z To update references, rerun the tests and pass the `--bless` flag
2019-08-26T18:20:27.0097647Z To only update this specific test, also pass `--test-args unnecessary-extern-crate.rs`
2019-08-26T18:20:27.0097728Z error: 1 errors occurred comparing output.
2019-08-26T18:20:27.0097772Z status: exit code: 1
2019-08-26T18:20:27.0097772Z status: exit code: 1
2019-08-26T18:20:27.0098907Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unnecessary-extern-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unnecessary-extern-crate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unnecessary-extern-crate/auxiliary" "-A" "unused"
2019-08-26T18:20:27.0099285Z ------------------------------------------
2019-08-26T18:20:27.0099330Z 
2019-08-26T18:20:27.0099559Z ------------------------------------------
2019-08-26T18:20:27.0099626Z stderr:
---
2019-08-26T18:20:27.0138372Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-26T18:20:27.0138451Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-26T18:20:27.0157645Z 
2019-08-26T18:20:27.0159576Z 
2019-08-26T18:20:27.0161347Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-26T18:20:27.0161889Z 
2019-08-26T18:20:27.0161926Z 
2019-08-26T18:20:27.0189026Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-26T18:20:27.0189107Z Build completed unsuccessfully in 1:05:21
2019-08-26T18:20:27.0189107Z Build completed unsuccessfully in 1:05:21
2019-08-26T18:20:27.0222015Z == clock drift check ==
2019-08-26T18:20:27.0239181Z   local time: Mon Aug 26 18:20:27 UTC 2019
2019-08-26T18:20:27.1088692Z   network time: Mon, 26 Aug 2019 18:20:27 GMT
2019-08-26T18:20:27.1089433Z == end clock drift check ==
2019-08-26T18:20:28.1931064Z ##[error]Bash exited with code '1'.
2019-08-26T18:20:28.1974833Z ##[section]Starting: Checkout
2019-08-26T18:20:28.1976514Z ==============================================================================
2019-08-26T18:20:28.1976567Z Task         : Get sources
2019-08-26T18:20:28.1976630Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
