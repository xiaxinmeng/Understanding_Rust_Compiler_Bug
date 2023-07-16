plain
2019-07-21T15:21:51.0688850Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-21T15:21:51.0866260Z ##[command]git config gc.auto 0
2019-07-21T15:21:51.0934779Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-21T15:21:51.1003489Z ##[command]git config --get-all http.proxy
2019-07-21T15:21:51.1109524Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62829/merge:refs/remotes/pull/62829/merge
---
2019-07-21T15:22:24.4863515Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-21T15:22:24.4863541Z 
2019-07-21T15:22:24.4863728Z   git checkout -b <new-branch-name>
2019-07-21T15:22:24.4863753Z 
2019-07-21T15:22:24.4863818Z HEAD is now at 8851d095f Merge 4f28a2497bef3790c1ac357c7bce5b895b5b236e into 83dfe7b27cf2debecebedd3b038f9a1c2e05e051
2019-07-21T15:22:24.5014332Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-21T15:22:24.5016703Z ==============================================================================
2019-07-21T15:22:24.5016886Z Task         : Bash
2019-07-21T15:22:24.5016927Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-21T15:25:39.9074470Z ############################################                              61.8%
2019-07-21T15:25:39.9074976Z ######################################################################## 100.0%
2019-07-21T15:25:40.3147129Z extracting /checkout/obj/build/cache/2019-07-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-07-21T15:25:40.3944227Z     Updating crates.io index
2019-07-21T15:25:59.6134453Z     Updating git repository `https://github.com/gnzlbg/libtest`
---
2019-07-21T15:28:48.7315084Z tidy check
2019-07-21T15:28:49.6117859Z * 577 error codes
2019-07-21T15:28:49.6119115Z * highest error code: E0732
2019-07-21T15:28:49.8971581Z * 266 features
2019-07-21T15:28:50.4032568Z invalid source: "git+https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd286fd16c76bd61ddcbe162fca9f9d9d2"
2019-07-21T15:28:50.4485723Z + python2.7 ../x.py test
2019-07-21T15:28:50.6793508Z     Finished dev [unoptimized] target(s) in 0.18s
2019-07-21T15:28:51.8761298Z Building stage0 tool tidy (x86_64-unknown-linux-gnu)
2019-07-21T15:28:52.0622004Z     Finished release [optimized] target(s) in 0.18s
2019-07-21T15:28:52.0622004Z     Finished release [optimized] target(s) in 0.18s
2019-07-21T15:28:52.0664760Z tidy check
2019-07-21T15:28:52.8153802Z * 577 error codes
2019-07-21T15:28:52.8153905Z * highest error code: E0732
2019-07-21T15:28:53.0987209Z * 266 features
2019-07-21T15:28:53.6013842Z invalid source: "git+https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd286fd16c76bd61ddcbe162fca9f9d9d2"
2019-07-21T15:28:54.0321759Z    Compiling cc v1.0.35
2019-07-21T15:28:54.0322115Z    Compiling core v0.0.0 (/checkout/src/libcore)
2019-07-21T15:29:02.1175335Z    Compiling libc v0.2.54
2019-07-21T15:29:02.8768147Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
---
2019-07-21T15:29:42.2213203Z    Compiling unicode-width v0.1.5
2019-07-21T15:29:42.3364814Z    Compiling termcolor v1.0.4
2019-07-21T15:29:42.9791613Z    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
2019-07-21T15:29:43.5050211Z    Compiling getopts v0.2.19
2019-07-21T15:29:46.7877392Z    Compiling libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd)
2019-07-21T15:29:55.7319422Z     Finished release [optimized] target(s) in 13.78s
2019-07-21T15:29:55.7385198Z Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-07-21T15:29:55.7407998Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-21T15:29:56.1819857Z    Compiling semver-parser v0.7.0
---
2019-07-21T15:51:48.9323372Z    Compiling unicode-width v0.1.5
2019-07-21T15:51:49.0302111Z    Compiling termcolor v1.0.4
2019-07-21T15:51:49.8436813Z    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
2019-07-21T15:51:50.3250612Z    Compiling getopts v0.2.19
2019-07-21T15:51:54.0227831Z    Compiling libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd)
2019-07-21T15:52:05.3876286Z     Finished release [optimized] target(s) in 16.68s
2019-07-21T15:52:05.3941073Z Copying stage1 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-07-21T15:52:05.3963875Z Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-21T15:52:05.7991697Z    Compiling semver-parser v0.7.0
---
2019-07-21T16:19:00.7408089Z .................................................................................................... 200/5842
2019-07-21T16:19:04.7868096Z .................................................................................................... 300/5842
2019-07-21T16:19:08.3540266Z .................................................................................................... 400/5842
2019-07-21T16:19:11.9721679Z .................................................................................................... 500/5842
2019-07-21T16:19:15.5969757Z ........................................................................i........................... 600/5842
2019-07-21T16:19:24.1553525Z .................................................................................................... 800/5842
2019-07-21T16:19:29.4821105Z .................................................................................................... 900/5842
2019-07-21T16:19:34.2587660Z ...................................................................................................i 1000/5842
2019-07-21T16:19:34.2587660Z ...................................................................................................i 1000/5842
2019-07-21T16:19:39.5302423Z ..........Fi........................................................................................ 1100/5842
2019-07-21T16:19:43.3236880Z .............................iiiii.................................................................. 1200/5842
2019-07-21T16:19:49.1226111Z .................................................................................................... 1400/5842
2019-07-21T16:19:51.7163107Z .................................................................................................... 1500/5842
2019-07-21T16:19:55.3330556Z .................................................................................................... 1600/5842
2019-07-21T16:19:57.8857384Z .................................................................................................... 1700/5842
2019-07-21T16:19:57.8857384Z .................................................................................................... 1700/5842
2019-07-21T16:20:01.2163119Z .....................................................................i.............................. 1800/5842
2019-07-21T16:20:09.4781822Z .................................................................................................... 2000/5842
2019-07-21T16:20:13.5869964Z .....................................................................F.............................. 2100/5842
2019-07-21T16:20:13.5869964Z .....................................................................F.............................. 2100/5842
2019-07-21T16:20:17.3689481Z .................................................................F.F................................ 2200/5842
2019-07-21T16:20:20.8876421Z .........F..........................................i............................................... 2300/5842
2019-07-21T16:20:30.3029269Z .................................................................................................... 2500/5842
2019-07-21T16:20:34.2885895Z ...................................................................................................F 2600/5842
2019-07-21T16:20:39.2063301Z .................................................................................................... 2700/5842
2019-07-21T16:20:42.9483277Z .................................................................................................... 2800/5842
2019-07-21T16:20:42.9483277Z .................................................................................................... 2800/5842
2019-07-21T16:20:47.2122077Z .................................................................................................... 2900/5842
2019-07-21T16:20:52.1907699Z .................................................................................................... 3000/5842
2019-07-21T16:20:56.4642521Z .................................................................................................... 3100/5842
2019-07-21T16:21:01.3720995Z ....F............................................................................................... 3200/5842
2019-07-21T16:21:04.7459351Z .................................................................................................... 3300/5842
2019-07-21T16:21:08.2875878Z .................................................................................................... 3400/5842
2019-07-21T16:21:13.1953163Z ...........................................F........................................................ 3500/5842
2019-07-21T16:21:16.8384473Z ..................i................................................................................. 3600/5842
2019-07-21T16:21:20.8029337Z ............................................................................................ii...i.. 3700/5842
2019-07-21T16:21:24.6225699Z ii.................................................................................................. 3800/5842
2019-07-21T16:21:32.9271490Z .................................................................................................... 4000/5842
2019-07-21T16:21:32.9271490Z .................................................................................................... 4000/5842
2019-07-21T16:21:36.4929401Z ......ii............................................................................................ 4100/5842
2019-07-21T16:21:38.4598017Z ...........................i........................................................................ 4200/5842
2019-07-21T16:21:40.4233824Z .............................................................................................i...... 4300/5842
2019-07-21T16:21:47.2276150Z ...................................................................................................F 4500/5842
2019-07-21T16:22:04.0175689Z .................................................................................................... 4600/5842
2019-07-21T16:22:07.3482624Z .................................................................................................... 4700/5842
2019-07-21T16:22:10.9972107Z .................................................................................................... 4800/5842
2019-07-21T16:22:10.9972107Z .................................................................................................... 4800/5842
2019-07-21T16:22:15.3402452Z ............................................FF...F.................................................. 4900/5842
2019-07-21T16:22:30.4417599Z ..........................F......................................................................... 5100/5842
2019-07-21T16:22:34.7391640Z .................................................................................................... 5200/5842
2019-07-21T16:22:38.5077238Z .................................................................................................... 5300/5842
2019-07-21T16:22:38.5077238Z .................................................................................................... 5300/5842
2019-07-21T16:22:43.5154496Z ...............................................................FFFF................................. 5400/5842
2019-07-21T16:22:51.3663024Z .................................................................................................... 5600/5842
2019-07-21T16:22:54.4219412Z .................................................................................................... 5700/5842
2019-07-21T16:22:54.4219412Z .................................................................................................... 5700/5842
2019-07-21T16:22:57.2431592Z ..................................................................................i................. 5800/5842
2019-07-21T16:22:58.7308423Z failures:
2019-07-21T16:22:58.7367404Z 
2019-07-21T16:22:58.7369230Z ---- [ui] ui/custom_test_frameworks/mismatch.rs stdout ----
2019-07-21T16:22:58.7369693Z diff of stderr:
2019-07-21T16:22:58.7369693Z diff of stderr:
2019-07-21T16:22:58.7370426Z 
2019-07-21T16:22:58.7371142Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7371556Z +   --> $DIR/mismatch.rs:9:1
2019-07-21T16:22:58.7371719Z +    |
2019-07-21T16:22:58.7371866Z + LL | fn wrong_kind(){}
2019-07-21T16:22:58.7372096Z +    |
2019-07-21T16:22:58.7372096Z +    |
2019-07-21T16:22:58.7372602Z +    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7372931Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7373634Z + 
2019-07-21T16:22:58.7373809Z 1 error[E0277]: the trait bound `test::TestDescAndFn: example_runner::Testable` is not satisfied
2019-07-21T16:22:58.7374428Z 3    |
2019-07-21T16:22:58.7374550Z 
2019-07-21T16:22:58.7374680Z 6    |
2019-07-21T16:22:58.7374851Z 7    = note: required for the cast to the object type `dyn example_runner::Testable`
---
2019-07-21T16:22:58.7377196Z 12 
2019-07-21T16:22:58.7377306Z 
2019-07-21T16:22:58.7377535Z 
2019-07-21T16:22:58.7377668Z The actual stderr differed from the expected stderr.
2019-07-21T16:22:58.7378230Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/mismatch/mismatch.stderr
2019-07-21T16:22:58.7378654Z To update references, rerun the tests and pass the `--bless` flag
2019-07-21T16:22:58.7379108Z To only update this specific test, also pass `--test-args custom_test_frameworks/mismatch.rs`
2019-07-21T16:22:58.7379414Z error: 1 errors occurred comparing output.
2019-07-21T16:22:58.7379550Z status: exit code: 1
2019-07-21T16:22:58.7379550Z status: exit code: 1
2019-07-21T16:22:58.7380552Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/custom_test_frameworks/mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/mismatch" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/mismatch/auxiliary" "-A" "unused"
2019-07-21T16:22:58.7381549Z ------------------------------------------
2019-07-21T16:22:58.7381699Z 
2019-07-21T16:22:58.7382044Z ------------------------------------------
2019-07-21T16:22:58.7382202Z stderr:
2019-07-21T16:22:58.7382202Z stderr:
2019-07-21T16:22:58.7382504Z ------------------------------------------
2019-07-21T16:22:58.7383679Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7384442Z    |
2019-07-21T16:22:58.7384442Z    |
2019-07-21T16:22:58.7384585Z LL | fn wrong_kind(){}
2019-07-21T16:22:58.7384863Z    |
2019-07-21T16:22:58.7384863Z    |
2019-07-21T16:22:58.7385309Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7385526Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7385649Z 
2019-07-21T16:22:58.7385812Z error[E0277]: the trait bound `test::TestDescAndFn: example_runner::Testable` is not satisfied
2019-07-21T16:22:58.7386558Z    |
2019-07-21T16:22:58.7386558Z    |
2019-07-21T16:22:58.7386701Z LL | fn wrong_kind(){}
2019-07-21T16:22:58.7386971Z    | ^^^^^^^^^^^^^^^^^ the trait `example_runner::Testable` is not implemented for `test::TestDescAndFn`
2019-07-21T16:22:58.7387217Z    = note: required for the cast to the object type `dyn example_runner::Testable`
2019-07-21T16:22:58.7387339Z 
2019-07-21T16:22:58.7387468Z error: aborting due to 2 previous errors
2019-07-21T16:22:58.7387570Z 
---
2019-07-21T16:22:58.7388859Z 
2019-07-21T16:22:58.7389173Z ---- [ui] ui/inaccessible-test-modules.rs stdout ----
2019-07-21T16:22:58.7389353Z diff of stderr:
2019-07-21T16:22:58.7389463Z 
2019-07-21T16:22:58.7389582Z 16    |     no `__test_reexports` in the root
2019-07-21T16:22:58.7389722Z 17    |     help: a similar name exists in the module: `__test_reexports`
2019-07-21T16:22:58.7390155Z - error: aborting due to 2 previous errors
2019-07-21T16:22:58.7390155Z - error: aborting due to 2 previous errors
2019-07-21T16:22:58.7390651Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7391033Z +   --> $DIR/inaccessible-test-modules.rs:9:1
2019-07-21T16:22:58.7392435Z + LL | fn baz() {}
2019-07-21T16:22:58.7392595Z +    | ^^^^^^^^^^^
2019-07-21T16:22:58.7392734Z +    |
2019-07-21T16:22:58.7392734Z +    |
2019-07-21T16:22:58.7394065Z +    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7394440Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7395068Z - For more information about this error, try `rustc --explain E0432`.
2019-07-21T16:22:58.7395068Z - For more information about this error, try `rustc --explain E0432`.
2019-07-21T16:22:58.7395798Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7396035Z +    |
2019-07-21T16:22:58.7396624Z +    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7397209Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7397478Z + error: aborting due to 4 previous errors
2019-07-21T16:22:58.7397599Z + 
2019-07-21T16:22:58.7397748Z + Some errors have detailed explanations: E0432, E0658.
2019-07-21T16:22:58.7398129Z + For more information about an error, try `rustc --explain E0432`.
2019-07-21T16:22:58.7398129Z + For more information about an error, try `rustc --explain E0432`.
2019-07-21T16:22:58.7398296Z 22 
2019-07-21T16:22:58.7398424Z 
2019-07-21T16:22:58.7398525Z 
2019-07-21T16:22:58.7398656Z The actual stderr differed from the expected stderr.
2019-07-21T16:22:58.7399074Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inaccessible-test-modules/inaccessible-test-modules.stderr
2019-07-21T16:22:58.7399483Z To update references, rerun the tests and pass the `--bless` flag
2019-07-21T16:22:58.7399877Z To only update this specific test, also pass `--test-args inaccessible-test-modules.rs`
2019-07-21T16:22:58.7400162Z error: 1 errors occurred comparing output.
2019-07-21T16:22:58.7400304Z status: exit code: 1
2019-07-21T16:22:58.7400304Z status: exit code: 1
2019-07-21T16:22:58.7401161Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inaccessible-test-modules.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inaccessible-test-modules" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inaccessible-test-modules/auxiliary" "-A" "unused"
2019-07-21T16:22:58.7402890Z ------------------------------------------
2019-07-21T16:22:58.7403947Z 
2019-07-21T16:22:58.7410454Z ------------------------------------------
2019-07-21T16:22:58.7410861Z stderr:
2019-07-21T16:22:58.7410861Z stderr:
2019-07-21T16:22:58.7411197Z ------------------------------------------
2019-07-21T16:22:58.7411254Z error[E0432]: unresolved import `__test`
2019-07-21T16:22:58.7411470Z   --> /checkout/src/test/ui/inaccessible-test-modules.rs:5:5
2019-07-21T16:22:58.7411531Z    |
2019-07-21T16:22:58.7411576Z LL | use __test as x; //~ ERROR unresolved import `__test`
2019-07-21T16:22:58.7411807Z    |     |
2019-07-21T16:22:58.7411807Z    |     |
2019-07-21T16:22:58.7411845Z    |     no `__test` in the root
2019-07-21T16:22:58.7411894Z    |     help: a similar name exists in the module: `test`
2019-07-21T16:22:58.7411922Z 
2019-07-21T16:22:58.7411975Z error[E0432]: unresolved import `__test_reexports`
2019-07-21T16:22:58.7412228Z    |
2019-07-21T16:22:58.7412228Z    |
2019-07-21T16:22:58.7412286Z LL | use __test_reexports as y; //~ ERROR unresolved import `__test_reexports`
2019-07-21T16:22:58.7412569Z    |     |
2019-07-21T16:22:58.7412569Z    |     |
2019-07-21T16:22:58.7412622Z    |     no `__test_reexports` in the root
2019-07-21T16:22:58.7412674Z    |     help: a similar name exists in the module: `__test_reexports`
2019-07-21T16:22:58.7412701Z 
2019-07-21T16:22:58.7413720Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7414104Z    |
2019-07-21T16:22:58.7414157Z LL | fn baz() {}
2019-07-21T16:22:58.7414217Z    | ^^^^^^^^^^^
2019-07-21T16:22:58.7414257Z    |
2019-07-21T16:22:58.7414257Z    |
2019-07-21T16:22:58.7414583Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7414646Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7414681Z 
2019-07-21T16:22:58.7415097Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7415323Z    |
2019-07-21T16:22:58.7415692Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7415768Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7415849Z error: aborting due to 4 previous errors
2019-07-21T16:22:58.7415880Z 
2019-07-21T16:22:58.7415938Z Some errors have detailed explanations: E0432, E0658.
2019-07-21T16:22:58.7416232Z For more information about an error, try `rustc --explain E0432`.
2019-07-21T16:22:58.7416232Z For more information about an error, try `rustc --explain E0432`.
2019-07-21T16:22:58.7416269Z 
2019-07-21T16:22:58.7416687Z ------------------------------------------
2019-07-21T16:22:58.7416719Z 
2019-07-21T16:22:58.7416868Z 
2019-07-21T16:22:58.7417071Z ---- [ui] ui/issues/issue-12997-1.rs stdout ----
2019-07-21T16:22:58.7417127Z diff of stderr:
2019-07-21T16:22:58.7417151Z 
2019-07-21T16:22:58.7417188Z 10 LL | fn bar(x: isize, y: isize) { }
2019-07-21T16:22:58.7417285Z 12 
2019-07-21T16:22:58.7417481Z - error: aborting due to 2 previous errors
2019-07-21T16:22:58.7417481Z - error: aborting due to 2 previous errors
2019-07-21T16:22:58.7417809Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7417878Z +    |
2019-07-21T16:22:58.7418128Z +    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7418278Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7418368Z + error: aborting due to 3 previous errors
2019-07-21T16:22:58.7418416Z + 
2019-07-21T16:22:58.7418670Z + For more information about this error, try `rustc --explain E0658`.
2019-07-21T16:22:58.7418712Z 15 
2019-07-21T16:22:58.7418712Z 15 
2019-07-21T16:22:58.7418736Z 
2019-07-21T16:22:58.7418757Z 
2019-07-21T16:22:58.7418810Z The actual stderr differed from the expected stderr.
2019-07-21T16:22:58.7419081Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-1/issue-12997-1.stderr
2019-07-21T16:22:58.7419300Z To update references, rerun the tests and pass the `--bless` flag
2019-07-21T16:22:58.7419547Z To only update this specific test, also pass `--test-args issues/issue-12997-1.rs`
2019-07-21T16:22:58.7419614Z error: 1 errors occurred comparing output.
2019-07-21T16:22:58.7419668Z status: exit code: 1
2019-07-21T16:22:58.7419668Z status: exit code: 1
2019-07-21T16:22:58.7420308Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12997-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-1/auxiliary" "-A" "unused"
2019-07-21T16:22:58.7420598Z ------------------------------------------
2019-07-21T16:22:58.7420628Z 
2019-07-21T16:22:58.7420829Z ------------------------------------------
2019-07-21T16:22:58.7420868Z stderr:
2019-07-21T16:22:58.7420868Z stderr:
2019-07-21T16:22:58.7421054Z ------------------------------------------
2019-07-21T16:22:58.7421295Z error: functions used as benches must have signature `fn(&mut Bencher) -> impl Termination`
2019-07-21T16:22:58.7421515Z   --> /checkout/src/test/ui/issues/issue-12997-1.rs:6:1
2019-07-21T16:22:58.7421557Z    |
2019-07-21T16:22:58.7421609Z LL | fn foo() { } //~ ERROR functions used as benches
2019-07-21T16:22:58.7421674Z 
2019-07-21T16:22:58.7421674Z 
2019-07-21T16:22:58.7421904Z error: functions used as benches must have signature `fn(&mut Bencher) -> impl Termination`
2019-07-21T16:22:58.7422128Z   --> /checkout/src/test/ui/issues/issue-12997-1.rs:9:1
2019-07-21T16:22:58.7422168Z    |
2019-07-21T16:22:58.7422288Z LL | fn bar(x: isize, y: isize) { } //~ ERROR functions used as benches
2019-07-21T16:22:58.7422369Z 
2019-07-21T16:22:58.7422369Z 
2019-07-21T16:22:58.7422722Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7422908Z    |
2019-07-21T16:22:58.7423854Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7423919Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7424011Z error: aborting due to 3 previous errors
2019-07-21T16:22:58.7424039Z 
2019-07-21T16:22:58.7424303Z For more information about this error, try `rustc --explain E0658`.
2019-07-21T16:22:58.7424350Z 
2019-07-21T16:22:58.7424350Z 
2019-07-21T16:22:58.7424567Z ------------------------------------------
2019-07-21T16:22:58.7425531Z 
2019-07-21T16:22:58.7425582Z 
2019-07-21T16:22:58.7425919Z ---- [ui] ui/issues/issue-12997-2.rs stdout ----
2019-07-21T16:22:58.7425969Z diff of stderr:
2019-07-21T16:22:58.7425997Z 
2019-07-21T16:22:58.7426372Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7426617Z +   --> $DIR/issue-12997-2.rs:6:1
2019-07-21T16:22:58.7426662Z +    |
2019-07-21T16:22:58.7427041Z + LL | fn bar(x: isize) { }
2019-07-21T16:22:58.7427142Z +    |
2019-07-21T16:22:58.7427142Z +    |
2019-07-21T16:22:58.7427439Z +    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7427509Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7427549Z + 
2019-07-21T16:22:58.7427885Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7427958Z +    |
2019-07-21T16:22:58.7428205Z +    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7428253Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7428824Z 1 error[E0308]: mismatched types
2019-07-21T16:22:58.7429647Z 2   --> $DIR/issue-12997-2.rs:6:1
2019-07-21T16:22:58.7429734Z 3    |
2019-07-21T16:22:58.7429759Z 
---
2019-07-21T16:22:58.7431148Z 13 
2019-07-21T16:22:58.7431172Z 
2019-07-21T16:22:58.7431195Z 
2019-07-21T16:22:58.7431247Z The actual stderr differed from the expected stderr.
2019-07-21T16:22:58.7431507Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-2/issue-12997-2.stderr
2019-07-21T16:22:58.7431731Z To update references, rerun the tests and pass the `--bless` flag
2019-07-21T16:22:58.7431973Z To only update this specific test, also pass `--test-args issues/issue-12997-2.rs`
2019-07-21T16:22:58.7432041Z error: 1 errors occurred comparing output.
2019-07-21T16:22:58.7432079Z status: exit code: 1
2019-07-21T16:22:58.7432079Z status: exit code: 1
2019-07-21T16:22:58.7432722Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12997-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-2/auxiliary" "-A" "unused"
2019-07-21T16:22:58.7433963Z ------------------------------------------
2019-07-21T16:22:58.7434001Z 
2019-07-21T16:22:58.7434225Z ------------------------------------------
2019-07-21T16:22:58.7434285Z stderr:
2019-07-21T16:22:58.7434285Z stderr:
2019-07-21T16:22:58.7434502Z ------------------------------------------
2019-07-21T16:22:58.7434876Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7435196Z   --> /checkout/src/test/ui/issues/issue-12997-2.rs:6:1
2019-07-21T16:22:58.7435256Z    |
2019-07-21T16:22:58.7435298Z LL | fn bar(x: isize) { }
2019-07-21T16:22:58.7435395Z    |
2019-07-21T16:22:58.7435395Z    |
2019-07-21T16:22:58.7435700Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7435775Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7435810Z 
2019-07-21T16:22:58.7436332Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7436538Z    |
2019-07-21T16:22:58.7436824Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7436874Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7436953Z error[E0308]: mismatched types
2019-07-21T16:22:58.7437169Z   --> /checkout/src/test/ui/issues/issue-12997-2.rs:6:1
2019-07-21T16:22:58.7437231Z    |
2019-07-21T16:22:58.7437231Z    |
2019-07-21T16:22:58.7437267Z LL | fn bar(x: isize) { }
2019-07-21T16:22:58.7437308Z    | ^^^^^^^^^^^^^^^^^^^^ expected isize, found mutable reference
2019-07-21T16:22:58.7437396Z    = note: expected type `isize`
2019-07-21T16:22:58.7437436Z               found type `&mut test::Bencher`
2019-07-21T16:22:58.7437462Z 
2019-07-21T16:22:58.7437513Z error: aborting due to 3 previous errors
---
2019-07-21T16:22:58.7438079Z 
2019-07-21T16:22:58.7438286Z ---- [ui] ui/issues/issue-14772.rs stdout ----
2019-07-21T16:22:58.7438328Z diff of stderr:
2019-07-21T16:22:58.7438351Z 
2019-07-21T16:22:58.7438393Z 4 LL | mod foo {}
2019-07-21T16:22:58.7438478Z 6 
2019-07-21T16:22:58.7438667Z - error: aborting due to previous error
2019-07-21T16:22:58.7438667Z - error: aborting due to previous error
2019-07-21T16:22:58.7439010Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7439064Z +    |
2019-07-21T16:22:58.7439318Z +    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7439383Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7439462Z + error: aborting due to 2 previous errors
2019-07-21T16:22:58.7439511Z + 
2019-07-21T16:22:58.7439735Z + For more information about this error, try `rustc --explain E0658`.
2019-07-21T16:22:58.7439774Z 9 
2019-07-21T16:22:58.7439774Z 9 
2019-07-21T16:22:58.7439810Z 
2019-07-21T16:22:58.7439832Z 
2019-07-21T16:22:58.7439870Z The actual stderr differed from the expected stderr.
2019-07-21T16:22:58.7440235Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14772/issue-14772.stderr
2019-07-21T16:22:58.7440473Z To update references, rerun the tests and pass the `--bless` flag
2019-07-21T16:22:58.7440703Z To only update this specific test, also pass `--test-args issues/issue-14772.rs`
2019-07-21T16:22:58.7440784Z error: 1 errors occurred comparing output.
2019-07-21T16:22:58.7440830Z status: exit code: 1
2019-07-21T16:22:58.7440830Z status: exit code: 1
2019-07-21T16:22:58.7441454Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-14772.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14772" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14772/auxiliary" "-A" "unused"
2019-07-21T16:22:58.7441747Z ------------------------------------------
2019-07-21T16:22:58.7441789Z 
2019-07-21T16:22:58.7441979Z ------------------------------------------
2019-07-21T16:22:58.7442018Z stderr:
2019-07-21T16:22:58.7442018Z stderr:
2019-07-21T16:22:58.7442204Z ------------------------------------------
2019-07-21T16:22:58.7442260Z error: only functions may be used as tests
2019-07-21T16:22:58.7442528Z   --> /checkout/src/test/ui/issues/issue-14772.rs:4:1
2019-07-21T16:22:58.7442577Z    |
2019-07-21T16:22:58.7442632Z LL | mod foo {} //~ ERROR only functions may be used as tests
2019-07-21T16:22:58.7442701Z 
2019-07-21T16:22:58.7442701Z 
2019-07-21T16:22:58.7443788Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7443867Z    |
2019-07-21T16:22:58.7444219Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7444275Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7444351Z error: aborting due to 2 previous errors
2019-07-21T16:22:58.7444392Z 
2019-07-21T16:22:58.7444650Z For more information about this error, try `rustc --explain E0658`.
2019-07-21T16:22:58.7444683Z 
---
2019-07-21T16:22:58.7445271Z 
2019-07-21T16:22:58.7445327Z 4 LL | #![test]
2019-07-21T16:22:58.7445367Z 5    | ^^^^^^^^
2019-07-21T16:22:58.7445406Z 6 
2019-07-21T16:22:58.7445638Z - error: aborting due to previous error
2019-07-21T16:22:58.7446015Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7446082Z +    |
2019-07-21T16:22:58.7446379Z +    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7446436Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7446650Z + error: aborting due to 2 previous errors
2019-07-21T16:22:58.7446697Z + 
2019-07-21T16:22:58.7446964Z + For more information about this error, try `rustc --explain E0658`.
2019-07-21T16:22:58.7447008Z 9 
2019-07-21T16:22:58.7447008Z 9 
2019-07-21T16:22:58.7447034Z 
2019-07-21T16:22:58.7447058Z 
2019-07-21T16:22:58.7447100Z The actual stderr differed from the expected stderr.
2019-07-21T16:22:58.7447398Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28134/issue-28134.stderr
2019-07-21T16:22:58.7447640Z To update references, rerun the tests and pass the `--bless` flag
2019-07-21T16:22:58.7448068Z To only update this specific test, also pass `--test-args issues/issue-28134.rs`
2019-07-21T16:22:58.7448144Z error: 1 errors occurred comparing output.
2019-07-21T16:22:58.7448186Z status: exit code: 1
2019-07-21T16:22:58.7448186Z status: exit code: 1
2019-07-21T16:22:58.7449458Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-28134.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28134" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28134/auxiliary" "-A" "unused"
2019-07-21T16:22:58.7449753Z ------------------------------------------
2019-07-21T16:22:58.7449782Z 
2019-07-21T16:22:58.7449981Z ------------------------------------------
2019-07-21T16:22:58.7450033Z stderr:
2019-07-21T16:22:58.7450033Z stderr:
2019-07-21T16:22:58.7450221Z ------------------------------------------
2019-07-21T16:22:58.7450263Z error: only functions may be used as tests
2019-07-21T16:22:58.7450477Z   --> /checkout/src/test/ui/issues/issue-28134.rs:3:1
2019-07-21T16:22:58.7450517Z    |
2019-07-21T16:22:58.7450557Z LL | #![test] //~ ERROR only functions may be used as tests
2019-07-21T16:22:58.7450641Z 
2019-07-21T16:22:58.7450641Z 
2019-07-21T16:22:58.7451056Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7451118Z    |
2019-07-21T16:22:58.7451434Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7451483Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7451571Z error: aborting due to 2 previous errors
2019-07-21T16:22:58.7451595Z 
2019-07-21T16:22:58.7451821Z For more information about this error, try `rustc --explain E0658`.
2019-07-21T16:22:58.7451850Z 
2019-07-21T16:22:58.7451850Z 
2019-07-21T16:22:58.7452054Z ------------------------------------------
2019-07-21T16:22:58.7452082Z 
2019-07-21T16:22:58.7452103Z 
2019-07-21T16:22:58.7452312Z ---- [ui] ui/issues/issue-53675-a-test-called-panic.rs stdout ----
2019-07-21T16:22:58.7452354Z 
2019-07-21T16:22:58.7452558Z error: test compilation failed although it shouldn't!
2019-07-21T16:22:58.7452599Z status: exit code: 1
2019-07-21T16:22:58.7454031Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-53675-a-test-called-panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-53675-a-test-called-panic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-53675-a-test-called-panic/auxiliary" "-A" "unused"
2019-07-21T16:22:58.7454385Z ------------------------------------------
2019-07-21T16:22:58.7454418Z 
2019-07-21T16:22:58.7454636Z ------------------------------------------
2019-07-21T16:22:58.7454698Z stderr:
2019-07-21T16:22:58.7454698Z stderr:
2019-07-21T16:22:58.7454921Z ------------------------------------------
2019-07-21T16:22:58.7455295Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7455368Z    |
2019-07-21T16:22:58.7455665Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7455721Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7455887Z 
2019-07-21T16:22:58.7456297Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7456742Z    |
2019-07-21T16:22:58.7456778Z LL | /     fn panic() {
2019-07-21T16:22:58.7456815Z LL | |         assert!(true)
2019-07-21T16:22:58.7456874Z LL | |     }
2019-07-21T16:22:58.7456874Z LL | |     }
2019-07-21T16:22:58.7456910Z    | |_____^
2019-07-21T16:22:58.7456944Z    |
2019-07-21T16:22:58.7457214Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7457263Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7457291Z 
2019-07-21T16:22:58.7457620Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7457921Z    |
2019-07-21T16:22:58.7457957Z LL | /     fn panic() {
2019-07-21T16:22:58.7457957Z LL | /     fn panic() {
2019-07-21T16:22:58.7458007Z LL | |         assert!(true);
2019-07-21T16:22:58.7458079Z    | |_____^
2019-07-21T16:22:58.7458127Z    |
2019-07-21T16:22:58.7458127Z    |
2019-07-21T16:22:58.7458373Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7458494Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7458544Z 
2019-07-21T16:22:58.7458900Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7459188Z    |
2019-07-21T16:22:58.7459233Z LL | /     fn panic() {
2019-07-21T16:22:58.7459233Z LL | /     fn panic() {
2019-07-21T16:22:58.7459271Z LL | |         panic!("in expr")
2019-07-21T16:22:58.7459358Z    | |_____^
2019-07-21T16:22:58.7459392Z    |
2019-07-21T16:22:58.7459392Z    |
2019-07-21T16:22:58.7459661Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7459710Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7459738Z 
2019-07-21T16:22:58.7460077Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7460365Z    |
2019-07-21T16:22:58.7460400Z LL | /     fn panic() {
2019-07-21T16:22:58.7460400Z LL | /     fn panic() {
2019-07-21T16:22:58.7460451Z LL | |         panic!("in stmt");
2019-07-21T16:22:58.7460522Z    | |_____^
2019-07-21T16:22:58.7460576Z    |
2019-07-21T16:22:58.7460576Z    |
2019-07-21T16:22:58.7460822Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7460869Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7460946Z error: aborting due to 5 previous errors
2019-07-21T16:22:58.7460971Z 
2019-07-21T16:22:58.7461190Z For more information about this error, try `rustc --explain E0658`.
2019-07-21T16:22:58.7461232Z 
---
2019-07-21T16:22:58.7461720Z diff of stderr:
2019-07-21T16:22:58.7461744Z 
2019-07-21T16:22:58.7461923Z - error: cannot test inner items
2019-07-21T16:22:58.7462122Z -   --> $DIR/test-inner-fn.rs:5:5
2019-07-21T16:22:58.7462451Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7462874Z +   --> $DIR/test-inner-fn.rs:4:1
2019-07-21T16:22:58.7462934Z 3    |
2019-07-21T16:22:58.7463687Z - LL |     #[test]
2019-07-21T16:22:58.7463959Z + LL | / fn foo() {
2019-07-21T16:22:58.7464001Z + LL | |     #[test]
2019-07-21T16:22:58.7464001Z + LL | |     #[test]
2019-07-21T16:22:58.7464044Z + LL | |     fn bar() {}
2019-07-21T16:22:58.7464100Z + LL | |     bar();
2019-07-21T16:22:58.7464153Z + LL | | }
2019-07-21T16:22:58.7464234Z 6    |
2019-07-21T16:22:58.7464234Z 6    |
2019-07-21T16:22:58.7464499Z -    = note: requested on the command line with `-D unnameable-test-items`
2019-07-21T16:22:58.7464804Z +    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7464878Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7465141Z - error: cannot test inner items
2019-07-21T16:22:58.7465366Z -   --> $DIR/test-inner-fn.rs:13:9
2019-07-21T16:22:58.7465366Z -   --> $DIR/test-inner-fn.rs:13:9
2019-07-21T16:22:58.7465759Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7465987Z +   --> $DIR/test-inner-fn.rs:6:5
2019-07-21T16:22:58.7466049Z 11    |
2019-07-21T16:22:58.7466246Z - LL |         #[test]
2019-07-21T16:22:58.7466444Z -    |         ^^^^^^^
2019-07-21T16:22:58.7466591Z + LL |     fn bar() {}
2019-07-21T16:22:58.7466702Z +    |
2019-07-21T16:22:58.7466702Z +    |
2019-07-21T16:22:58.7467135Z +    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7467185Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7467426Z - error: aborting due to 2 previous errors
2019-07-21T16:22:58.7467426Z - error: aborting due to 2 previous errors
2019-07-21T16:22:58.7467768Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7467828Z +    |
2019-07-21T16:22:58.7468087Z +    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7468136Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7468174Z 16 
2019-07-21T16:22:58.7468528Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7468730Z +   --> $DIR/test-inner-fn.rs:12:5
2019-07-21T16:22:58.7468818Z + LL | /     fn foo() {
2019-07-21T16:22:58.7468855Z + LL | |         #[test]
2019-07-21T16:22:58.7468855Z + LL | |         #[test]
2019-07-21T16:22:58.7468892Z + LL | |         fn bar() {}
2019-07-21T16:22:58.7468929Z + LL | |         bar();
2019-07-21T16:22:58.7468979Z + LL | |     }
2019-07-21T16:22:58.7469055Z +    |
2019-07-21T16:22:58.7469055Z +    |
2019-07-21T16:22:58.7469318Z +    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7469367Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7469405Z + 
2019-07-21T16:22:58.7469757Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7469957Z +   --> $DIR/test-inner-fn.rs:14:9
2019-07-21T16:22:58.7469995Z +    |
2019-07-21T16:22:58.7470044Z + LL |         fn bar() {}
2019-07-21T16:22:58.7470115Z +    |
2019-07-21T16:22:58.7470115Z +    |
2019-07-21T16:22:58.7470375Z +    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7470423Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7470587Z + error: aborting due to 5 previous errors
2019-07-21T16:22:58.7508240Z + 
2019-07-21T16:22:58.7508634Z + For more information about this error, try `rustc --explain E0658`.
2019-07-21T16:22:58.7508678Z 17 
2019-07-21T16:22:58.7508678Z 17 
2019-07-21T16:22:58.7508714Z 
2019-07-21T16:22:58.7508737Z 
2019-07-21T16:22:58.7508777Z The actual stderr differed from the expected stderr.
2019-07-21T16:22:58.7509052Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/test-inner-fn/test-inner-fn.stderr
2019-07-21T16:22:58.7509293Z To update references, rerun the tests and pass the `--bless` flag
2019-07-21T16:22:58.7509887Z To only update this specific test, also pass `--test-args lint/test-inner-fn.rs`
2019-07-21T16:22:58.7509971Z error: 1 errors occurred comparing output.
2019-07-21T16:22:58.7510013Z status: exit code: 1
2019-07-21T16:22:58.7510013Z status: exit code: 1
2019-07-21T16:22:58.7510934Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/test-inner-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/test-inner-fn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-D" "unnameable_test_items" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/test-inner-fn/auxiliary" "-A" "unused"
2019-07-21T16:22:58.7511399Z ------------------------------------------
2019-07-21T16:22:58.7511450Z 
2019-07-21T16:22:58.7511709Z ------------------------------------------
2019-07-21T16:22:58.7511751Z stderr:
2019-07-21T16:22:58.7511751Z stderr:
2019-07-21T16:22:58.7511961Z ------------------------------------------
2019-07-21T16:22:58.7512319Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7512617Z    |
2019-07-21T16:22:58.7512654Z LL | / fn foo() {
2019-07-21T16:22:58.7512654Z LL | / fn foo() {
2019-07-21T16:22:58.7512699Z LL | |     #[test] //~ ERROR cannot test inner items [unnameable_test_items]
2019-07-21T16:22:58.7512861Z LL | |     fn bar() {}
2019-07-21T16:22:58.7512908Z LL | |     bar();
2019-07-21T16:22:58.7512984Z    | |_^
2019-07-21T16:22:58.7513414Z    |
2019-07-21T16:22:58.7513414Z    |
2019-07-21T16:22:58.7513867Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7513927Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7513971Z 
2019-07-21T16:22:58.7514364Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7514675Z    |
2019-07-21T16:22:58.7514675Z    |
2019-07-21T16:22:58.7514723Z LL |     fn bar() {}
2019-07-21T16:22:58.7514806Z    |
2019-07-21T16:22:58.7514806Z    |
2019-07-21T16:22:58.7515105Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7515162Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7515193Z 
2019-07-21T16:22:58.7515584Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7515655Z    |
2019-07-21T16:22:58.7515933Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7516002Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7516035Z 
2019-07-21T16:22:58.7516531Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7517073Z    |
2019-07-21T16:22:58.7517109Z LL | /     fn foo() {
2019-07-21T16:22:58.7517109Z LL | /     fn foo() {
2019-07-21T16:22:58.7517152Z LL | |         #[test] //~ ERROR cannot test inner items [unnameable_test_items]
2019-07-21T16:22:58.7517204Z LL | |         fn bar() {}
2019-07-21T16:22:58.7517241Z LL | |         bar();
2019-07-21T16:22:58.7517327Z    | |_____^
2019-07-21T16:22:58.7517361Z    |
2019-07-21T16:22:58.7517361Z    |
2019-07-21T16:22:58.7517617Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7517674Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7517702Z 
2019-07-21T16:22:58.7518033Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7518306Z    |
2019-07-21T16:22:58.7518306Z    |
2019-07-21T16:22:58.7518341Z LL |         fn bar() {}
2019-07-21T16:22:58.7518425Z    |
2019-07-21T16:22:58.7518425Z    |
2019-07-21T16:22:58.7518666Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7518724Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7518866Z error: aborting due to 5 previous errors
2019-07-21T16:22:58.7518899Z 
2019-07-21T16:22:58.7519164Z For more information about this error, try `rustc --explain E0658`.
2019-07-21T16:22:58.7519194Z 
---
2019-07-21T16:22:58.7519718Z 
2019-07-21T16:22:58.7519775Z 4 LL |     NonExistent;
2019-07-21T16:22:58.7519815Z 5    |     ^^^^^^^^^^^ not found in this scope
2019-07-21T16:22:58.7519851Z 6 
2019-07-21T16:22:58.7520042Z - error: aborting due to previous error
2019-07-21T16:22:58.7520378Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7520431Z +    |
2019-07-21T16:22:58.7520694Z +    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7520745Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7521018Z - For more information about this error, try `rustc --explain E0425`.
2019-07-21T16:22:58.7521062Z + error: aborting due to 2 previous errors
2019-07-21T16:22:58.7521098Z + 
2019-07-21T16:22:58.7521144Z + Some errors have detailed explanations: E0425, E0658.
2019-07-21T16:22:58.7521144Z + Some errors have detailed explanations: E0425, E0658.
2019-07-21T16:22:58.7521370Z + For more information about an error, try `rustc --explain E0425`.
2019-07-21T16:22:58.7521409Z 10 
2019-07-21T16:22:58.7521433Z 
2019-07-21T16:22:58.7521467Z 
2019-07-21T16:22:58.7521505Z The actual stderr differed from the expected stderr.
2019-07-21T16:22:58.7521789Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs-test/ambiguous-builtin-attrs-test.stderr
2019-07-21T16:22:58.7522030Z To update references, rerun the tests and pass the `--bless` flag
2019-07-21T16:22:58.7522296Z To only update this specific test, also pass `--test-args proc-macro/ambiguous-builtin-attrs-test.rs`
2019-07-21T16:22:58.7522379Z error: 1 errors occurred comparing output.
2019-07-21T16:22:58.7522419Z status: exit code: 1
2019-07-21T16:22:58.7522419Z status: exit code: 1
2019-07-21T16:22:58.7523741Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/ambiguous-builtin-attrs-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs-test" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs-test/auxiliary" "-A" "unused"
2019-07-21T16:22:58.7524280Z ------------------------------------------
2019-07-21T16:22:58.7524330Z 
2019-07-21T16:22:58.7524554Z ------------------------------------------
2019-07-21T16:22:58.7524598Z stderr:
2019-07-21T16:22:58.7524598Z stderr:
2019-07-21T16:22:58.7524810Z ------------------------------------------
2019-07-21T16:22:58.7524870Z error[E0425]: cannot find value `NonExistent` in this scope
2019-07-21T16:22:58.7525176Z    |
2019-07-21T16:22:58.7525176Z    |
2019-07-21T16:22:58.7525239Z LL |     NonExistent; //~ ERROR cannot find value `NonExistent` in this scope
2019-07-21T16:22:58.7525320Z 
2019-07-21T16:22:58.7525320Z 
2019-07-21T16:22:58.7525701Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7525760Z    |
2019-07-21T16:22:58.7526150Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7526216Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7526289Z error: aborting due to 2 previous errors
2019-07-21T16:22:58.7526325Z 
2019-07-21T16:22:58.7526369Z Some errors have detailed explanations: E0425, E0658.
2019-07-21T16:22:58.7526770Z For more information about an error, try `rustc --explain E0425`.
---
2019-07-21T16:22:58.7527475Z 
2019-07-21T16:22:58.7527509Z 8 LL | | }
2019-07-21T16:22:58.7527543Z 9    | |_^
2019-07-21T16:22:58.7527586Z 10 
2019-07-21T16:22:58.7527774Z - error: aborting due to previous error
2019-07-21T16:22:58.7528109Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7528515Z +   --> $DIR/termination-trait-in-test-should-panic.rs:7:5
2019-07-21T16:22:58.7528603Z + LL | use test::Bencher;
2019-07-21T16:22:58.7528650Z +    |     ^^^^^^^^^^^^^
2019-07-21T16:22:58.7528684Z +    |
2019-07-21T16:22:58.7528684Z +    |
2019-07-21T16:22:58.7528954Z +    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7529014Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7529054Z 12 
2019-07-21T16:22:58.7529398Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7529452Z +    |
2019-07-21T16:22:58.7529712Z +    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7529761Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7529836Z + error: aborting due to 3 previous errors
2019-07-21T16:22:58.7529883Z + 
2019-07-21T16:22:58.7530106Z + For more information about this error, try `rustc --explain E0658`.
2019-07-21T16:22:58.7530147Z 13 
2019-07-21T16:22:58.7530147Z 13 
2019-07-21T16:22:58.7530183Z 
2019-07-21T16:22:58.7530206Z 
2019-07-21T16:22:58.7530244Z The actual stderr differed from the expected stderr.
2019-07-21T16:22:58.7530694Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test-should-panic/termination-trait-in-test-should-panic.stderr
2019-07-21T16:22:58.7530930Z To update references, rerun the tests and pass the `--bless` flag
2019-07-21T16:22:58.7531214Z To only update this specific test, also pass `--test-args rfc-1937-termination-trait/termination-trait-in-test-should-panic.rs`
2019-07-21T16:22:58.7531304Z error: 1 errors occurred comparing output.
2019-07-21T16:22:58.7531344Z status: exit code: 1
2019-07-21T16:22:58.7531344Z status: exit code: 1
2019-07-21T16:22:58.7532096Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-in-test-should-panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test-should-panic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test-should-panic/auxiliary" "-A" "unused"
2019-07-21T16:22:58.7532414Z ------------------------------------------
2019-07-21T16:22:58.7532453Z 
2019-07-21T16:22:58.7532854Z ------------------------------------------
2019-07-21T16:22:58.7532906Z stderr:
2019-07-21T16:22:58.7532906Z stderr:
2019-07-21T16:22:58.7533745Z ------------------------------------------
2019-07-21T16:22:58.7533804Z error: functions using `#[should_panic]` must return `()`
2019-07-21T16:22:58.7534159Z    |
2019-07-21T16:22:58.7534159Z    |
2019-07-21T16:22:58.7534390Z LL | / fn not_a_num() -> Result<(), ParseIntError> {
2019-07-21T16:22:58.7534457Z LL | |     //~^ ERROR functions using `#[should_panic]` must return `()`
2019-07-21T16:22:58.7534514Z LL | |     let _: u32 = "abc".parse()?;
2019-07-21T16:22:58.7534558Z LL | |     Ok(())
2019-07-21T16:22:58.7534638Z    | |_^
2019-07-21T16:22:58.7534673Z 
2019-07-21T16:22:58.7534673Z 
2019-07-21T16:22:58.7535057Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7535415Z    |
2019-07-21T16:22:58.7535456Z LL | use test::Bencher;
2019-07-21T16:22:58.7535498Z    |     ^^^^^^^^^^^^^
2019-07-21T16:22:58.7535548Z    |
2019-07-21T16:22:58.7535548Z    |
2019-07-21T16:22:58.7535845Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7535902Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7535953Z 
2019-07-21T16:22:58.7536337Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7536395Z    |
2019-07-21T16:22:58.7536922Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7536972Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7537055Z error: aborting due to 3 previous errors
2019-07-21T16:22:58.7537079Z 
2019-07-21T16:22:58.7537301Z For more information about this error, try `rustc --explain E0658`.
2019-07-21T16:22:58.7537331Z 
2019-07-21T16:22:58.7537331Z 
2019-07-21T16:22:58.7537529Z ------------------------------------------
2019-07-21T16:22:58.7537557Z 
2019-07-21T16:22:58.7537579Z 
2019-07-21T16:22:58.7537799Z ---- [ui] ui/rfc-1937-termination-trait/termination-trait-in-test.rs stdout ----
2019-07-21T16:22:58.7537950Z 
2019-07-21T16:22:58.7538180Z error: test compilation failed although it shouldn't!
2019-07-21T16:22:58.7538222Z status: exit code: 1
2019-07-21T16:22:58.7538937Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-in-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test/auxiliary" "-A" "unused"
2019-07-21T16:22:58.7539224Z ------------------------------------------
2019-07-21T16:22:58.7539253Z 
2019-07-21T16:22:58.7539447Z ------------------------------------------
2019-07-21T16:22:58.7539492Z stderr:
2019-07-21T16:22:58.7539492Z stderr:
2019-07-21T16:22:58.7539687Z ------------------------------------------
2019-07-21T16:22:58.7540015Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7540390Z    |
2019-07-21T16:22:58.7540434Z LL | use test::Bencher;
2019-07-21T16:22:58.7540480Z    |     ^^^^^^^^^^^^^
2019-07-21T16:22:58.7540514Z    |
2019-07-21T16:22:58.7540514Z    |
2019-07-21T16:22:58.7540793Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7540854Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7540884Z 
2019-07-21T16:22:58.7541236Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7541557Z    |
2019-07-21T16:22:58.7541557Z    |
2019-07-21T16:22:58.7541769Z LL | / fn is_a_num() -> Result<(), ParseIntError> {
2019-07-21T16:22:58.7541814Z LL | |     let _: u32 = "22".parse()?;
2019-07-21T16:22:58.7541860Z LL | |     Ok(())
2019-07-21T16:22:58.7541941Z    | |_^
2019-07-21T16:22:58.7541987Z    |
2019-07-21T16:22:58.7541987Z    |
2019-07-21T16:22:58.7542253Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7542304Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7542347Z 
2019-07-21T16:22:58.7542695Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7543755Z    |
2019-07-21T16:22:58.7543755Z    |
2019-07-21T16:22:58.7544051Z LL | / fn test_a_positive_bench(_: &mut Bencher) -> Result<(), ParseIntError> {
2019-07-21T16:22:58.7544101Z LL | |     Ok(())
2019-07-21T16:22:58.7544189Z    | |_^
2019-07-21T16:22:58.7544227Z    |
2019-07-21T16:22:58.7544227Z    |
2019-07-21T16:22:58.7544527Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7544595Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7544628Z 
2019-07-21T16:22:58.7545010Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7545348Z    |
2019-07-21T16:22:58.7545348Z    |
2019-07-21T16:22:58.7545742Z LL | fn test_a_positive_bench(_: &mut Bencher) -> Result<(), ParseIntError> {
2019-07-21T16:22:58.7545844Z    |
2019-07-21T16:22:58.7545844Z    |
2019-07-21T16:22:58.7546145Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7546201Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7546233Z 
2019-07-21T16:22:58.7546669Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7547211Z    |
2019-07-21T16:22:58.7547211Z    |
2019-07-21T16:22:58.7547424Z LL | / fn test_a_neg_bench(_: &mut Bencher) -> Result<(), ParseIntError> {
2019-07-21T16:22:58.7547481Z LL | |     let _: u32 = "abc".parse()?;
2019-07-21T16:22:58.7547518Z LL | |     Ok(())
2019-07-21T16:22:58.7547602Z    | |_^
2019-07-21T16:22:58.7547636Z    |
2019-07-21T16:22:58.7547636Z    |
2019-07-21T16:22:58.7547881Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7547937Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7547964Z 
2019-07-21T16:22:58.7548372Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7548705Z    |
2019-07-21T16:22:58.7548705Z    |
2019-07-21T16:22:58.7548916Z LL | fn test_a_neg_bench(_: &mut Bencher) -> Result<(), ParseIntError> {
2019-07-21T16:22:58.7549001Z    |
2019-07-21T16:22:58.7549001Z    |
2019-07-21T16:22:58.7549249Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7549317Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7549345Z 
2019-07-21T16:22:58.7549678Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7549739Z    |
2019-07-21T16:22:58.7549982Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7550037Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7550112Z error: aborting due to 7 previous errors
2019-07-21T16:22:58.7550137Z 
2019-07-21T16:22:58.7550359Z For more information about this error, try `rustc --explain E0658`.
2019-07-21T16:22:58.7550396Z 
2019-07-21T16:22:58.7550396Z 
2019-07-21T16:22:58.7550586Z ------------------------------------------
2019-07-21T16:22:58.7550613Z 
2019-07-21T16:22:58.7550635Z 
2019-07-21T16:22:58.7550868Z ---- [ui] ui/rfc-1937-termination-trait/termination-trait-test-wrong-type.rs stdout ----
2019-07-21T16:22:58.7550921Z diff of stderr:
2019-07-21T16:22:58.7550945Z 
2019-07-21T16:22:58.7551272Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7551498Z +   --> $DIR/termination-trait-test-wrong-type.rs:6:1
2019-07-21T16:22:58.7551540Z +    |
2019-07-21T16:22:58.7551770Z + LL | / fn can_parse_zero_as_f32() -> Result<f32, ParseFloatError> {
2019-07-21T16:22:58.7551816Z + LL | |     "0".parse()
2019-07-21T16:22:58.7551856Z + LL | | }
2019-07-21T16:22:58.7551937Z +    |
2019-07-21T16:22:58.7551937Z +    |
2019-07-21T16:22:58.7552185Z +    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7552247Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7552286Z + 
2019-07-21T16:22:58.7552843Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7552912Z +    |
2019-07-21T16:22:58.7553738Z +    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7553801Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7553854Z + 
2019-07-21T16:22:58.7553915Z 1 error[E0277]: `main` has invalid return type `std::result::Result<f32, std::num::ParseFloatError>`
2019-07-21T16:22:58.7554231Z 3    |
2019-07-21T16:22:58.7554258Z 
2019-07-21T16:22:58.7554258Z 
2019-07-21T16:22:58.7554310Z 9    = help: the trait `std::process::Termination` is not implemented for `std::result::Result<f32, std::num::ParseFloatError>`
2019-07-21T16:22:58.7554372Z 10    = note: required by `test::assert_test_result`
2019-07-21T16:22:58.7554641Z - error: aborting due to previous error
2019-07-21T16:22:58.7554689Z + error: aborting due to 3 previous errors
2019-07-21T16:22:58.7554737Z 13 
2019-07-21T16:22:58.7554981Z - For more information about this error, try `rustc --explain E0277`.
2019-07-21T16:22:58.7554981Z - For more information about this error, try `rustc --explain E0277`.
2019-07-21T16:22:58.7555033Z + Some errors have detailed explanations: E0277, E0658.
2019-07-21T16:22:58.7555286Z + For more information about an error, try `rustc --explain E0277`.
2019-07-21T16:22:58.7555332Z 15 
2019-07-21T16:22:58.7555459Z 
2019-07-21T16:22:58.7555494Z 
2019-07-21T16:22:58.7555549Z The actual stderr differed from the expected stderr.
2019-07-21T16:22:58.7555932Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/termination-trait-test-wrong-type.stderr
2019-07-21T16:22:58.7556187Z To update references, rerun the tests and pass the `--bless` flag
2019-07-21T16:22:58.7556615Z To only update this specific test, also pass `--test-args rfc-1937-termination-trait/termination-trait-test-wrong-type.rs`
2019-07-21T16:22:58.7556694Z error: 1 errors occurred comparing output.
2019-07-21T16:22:58.7556746Z status: exit code: 1
2019-07-21T16:22:58.7556746Z status: exit code: 1
2019-07-21T16:22:58.7557477Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/auxiliary" "-A" "unused"
2019-07-21T16:22:58.7557768Z ------------------------------------------
2019-07-21T16:22:58.7557804Z 
2019-07-21T16:22:58.7558007Z ------------------------------------------
2019-07-21T16:22:58.7558046Z stderr:
2019-07-21T16:22:58.7558046Z stderr:
2019-07-21T16:22:58.7558234Z ------------------------------------------
2019-07-21T16:22:58.7558565Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7558867Z    |
2019-07-21T16:22:58.7558867Z    |
2019-07-21T16:22:58.7559094Z LL | / fn can_parse_zero_as_f32() -> Result<f32, ParseFloatError> { //~ ERROR
2019-07-21T16:22:58.7559136Z LL | |     "0".parse()
2019-07-21T16:22:58.7559212Z    | |_^
2019-07-21T16:22:58.7559245Z    |
2019-07-21T16:22:58.7559245Z    |
2019-07-21T16:22:58.7559496Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7559555Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7559683Z 
2019-07-21T16:22:58.7560043Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7560108Z    |
2019-07-21T16:22:58.7560359Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7560416Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7560456Z 
2019-07-21T16:22:58.7560499Z error[E0277]: `main` has invalid return type `std::result::Result<f32, std::num::ParseFloatError>`
2019-07-21T16:22:58.7560800Z    |
2019-07-21T16:22:58.7560800Z    |
2019-07-21T16:22:58.7561019Z LL | / fn can_parse_zero_as_f32() -> Result<f32, ParseFloatError> { //~ ERROR
2019-07-21T16:22:58.7561061Z LL | |     "0".parse()
2019-07-21T16:22:58.7561104Z LL | | }
2019-07-21T16:22:58.7561158Z    | |_^ `main` can only return types that implement `std::process::Termination`
2019-07-21T16:22:58.7561196Z    |
2019-07-21T16:22:58.7561242Z    = help: the trait `std::process::Termination` is not implemented for `std::result::Result<f32, std::num::ParseFloatError>`
2019-07-21T16:22:58.7561296Z    = note: required by `test::assert_test_result`
2019-07-21T16:22:58.7561431Z error: aborting due to 3 previous errors
2019-07-21T16:22:58.7561463Z 
2019-07-21T16:22:58.7561507Z Some errors have detailed explanations: E0277, E0658.
2019-07-21T16:22:58.7561750Z For more information about an error, try `rustc --explain E0277`.
2019-07-21T16:22:58.7561750Z For more information about an error, try `rustc --explain E0277`.
2019-07-21T16:22:58.7561779Z 
2019-07-21T16:22:58.7561973Z ------------------------------------------
2019-07-21T16:22:58.7562000Z 
2019-07-21T16:22:58.7562022Z 
2019-07-21T16:22:58.7562217Z ---- [ui] ui/rustc_private-libtest.rs stdout ----
2019-07-21T16:22:58.7562264Z diff of stderr:
2019-07-21T16:22:58.7562297Z 
2019-07-21T16:22:58.7562332Z 4 LL |     use libtest::*;
2019-07-21T16:22:58.7562373Z 5    |         ^^^^^^^ maybe a missing `extern crate libtest;`?
2019-07-21T16:22:58.7562608Z - error: aborting due to previous error
2019-07-21T16:22:58.7562608Z - error: aborting due to previous error
2019-07-21T16:22:58.7563535Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7563865Z +   --> $DIR/rustc_private-libtest.rs:2:5
2019-07-21T16:22:58.7563960Z + LL |     extern crate libtest;
2019-07-21T16:22:58.7564017Z +    |     ^^^^^^^^^^^^^^^^^^^^^
2019-07-21T16:22:58.7564058Z +    |
2019-07-21T16:22:58.7564058Z +    |
2019-07-21T16:22:58.7564356Z +    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7564424Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7564738Z - For more information about this error, try `rustc --explain E0432`.
2019-07-21T16:22:58.7564796Z + error: aborting due to 2 previous errors
2019-07-21T16:22:58.7564836Z + 
2019-07-21T16:22:58.7564880Z + Some errors have detailed explanations: E0432, E0658.
2019-07-21T16:22:58.7564880Z + Some errors have detailed explanations: E0432, E0658.
2019-07-21T16:22:58.7565132Z + For more information about an error, try `rustc --explain E0432`.
2019-07-21T16:22:58.7565178Z 10 
2019-07-21T16:22:58.7565205Z 
2019-07-21T16:22:58.7565230Z 
2019-07-21T16:22:58.7565289Z The actual stderr differed from the expected stderr.
2019-07-21T16:22:58.7565591Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc_private-libtest/rustc_private-libtest.stderr
2019-07-21T16:22:58.7565841Z To update references, rerun the tests and pass the `--bless` flag
2019-07-21T16:22:58.7566141Z To only update this specific test, also pass `--test-args rustc_private-libtest.rs`
2019-07-21T16:22:58.7566223Z error: 1 errors occurred comparing output.
2019-07-21T16:22:58.7566397Z status: exit code: 1
2019-07-21T16:22:58.7566397Z status: exit code: 1
2019-07-21T16:22:58.7567213Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rustc_private-libtest.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc_private-libtest" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc_private-libtest/auxiliary" "-A" "unused"
2019-07-21T16:22:58.7567496Z ------------------------------------------
2019-07-21T16:22:58.7567525Z 
2019-07-21T16:22:58.7567725Z ------------------------------------------
2019-07-21T16:22:58.7567764Z stderr:
2019-07-21T16:22:58.7567764Z stderr:
2019-07-21T16:22:58.7567952Z ------------------------------------------
2019-07-21T16:22:58.7567993Z error[E0432]: unresolved import `libtest`
2019-07-21T16:22:58.7568217Z   --> /checkout/src/test/ui/rustc_private-libtest.rs:3:9
2019-07-21T16:22:58.7568467Z    |
2019-07-21T16:22:58.7568519Z LL |     use libtest::*; //~ ERROR unresolved import
2019-07-21T16:22:58.7568574Z    |         ^^^^^^^ maybe a missing `extern crate libtest;`?
2019-07-21T16:22:58.7568600Z 
2019-07-21T16:22:58.7569131Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7569457Z    |
2019-07-21T16:22:58.7569494Z LL |     extern crate libtest;
2019-07-21T16:22:58.7569539Z    |     ^^^^^^^^^^^^^^^^^^^^^
2019-07-21T16:22:58.7569573Z    |
2019-07-21T16:22:58.7569573Z    |
2019-07-21T16:22:58.7569945Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7570005Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7570080Z error: aborting due to 2 previous errors
2019-07-21T16:22:58.7570104Z 
2019-07-21T16:22:58.7570150Z Some errors have detailed explanations: E0432, E0658.
2019-07-21T16:22:58.7570387Z For more information about an error, try `rustc --explain E0432`.
2019-07-21T16:22:58.7570387Z For more information about an error, try `rustc --explain E0432`.
2019-07-21T16:22:58.7570416Z 
2019-07-21T16:22:58.7570615Z ------------------------------------------
2019-07-21T16:22:58.7570642Z 
2019-07-21T16:22:58.7570664Z 
2019-07-21T16:22:58.7570858Z ---- [ui] ui/test-on-macro.rs stdout ----
2019-07-21T16:22:58.7570885Z 
2019-07-21T16:22:58.7571096Z error: test compilation failed although it shouldn't!
2019-07-21T16:22:58.7571138Z status: exit code: 1
2019-07-21T16:22:58.7571738Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-on-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-on-macro" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-on-macro/auxiliary" "-A" "unused"
2019-07-21T16:22:58.7572024Z ------------------------------------------
2019-07-21T16:22:58.7572059Z 
2019-07-21T16:22:58.7572250Z ------------------------------------------
2019-07-21T16:22:58.7572288Z stderr:
2019-07-21T16:22:58.7572288Z stderr:
2019-07-21T16:22:58.7572480Z ------------------------------------------
2019-07-21T16:22:58.7572535Z warning: `#[test]` attribute should not be used on macros. Use `#[cfg(test)]` instead.
2019-07-21T16:22:58.7572898Z    |
2019-07-21T16:22:58.7572942Z LL | foo!();
2019-07-21T16:22:58.7572980Z    | ^^^^^^^
2019-07-21T16:22:58.7573004Z 
2019-07-21T16:22:58.7573004Z 
2019-07-21T16:22:58.7573897Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7574094Z    |
2019-07-21T16:22:58.7574443Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7574499Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7574573Z error: aborting due to previous error
2019-07-21T16:22:58.7574614Z 
2019-07-21T16:22:58.7574879Z For more information about this error, try `rustc --explain E0658`.
2019-07-21T16:22:58.7574912Z 
2019-07-21T16:22:58.7574912Z 
2019-07-21T16:22:58.7575127Z ------------------------------------------
2019-07-21T16:22:58.7575168Z 
2019-07-21T16:22:58.7575194Z 
2019-07-21T16:22:58.7575417Z ---- [ui] ui/test-should-panic-attr.rs stdout ----
2019-07-21T16:22:58.7575449Z 
2019-07-21T16:22:58.7575671Z error: test compilation failed although it shouldn't!
2019-07-21T16:22:58.7575725Z status: exit code: 1
2019-07-21T16:22:58.7576943Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-should-panic-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-should-panic-attr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-should-panic-attr/auxiliary" "-A" "unused"
2019-07-21T16:22:58.7577276Z ------------------------------------------
2019-07-21T16:22:58.7577305Z 
2019-07-21T16:22:58.7577507Z ------------------------------------------
2019-07-21T16:22:58.7577545Z stderr:
2019-07-21T16:22:58.7577545Z stderr:
2019-07-21T16:22:58.7577731Z ------------------------------------------
2019-07-21T16:22:58.7577782Z warning: argument must be of the form: `expected = "error message"`
2019-07-21T16:22:58.7578044Z    |
2019-07-21T16:22:58.7578044Z    |
2019-07-21T16:22:58.7578088Z LL | #[should_panic(expected)]
2019-07-21T16:22:58.7578160Z    |
2019-07-21T16:22:58.7578210Z    = note: Errors in this attribute were erroneously allowed and will become a hard error in a future release.
2019-07-21T16:22:58.7578242Z 
2019-07-21T16:22:58.7578242Z 
2019-07-21T16:22:58.7578283Z warning: argument must be of the form: `expected = "error message"`
2019-07-21T16:22:58.7578554Z    |
2019-07-21T16:22:58.7578554Z    |
2019-07-21T16:22:58.7578589Z LL | #[should_panic(expect)]
2019-07-21T16:22:58.7578675Z    |
2019-07-21T16:22:58.7578719Z    = note: Errors in this attribute were erroneously allowed and will become a hard error in a future release.
2019-07-21T16:22:58.7578748Z 
2019-07-21T16:22:58.7578748Z 
2019-07-21T16:22:58.7578787Z warning: argument must be of the form: `expected = "error message"`
2019-07-21T16:22:58.7579054Z    |
2019-07-21T16:22:58.7579054Z    |
2019-07-21T16:22:58.7579091Z LL | #[should_panic(expected(foo, bar))]
2019-07-21T16:22:58.7579174Z    |
2019-07-21T16:22:58.7579218Z    = note: Errors in this attribute were erroneously allowed and will become a hard error in a future release.
2019-07-21T16:22:58.7579253Z 
2019-07-21T16:22:58.7579253Z 
2019-07-21T16:22:58.7579299Z warning: argument must be of the form: `expected = "error message"`
2019-07-21T16:22:58.7579549Z    |
2019-07-21T16:22:58.7579549Z    |
2019-07-21T16:22:58.7579595Z LL | #[should_panic(expected = "foo", bar)]
2019-07-21T16:22:58.7579670Z    |
2019-07-21T16:22:58.7579719Z    = note: Errors in this attribute were erroneously allowed and will become a hard error in a future release.
2019-07-21T16:22:58.7579825Z 
2019-07-21T16:22:58.7579825Z 
2019-07-21T16:22:58.7580180Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7580457Z    |
2019-07-21T16:22:58.7580457Z    |
2019-07-21T16:22:58.7580492Z LL | / fn test1() {
2019-07-21T16:22:58.7580542Z LL | |     panic!();
2019-07-21T16:22:58.7580621Z    | |_^
2019-07-21T16:22:58.7580666Z    |
2019-07-21T16:22:58.7580666Z    |
2019-07-21T16:22:58.7580923Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7580973Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7581000Z 
2019-07-21T16:22:58.7581342Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7581619Z    |
2019-07-21T16:22:58.7581619Z    |
2019-07-21T16:22:58.7581655Z LL | / fn test2() {
2019-07-21T16:22:58.7581691Z LL | |     panic!();
2019-07-21T16:22:58.7581770Z    | |_^
2019-07-21T16:22:58.7581803Z    |
2019-07-21T16:22:58.7581803Z    |
2019-07-21T16:22:58.7582051Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7582184Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7582220Z 
2019-07-21T16:22:58.7582596Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7583003Z    |
2019-07-21T16:22:58.7583003Z    |
2019-07-21T16:22:58.7583428Z LL | / fn test3() {
2019-07-21T16:22:58.7583526Z LL | |     panic!();
2019-07-21T16:22:58.7583619Z    | |_^
2019-07-21T16:22:58.7583657Z    |
2019-07-21T16:22:58.7583657Z    |
2019-07-21T16:22:58.7584021Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7584078Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7584110Z 
2019-07-21T16:22:58.7584517Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7584817Z    |
2019-07-21T16:22:58.7584817Z    |
2019-07-21T16:22:58.7584867Z LL | / fn test4() {
2019-07-21T16:22:58.7584909Z LL | |     panic!();
2019-07-21T16:22:58.7584995Z    | |_^
2019-07-21T16:22:58.7585032Z    |
2019-07-21T16:22:58.7585032Z    |
2019-07-21T16:22:58.7585311Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7585375Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7585415Z 
2019-07-21T16:22:58.7585794Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7586094Z    |
2019-07-21T16:22:58.7586094Z    |
2019-07-21T16:22:58.7586134Z LL | / fn test5() {
2019-07-21T16:22:58.7586183Z LL | |     panic!();
2019-07-21T16:22:58.7586268Z    | |_^
2019-07-21T16:22:58.7586306Z    |
2019-07-21T16:22:58.7586306Z    |
2019-07-21T16:22:58.7586817Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7586865Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7586892Z 
2019-07-21T16:22:58.7587234Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7587389Z    |
2019-07-21T16:22:58.7587666Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7587727Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7587791Z error: aborting due to 6 previous errors
2019-07-21T16:22:58.7587821Z 
2019-07-21T16:22:58.7588053Z For more information about this error, try `rustc --explain E0658`.
2019-07-21T16:22:58.7588083Z 
2019-07-21T16:22:58.7588083Z 
2019-07-21T16:22:58.7588273Z ------------------------------------------
2019-07-21T16:22:58.7588300Z 
2019-07-21T16:22:58.7588334Z 
2019-07-21T16:22:58.7588545Z ---- [ui] ui/test-shadowing/test-cant-be-shadowed.rs stdout ----
2019-07-21T16:22:58.7588573Z 
2019-07-21T16:22:58.7588768Z error: test compilation failed although it shouldn't!
2019-07-21T16:22:58.7588817Z status: exit code: 1
2019-07-21T16:22:58.7589550Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-shadowing/test-cant-be-shadowed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-shadowing/test-cant-be-shadowed" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-shadowing/test-cant-be-shadowed/auxiliary" "-A" "unused"
2019-07-21T16:22:58.7589873Z ------------------------------------------
2019-07-21T16:22:58.7589902Z 
2019-07-21T16:22:58.7590104Z ------------------------------------------
2019-07-21T16:22:58.7590142Z stderr:
2019-07-21T16:22:58.7590142Z stderr:
2019-07-21T16:22:58.7590328Z ------------------------------------------
2019-07-21T16:22:58.7590662Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7590955Z    |
2019-07-21T16:22:58.7590991Z LL | fn foo(){}
2019-07-21T16:22:58.7591026Z    | ^^^^^^^^^^
2019-07-21T16:22:58.7591060Z    |
2019-07-21T16:22:58.7591060Z    |
2019-07-21T16:22:58.7591324Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7591380Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7591408Z 
2019-07-21T16:22:58.7591751Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7592023Z    |
2019-07-21T16:22:58.7592023Z    |
2019-07-21T16:22:58.7592066Z LL | fn bar() {}
2019-07-21T16:22:58.7592142Z    |
2019-07-21T16:22:58.7592142Z    |
2019-07-21T16:22:58.7592394Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7592446Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7592474Z 
2019-07-21T16:22:58.7592960Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7593014Z    |
2019-07-21T16:22:58.7593843Z    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7593915Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7593989Z error: aborting due to 3 previous errors
2019-07-21T16:22:58.7594017Z 
2019-07-21T16:22:58.7594288Z For more information about this error, try `rustc --explain E0658`.
2019-07-21T16:22:58.7594321Z 
2019-07-21T16:22:58.7594321Z 
2019-07-21T16:22:58.7594690Z ------------------------------------------
2019-07-21T16:22:58.7594723Z 
2019-07-21T16:22:58.7594764Z 
2019-07-21T16:22:58.7594987Z ---- [ui] ui/test-warns-dead-code.rs stdout ----
2019-07-21T16:22:58.7595034Z diff of stderr:
2019-07-21T16:22:58.7595061Z 
2019-07-21T16:22:58.7595288Z - error: function is never used: `dead`
2019-07-21T16:22:58.7595509Z -   --> $DIR/test-warns-dead-code.rs:5:1
2019-07-21T16:22:58.7595893Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-21T16:22:58.7595962Z 3    |
2019-07-21T16:22:58.7596161Z - LL | fn dead() {}
2019-07-21T16:22:58.7596561Z -    |
2019-07-21T16:22:58.7596890Z - note: lint level defined here
2019-07-21T16:22:58.7597194Z -   --> $DIR/test-warns-dead-code.rs:3:9
2019-07-21T16:22:58.7597367Z -    |
2019-07-21T16:22:58.7597367Z -    |
2019-07-21T16:22:58.7597544Z - LL | #![deny(dead_code)]
2019-07-21T16:22:58.7597728Z -    |         ^^^^^^^^^
2019-07-21T16:22:58.7597989Z +    = note: for more information, see ***/issues/27812
2019-07-21T16:22:58.7598041Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-21T16:22:58.7598130Z 13 error: aborting due to previous error
2019-07-21T16:22:58.7598169Z 14 
2019-07-21T16:22:58.7598193Z 
---
2019-07-21T16:22:58.7607487Z test result: FAILED. 5804 passed; 17 failed; 21 ignored; 0 measured; 0 filtered out
2019-07-21T16:22:58.7607519Z 
2019-07-21T16:22:58.7607541Z 
2019-07-21T16:22:58.7607563Z 
2019-07-21T16:22:58.7609319Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-21T16:22:58.7609550Z 
2019-07-21T16:22:58.7609576Z 
2019-07-21T16:22:58.7609879Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-21T16:22:58.7609940Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-21T16:22:58.7609940Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-21T16:22:58.7610000Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-21T16:22:58.7610043Z Build completed unsuccessfully in 0:54:08
2019-07-21T16:22:59.9494980Z ##[error]Bash exited with code '1'.
2019-07-21T16:22:59.9532290Z ##[section]Starting: Checkout
2019-07-21T16:22:59.9534497Z ==============================================================================
2019-07-21T16:22:59.9534572Z Task         : Get sources
2019-07-21T16:22:59.9534619Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
