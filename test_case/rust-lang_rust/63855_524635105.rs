plain
2019-08-25T13:20:33.8282869Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-25T13:20:33.8487858Z ##[command]git config gc.auto 0
2019-08-25T13:20:33.8579537Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-25T13:20:33.8625844Z ##[command]git config --get-all http.proxy
2019-08-25T13:20:34.6059518Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63855/merge:refs/remotes/pull/63855/merge
---
2019-08-25T13:21:08.1891758Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-25T13:21:08.1893478Z 
2019-08-25T13:21:08.1917690Z   git checkout -b <new-branch-name>
2019-08-25T13:21:08.1918130Z 
2019-08-25T13:21:08.1918968Z HEAD is now at 10172b49c Merge 1e6d3e2df492a61e02ccd5484f2c855294a05ef8 into d760df5aea483aae041c9a241e7acacf48f75035
2019-08-25T13:21:08.2014902Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-25T13:21:08.2018022Z ==============================================================================
2019-08-25T13:21:08.2018107Z Task         : Bash
2019-08-25T13:21:08.2018160Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-25T14:24:45.3746853Z .................................................................................................... 1500/8953
2019-08-25T14:24:51.2353248Z .................................................................................................... 1600/8953
2019-08-25T14:25:04.4028572Z .............................................i...............i...................................... 1700/8953
2019-08-25T14:25:12.8089078Z .................................................................................................... 1800/8953
2019-08-25T14:25:27.5737542Z .....................................iiiii.......................................................... 1900/8953
2019-08-25T14:25:38.7067486Z F................................................................................................... 2100/8953
2019-08-25T14:25:41.4284825Z .................................................................................................... 2200/8953
2019-08-25T14:25:45.9780715Z ....................................................F............................................... 2300/8953
2019-08-25T14:25:53.5391821Z .................................................................................................... 2400/8953
---
2019-08-25T14:28:58.7662769Z .........................i...............i.......................................................... 4700/8953
2019-08-25T14:29:10.7322598Z .................................................................................................... 4800/8953
2019-08-25T14:29:17.0880193Z .................................................................................................... 4900/8953
2019-08-25T14:29:28.2842713Z .................................................................................................... 5000/8953
2019-08-25T14:29:33.9039288Z ......ii.ii......................................................................................... 5100/8953
2019-08-25T14:29:48.5553544Z .................................................................................................... 5300/8953
2019-08-25T14:29:55.7748869Z ..............................................................i..................................... 5400/8953
2019-08-25T14:30:03.1150467Z .................................................................................................... 5500/8953
2019-08-25T14:30:11.1280091Z .................................................................................................... 5600/8953
2019-08-25T14:30:11.1280091Z .................................................................................................... 5600/8953
2019-08-25T14:30:22.1965355Z ........................................................ii...i..ii...........i...................... 5700/8953
2019-08-25T14:30:45.3355227Z .................................................................................................... 5900/8953
2019-08-25T14:30:50.5217731Z .................................................................................................... 6000/8953
2019-08-25T14:30:50.5217731Z .................................................................................................... 6000/8953
2019-08-25T14:30:57.8983064Z .........................................................i..ii...................................... 6100/8953
2019-08-25T14:31:26.3921819Z .................................................................................................... 6300/8953
2019-08-25T14:31:28.7192848Z ...i................................................................................................ 6400/8953
2019-08-25T14:31:31.1243267Z ...........................................................................i........................ 6500/8953
2019-08-25T14:31:34.0520565Z .................................................................................................... 6600/8953
2019-08-25T14:31:34.0520565Z .................................................................................................... 6600/8953
2019-08-25T14:31:40.2444957Z .................................................................................................... 6700/8953
2019-08-25T14:32:07.4607393Z ....................F............................................................................... 6800/8953
2019-08-25T14:32:20.0869134Z .................................................................................................... 6900/8953
2019-08-25T14:32:25.5092126Z .................................................................................................... 7000/8953
2019-08-25T14:32:31.1169121Z .................................................................................................... 7100/8953
2019-08-25T14:32:36.3702254Z .................................................................................................... 7200/8953
2019-08-25T14:32:43.0788270Z .................................................................................................... 7300/8953
2019-08-25T14:32:52.9250878Z ....................................................F..FF..F.F.FF.FFF......F.F...................... 7400/8953
2019-08-25T14:33:10.6391117Z .......................ii......i.................................................................... 7600/8953
2019-08-25T14:33:16.3371363Z .................................................................................................... 7700/8953
2019-08-25T14:33:29.7786822Z .................................................................................................... 7800/8953
2019-08-25T14:33:41.3326335Z .................................................................................................... 7900/8953
---
2019-08-25T14:35:38.1301334Z failures:
2019-08-25T14:35:38.1374937Z 
2019-08-25T14:35:38.1375428Z ---- [ui] ui/editions/edition-feature-ok.rs stdout ----
2019-08-25T14:35:38.1375517Z normalized stderr:
2019-08-25T14:35:38.1375576Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-25T14:35:38.1376391Z    |
2019-08-25T14:35:38.1376391Z    |
2019-08-25T14:35:38.1376436Z LL | #![feature(rust_2018_preview)]
2019-08-25T14:35:38.1376513Z 
2019-08-25T14:35:38.1376556Z 
2019-08-25T14:35:38.1376580Z 
2019-08-25T14:35:38.1376620Z 
2019-08-25T14:35:38.1376620Z 
2019-08-25T14:35:38.1376666Z The actual stderr differed from the expected stderr.
2019-08-25T14:35:38.1377011Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-feature-ok/edition-feature-ok.stderr
2019-08-25T14:35:38.1383343Z To update references, rerun the tests and pass the `--bless` flag
2019-08-25T14:35:38.1383672Z To only update this specific test, also pass `--test-args editions/edition-feature-ok.rs`
2019-08-25T14:35:38.1383759Z error: 1 errors occurred comparing output.
2019-08-25T14:35:38.1383806Z status: exit code: 0
2019-08-25T14:35:38.1383806Z status: exit code: 0
2019-08-25T14:35:38.1384969Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/editions/edition-feature-ok.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-feature-ok" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-feature-ok/auxiliary" "-A" "unused"
2019-08-25T14:35:38.1385391Z ------------------------------------------
2019-08-25T14:35:38.1385426Z 
2019-08-25T14:35:38.1385650Z ------------------------------------------
2019-08-25T14:35:38.1385714Z stderr:
2019-08-25T14:35:38.1385714Z stderr:
2019-08-25T14:35:38.1385933Z ------------------------------------------
2019-08-25T14:35:38.1385987Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-25T14:35:38.1386302Z    |
2019-08-25T14:35:38.1386302Z    |
2019-08-25T14:35:38.1386347Z LL | #![feature(rust_2018_preview)]
2019-08-25T14:35:38.1386438Z 
2019-08-25T14:35:38.1386463Z 
2019-08-25T14:35:38.1386994Z ------------------------------------------
2019-08-25T14:35:38.1387054Z 
2019-08-25T14:35:38.1387054Z 
2019-08-25T14:35:38.1387100Z 
2019-08-25T14:35:38.1387388Z ---- [ui] ui/epoch-gate-feature.rs stdout ----
2019-08-25T14:35:38.1387448Z normalized stderr:
2019-08-25T14:35:38.1387499Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-25T14:35:38.1387744Z   --> $DIR/epoch-gate-feature.rs:8:12
2019-08-25T14:35:38.1387791Z    |
2019-08-25T14:35:38.1387834Z LL | #![feature(rust_2018_preview)]
2019-08-25T14:35:38.1387922Z 
2019-08-25T14:35:38.1387947Z 
2019-08-25T14:35:38.1387972Z 
2019-08-25T14:35:38.1387997Z 
2019-08-25T14:35:38.1387997Z 
2019-08-25T14:35:38.1388057Z The actual stderr differed from the expected stderr.
2019-08-25T14:35:38.1388360Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/epoch-gate-feature/epoch-gate-feature.stderr
2019-08-25T14:35:38.1388611Z To update references, rerun the tests and pass the `--bless` flag
2019-08-25T14:35:38.1389049Z To only update this specific test, also pass `--test-args epoch-gate-feature.rs`
2019-08-25T14:35:38.1389144Z error: 1 errors occurred comparing output.
2019-08-25T14:35:38.1389215Z status: exit code: 0
2019-08-25T14:35:38.1389215Z status: exit code: 0
2019-08-25T14:35:38.1389947Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/epoch-gate-feature.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/epoch-gate-feature/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/epoch-gate-feature/auxiliary" "-A" "unused"
2019-08-25T14:35:38.1390378Z ------------------------------------------
2019-08-25T14:35:38.1390410Z 
2019-08-25T14:35:38.1390640Z ------------------------------------------
2019-08-25T14:35:38.1390695Z stderr:
2019-08-25T14:35:38.1390695Z stderr:
2019-08-25T14:35:38.1390907Z ------------------------------------------
2019-08-25T14:35:38.1390959Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-25T14:35:38.1391380Z    |
2019-08-25T14:35:38.1391380Z    |
2019-08-25T14:35:38.1391542Z LL | #![feature(rust_2018_preview)]
2019-08-25T14:35:38.1391636Z 
2019-08-25T14:35:38.1391660Z 
2019-08-25T14:35:38.1392254Z ------------------------------------------
2019-08-25T14:35:38.1392315Z 
2019-08-25T14:35:38.1392315Z 
2019-08-25T14:35:38.1392340Z 
2019-08-25T14:35:38.1392567Z ---- [ui] ui/error-codes/E0705.rs stdout ----
2019-08-25T14:35:38.1392617Z diff of stderr:
2019-08-25T14:35:38.1392661Z 
2019-08-25T14:35:38.1392928Z - warning[E0705]: the feature `test_2018_feature` is included in the Rust 2018 edition
2019-08-25T14:35:38.1393141Z -   --> $DIR/E0705.rs:6:12
2019-08-25T14:35:38.1393207Z + warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-25T14:35:38.1393442Z +   --> $DIR/E0705.rs:8:12
2019-08-25T14:35:38.1393713Z - LL | #![feature(test_2018_feature)]
2019-08-25T14:35:38.1393713Z - LL | #![feature(test_2018_feature)]
2019-08-25T14:35:38.1393779Z + LL | #![feature(rust_2018_preview)]
2019-08-25T14:35:38.1393867Z 6 
2019-08-25T14:35:38.1393921Z 7 
2019-08-25T14:35:38.1393947Z 
2019-08-25T14:35:38.1393972Z 
2019-08-25T14:35:38.1393972Z 
2019-08-25T14:35:38.1394016Z The actual stderr differed from the expected stderr.
2019-08-25T14:35:38.1394322Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0705/E0705.stderr
2019-08-25T14:35:38.1394572Z To update references, rerun the tests and pass the `--bless` flag
2019-08-25T14:35:38.1394868Z To only update this specific test, also pass `--test-args error-codes/E0705.rs`
2019-08-25T14:35:38.1394971Z error: 1 errors occurred comparing output.
2019-08-25T14:35:38.1395026Z status: exit code: 0
2019-08-25T14:35:38.1395026Z status: exit code: 0
2019-08-25T14:35:38.1395858Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0705.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0705" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0705/auxiliary" "-A" "unused"
2019-08-25T14:35:38.1396855Z ------------------------------------------
2019-08-25T14:35:38.1396909Z 
2019-08-25T14:35:38.1397244Z ------------------------------------------
2019-08-25T14:35:38.1397291Z stderr:
2019-08-25T14:35:38.1397291Z stderr:
2019-08-25T14:35:38.1397522Z ------------------------------------------
2019-08-25T14:35:38.1397706Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-25T14:35:38.1398063Z    |
2019-08-25T14:35:38.1398063Z    |
2019-08-25T14:35:38.1398106Z LL | #![feature(rust_2018_preview)]
2019-08-25T14:35:38.1398192Z 
2019-08-25T14:35:38.1398232Z 
2019-08-25T14:35:38.1398456Z ------------------------------------------
2019-08-25T14:35:38.1398487Z 
2019-08-25T14:35:38.1398487Z 
2019-08-25T14:35:38.1398512Z 
2019-08-25T14:35:38.1398748Z ---- [ui] ui/proc-macro/custom-attr-only-one-derive.rs stdout ----
2019-08-25T14:35:38.1398815Z normalized stderr:
2019-08-25T14:35:38.1398865Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-25T14:35:38.1399156Z    |
2019-08-25T14:35:38.1399156Z    |
2019-08-25T14:35:38.1399200Z LL | #![feature(rust_2018_preview)]
2019-08-25T14:35:38.1399291Z 
2019-08-25T14:35:38.1399316Z 
2019-08-25T14:35:38.1399350Z 
2019-08-25T14:35:38.1399375Z 
2019-08-25T14:35:38.1399375Z 
2019-08-25T14:35:38.1399419Z The actual stderr differed from the expected stderr.
2019-08-25T14:35:38.1399762Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/custom-attr-only-one-derive/custom-attr-only-one-derive.stderr
2019-08-25T14:35:38.1400239Z To update references, rerun the tests and pass the `--bless` flag
2019-08-25T14:35:38.1400553Z To only update this specific test, also pass `--test-args proc-macro/custom-attr-only-one-derive.rs`
2019-08-25T14:35:38.1400637Z error: 1 errors occurred comparing output.
2019-08-25T14:35:38.1400681Z status: exit code: 0
2019-08-25T14:35:38.1400681Z status: exit code: 0
2019-08-25T14:35:38.1401563Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/custom-attr-only-one-derive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/custom-attr-only-one-derive/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/custom-attr-only-one-derive/auxiliary" "-A" "unused"
2019-08-25T14:35:38.1402439Z ------------------------------------------
2019-08-25T14:35:38.1402483Z 
2019-08-25T14:35:38.1402708Z ------------------------------------------
2019-08-25T14:35:38.1402775Z stderr:
2019-08-25T14:35:38.1402775Z stderr:
2019-08-25T14:35:38.1402994Z ------------------------------------------
2019-08-25T14:35:38.1403048Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-25T14:35:38.1403374Z    |
2019-08-25T14:35:38.1403374Z    |
2019-08-25T14:35:38.1403417Z LL | #![feature(rust_2018_preview)]
2019-08-25T14:35:38.1403507Z 
2019-08-25T14:35:38.1403543Z 
2019-08-25T14:35:38.1403765Z ------------------------------------------
2019-08-25T14:35:38.1403797Z 
2019-08-25T14:35:38.1403797Z 
2019-08-25T14:35:38.1403837Z 
2019-08-25T14:35:38.1404098Z ---- [ui] ui/rust-2018/edition-lint-fully-qualified-paths.rs stdout ----
2019-08-25T14:35:38.1404147Z diff of stderr:
2019-08-25T14:35:38.1404175Z 
2019-08-25T14:35:38.1404240Z + warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-25T14:35:38.1404480Z +   --> $DIR/edition-lint-fully-qualified-paths.rs:3:12
2019-08-25T14:35:38.1404526Z +    |
2019-08-25T14:35:38.1404588Z + LL | #![feature(rust_2018_preview, crate_visibility_modifier)]
2019-08-25T14:35:38.1404678Z + 
2019-08-25T14:35:38.1404678Z + 
2019-08-25T14:35:38.1404746Z 1 error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-25T14:35:38.1405044Z 3    |
2019-08-25T14:35:38.1405188Z 
2019-08-25T14:35:38.1405255Z 23 
2019-08-25T14:35:38.1405299Z 24 error: aborting due to 2 previous errors
2019-08-25T14:35:38.1405299Z 24 error: aborting due to 2 previous errors
2019-08-25T14:35:38.1405347Z 25 
2019-08-25T14:35:38.1405745Z + For more information about this error, try `rustc --explain E0705`.
2019-08-25T14:35:38.1405792Z 26 
2019-08-25T14:35:38.1405817Z 
2019-08-25T14:35:38.1405841Z 
2019-08-25T14:35:38.1405901Z The actual stderr differed from the expected stderr.
2019-08-25T14:35:38.1406227Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-fully-qualified-paths/edition-lint-fully-qualified-paths.stderr
2019-08-25T14:35:38.1406473Z To update references, rerun the tests and pass the `--bless` flag
2019-08-25T14:35:38.1406768Z To only update this specific test, also pass `--test-args rust-2018/edition-lint-fully-qualified-paths.rs`
2019-08-25T14:35:38.1406846Z error: 1 errors occurred comparing output.
2019-08-25T14:35:38.1406903Z status: exit code: 1
2019-08-25T14:35:38.1406903Z status: exit code: 1
2019-08-25T14:35:38.1407660Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/edition-lint-fully-qualified-paths.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-fully-qualified-paths" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-fully-qualified-paths/auxiliary" "-A" "unused"
2019-08-25T14:35:38.1408092Z ------------------------------------------
2019-08-25T14:35:38.1408125Z 
2019-08-25T14:35:38.1408355Z ------------------------------------------
2019-08-25T14:35:38.1408399Z stderr:
2019-08-25T14:35:38.1408399Z stderr:
2019-08-25T14:35:38.1408607Z ------------------------------------------
2019-08-25T14:35:38.1408669Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-25T14:35:38.1409006Z    |
2019-08-25T14:35:38.1409006Z    |
2019-08-25T14:35:38.1409051Z LL | #![feature(rust_2018_preview, crate_visibility_modifier)]
2019-08-25T14:35:38.1409140Z 
2019-08-25T14:35:38.1409140Z 
2019-08-25T14:35:38.1409189Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-25T14:35:38.1409521Z    |
2019-08-25T14:35:38.1409521Z    |
2019-08-25T14:35:38.1409563Z LL |     let _: <foo::Baz as ::foo::Foo>::Bar = ();
2019-08-25T14:35:38.1409630Z    |                         ^^^^^^^^^^ help: use `crate`: `crate::foo::Foo`
2019-08-25T14:35:38.1409718Z note: lint level defined here
2019-08-25T14:35:38.1410019Z   --> /checkout/src/test/ui/rust-2018/edition-lint-fully-qualified-paths.rs:4:9
2019-08-25T14:35:38.1410069Z    |
2019-08-25T14:35:38.1410069Z    |
2019-08-25T14:35:38.1410116Z LL | #![deny(absolute_paths_not_starting_with_crate)]
2019-08-25T14:35:38.1410190Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-25T14:35:38.1410248Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-25T14:35:38.1410673Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-25T14:35:38.1410729Z 
2019-08-25T14:35:38.1410786Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-25T14:35:38.1411143Z    |
2019-08-25T14:35:38.1411143Z    |
2019-08-25T14:35:38.1411189Z LL |     let _: <::foo::Baz as foo::Foo>::Bar = ();
2019-08-25T14:35:38.1411238Z    |             ^^^^^^^^^^ help: use `crate`: `crate::foo::Baz`
2019-08-25T14:35:38.1411283Z    |
2019-08-25T14:35:38.1411554Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-25T14:35:38.1412310Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-25T14:35:38.1412419Z error: aborting due to 2 previous errors
2019-08-25T14:35:38.1412448Z 
2019-08-25T14:35:38.1412707Z For more information about this error, try `rustc --explain E0705`.
2019-08-25T14:35:38.1412759Z 
2019-08-25T14:35:38.1412759Z 
2019-08-25T14:35:38.1412979Z ------------------------------------------
2019-08-25T14:35:38.1413011Z 
2019-08-25T14:35:38.1413036Z 
2019-08-25T14:35:38.1413277Z ---- [ui] ui/rust-2018/edition-lint-nested-empty-paths.rs stdout ----
2019-08-25T14:35:38.1413344Z diff of stderr:
2019-08-25T14:35:38.1413371Z 
2019-08-25T14:35:38.1413419Z + warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-25T14:35:38.1413672Z +   --> $DIR/edition-lint-nested-empty-paths.rs:3:12
2019-08-25T14:35:38.1413730Z +    |
2019-08-25T14:35:38.1413776Z + LL | #![feature(rust_2018_preview, crate_visibility_modifier)]
2019-08-25T14:35:38.1414001Z + 
2019-08-25T14:35:38.1414001Z + 
2019-08-25T14:35:38.1414052Z 1 error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-25T14:35:38.1414397Z 3    |
2019-08-25T14:35:38.1414424Z 
2019-08-25T14:35:38.1414462Z 32 
2019-08-25T14:35:38.1414522Z 33 error: aborting due to 3 previous errors
2019-08-25T14:35:38.1414522Z 33 error: aborting due to 3 previous errors
2019-08-25T14:35:38.1414564Z 34 
2019-08-25T14:35:38.1414809Z + For more information about this error, try `rustc --explain E0705`.
2019-08-25T14:35:38.1414856Z 35 
2019-08-25T14:35:38.1414898Z 
2019-08-25T14:35:38.1414923Z 
2019-08-25T14:35:38.1414968Z The actual stderr differed from the expected stderr.
2019-08-25T14:35:38.1415310Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-nested-empty-paths/edition-lint-nested-empty-paths.stderr
2019-08-25T14:35:38.1415689Z To update references, rerun the tests and pass the `--bless` flag
2019-08-25T14:35:38.1415998Z To only update this specific test, also pass `--test-args rust-2018/edition-lint-nested-empty-paths.rs`
2019-08-25T14:35:38.1416099Z error: 1 errors occurred comparing output.
2019-08-25T14:35:38.1416144Z status: exit code: 1
2019-08-25T14:35:38.1416144Z status: exit code: 1
2019-08-25T14:35:38.1416929Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/edition-lint-nested-empty-paths.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-nested-empty-paths" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-nested-empty-paths/auxiliary" "-A" "unused"
2019-08-25T14:35:38.1417272Z ------------------------------------------
2019-08-25T14:35:38.1417316Z 
2019-08-25T14:35:38.1417549Z ------------------------------------------
2019-08-25T14:35:38.1417596Z stderr:
2019-08-25T14:35:38.1417596Z stderr:
2019-08-25T14:35:38.1417839Z ------------------------------------------
2019-08-25T14:35:38.1417893Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-25T14:35:38.1418234Z    |
2019-08-25T14:35:38.1418234Z    |
2019-08-25T14:35:38.1418280Z LL | #![feature(rust_2018_preview, crate_visibility_modifier)]
2019-08-25T14:35:38.1418357Z 
2019-08-25T14:35:38.1418357Z 
2019-08-25T14:35:38.1418467Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-25T14:35:38.1418918Z    |
2019-08-25T14:35:38.1418918Z    |
2019-08-25T14:35:38.1418960Z LL | use foo::{bar::{baz::{}}};
2019-08-25T14:35:38.1419019Z    |     ^^^^^^^^^^^^^^^^^^^^^ help: use `crate`: `crate::foo::{bar::{baz::{}}}`
2019-08-25T14:35:38.1419119Z note: lint level defined here
2019-08-25T14:35:38.1419414Z   --> /checkout/src/test/ui/rust-2018/edition-lint-nested-empty-paths.rs:4:9
2019-08-25T14:35:38.1419464Z    |
2019-08-25T14:35:38.1419464Z    |
2019-08-25T14:35:38.1419526Z LL | #![deny(absolute_paths_not_starting_with_crate)]
2019-08-25T14:35:38.1419574Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-25T14:35:38.1419633Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-25T14:35:38.1419987Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-25T14:35:38.1420023Z 
2019-08-25T14:35:38.1420086Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-25T14:35:38.1420527Z    |
2019-08-25T14:35:38.1420527Z    |
2019-08-25T14:35:38.1420570Z LL | use foo::{bar::{XX, baz::{}}};
2019-08-25T14:35:38.1420639Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `crate`: `crate::foo::{bar::{XX, baz::{}}}`
2019-08-25T14:35:38.1420686Z    |
2019-08-25T14:35:38.1420741Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-25T14:35:38.1421108Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-25T14:35:38.1421145Z 
2019-08-25T14:35:38.1421198Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-25T14:35:38.1421661Z    |
2019-08-25T14:35:38.1421661Z    |
2019-08-25T14:35:38.1421714Z LL | use foo::{bar::{baz::{}, baz1::{}}};
2019-08-25T14:35:38.1422082Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `crate`: `crate::foo::{bar::{baz::{}, baz1::{}}}`
2019-08-25T14:35:38.1422142Z    |
2019-08-25T14:35:38.1422196Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-25T14:35:38.1422606Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-25T14:35:38.1422687Z error: aborting due to 3 previous errors
2019-08-25T14:35:38.1422731Z 
2019-08-25T14:35:38.1422985Z For more information about this error, try `rustc --explain E0705`.
2019-08-25T14:35:38.1423018Z 
2019-08-25T14:35:38.1423018Z 
2019-08-25T14:35:38.1423237Z ------------------------------------------
2019-08-25T14:35:38.1423284Z 
2019-08-25T14:35:38.1423309Z 
2019-08-25T14:35:38.1423549Z ---- [ui] ui/rust-2018/edition-lint-nested-paths.rs stdout ----
2019-08-25T14:35:38.1423599Z diff of stderr:
2019-08-25T14:35:38.1423636Z 
2019-08-25T14:35:38.1423701Z + warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-25T14:35:38.1423933Z +   --> $DIR/edition-lint-nested-paths.rs:3:12
2019-08-25T14:35:38.1423987Z +    |
2019-08-25T14:35:38.1424050Z + LL | #![feature(rust_2018_preview, crate_visibility_modifier)]
2019-08-25T14:35:38.1424138Z + 
2019-08-25T14:35:38.1424138Z + 
2019-08-25T14:35:38.1424204Z 1 error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-25T14:35:38.1424488Z 3    |
2019-08-25T14:35:38.1424513Z 
2019-08-25T14:35:38.1424568Z 23 
2019-08-25T14:35:38.1424612Z 24 error: aborting due to 2 previous errors
2019-08-25T14:35:38.1424612Z 24 error: aborting due to 2 previous errors
2019-08-25T14:35:38.1424653Z 25 
2019-08-25T14:35:38.1424916Z + For more information about this error, try `rustc --explain E0705`.
2019-08-25T14:35:38.1424963Z 26 
2019-08-25T14:35:38.1424989Z 
2019-08-25T14:35:38.1425126Z 
2019-08-25T14:35:38.1425198Z The actual stderr differed from the expected stderr.
2019-08-25T14:35:38.1425654Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-nested-paths/edition-lint-nested-paths.stderr
2019-08-25T14:35:38.1426198Z To update references, rerun the tests and pass the `--bless` flag
2019-08-25T14:35:38.1426509Z To only update this specific test, also pass `--test-args rust-2018/edition-lint-nested-paths.rs`
2019-08-25T14:35:38.1426762Z error: 1 errors occurred comparing output.
2019-08-25T14:35:38.1426854Z status: exit code: 1
2019-08-25T14:35:38.1426854Z status: exit code: 1
2019-08-25T14:35:38.1429691Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/edition-lint-nested-paths.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-nested-paths" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-nested-paths/auxiliary" "-A" "unused"
2019-08-25T14:35:38.1430231Z ------------------------------------------
2019-08-25T14:35:38.1430266Z 
2019-08-25T14:35:38.1430498Z ------------------------------------------
2019-08-25T14:35:38.1430543Z stderr:
2019-08-25T14:35:38.1430543Z stderr:
2019-08-25T14:35:38.1430753Z ------------------------------------------
2019-08-25T14:35:38.1430806Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-25T14:35:38.1431127Z    |
2019-08-25T14:35:38.1431127Z    |
2019-08-25T14:35:38.1431172Z LL | #![feature(rust_2018_preview, crate_visibility_modifier)]
2019-08-25T14:35:38.1431263Z 
2019-08-25T14:35:38.1431263Z 
2019-08-25T14:35:38.1431323Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-25T14:35:38.1432233Z    |
2019-08-25T14:35:38.1432233Z    |
2019-08-25T14:35:38.1432283Z LL | use foo::{a, b};
2019-08-25T14:35:38.1432349Z    |     ^^^^^^^^^^^ help: use `crate`: `crate::foo::{a, b}`
2019-08-25T14:35:38.1432434Z note: lint level defined here
2019-08-25T14:35:38.1432775Z   --> /checkout/src/test/ui/rust-2018/edition-lint-nested-paths.rs:4:9
2019-08-25T14:35:38.1432824Z    |
2019-08-25T14:35:38.1432824Z    |
2019-08-25T14:35:38.1432868Z LL | #![deny(absolute_paths_not_starting_with_crate)]
2019-08-25T14:35:38.1432916Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-25T14:35:38.1432989Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-25T14:35:38.1433341Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-25T14:35:38.1433393Z 
2019-08-25T14:35:38.1433446Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-25T14:35:38.1435188Z    |
2019-08-25T14:35:38.1435188Z    |
2019-08-25T14:35:38.1435234Z LL |         use foo::{self as x, c};
2019-08-25T14:35:38.1435285Z    |             ^^^^^^^^^^^^^^^^^^^ help: use `crate`: `crate::foo::{self as x, c}`
2019-08-25T14:35:38.1435330Z    |
2019-08-25T14:35:38.1435400Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-25T14:35:38.1435744Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-25T14:35:38.1435842Z error: aborting due to 2 previous errors
2019-08-25T14:35:38.1435869Z 
2019-08-25T14:35:38.1436446Z For more information about this error, try `rustc --explain E0705`.
2019-08-25T14:35:38.1436495Z 
2019-08-25T14:35:38.1436495Z 
2019-08-25T14:35:38.1436775Z ------------------------------------------
2019-08-25T14:35:38.1436807Z 
2019-08-25T14:35:38.1436843Z 
2019-08-25T14:35:38.1437080Z ---- [ui] ui/rust-2018/edition-lint-paths.rs stdout ----
2019-08-25T14:35:38.1437253Z diff of stderr:
2019-08-25T14:35:38.1437281Z 
2019-08-25T14:35:38.1437327Z + warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-25T14:35:38.1437563Z +   --> $DIR/edition-lint-paths.rs:4:12
2019-08-25T14:35:38.1437607Z +    |
2019-08-25T14:35:38.1437649Z + LL | #![feature(rust_2018_preview)]
2019-08-25T14:35:38.1437746Z + 
2019-08-25T14:35:38.1437746Z + 
2019-08-25T14:35:38.1437795Z 1 error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-25T14:35:38.1438078Z 3    |
2019-08-25T14:35:38.1438103Z 
2019-08-25T14:35:38.1438140Z 68 
2019-08-25T14:35:38.1438190Z 69 error: aborting due to 7 previous errors
2019-08-25T14:35:38.1438190Z 69 error: aborting due to 7 previous errors
2019-08-25T14:35:38.1438246Z 70 
2019-08-25T14:35:38.1438486Z + For more information about this error, try `rustc --explain E0705`.
2019-08-25T14:35:38.1438638Z 71 
2019-08-25T14:35:38.1438680Z 
2019-08-25T14:35:38.1438705Z 
2019-08-25T14:35:38.1438747Z The actual stderr differed from the expected stderr.
2019-08-25T14:35:38.1439074Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-paths/edition-lint-paths.stderr
2019-08-25T14:35:38.1439336Z To update references, rerun the tests and pass the `--bless` flag
2019-08-25T14:35:38.1439598Z To only update this specific test, also pass `--test-args rust-2018/edition-lint-paths.rs`
2019-08-25T14:35:38.1439689Z error: 1 errors occurred comparing output.
2019-08-25T14:35:38.1439732Z status: exit code: 1
2019-08-25T14:35:38.1439732Z status: exit code: 1
2019-08-25T14:35:38.1440449Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/edition-lint-paths.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-paths" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-paths/auxiliary" "-A" "unused"
2019-08-25T14:35:38.1440785Z ------------------------------------------
2019-08-25T14:35:38.1440817Z 
2019-08-25T14:35:38.1441049Z ------------------------------------------
2019-08-25T14:35:38.1441093Z stderr:
2019-08-25T14:35:38.1441093Z stderr:
2019-08-25T14:35:38.1441304Z ------------------------------------------
2019-08-25T14:35:38.1441372Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-25T14:35:38.1442134Z    |
2019-08-25T14:35:38.1442134Z    |
2019-08-25T14:35:38.1442205Z LL | #![feature(rust_2018_preview)]
2019-08-25T14:35:38.1442285Z 
2019-08-25T14:35:38.1442285Z 
2019-08-25T14:35:38.1442351Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-25T14:35:38.1442725Z    |
2019-08-25T14:35:38.1442725Z    |
2019-08-25T14:35:38.1442783Z LL |     use ::bar::Bar;
2019-08-25T14:35:38.1442831Z    |         ^^^^^^^^^^ help: use `crate`: `crate::bar::Bar`
2019-08-25T14:35:38.1442915Z note: lint level defined here
2019-08-25T14:35:38.1443178Z   --> /checkout/src/test/ui/rust-2018/edition-lint-paths.rs:5:9
2019-08-25T14:35:38.1443226Z    |
2019-08-25T14:35:38.1443226Z    |
2019-08-25T14:35:38.1443270Z LL | #![deny(absolute_paths_not_starting_with_crate)]
2019-08-25T14:35:38.1443336Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-25T14:35:38.1443508Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-25T14:35:38.1443903Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-25T14:35:38.1443951Z 
2019-08-25T14:35:38.1444003Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-25T14:35:38.1444332Z    |
2019-08-25T14:35:38.1444373Z LL |     use bar;
2019-08-25T14:35:38.1444418Z    |         ^^^ help: use `crate`: `crate::bar`
2019-08-25T14:35:38.1444474Z    |
2019-08-25T14:35:38.1444474Z    |
2019-08-25T14:35:38.1444528Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-25T14:35:38.1444839Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-25T14:35:38.1444889Z 
2019-08-25T14:35:38.1444950Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-25T14:35:38.1445371Z    |
2019-08-25T14:35:38.1445371Z    |
2019-08-25T14:35:38.1445414Z LL |     use {Bar as SomethingElse, main};
2019-08-25T14:35:38.1445467Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `crate`: `crate::{Bar as SomethingElse, main}`
2019-08-25T14:35:38.1445527Z    |
2019-08-25T14:35:38.1445581Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-25T14:35:38.1445917Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-25T14:35:38.1445968Z 
2019-08-25T14:35:38.1446126Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-25T14:35:38.1446439Z    |
2019-08-25T14:35:38.1446487Z LL | use bar::Bar;
2019-08-25T14:35:38.1446487Z LL | use bar::Bar;
2019-08-25T14:35:38.1446532Z    |     ^^^^^^^^ help: use `crate`: `crate::bar::Bar`
2019-08-25T14:35:38.1446577Z    |
2019-08-25T14:35:38.1446656Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-25T14:35:38.1446985Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-25T14:35:38.1447036Z 
2019-08-25T14:35:38.1447091Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-25T14:35:38.1447413Z    |
2019-08-25T14:35:38.1447470Z LL |     use *;
2019-08-25T14:35:38.1447470Z LL |     use *;
2019-08-25T14:35:38.1447516Z    |         ^ help: use `crate`: `crate::*`
2019-08-25T14:35:38.1447560Z    |
2019-08-25T14:35:38.1447629Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-25T14:35:38.1447955Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-25T14:35:38.1447991Z 
2019-08-25T14:35:38.1448060Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-25T14:35:38.1448393Z    |
2019-08-25T14:35:38.1448393Z    |
2019-08-25T14:35:38.1448452Z LL | impl ::foo::SomeTrait for u32 { }
2019-08-25T14:35:38.1448500Z    |      ^^^^^^^^^^^^^^^^ help: use `crate`: `crate::foo::SomeTrait`
2019-08-25T14:35:38.1448544Z    |
2019-08-25T14:35:38.1448613Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-25T14:35:38.1448932Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-25T14:35:38.1448967Z 
2019-08-25T14:35:38.1449036Z error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
2019-08-25T14:35:38.1449565Z    |
2019-08-25T14:35:38.1449625Z LL |     let x = ::bar::Bar;
2019-08-25T14:35:38.1449625Z LL |     let x = ::bar::Bar;
2019-08-25T14:35:38.1449682Z    |             ^^^^^^^^^^ help: use `crate`: `crate::bar::Bar`
2019-08-25T14:35:38.1449727Z    |
2019-08-25T14:35:38.1449796Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-08-25T14:35:38.1450159Z    = note: for more information, see issue #53130 <***/issues/53130>
2019-08-25T14:35:38.1450258Z error: aborting due to 7 previous errors
2019-08-25T14:35:38.1450289Z 
2019-08-25T14:35:38.1450563Z For more information about this error, try `rustc --explain E0705`.
2019-08-25T14:35:38.1450599Z 
2019-08-25T14:35:38.1450599Z 
2019-08-25T14:35:38.1450851Z ------------------------------------------
2019-08-25T14:35:38.1450886Z 
2019-08-25T14:35:38.1450912Z 
2019-08-25T14:35:38.1451177Z ---- [ui] ui/rust-2018/extern-crate-idiomatic.rs stdout ----
2019-08-25T14:35:38.1451246Z normalized stderr:
2019-08-25T14:35:38.1451299Z warning[E0705]: the feature `rust_2018_preview` is included in the Rust 2018 edition
2019-08-25T14:35:38.1452050Z    |
2019-08-25T14:35:38.1452050Z    |
2019-08-25T14:35:38.1452114Z LL | #![feature(rust_2018_preview)]
2019-08-25T14:35:38.1452188Z 
2019-08-25T14:35:38.1452213Z 
2019-08-25T14:35:38.1452252Z 
2019-08-25T14:35:38.1452277Z 
2019-08-25T14:35:38.1452277Z 
2019-08-25T14:35:38.1452321Z The actual stderr differed from the expected stderr.
2019-08-25T14:35:38.1452649Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic/extern-crate-idiomatic.stderr
2019-08-25T14:35:38.1452925Z To update references, rerun the tests and pass the `--bless` flag
2019-08-25T14:35:38.1453205Z To only update this specific test, also pass `--test-args rust-2018/extern-crate-idiomatic.rs`
2019-08-25T14:35:38.1453313Z error: 1 errors occurred comparing output.
2019-08-25T14:35:38.1453356Z status: exit code: 0
2019-08-25T14:35:38.1453356Z status: exit code: 0
2019-08-25T14:35:38.1454134Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/extern-crate-idiomatic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern" "edition_lint_paths" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic/auxiliary" "-A" "unused"
2019-08-25T14:35:38.1454466Z ------------------------------------------
---
2019-08-25T14:35:38.1537258Z test result: FAILED. 8898 passed; 17 failed; 38 ignored; 0 measured; 0 filtered out
2019-08-25T14:35:38.1537306Z 
2019-08-25T14:35:38.1537330Z 
2019-08-25T14:35:38.1537355Z 
2019-08-25T14:35:38.1539055Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-25T14:35:38.1539416Z 
2019-08-25T14:35:38.1539449Z 
2019-08-25T14:35:38.1539827Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-25T14:35:38.1539890Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-25T14:35:38.1539890Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-25T14:35:38.1539959Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-25T14:35:38.1540029Z Build completed unsuccessfully in 1:07:57
2019-08-25T14:35:38.1540076Z == clock drift check ==
2019-08-25T14:35:38.1540131Z   local time: Sun Aug 25 14:35:38 UTC 2019
2019-08-25T14:35:38.3008817Z   network time: Sun, 25 Aug 2019 14:35:38 GMT
2019-08-25T14:35:38.3009486Z == end clock drift check ==
2019-08-25T14:35:39.0289183Z ##[error]Bash exited with code '1'.
2019-08-25T14:35:39.0335799Z ##[section]Starting: Checkout
2019-08-25T14:35:39.0337864Z ==============================================================================
2019-08-25T14:35:39.0337930Z Task         : Get sources
2019-08-25T14:35:39.0337971Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
