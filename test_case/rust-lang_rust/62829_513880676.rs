plain
2019-07-22T16:29:40.3734229Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-22T16:29:40.3954683Z ##[command]git config gc.auto 0
2019-07-22T16:29:40.4033436Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-22T16:29:40.4091032Z ##[command]git config --get-all http.proxy
2019-07-22T16:29:40.4228790Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62829/merge:refs/remotes/pull/62829/merge
---
2019-07-22T16:30:14.8118256Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-22T16:30:14.8118286Z 
2019-07-22T16:30:14.8118515Z   git checkout -b <new-branch-name>
2019-07-22T16:30:14.8118546Z 
2019-07-22T16:30:14.8118593Z HEAD is now at 93f85635e Merge 18e201eeb86bf69ed5786e2720ee9433742b4902 into 4bc1ce7bdb7f5dc9ea07c0b630c087d8e11140e4
2019-07-22T16:30:14.8262401Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-22T16:30:14.8265618Z ==============================================================================
2019-07-22T16:30:14.8265685Z Task         : Bash
2019-07-22T16:30:14.8265738Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-22T16:33:30.0888992Z ##########################                                                36.7%
2019-07-22T16:33:30.0889117Z ######################################################################## 100.0%
2019-07-22T16:33:30.7379333Z extracting /checkout/obj/build/cache/2019-07-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-07-22T16:33:32.0625607Z     Updating crates.io index
2019-07-22T16:33:54.3406147Z     Updating git repository `https://github.com/gnzlbg/libtest`
---
2019-07-22T16:36:56.6529906Z tidy check
2019-07-22T16:36:57.6296459Z * 577 error codes
2019-07-22T16:36:57.6296600Z * highest error code: E0732
2019-07-22T16:36:57.9663289Z * 266 features
2019-07-22T16:36:58.5036213Z invalid source: "git+https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd286fd16c76bd61ddcbe162fca9f9d9d2"
2019-07-22T16:36:58.5575613Z + python2.7 ../x.py test
2019-07-22T16:36:58.8061231Z     Finished dev [unoptimized] target(s) in 0.19s
2019-07-22T16:37:00.0969830Z Building stage0 tool tidy (x86_64-unknown-linux-gnu)
2019-07-22T16:37:00.3150402Z     Finished release [optimized] target(s) in 0.21s
2019-07-22T16:37:00.3150402Z     Finished release [optimized] target(s) in 0.21s
2019-07-22T16:37:00.3178543Z tidy check
2019-07-22T16:37:01.1423006Z * 577 error codes
2019-07-22T16:37:01.1423131Z * highest error code: E0732
2019-07-22T16:37:01.4638289Z * 266 features
2019-07-22T16:37:02.0373172Z invalid source: "git+https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd286fd16c76bd61ddcbe162fca9f9d9d2"
2019-07-22T16:37:02.5294899Z    Compiling cc v1.0.35
2019-07-22T16:37:02.5295330Z    Compiling core v0.0.0 (/checkout/src/libcore)
2019-07-22T16:37:11.2890728Z    Compiling libc v0.2.54
2019-07-22T16:37:12.0873522Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
---
2019-07-22T16:37:54.5873863Z    Compiling unicode-width v0.1.5
2019-07-22T16:37:54.7155271Z    Compiling termcolor v1.0.4
2019-07-22T16:37:55.4308916Z    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
2019-07-22T16:37:55.9720932Z    Compiling getopts v0.2.19
2019-07-22T16:37:59.5567849Z    Compiling libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd)
2019-07-22T16:38:09.1173245Z     Finished release [optimized] target(s) in 14.85s
2019-07-22T16:38:09.1247265Z Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-07-22T16:38:09.1271230Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-22T16:38:10.0828008Z    Compiling semver-parser v0.7.0
---
2019-07-22T17:01:40.1319742Z    Compiling unicode-width v0.1.5
2019-07-22T17:01:40.2389661Z    Compiling termcolor v1.0.4
2019-07-22T17:01:41.1357362Z    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
2019-07-22T17:01:41.6413089Z    Compiling getopts v0.2.19
2019-07-22T17:01:45.8647445Z    Compiling libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd)
2019-07-22T17:01:58.2751094Z     Finished release [optimized] target(s) in 18.41s
2019-07-22T17:01:58.2818741Z Copying stage1 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-07-22T17:01:58.2845040Z Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-22T17:01:58.7408007Z    Compiling semver-parser v0.7.0
---
2019-07-22T17:30:53.9303857Z .................................................................................................... 200/5843
2019-07-22T17:30:58.2308697Z .................................................................................................... 300/5843
2019-07-22T17:31:02.0792276Z .................................................................................................... 400/5843
2019-07-22T17:31:05.9697890Z .................................................................................................... 500/5843
2019-07-22T17:31:09.9113709Z ........................................................................i........................... 600/5843
2019-07-22T17:31:19.0398203Z .................................................................................................... 800/5843
2019-07-22T17:31:24.7130657Z .................................................................................................... 900/5843
2019-07-22T17:31:29.9400316Z ...................................................................................................i 1000/5843
2019-07-22T17:31:29.9400316Z ...................................................................................................i 1000/5843
2019-07-22T17:31:35.6094784Z ...........iF....................................................................................... 1100/5843
2019-07-22T17:31:39.6868988Z .............................iiiii.................................................................. 1200/5843
2019-07-22T17:31:45.8622844Z .................................................................................................... 1400/5843
2019-07-22T17:31:48.6532999Z .................................................................................................... 1500/5843
2019-07-22T17:31:52.5111059Z .................................................................................................... 1600/5843
2019-07-22T17:31:55.2508603Z .................................................................................................... 1700/5843
2019-07-22T17:31:55.2508603Z .................................................................................................... 1700/5843
2019-07-22T17:31:58.7313006Z .....................................................................i.............................. 1800/5843
2019-07-22T17:32:07.6022517Z .................................................................................................... 2000/5843
2019-07-22T17:32:11.9861763Z .....................................................................F.............................. 2100/5843
2019-07-22T17:32:11.9861763Z .....................................................................F.............................. 2100/5843
2019-07-22T17:32:15.8662290Z .................................................................F.F................................ 2200/5843
2019-07-22T17:32:19.8796638Z .........F..........................................i............................................... 2300/5843
2019-07-22T17:32:30.0892034Z .................................................................................................... 2500/5843
2019-07-22T17:32:34.3024430Z .................................................................................................... 2600/5843
2019-07-22T17:32:39.5643495Z F................................................................................................... 2700/5843
2019-07-22T17:32:43.6575709Z .................................................................................................... 2800/5843
2019-07-22T17:32:43.6575709Z .................................................................................................... 2800/5843
2019-07-22T17:32:48.3382726Z .................................................................................................... 2900/5843
2019-07-22T17:32:53.7622626Z .................................................................................................... 3000/5843
2019-07-22T17:32:58.3803094Z .................................................................................................... 3100/5843
2019-07-22T17:33:03.7454531Z ...F................................................................................................ 3200/5843
2019-07-22T17:33:07.4754326Z .................................................................................................... 3300/5843
2019-07-22T17:33:11.4032452Z .................................................................................................... 3400/5843
2019-07-22T17:33:16.6812266Z ...........................................F........................................................ 3500/5843
2019-07-22T17:33:20.5160752Z ...................i................................................................................ 3600/5843
2019-07-22T17:33:24.7770683Z .............................................................................................ii...i. 3700/5843
2019-07-22T17:33:28.7208996Z .ii................................................................................................. 3800/5843
2019-07-22T17:33:37.8016041Z .................................................................................................... 4000/5843
2019-07-22T17:33:37.8016041Z .................................................................................................... 4000/5843
2019-07-22T17:33:41.7454801Z .......ii........................................................................................... 4100/5843
2019-07-22T17:33:43.8177735Z ............................i....................................................................... 4200/5843
2019-07-22T17:33:45.9502437Z ..............................................................................................i..... 4300/5843
2019-07-22T17:33:52.8695899Z .................................................................................................... 4500/5843
2019-07-22T17:34:11.1607903Z F................................................................................................... 4600/5843
2019-07-22T17:34:14.8053292Z .................................................................................................... 4700/5843
2019-07-22T17:34:18.8049788Z .................................................................................................... 4800/5843
2019-07-22T17:34:18.8049788Z .................................................................................................... 4800/5843
2019-07-22T17:34:23.5004828Z .............................................FF...F................................................. 4900/5843
2019-07-22T17:34:39.9764684Z ...........................F........................................................................ 5100/5843
2019-07-22T17:34:43.9754902Z .................................................................................................... 5200/5843
2019-07-22T17:34:48.0132730Z .................................................................................................... 5300/5843
2019-07-22T17:34:48.0132730Z .................................................................................................... 5300/5843
2019-07-22T17:34:53.4951101Z ................................................................FFFF................................ 5400/5843
2019-07-22T17:35:01.8932150Z .................................................................................................... 5600/5843
2019-07-22T17:35:05.2239180Z .................................................................................................... 5700/5843
2019-07-22T17:35:05.2239180Z .................................................................................................... 5700/5843
2019-07-22T17:35:08.3169726Z ...................................................................................i................ 5800/5843
2019-07-22T17:35:09.9183516Z failures:
2019-07-22T17:35:09.9263545Z 
2019-07-22T17:35:09.9264569Z ---- [ui] ui/custom_test_frameworks/mismatch.rs stdout ----
2019-07-22T17:35:09.9265036Z diff of stderr:
2019-07-22T17:35:09.9265036Z diff of stderr:
2019-07-22T17:35:09.9265247Z 
2019-07-22T17:35:09.9266009Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9266511Z +   --> $DIR/mismatch.rs:9:1
2019-07-22T17:35:09.9266722Z +    |
2019-07-22T17:35:09.9266903Z + LL | fn wrong_kind(){}
2019-07-22T17:35:09.9268502Z +    |
2019-07-22T17:35:09.9268502Z +    |
2019-07-22T17:35:09.9269478Z +    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9269805Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9269967Z + 
2019-07-22T17:35:09.9270154Z 1 error[E0277]: the trait bound `test::TestDescAndFn: example_runner::Testable` is not satisfied
2019-07-22T17:35:09.9271327Z 3    |
2019-07-22T17:35:09.9271485Z 
2019-07-22T17:35:09.9271634Z 6    |
2019-07-22T17:35:09.9271997Z 7    = note: required for the cast to the object type `dyn example_runner::Testable`
---
2019-07-22T17:35:09.9274397Z 12 
2019-07-22T17:35:09.9274547Z 
2019-07-22T17:35:09.9274681Z 
2019-07-22T17:35:09.9274834Z The actual stderr differed from the expected stderr.
2019-07-22T17:35:09.9275312Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/mismatch/mismatch.stderr
2019-07-22T17:35:09.9275785Z To update references, rerun the tests and pass the `--bless` flag
2019-07-22T17:35:09.9276268Z To only update this specific test, also pass `--test-args custom_test_frameworks/mismatch.rs`
2019-07-22T17:35:09.9276620Z error: 1 errors occurred comparing output.
2019-07-22T17:35:09.9276770Z status: exit code: 1
2019-07-22T17:35:09.9276770Z status: exit code: 1
2019-07-22T17:35:09.9277785Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/custom_test_frameworks/mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/mismatch" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/mismatch/auxiliary" "-A" "unused"
2019-07-22T17:35:09.9278761Z ------------------------------------------
2019-07-22T17:35:09.9278937Z 
2019-07-22T17:35:09.9279405Z ------------------------------------------
2019-07-22T17:35:09.9326291Z stderr:
2019-07-22T17:35:09.9326291Z stderr:
2019-07-22T17:35:09.9326775Z ------------------------------------------
2019-07-22T17:35:09.9327210Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9327590Z    |
2019-07-22T17:35:09.9327590Z    |
2019-07-22T17:35:09.9327639Z LL | fn wrong_kind(){}
2019-07-22T17:35:09.9327741Z    |
2019-07-22T17:35:09.9327741Z    |
2019-07-22T17:35:09.9328084Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9328158Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9328197Z 
2019-07-22T17:35:09.9328253Z error[E0277]: the trait bound `test::TestDescAndFn: example_runner::Testable` is not satisfied
2019-07-22T17:35:09.9328685Z    |
2019-07-22T17:35:09.9328685Z    |
2019-07-22T17:35:09.9328731Z LL | fn wrong_kind(){}
2019-07-22T17:35:09.9328795Z    | ^^^^^^^^^^^^^^^^^ the trait `example_runner::Testable` is not implemented for `test::TestDescAndFn`
2019-07-22T17:35:09.9328902Z    = note: required for the cast to the object type `dyn example_runner::Testable`
2019-07-22T17:35:09.9328946Z 
2019-07-22T17:35:09.9329001Z error: aborting due to 2 previous errors
2019-07-22T17:35:09.9329033Z 
---
2019-07-22T17:35:09.9329699Z 
2019-07-22T17:35:09.9329953Z ---- [ui] ui/inaccessible-test-modules.rs stdout ----
2019-07-22T17:35:09.9330005Z diff of stderr:
2019-07-22T17:35:09.9330191Z 
2019-07-22T17:35:09.9330250Z 16    |     no `__test_reexports` in the root
2019-07-22T17:35:09.9330322Z 17    |     help: a similar name exists in the module: `__test_reexports`
2019-07-22T17:35:09.9330656Z - error: aborting due to 2 previous errors
2019-07-22T17:35:09.9330656Z - error: aborting due to 2 previous errors
2019-07-22T17:35:09.9331508Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9331796Z +   --> $DIR/inaccessible-test-modules.rs:9:1
2019-07-22T17:35:09.9331920Z + LL | fn baz() {}
2019-07-22T17:35:09.9331965Z +    | ^^^^^^^^^^^
2019-07-22T17:35:09.9332010Z +    |
2019-07-22T17:35:09.9332010Z +    |
2019-07-22T17:35:09.9332362Z +    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9332427Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9332790Z - For more information about this error, try `rustc --explain E0432`.
2019-07-22T17:35:09.9332790Z - For more information about this error, try `rustc --explain E0432`.
2019-07-22T17:35:09.9333206Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9333289Z +    |
2019-07-22T17:35:09.9333597Z +    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9333826Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9333927Z + error: aborting due to 4 previous errors
2019-07-22T17:35:09.9333973Z + 
2019-07-22T17:35:09.9334128Z + Some errors have detailed explanations: E0432, E0658.
2019-07-22T17:35:09.9334448Z + For more information about an error, try `rustc --explain E0432`.
2019-07-22T17:35:09.9334448Z + For more information about an error, try `rustc --explain E0432`.
2019-07-22T17:35:09.9334500Z 22 
2019-07-22T17:35:09.9334548Z 
2019-07-22T17:35:09.9334577Z 
2019-07-22T17:35:09.9334638Z The actual stderr differed from the expected stderr.
2019-07-22T17:35:09.9334982Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inaccessible-test-modules/inaccessible-test-modules.stderr
2019-07-22T17:35:09.9335270Z To update references, rerun the tests and pass the `--bless` flag
2019-07-22T17:35:09.9335863Z To only update this specific test, also pass `--test-args inaccessible-test-modules.rs`
2019-07-22T17:35:09.9335991Z error: 1 errors occurred comparing output.
2019-07-22T17:35:09.9336040Z status: exit code: 1
2019-07-22T17:35:09.9336040Z status: exit code: 1
2019-07-22T17:35:09.9336910Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inaccessible-test-modules.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inaccessible-test-modules" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inaccessible-test-modules/auxiliary" "-A" "unused"
2019-07-22T17:35:09.9337276Z ------------------------------------------
2019-07-22T17:35:09.9337313Z 
2019-07-22T17:35:09.9337570Z ------------------------------------------
2019-07-22T17:35:09.9337620Z stderr:
2019-07-22T17:35:09.9337620Z stderr:
2019-07-22T17:35:09.9337919Z ------------------------------------------
2019-07-22T17:35:09.9337990Z error[E0432]: unresolved import `__test`
2019-07-22T17:35:09.9338249Z   --> /checkout/src/test/ui/inaccessible-test-modules.rs:5:5
2019-07-22T17:35:09.9338304Z    |
2019-07-22T17:35:09.9338375Z LL | use __test as x; //~ ERROR unresolved import `__test`
2019-07-22T17:35:09.9338644Z    |     |
2019-07-22T17:35:09.9338644Z    |     |
2019-07-22T17:35:09.9338706Z    |     no `__test` in the root
2019-07-22T17:35:09.9338760Z    |     help: a similar name exists in the module: `test`
2019-07-22T17:35:09.9338795Z 
2019-07-22T17:35:09.9338961Z error[E0432]: unresolved import `__test_reexports`
2019-07-22T17:35:09.9339332Z    |
2019-07-22T17:35:09.9339332Z    |
2019-07-22T17:35:09.9339386Z LL | use __test_reexports as y; //~ ERROR unresolved import `__test_reexports`
2019-07-22T17:35:09.9339687Z    |     |
2019-07-22T17:35:09.9339687Z    |     |
2019-07-22T17:35:09.9339746Z    |     no `__test_reexports` in the root
2019-07-22T17:35:09.9339816Z    |     help: a similar name exists in the module: `__test_reexports`
2019-07-22T17:35:09.9339852Z 
2019-07-22T17:35:09.9340260Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9340603Z    |
2019-07-22T17:35:09.9340649Z LL | fn baz() {}
2019-07-22T17:35:09.9340705Z    | ^^^^^^^^^^^
2019-07-22T17:35:09.9341901Z    |
2019-07-22T17:35:09.9341901Z    |
2019-07-22T17:35:09.9344776Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9344887Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9344928Z 
2019-07-22T17:35:09.9345441Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9345672Z    |
2019-07-22T17:35:09.9346053Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9346118Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9346219Z error: aborting due to 4 previous errors
2019-07-22T17:35:09.9346251Z 
2019-07-22T17:35:09.9346299Z Some errors have detailed explanations: E0432, E0658.
2019-07-22T17:35:09.9346608Z For more information about an error, try `rustc --explain E0432`.
2019-07-22T17:35:09.9346608Z For more information about an error, try `rustc --explain E0432`.
2019-07-22T17:35:09.9346646Z 
2019-07-22T17:35:09.9346884Z ------------------------------------------
2019-07-22T17:35:09.9346918Z 
2019-07-22T17:35:09.9346945Z 
2019-07-22T17:35:09.9347206Z ---- [ui] ui/issues/issue-12997-1.rs stdout ----
2019-07-22T17:35:09.9347258Z diff of stderr:
2019-07-22T17:35:09.9347288Z 
2019-07-22T17:35:09.9347335Z 10 LL | fn bar(x: isize, y: isize) { }
2019-07-22T17:35:09.9347460Z 12 
2019-07-22T17:35:09.9347701Z - error: aborting due to 2 previous errors
2019-07-22T17:35:09.9347701Z - error: aborting due to 2 previous errors
2019-07-22T17:35:09.9348131Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9348198Z +    |
2019-07-22T17:35:09.9348525Z +    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9348601Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9348721Z + error: aborting due to 3 previous errors
2019-07-22T17:35:09.9348768Z + 
2019-07-22T17:35:09.9349047Z + For more information about this error, try `rustc --explain E0658`.
2019-07-22T17:35:09.9349115Z 15 
2019-07-22T17:35:09.9349115Z 15 
2019-07-22T17:35:09.9349145Z 
2019-07-22T17:35:09.9349172Z 
2019-07-22T17:35:09.9349222Z The actual stderr differed from the expected stderr.
2019-07-22T17:35:09.9349582Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-1/issue-12997-1.stderr
2019-07-22T17:35:09.9349856Z To update references, rerun the tests and pass the `--bless` flag
2019-07-22T17:35:09.9350145Z To only update this specific test, also pass `--test-args issues/issue-12997-1.rs`
2019-07-22T17:35:09.9350249Z error: 1 errors occurred comparing output.
2019-07-22T17:35:09.9350297Z status: exit code: 1
2019-07-22T17:35:09.9350297Z status: exit code: 1
2019-07-22T17:35:09.9351655Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12997-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-1/auxiliary" "-A" "unused"
2019-07-22T17:35:09.9352096Z ------------------------------------------
2019-07-22T17:35:09.9352135Z 
2019-07-22T17:35:09.9352377Z ------------------------------------------
2019-07-22T17:35:09.9352427Z stderr:
2019-07-22T17:35:09.9352427Z stderr:
2019-07-22T17:35:09.9352674Z ------------------------------------------
2019-07-22T17:35:09.9352966Z error: functions used as benches must have signature `fn(&mut Bencher) -> impl Termination`
2019-07-22T17:35:09.9353242Z   --> /checkout/src/test/ui/issues/issue-12997-1.rs:6:1
2019-07-22T17:35:09.9353314Z    |
2019-07-22T17:35:09.9353366Z LL | fn foo() { } //~ ERROR functions used as benches
2019-07-22T17:35:09.9353447Z 
2019-07-22T17:35:09.9353447Z 
2019-07-22T17:35:09.9353752Z error: functions used as benches must have signature `fn(&mut Bencher) -> impl Termination`
2019-07-22T17:35:09.9354013Z   --> /checkout/src/test/ui/issues/issue-12997-1.rs:9:1
2019-07-22T17:35:09.9354162Z    |
2019-07-22T17:35:09.9354271Z LL | fn bar(x: isize, y: isize) { } //~ ERROR functions used as benches
2019-07-22T17:35:09.9354358Z 
2019-07-22T17:35:09.9354358Z 
2019-07-22T17:35:09.9354813Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9354880Z    |
2019-07-22T17:35:09.9355228Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9355301Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9355385Z error: aborting due to 3 previous errors
2019-07-22T17:35:09.9355432Z 
2019-07-22T17:35:09.9355709Z For more information about this error, try `rustc --explain E0658`.
2019-07-22T17:35:09.9355745Z 
2019-07-22T17:35:09.9355745Z 
2019-07-22T17:35:09.9355978Z ------------------------------------------
2019-07-22T17:35:09.9356038Z 
2019-07-22T17:35:09.9356066Z 
2019-07-22T17:35:09.9356312Z ---- [ui] ui/issues/issue-12997-2.rs stdout ----
2019-07-22T17:35:09.9356364Z diff of stderr:
2019-07-22T17:35:09.9356395Z 
2019-07-22T17:35:09.9356818Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9357068Z +   --> $DIR/issue-12997-2.rs:6:1
2019-07-22T17:35:09.9357133Z +    |
2019-07-22T17:35:09.9357181Z + LL | fn bar(x: isize) { }
2019-07-22T17:35:09.9357285Z +    |
2019-07-22T17:35:09.9357285Z +    |
2019-07-22T17:35:09.9357616Z +    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9357685Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9357738Z + 
2019-07-22T17:35:09.9358210Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9358292Z +    |
2019-07-22T17:35:09.9358650Z +    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9358716Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9358835Z 1 error[E0308]: mismatched types
2019-07-22T17:35:09.9359107Z 2   --> $DIR/issue-12997-2.rs:6:1
2019-07-22T17:35:09.9359160Z 3    |
2019-07-22T17:35:09.9359192Z 
---
2019-07-22T17:35:09.9360959Z 13 
2019-07-22T17:35:09.9361009Z 
2019-07-22T17:35:09.9361038Z 
2019-07-22T17:35:09.9361087Z The actual stderr differed from the expected stderr.
2019-07-22T17:35:09.9361450Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-2/issue-12997-2.stderr
2019-07-22T17:35:09.9361748Z To update references, rerun the tests and pass the `--bless` flag
2019-07-22T17:35:09.9362035Z To only update this specific test, also pass `--test-args issues/issue-12997-2.rs`
2019-07-22T17:35:09.9362138Z error: 1 errors occurred comparing output.
2019-07-22T17:35:09.9362188Z status: exit code: 1
2019-07-22T17:35:09.9362188Z status: exit code: 1
2019-07-22T17:35:09.9363151Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12997-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-2/auxiliary" "-A" "unused"
2019-07-22T17:35:09.9363697Z ------------------------------------------
2019-07-22T17:35:09.9363734Z 
2019-07-22T17:35:09.9364002Z ------------------------------------------
2019-07-22T17:35:09.9364053Z stderr:
2019-07-22T17:35:09.9364053Z stderr:
2019-07-22T17:35:09.9364287Z ------------------------------------------
2019-07-22T17:35:09.9364720Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9365060Z   --> /checkout/src/test/ui/issues/issue-12997-2.rs:6:1
2019-07-22T17:35:09.9365132Z    |
2019-07-22T17:35:09.9365180Z LL | fn bar(x: isize) { }
2019-07-22T17:35:09.9365273Z    |
2019-07-22T17:35:09.9365273Z    |
2019-07-22T17:35:09.9365618Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9365682Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9365719Z 
2019-07-22T17:35:09.9366163Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9366228Z    |
2019-07-22T17:35:09.9366549Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9366612Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9366693Z error[E0308]: mismatched types
2019-07-22T17:35:09.9366988Z   --> /checkout/src/test/ui/issues/issue-12997-2.rs:6:1
2019-07-22T17:35:09.9367038Z    |
2019-07-22T17:35:09.9367038Z    |
2019-07-22T17:35:09.9367084Z LL | fn bar(x: isize) { }
2019-07-22T17:35:09.9367153Z    | ^^^^^^^^^^^^^^^^^^^^ expected isize, found mutable reference
2019-07-22T17:35:09.9367248Z    = note: expected type `isize`
2019-07-22T17:35:09.9367317Z               found type `&mut test::Bencher`
2019-07-22T17:35:09.9367350Z 
2019-07-22T17:35:09.9367398Z error: aborting due to 3 previous errors
---
2019-07-22T17:35:09.9368262Z 
2019-07-22T17:35:09.9368502Z ---- [ui] ui/issues/issue-14772.rs stdout ----
2019-07-22T17:35:09.9368553Z diff of stderr:
2019-07-22T17:35:09.9368595Z 
2019-07-22T17:35:09.9368654Z 4 LL | mod foo {}
2019-07-22T17:35:09.9368744Z 6 
2019-07-22T17:35:09.9368995Z - error: aborting due to previous error
2019-07-22T17:35:09.9368995Z - error: aborting due to previous error
2019-07-22T17:35:09.9369409Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9369475Z +    |
2019-07-22T17:35:09.9369806Z +    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9369881Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9370000Z + error: aborting due to 2 previous errors
2019-07-22T17:35:09.9370045Z + 
2019-07-22T17:35:09.9370339Z + For more information about this error, try `rustc --explain E0658`.
2019-07-22T17:35:09.9370391Z 9 
2019-07-22T17:35:09.9370391Z 9 
2019-07-22T17:35:09.9370420Z 
2019-07-22T17:35:09.9370546Z 
2019-07-22T17:35:09.9370596Z The actual stderr differed from the expected stderr.
2019-07-22T17:35:09.9371349Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14772/issue-14772.stderr
2019-07-22T17:35:09.9371633Z To update references, rerun the tests and pass the `--bless` flag
2019-07-22T17:35:09.9371941Z To only update this specific test, also pass `--test-args issues/issue-14772.rs`
2019-07-22T17:35:09.9372030Z error: 1 errors occurred comparing output.
2019-07-22T17:35:09.9372272Z status: exit code: 1
2019-07-22T17:35:09.9372272Z status: exit code: 1
2019-07-22T17:35:09.9373152Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-14772.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14772" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14772/auxiliary" "-A" "unused"
2019-07-22T17:35:09.9373534Z ------------------------------------------
2019-07-22T17:35:09.9373572Z 
2019-07-22T17:35:09.9373819Z ------------------------------------------
2019-07-22T17:35:09.9373886Z stderr:
2019-07-22T17:35:09.9373886Z stderr:
2019-07-22T17:35:09.9374424Z ------------------------------------------
2019-07-22T17:35:09.9374482Z error: only functions may be used as tests
2019-07-22T17:35:09.9374952Z   --> /checkout/src/test/ui/issues/issue-14772.rs:4:1
2019-07-22T17:35:09.9375008Z    |
2019-07-22T17:35:09.9375061Z LL | mod foo {} //~ ERROR only functions may be used as tests
2019-07-22T17:35:09.9375166Z 
2019-07-22T17:35:09.9375166Z 
2019-07-22T17:35:09.9375608Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9375706Z    |
2019-07-22T17:35:09.9376073Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9376137Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9376236Z error: aborting due to 2 previous errors
2019-07-22T17:35:09.9376268Z 
2019-07-22T17:35:09.9376552Z For more information about this error, try `rustc --explain E0658`.
2019-07-22T17:35:09.9376602Z 
---
2019-07-22T17:35:09.9377420Z 
2019-07-22T17:35:09.9377463Z 4 LL | #![test]
2019-07-22T17:35:09.9377521Z 5    | ^^^^^^^^
2019-07-22T17:35:09.9377565Z 6 
2019-07-22T17:35:09.9377798Z - error: aborting due to previous error
2019-07-22T17:35:09.9378224Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9378305Z +    |
2019-07-22T17:35:09.9378626Z +    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9378708Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9378810Z + error: aborting due to 2 previous errors
2019-07-22T17:35:09.9378871Z + 
2019-07-22T17:35:09.9379158Z + For more information about this error, try `rustc --explain E0658`.
2019-07-22T17:35:09.9379209Z 9 
2019-07-22T17:35:09.9379209Z 9 
2019-07-22T17:35:09.9379238Z 
2019-07-22T17:35:09.9379281Z 
2019-07-22T17:35:09.9379330Z The actual stderr differed from the expected stderr.
2019-07-22T17:35:09.9379655Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28134/issue-28134.stderr
2019-07-22T17:35:09.9380081Z To update references, rerun the tests and pass the `--bless` flag
2019-07-22T17:35:09.9380370Z To only update this specific test, also pass `--test-args issues/issue-28134.rs`
2019-07-22T17:35:09.9380472Z error: 1 errors occurred comparing output.
2019-07-22T17:35:09.9380521Z status: exit code: 1
2019-07-22T17:35:09.9380521Z status: exit code: 1
2019-07-22T17:35:09.9381738Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-28134.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28134" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28134/auxiliary" "-A" "unused"
2019-07-22T17:35:09.9382116Z ------------------------------------------
2019-07-22T17:35:09.9382162Z 
2019-07-22T17:35:09.9382420Z ------------------------------------------
2019-07-22T17:35:09.9382469Z stderr:
2019-07-22T17:35:09.9382469Z stderr:
2019-07-22T17:35:09.9382702Z ------------------------------------------
2019-07-22T17:35:09.9382771Z error: only functions may be used as tests
2019-07-22T17:35:09.9383023Z   --> /checkout/src/test/ui/issues/issue-28134.rs:3:1
2019-07-22T17:35:09.9383074Z    |
2019-07-22T17:35:09.9383144Z LL | #![test] //~ ERROR only functions may be used as tests
2019-07-22T17:35:09.9383231Z 
2019-07-22T17:35:09.9383231Z 
2019-07-22T17:35:09.9383670Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9383739Z    |
2019-07-22T17:35:09.9384067Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9384217Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9384311Z error: aborting due to 2 previous errors
2019-07-22T17:35:09.9384342Z 
2019-07-22T17:35:09.9384635Z For more information about this error, try `rustc --explain E0658`.
2019-07-22T17:35:09.9384672Z 
2019-07-22T17:35:09.9384672Z 
2019-07-22T17:35:09.9384906Z ------------------------------------------
2019-07-22T17:35:09.9384940Z 
2019-07-22T17:35:09.9384983Z 
2019-07-22T17:35:09.9385246Z ---- [ui] ui/issues/issue-53675-a-test-called-panic.rs stdout ----
2019-07-22T17:35:09.9385281Z 
2019-07-22T17:35:09.9385525Z error: test compilation failed although it shouldn't!
2019-07-22T17:35:09.9385786Z status: exit code: 1
2019-07-22T17:35:09.9386680Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-53675-a-test-called-panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-53675-a-test-called-panic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-53675-a-test-called-panic/auxiliary" "-A" "unused"
2019-07-22T17:35:09.9387050Z ------------------------------------------
2019-07-22T17:35:09.9387086Z 
2019-07-22T17:35:09.9387339Z ------------------------------------------
2019-07-22T17:35:09.9387389Z stderr:
2019-07-22T17:35:09.9387389Z stderr:
2019-07-22T17:35:09.9387621Z ------------------------------------------
2019-07-22T17:35:09.9388057Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9388125Z    |
2019-07-22T17:35:09.9388463Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9388526Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9388683Z 
2019-07-22T17:35:09.9389137Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9389504Z    |
2019-07-22T17:35:09.9389551Z LL | /     fn panic() {
2019-07-22T17:35:09.9389615Z LL | |         assert!(true)
2019-07-22T17:35:09.9389662Z LL | |     }
2019-07-22T17:35:09.9389662Z LL | |     }
2019-07-22T17:35:09.9389718Z    | |_____^
2019-07-22T17:35:09.9389777Z    |
2019-07-22T17:35:09.9390092Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9390155Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9390206Z 
2019-07-22T17:35:09.9390979Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9391404Z    |
2019-07-22T17:35:09.9391450Z LL | /     fn panic() {
2019-07-22T17:35:09.9391450Z LL | /     fn panic() {
2019-07-22T17:35:09.9391497Z LL | |         assert!(true);
2019-07-22T17:35:09.9391606Z    | |_____^
2019-07-22T17:35:09.9391649Z    |
2019-07-22T17:35:09.9391649Z    |
2019-07-22T17:35:09.9391987Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9392062Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9392100Z 
2019-07-22T17:35:09.9392521Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9392890Z    |
2019-07-22T17:35:09.9392936Z LL | /     fn panic() {
2019-07-22T17:35:09.9392936Z LL | /     fn panic() {
2019-07-22T17:35:09.9392999Z LL | |         panic!("in expr")
2019-07-22T17:35:09.9393090Z    | |_____^
2019-07-22T17:35:09.9393149Z    |
2019-07-22T17:35:09.9393149Z    |
2019-07-22T17:35:09.9393456Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9393523Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9393577Z 
2019-07-22T17:35:09.9394141Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9394578Z    |
2019-07-22T17:35:09.9394628Z LL | /     fn panic() {
2019-07-22T17:35:09.9394628Z LL | /     fn panic() {
2019-07-22T17:35:09.9394680Z LL | |         panic!("in stmt");
2019-07-22T17:35:09.9394804Z    | |_____^
2019-07-22T17:35:09.9394850Z    |
2019-07-22T17:35:09.9394850Z    |
2019-07-22T17:35:09.9395209Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9395276Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9395365Z error: aborting due to 5 previous errors
2019-07-22T17:35:09.9395416Z 
2019-07-22T17:35:09.9395719Z For more information about this error, try `rustc --explain E0658`.
2019-07-22T17:35:09.9395758Z 
---
2019-07-22T17:35:09.9396427Z diff of stderr:
2019-07-22T17:35:09.9396460Z 
2019-07-22T17:35:09.9396723Z - error: cannot test inner items
2019-07-22T17:35:09.9396977Z -   --> $DIR/test-inner-fn.rs:5:5
2019-07-22T17:35:09.9397419Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9397845Z +   --> $DIR/test-inner-fn.rs:4:1
2019-07-22T17:35:09.9397900Z 3    |
2019-07-22T17:35:09.9398153Z - LL |     #[test]
2019-07-22T17:35:09.9398437Z + LL | / fn foo() {
2019-07-22T17:35:09.9398488Z + LL | |     #[test]
2019-07-22T17:35:09.9398488Z + LL | |     #[test]
2019-07-22T17:35:09.9398581Z + LL | |     fn bar() {}
2019-07-22T17:35:09.9398630Z + LL | |     bar();
2019-07-22T17:35:09.9398678Z + LL | | }
2019-07-22T17:35:09.9398800Z 6    |
2019-07-22T17:35:09.9398800Z 6    |
2019-07-22T17:35:09.9399095Z -    = note: requested on the command line with `-D unnameable-test-items`
2019-07-22T17:35:09.9399458Z +    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9399526Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9399850Z - error: cannot test inner items
2019-07-22T17:35:09.9400116Z -   --> $DIR/test-inner-fn.rs:13:9
2019-07-22T17:35:09.9400116Z -   --> $DIR/test-inner-fn.rs:13:9
2019-07-22T17:35:09.9400549Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9401298Z +   --> $DIR/test-inner-fn.rs:6:5
2019-07-22T17:35:09.9401366Z 11    |
2019-07-22T17:35:09.9401594Z - LL |         #[test]
2019-07-22T17:35:09.9401832Z -    |         ^^^^^^^
2019-07-22T17:35:09.9401883Z + LL |     fn bar() {}
2019-07-22T17:35:09.9402003Z +    |
2019-07-22T17:35:09.9402003Z +    |
2019-07-22T17:35:09.9402583Z +    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9402659Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9403035Z - error: aborting due to 2 previous errors
2019-07-22T17:35:09.9403035Z - error: aborting due to 2 previous errors
2019-07-22T17:35:09.9403455Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9403551Z +    |
2019-07-22T17:35:09.9403864Z +    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9403928Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9403994Z 16 
2019-07-22T17:35:09.9404549Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9404844Z +   --> $DIR/test-inner-fn.rs:12:5
2019-07-22T17:35:09.9404957Z + LL | /     fn foo() {
2019-07-22T17:35:09.9405005Z + LL | |         #[test]
2019-07-22T17:35:09.9405005Z + LL | |         #[test]
2019-07-22T17:35:09.9405068Z + LL | |         fn bar() {}
2019-07-22T17:35:09.9405117Z + LL | |         bar();
2019-07-22T17:35:09.9405174Z + LL | |     }
2019-07-22T17:35:09.9405280Z +    |
2019-07-22T17:35:09.9405280Z +    |
2019-07-22T17:35:09.9405663Z +    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9405741Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9405792Z + 
2019-07-22T17:35:09.9406212Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9406488Z +   --> $DIR/test-inner-fn.rs:14:9
2019-07-22T17:35:09.9406540Z +    |
2019-07-22T17:35:09.9406587Z + LL |         fn bar() {}
2019-07-22T17:35:09.9406696Z +    |
2019-07-22T17:35:09.9406696Z +    |
2019-07-22T17:35:09.9407002Z +    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9407081Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9407286Z + error: aborting due to 5 previous errors
2019-07-22T17:35:09.9407331Z + 
2019-07-22T17:35:09.9407653Z + For more information about this error, try `rustc --explain E0658`.
2019-07-22T17:35:09.9407706Z 17 
2019-07-22T17:35:09.9407706Z 17 
2019-07-22T17:35:09.9407737Z 
2019-07-22T17:35:09.9407767Z 
2019-07-22T17:35:09.9407835Z The actual stderr differed from the expected stderr.
2019-07-22T17:35:09.9408156Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/test-inner-fn/test-inner-fn.stderr
2019-07-22T17:35:09.9408437Z To update references, rerun the tests and pass the `--bless` flag
2019-07-22T17:35:09.9408735Z To only update this specific test, also pass `--test-args lint/test-inner-fn.rs`
2019-07-22T17:35:09.9408822Z error: 1 errors occurred comparing output.
2019-07-22T17:35:09.9408887Z status: exit code: 1
2019-07-22T17:35:09.9408887Z status: exit code: 1
2019-07-22T17:35:09.9409699Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/test-inner-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/test-inner-fn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-D" "unnameable_test_items" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/test-inner-fn/auxiliary" "-A" "unused"
2019-07-22T17:35:09.9410068Z ------------------------------------------
2019-07-22T17:35:09.9410115Z 
2019-07-22T17:35:09.9410373Z ------------------------------------------
2019-07-22T17:35:09.9410423Z stderr:
2019-07-22T17:35:09.9410423Z stderr:
2019-07-22T17:35:09.9410657Z ------------------------------------------
2019-07-22T17:35:09.9411470Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9411851Z    |
2019-07-22T17:35:09.9411916Z LL | / fn foo() {
2019-07-22T17:35:09.9411916Z LL | / fn foo() {
2019-07-22T17:35:09.9411972Z LL | |     #[test] //~ ERROR cannot test inner items [unnameable_test_items]
2019-07-22T17:35:09.9412025Z LL | |     fn bar() {}
2019-07-22T17:35:09.9412072Z LL | |     bar();
2019-07-22T17:35:09.9412180Z    | |_^
2019-07-22T17:35:09.9412223Z    |
2019-07-22T17:35:09.9412223Z    |
2019-07-22T17:35:09.9412572Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9412754Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9412801Z 
2019-07-22T17:35:09.9413277Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9413610Z    |
2019-07-22T17:35:09.9413610Z    |
2019-07-22T17:35:09.9413672Z LL |     fn bar() {}
2019-07-22T17:35:09.9413764Z    |
2019-07-22T17:35:09.9413764Z    |
2019-07-22T17:35:09.9414166Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9414228Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9414265Z 
2019-07-22T17:35:09.9414708Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9414773Z    |
2019-07-22T17:35:09.9415076Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9415154Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9415189Z 
2019-07-22T17:35:09.9415602Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9416088Z    |
2019-07-22T17:35:09.9416137Z LL | /     fn foo() {
2019-07-22T17:35:09.9416137Z LL | /     fn foo() {
2019-07-22T17:35:09.9416211Z LL | |         #[test] //~ ERROR cannot test inner items [unnameable_test_items]
2019-07-22T17:35:09.9416267Z LL | |         fn bar() {}
2019-07-22T17:35:09.9416317Z LL | |         bar();
2019-07-22T17:35:09.9416431Z    | |_____^
2019-07-22T17:35:09.9416477Z    |
2019-07-22T17:35:09.9416477Z    |
2019-07-22T17:35:09.9416830Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9416912Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9416952Z 
2019-07-22T17:35:09.9417396Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9417777Z    |
2019-07-22T17:35:09.9417777Z    |
2019-07-22T17:35:09.9417826Z LL |         fn bar() {}
2019-07-22T17:35:09.9417940Z    |
2019-07-22T17:35:09.9417940Z    |
2019-07-22T17:35:09.9418270Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9418351Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9418439Z error: aborting due to 5 previous errors
2019-07-22T17:35:09.9418483Z 
2019-07-22T17:35:09.9418862Z For more information about this error, try `rustc --explain E0658`.
2019-07-22T17:35:09.9418902Z 
---
2019-07-22T17:35:09.9419624Z 
2019-07-22T17:35:09.9419687Z 4 LL |     NonExistent;
2019-07-22T17:35:09.9419740Z 5    |     ^^^^^^^^^^^ not found in this scope
2019-07-22T17:35:09.9419790Z 6 
2019-07-22T17:35:09.9420045Z - error: aborting due to previous error
2019-07-22T17:35:09.9420499Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9420569Z +    |
2019-07-22T17:35:09.9421551Z +    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9421636Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9422044Z - For more information about this error, try `rustc --explain E0425`.
2019-07-22T17:35:09.9422103Z + error: aborting due to 2 previous errors
2019-07-22T17:35:09.9422149Z + 
2019-07-22T17:35:09.9422216Z + Some errors have detailed explanations: E0425, E0658.
2019-07-22T17:35:09.9422216Z + Some errors have detailed explanations: E0425, E0658.
2019-07-22T17:35:09.9422501Z + For more information about an error, try `rustc --explain E0425`.
2019-07-22T17:35:09.9422554Z 10 
2019-07-22T17:35:09.9422601Z 
2019-07-22T17:35:09.9422630Z 
2019-07-22T17:35:09.9422679Z The actual stderr differed from the expected stderr.
2019-07-22T17:35:09.9423037Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs-test/ambiguous-builtin-attrs-test.stderr
2019-07-22T17:35:09.9423328Z To update references, rerun the tests and pass the `--bless` flag
2019-07-22T17:35:09.9423643Z To only update this specific test, also pass `--test-args proc-macro/ambiguous-builtin-attrs-test.rs`
2019-07-22T17:35:09.9423749Z error: 1 errors occurred comparing output.
2019-07-22T17:35:09.9423797Z status: exit code: 1
2019-07-22T17:35:09.9423797Z status: exit code: 1
2019-07-22T17:35:09.9424649Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/ambiguous-builtin-attrs-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs-test" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs-test/auxiliary" "-A" "unused"
2019-07-22T17:35:09.9425147Z ------------------------------------------
2019-07-22T17:35:09.9425212Z 
2019-07-22T17:35:09.9425456Z ------------------------------------------
2019-07-22T17:35:09.9425506Z stderr:
2019-07-22T17:35:09.9425506Z stderr:
2019-07-22T17:35:09.9425752Z ------------------------------------------
2019-07-22T17:35:09.9425809Z error[E0425]: cannot find value `NonExistent` in this scope
2019-07-22T17:35:09.9426171Z    |
2019-07-22T17:35:09.9426171Z    |
2019-07-22T17:35:09.9426226Z LL |     NonExistent; //~ ERROR cannot find value `NonExistent` in this scope
2019-07-22T17:35:09.9426314Z 
2019-07-22T17:35:09.9426314Z 
2019-07-22T17:35:09.9426744Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9426812Z    |
2019-07-22T17:35:09.9427144Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9427217Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9427300Z error: aborting due to 2 previous errors
2019-07-22T17:35:09.9427349Z 
2019-07-22T17:35:09.9427397Z Some errors have detailed explanations: E0425, E0658.
2019-07-22T17:35:09.9427678Z For more information about an error, try `rustc --explain E0425`.
---
2019-07-22T17:35:09.9428430Z 
2019-07-22T17:35:09.9428473Z 8 LL | | }
2019-07-22T17:35:09.9428576Z 9    | |_^
2019-07-22T17:35:09.9428619Z 10 
2019-07-22T17:35:09.9428855Z - error: aborting due to previous error
2019-07-22T17:35:09.9429367Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9429701Z +   --> $DIR/termination-trait-in-test-should-panic.rs:7:5
2019-07-22T17:35:09.9429823Z + LL | use test::Bencher;
2019-07-22T17:35:09.9429871Z +    |     ^^^^^^^^^^^^^
2019-07-22T17:35:09.9429927Z +    |
2019-07-22T17:35:09.9429927Z +    |
2019-07-22T17:35:09.9430261Z +    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9430326Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9430378Z 12 
2019-07-22T17:35:09.9431140Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9431219Z +    |
2019-07-22T17:35:09.9431610Z +    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9431689Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9431801Z + error: aborting due to 3 previous errors
2019-07-22T17:35:09.9431847Z + 
2019-07-22T17:35:09.9432128Z + For more information about this error, try `rustc --explain E0658`.
2019-07-22T17:35:09.9432180Z 13 
2019-07-22T17:35:09.9432180Z 13 
2019-07-22T17:35:09.9432226Z 
2019-07-22T17:35:09.9432384Z 
2019-07-22T17:35:09.9432434Z The actual stderr differed from the expected stderr.
2019-07-22T17:35:09.9432868Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test-should-panic/termination-trait-in-test-should-panic.stderr
2019-07-22T17:35:09.9433167Z To update references, rerun the tests and pass the `--bless` flag
2019-07-22T17:35:09.9433499Z To only update this specific test, also pass `--test-args rfc-1937-termination-trait/termination-trait-in-test-should-panic.rs`
2019-07-22T17:35:09.9433618Z error: 1 errors occurred comparing output.
2019-07-22T17:35:09.9433669Z status: exit code: 1
2019-07-22T17:35:09.9433669Z status: exit code: 1
2019-07-22T17:35:09.9434613Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-in-test-should-panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test-should-panic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test-should-panic/auxiliary" "-A" "unused"
2019-07-22T17:35:09.9435029Z ------------------------------------------
2019-07-22T17:35:09.9435069Z 
2019-07-22T17:35:09.9435341Z ------------------------------------------
2019-07-22T17:35:09.9435394Z stderr:
2019-07-22T17:35:09.9435394Z stderr:
2019-07-22T17:35:09.9435670Z ------------------------------------------
2019-07-22T17:35:09.9435730Z error: functions using `#[should_panic]` must return `()`
2019-07-22T17:35:09.9436142Z    |
2019-07-22T17:35:09.9436142Z    |
2019-07-22T17:35:09.9436416Z LL | / fn not_a_num() -> Result<(), ParseIntError> {
2019-07-22T17:35:09.9436491Z LL | |     //~^ ERROR functions using `#[should_panic]` must return `()`
2019-07-22T17:35:09.9436564Z LL | |     let _: u32 = "abc".parse()?;
2019-07-22T17:35:09.9436615Z LL | |     Ok(())
2019-07-22T17:35:09.9436727Z    | |_^
2019-07-22T17:35:09.9436757Z 
2019-07-22T17:35:09.9436757Z 
2019-07-22T17:35:09.9437192Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9437717Z    |
2019-07-22T17:35:09.9437765Z LL | use test::Bencher;
2019-07-22T17:35:09.9437827Z    |     ^^^^^^^^^^^^^
2019-07-22T17:35:09.9437876Z    |
2019-07-22T17:35:09.9437876Z    |
2019-07-22T17:35:09.9438250Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9438346Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9438386Z 
2019-07-22T17:35:09.9438830Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9438913Z    |
2019-07-22T17:35:09.9439242Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9439307Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9439421Z error: aborting due to 3 previous errors
2019-07-22T17:35:09.9439454Z 
2019-07-22T17:35:09.9439756Z For more information about this error, try `rustc --explain E0658`.
2019-07-22T17:35:09.9439796Z 
2019-07-22T17:35:09.9439796Z 
2019-07-22T17:35:09.9440070Z ------------------------------------------
2019-07-22T17:35:09.9440105Z 
2019-07-22T17:35:09.9440135Z 
2019-07-22T17:35:09.9440433Z ---- [ui] ui/rfc-1937-termination-trait/termination-trait-in-test.rs stdout ----
2019-07-22T17:35:09.9440927Z 
2019-07-22T17:35:09.9441273Z error: test compilation failed although it shouldn't!
2019-07-22T17:35:09.9441329Z status: exit code: 1
2019-07-22T17:35:09.9442258Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-in-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test/auxiliary" "-A" "unused"
2019-07-22T17:35:09.9442809Z ------------------------------------------
2019-07-22T17:35:09.9442858Z 
2019-07-22T17:35:09.9443099Z ------------------------------------------
2019-07-22T17:35:09.9443165Z stderr:
2019-07-22T17:35:09.9443165Z stderr:
2019-07-22T17:35:09.9443400Z ------------------------------------------
2019-07-22T17:35:09.9443814Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9444223Z    |
2019-07-22T17:35:09.9444280Z LL | use test::Bencher;
2019-07-22T17:35:09.9444343Z    |     ^^^^^^^^^^^^^
2019-07-22T17:35:09.9444390Z    |
2019-07-22T17:35:09.9444390Z    |
2019-07-22T17:35:09.9444722Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9444803Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9444841Z 
2019-07-22T17:35:09.9445286Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9445757Z    |
2019-07-22T17:35:09.9445757Z    |
2019-07-22T17:35:09.9446024Z LL | / fn is_a_num() -> Result<(), ParseIntError> {
2019-07-22T17:35:09.9446097Z LL | |     let _: u32 = "22".parse()?;
2019-07-22T17:35:09.9446151Z LL | |     Ok(())
2019-07-22T17:35:09.9446248Z    | |_^
2019-07-22T17:35:09.9446309Z    |
2019-07-22T17:35:09.9446309Z    |
2019-07-22T17:35:09.9446838Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9446916Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9446973Z 
2019-07-22T17:35:09.9447455Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9447874Z    |
2019-07-22T17:35:09.9447874Z    |
2019-07-22T17:35:09.9448168Z LL | / fn test_a_positive_bench(_: &mut Bencher) -> Result<(), ParseIntError> {
2019-07-22T17:35:09.9448227Z LL | |     Ok(())
2019-07-22T17:35:09.9448340Z    | |_^
2019-07-22T17:35:09.9448385Z    |
2019-07-22T17:35:09.9448385Z    |
2019-07-22T17:35:09.9448740Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9448818Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9448856Z 
2019-07-22T17:35:09.9449316Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9449824Z    |
2019-07-22T17:35:09.9449824Z    |
2019-07-22T17:35:09.9450166Z LL | fn test_a_positive_bench(_: &mut Bencher) -> Result<(), ParseIntError> {
2019-07-22T17:35:09.9450278Z    |
2019-07-22T17:35:09.9450278Z    |
2019-07-22T17:35:09.9450626Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9450691Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9451124Z 
2019-07-22T17:35:09.9452028Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9452412Z    |
2019-07-22T17:35:09.9452412Z    |
2019-07-22T17:35:09.9452696Z LL | / fn test_a_neg_bench(_: &mut Bencher) -> Result<(), ParseIntError> {
2019-07-22T17:35:09.9452755Z LL | |     let _: u32 = "abc".parse()?;
2019-07-22T17:35:09.9452815Z LL | |     Ok(())
2019-07-22T17:35:09.9452924Z    | |_^
2019-07-22T17:35:09.9452968Z    |
2019-07-22T17:35:09.9452968Z    |
2019-07-22T17:35:09.9453310Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9453376Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9453413Z 
2019-07-22T17:35:09.9453843Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9454224Z    |
2019-07-22T17:35:09.9454224Z    |
2019-07-22T17:35:09.9454486Z LL | fn test_a_neg_bench(_: &mut Bencher) -> Result<(), ParseIntError> {
2019-07-22T17:35:09.9454670Z    |
2019-07-22T17:35:09.9454670Z    |
2019-07-22T17:35:09.9454972Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9455064Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9455101Z 
2019-07-22T17:35:09.9455520Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9455601Z    |
2019-07-22T17:35:09.9455901Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9456104Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9456198Z error: aborting due to 7 previous errors
2019-07-22T17:35:09.9456230Z 
2019-07-22T17:35:09.9456560Z For more information about this error, try `rustc --explain E0658`.
2019-07-22T17:35:09.9456598Z 
2019-07-22T17:35:09.9456598Z 
2019-07-22T17:35:09.9456837Z ------------------------------------------
2019-07-22T17:35:09.9456871Z 
2019-07-22T17:35:09.9456911Z 
2019-07-22T17:35:09.9457214Z ---- [ui] ui/rfc-1937-termination-trait/termination-trait-test-wrong-type.rs stdout ----
2019-07-22T17:35:09.9457270Z diff of stderr:
2019-07-22T17:35:09.9457301Z 
2019-07-22T17:35:09.9457726Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9457998Z +   --> $DIR/termination-trait-test-wrong-type.rs:6:1
2019-07-22T17:35:09.9458052Z +    |
2019-07-22T17:35:09.9458348Z + LL | / fn can_parse_zero_as_f32() -> Result<f32, ParseFloatError> {
2019-07-22T17:35:09.9458406Z + LL | |     "0".parse()
2019-07-22T17:35:09.9458457Z + LL | | }
2019-07-22T17:35:09.9458632Z +    |
2019-07-22T17:35:09.9458632Z +    |
2019-07-22T17:35:09.9458954Z +    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9459037Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9459191Z + 
2019-07-22T17:35:09.9459663Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9459751Z +    |
2019-07-22T17:35:09.9460093Z +    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9460175Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9460228Z + 
2019-07-22T17:35:09.9460298Z 1 error[E0277]: `main` has invalid return type `std::result::Result<f32, std::num::ParseFloatError>`
2019-07-22T17:35:09.9460664Z 3    |
2019-07-22T17:35:09.9460695Z 
2019-07-22T17:35:09.9460695Z 
2019-07-22T17:35:09.9461073Z 9    = help: the trait `std::process::Termination` is not implemented for `std::result::Result<f32, std::num::ParseFloatError>`
2019-07-22T17:35:09.9461172Z 10    = note: required by `test::assert_test_result`
2019-07-22T17:35:09.9461542Z - error: aborting due to previous error
2019-07-22T17:35:09.9461615Z + error: aborting due to 3 previous errors
2019-07-22T17:35:09.9461662Z 13 
2019-07-22T17:35:09.9461932Z - For more information about this error, try `rustc --explain E0277`.
2019-07-22T17:35:09.9461932Z - For more information about this error, try `rustc --explain E0277`.
2019-07-22T17:35:09.9462006Z + Some errors have detailed explanations: E0277, E0658.
2019-07-22T17:35:09.9462274Z + For more information about an error, try `rustc --explain E0277`.
2019-07-22T17:35:09.9462326Z 15 
2019-07-22T17:35:09.9462355Z 
2019-07-22T17:35:09.9462383Z 
2019-07-22T17:35:09.9462459Z The actual stderr differed from the expected stderr.
2019-07-22T17:35:09.9462849Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/termination-trait-test-wrong-type.stderr
2019-07-22T17:35:09.9463142Z To update references, rerun the tests and pass the `--bless` flag
2019-07-22T17:35:09.9463483Z To only update this specific test, also pass `--test-args rfc-1937-termination-trait/termination-trait-test-wrong-type.rs`
2019-07-22T17:35:09.9463572Z error: 1 errors occurred comparing output.
2019-07-22T17:35:09.9463638Z status: exit code: 1
2019-07-22T17:35:09.9463638Z status: exit code: 1
2019-07-22T17:35:09.9464683Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/auxiliary" "-A" "unused"
2019-07-22T17:35:09.9465107Z ------------------------------------------
2019-07-22T17:35:09.9465145Z 
2019-07-22T17:35:09.9465400Z ------------------------------------------
2019-07-22T17:35:09.9465450Z stderr:
2019-07-22T17:35:09.9465450Z stderr:
2019-07-22T17:35:09.9465683Z ------------------------------------------
2019-07-22T17:35:09.9466110Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9466514Z    |
2019-07-22T17:35:09.9466514Z    |
2019-07-22T17:35:09.9466794Z LL | / fn can_parse_zero_as_f32() -> Result<f32, ParseFloatError> { //~ ERROR
2019-07-22T17:35:09.9466850Z LL | |     "0".parse()
2019-07-22T17:35:09.9466957Z    | |_^
2019-07-22T17:35:09.9467001Z    |
2019-07-22T17:35:09.9467001Z    |
2019-07-22T17:35:09.9467327Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9467508Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9467545Z 
2019-07-22T17:35:09.9467995Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9468092Z    |
2019-07-22T17:35:09.9468402Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9468464Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9468528Z 
2019-07-22T17:35:09.9468585Z error[E0277]: `main` has invalid return type `std::result::Result<f32, std::num::ParseFloatError>`
2019-07-22T17:35:09.9468974Z    |
2019-07-22T17:35:09.9468974Z    |
2019-07-22T17:35:09.9469244Z LL | / fn can_parse_zero_as_f32() -> Result<f32, ParseFloatError> { //~ ERROR
2019-07-22T17:35:09.9469309Z LL | |     "0".parse()
2019-07-22T17:35:09.9469370Z LL | | }
2019-07-22T17:35:09.9469423Z    | |_^ `main` can only return types that implement `std::process::Termination`
2019-07-22T17:35:09.9469472Z    |
2019-07-22T17:35:09.9469546Z    = help: the trait `std::process::Termination` is not implemented for `std::result::Result<f32, std::num::ParseFloatError>`
2019-07-22T17:35:09.9469608Z    = note: required by `test::assert_test_result`
2019-07-22T17:35:09.9469688Z error: aborting due to 3 previous errors
2019-07-22T17:35:09.9469737Z 
2019-07-22T17:35:09.9469794Z Some errors have detailed explanations: E0277, E0658.
2019-07-22T17:35:09.9470066Z For more information about an error, try `rustc --explain E0277`.
2019-07-22T17:35:09.9470066Z For more information about an error, try `rustc --explain E0277`.
2019-07-22T17:35:09.9470102Z 
2019-07-22T17:35:09.9470354Z ------------------------------------------
2019-07-22T17:35:09.9470388Z 
2019-07-22T17:35:09.9470416Z 
2019-07-22T17:35:09.9470658Z ---- [ui] ui/rustc_private-libtest.rs stdout ----
2019-07-22T17:35:09.9471038Z diff of stderr:
2019-07-22T17:35:09.9471080Z 
2019-07-22T17:35:09.9471129Z 4 LL |     use libtest::*;
2019-07-22T17:35:09.9471181Z 5    |         ^^^^^^^ maybe a missing `extern crate libtest;`?
2019-07-22T17:35:09.9471544Z - error: aborting due to previous error
2019-07-22T17:35:09.9471544Z - error: aborting due to previous error
2019-07-22T17:35:09.9471958Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9472352Z +   --> $DIR/rustc_private-libtest.rs:2:5
2019-07-22T17:35:09.9472465Z + LL |     extern crate libtest;
2019-07-22T17:35:09.9472532Z +    |     ^^^^^^^^^^^^^^^^^^^^^
2019-07-22T17:35:09.9472579Z +    |
2019-07-22T17:35:09.9472579Z +    |
2019-07-22T17:35:09.9472940Z +    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9473021Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9473389Z - For more information about this error, try `rustc --explain E0432`.
2019-07-22T17:35:09.9473448Z + error: aborting due to 2 previous errors
2019-07-22T17:35:09.9473494Z + 
2019-07-22T17:35:09.9473544Z + Some errors have detailed explanations: E0432, E0658.
2019-07-22T17:35:09.9473544Z + Some errors have detailed explanations: E0432, E0658.
2019-07-22T17:35:09.9473828Z + For more information about an error, try `rustc --explain E0432`.
2019-07-22T17:35:09.9473880Z 10 
2019-07-22T17:35:09.9473910Z 
2019-07-22T17:35:09.9473939Z 
2019-07-22T17:35:09.9474004Z The actual stderr differed from the expected stderr.
2019-07-22T17:35:09.9474345Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc_private-libtest/rustc_private-libtest.stderr
2019-07-22T17:35:09.9474619Z To update references, rerun the tests and pass the `--bless` flag
2019-07-22T17:35:09.9474921Z To only update this specific test, also pass `--test-args rustc_private-libtest.rs`
2019-07-22T17:35:09.9475008Z error: 1 errors occurred comparing output.
2019-07-22T17:35:09.9475173Z status: exit code: 1
2019-07-22T17:35:09.9475173Z status: exit code: 1
2019-07-22T17:35:09.9475995Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rustc_private-libtest.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc_private-libtest" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc_private-libtest/auxiliary" "-A" "unused"
2019-07-22T17:35:09.9476352Z ------------------------------------------
2019-07-22T17:35:09.9476388Z 
2019-07-22T17:35:09.9476640Z ------------------------------------------
2019-07-22T17:35:09.9476691Z stderr:
2019-07-22T17:35:09.9476691Z stderr:
2019-07-22T17:35:09.9476925Z ------------------------------------------
2019-07-22T17:35:09.9477005Z error[E0432]: unresolved import `libtest`
2019-07-22T17:35:09.9477262Z   --> /checkout/src/test/ui/rustc_private-libtest.rs:3:9
2019-07-22T17:35:09.9477316Z    |
2019-07-22T17:35:09.9477369Z LL |     use libtest::*; //~ ERROR unresolved import
2019-07-22T17:35:09.9477442Z    |         ^^^^^^^ maybe a missing `extern crate libtest;`?
2019-07-22T17:35:09.9477477Z 
2019-07-22T17:35:09.9477885Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9478241Z    |
2019-07-22T17:35:09.9478288Z LL |     extern crate libtest;
2019-07-22T17:35:09.9478352Z    |     ^^^^^^^^^^^^^^^^^^^^^
2019-07-22T17:35:09.9478398Z    |
2019-07-22T17:35:09.9478398Z    |
2019-07-22T17:35:09.9478851Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9478939Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9479041Z error: aborting due to 2 previous errors
2019-07-22T17:35:09.9479090Z 
2019-07-22T17:35:09.9479143Z Some errors have detailed explanations: E0432, E0658.
2019-07-22T17:35:09.9479467Z For more information about an error, try `rustc --explain E0432`.
2019-07-22T17:35:09.9479467Z For more information about an error, try `rustc --explain E0432`.
2019-07-22T17:35:09.9479506Z 
2019-07-22T17:35:09.9479782Z ------------------------------------------
2019-07-22T17:35:09.9479818Z 
2019-07-22T17:35:09.9479850Z 
2019-07-22T17:35:09.9480105Z ---- [ui] ui/test-on-macro.rs stdout ----
2019-07-22T17:35:09.9480141Z 
2019-07-22T17:35:09.9480522Z error: test compilation failed although it shouldn't!
2019-07-22T17:35:09.9480587Z status: exit code: 1
2019-07-22T17:35:09.9481788Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-on-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-on-macro" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-on-macro/auxiliary" "-A" "unused"
2019-07-22T17:35:09.9482176Z ------------------------------------------
2019-07-22T17:35:09.9482228Z 
2019-07-22T17:35:09.9482664Z ------------------------------------------
2019-07-22T17:35:09.9482721Z stderr:
2019-07-22T17:35:09.9482721Z stderr:
2019-07-22T17:35:09.9482976Z ------------------------------------------
2019-07-22T17:35:09.9483048Z warning: `#[test]` attribute should not be used on macros. Use `#[cfg(test)]` instead.
2019-07-22T17:35:09.9483381Z    |
2019-07-22T17:35:09.9483428Z LL | foo!();
2019-07-22T17:35:09.9483474Z    | ^^^^^^^
2019-07-22T17:35:09.9483505Z 
2019-07-22T17:35:09.9483505Z 
2019-07-22T17:35:09.9483931Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9484136Z    |
2019-07-22T17:35:09.9484523Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9484589Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9484672Z error: aborting due to previous error
2019-07-22T17:35:09.9484721Z 
2019-07-22T17:35:09.9484998Z For more information about this error, try `rustc --explain E0658`.
2019-07-22T17:35:09.9485045Z 
2019-07-22T17:35:09.9485045Z 
2019-07-22T17:35:09.9485279Z ------------------------------------------
2019-07-22T17:35:09.9485329Z 
2019-07-22T17:35:09.9485357Z 
2019-07-22T17:35:09.9485598Z ---- [ui] ui/test-should-panic-attr.rs stdout ----
2019-07-22T17:35:09.9485632Z 
2019-07-22T17:35:09.9485892Z error: test compilation failed although it shouldn't!
2019-07-22T17:35:09.9485945Z status: exit code: 1
2019-07-22T17:35:09.9486755Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-should-panic-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-should-panic-attr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-should-panic-attr/auxiliary" "-A" "unused"
2019-07-22T17:35:09.9487122Z ------------------------------------------
2019-07-22T17:35:09.9487157Z 
2019-07-22T17:35:09.9487410Z ------------------------------------------
2019-07-22T17:35:09.9487460Z stderr:
2019-07-22T17:35:09.9487460Z stderr:
2019-07-22T17:35:09.9487691Z ------------------------------------------
2019-07-22T17:35:09.9487766Z warning: argument must be of the form: `expected = "error message"`
2019-07-22T17:35:09.9488094Z    |
2019-07-22T17:35:09.9488094Z    |
2019-07-22T17:35:09.9488158Z LL | #[should_panic(expected)]
2019-07-22T17:35:09.9488251Z    |
2019-07-22T17:35:09.9488324Z    = note: Errors in this attribute were erroneously allowed and will become a hard error in a future release.
2019-07-22T17:35:09.9488367Z 
2019-07-22T17:35:09.9488367Z 
2019-07-22T17:35:09.9488419Z warning: argument must be of the form: `expected = "error message"`
2019-07-22T17:35:09.9488857Z    |
2019-07-22T17:35:09.9488857Z    |
2019-07-22T17:35:09.9488904Z LL | #[should_panic(expect)]
2019-07-22T17:35:09.9489012Z    |
2019-07-22T17:35:09.9489070Z    = note: Errors in this attribute were erroneously allowed and will become a hard error in a future release.
2019-07-22T17:35:09.9489110Z 
2019-07-22T17:35:09.9489110Z 
2019-07-22T17:35:09.9489176Z warning: argument must be of the form: `expected = "error message"`
2019-07-22T17:35:09.9489529Z    |
2019-07-22T17:35:09.9489529Z    |
2019-07-22T17:35:09.9489577Z LL | #[should_panic(expected(foo, bar))]
2019-07-22T17:35:09.9489688Z    |
2019-07-22T17:35:09.9489744Z    = note: Errors in this attribute were erroneously allowed and will become a hard error in a future release.
2019-07-22T17:35:09.9489799Z 
2019-07-22T17:35:09.9489799Z 
2019-07-22T17:35:09.9489849Z warning: argument must be of the form: `expected = "error message"`
2019-07-22T17:35:09.9490187Z    |
2019-07-22T17:35:09.9490187Z    |
2019-07-22T17:35:09.9490236Z LL | #[should_panic(expected = "foo", bar)]
2019-07-22T17:35:09.9490334Z    |
2019-07-22T17:35:09.9490408Z    = note: Errors in this attribute were erroneously allowed and will become a hard error in a future release.
2019-07-22T17:35:09.9490527Z 
2019-07-22T17:35:09.9490527Z 
2019-07-22T17:35:09.9491351Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9491715Z    |
2019-07-22T17:35:09.9491715Z    |
2019-07-22T17:35:09.9491760Z LL | / fn test1() {
2019-07-22T17:35:09.9491823Z LL | |     panic!();
2019-07-22T17:35:09.9491914Z    | |_^
2019-07-22T17:35:09.9491985Z    |
2019-07-22T17:35:09.9491985Z    |
2019-07-22T17:35:09.9492315Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9492378Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9492432Z 
2019-07-22T17:35:09.9492853Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9493205Z    |
2019-07-22T17:35:09.9493205Z    |
2019-07-22T17:35:09.9493251Z LL | / fn test2() {
2019-07-22T17:35:09.9493298Z LL | |     panic!();
2019-07-22T17:35:09.9493406Z    | |_^
2019-07-22T17:35:09.9493449Z    |
2019-07-22T17:35:09.9493449Z    |
2019-07-22T17:35:09.9493769Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9493832Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9493869Z 
2019-07-22T17:35:09.9494293Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9494631Z    |
2019-07-22T17:35:09.9494631Z    |
2019-07-22T17:35:09.9494677Z LL | / fn test3() {
2019-07-22T17:35:09.9494750Z LL | |     panic!();
2019-07-22T17:35:09.9494839Z    | |_^
2019-07-22T17:35:09.9494897Z    |
2019-07-22T17:35:09.9494897Z    |
2019-07-22T17:35:09.9495200Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9495262Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9495314Z 
2019-07-22T17:35:09.9495729Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9496193Z    |
2019-07-22T17:35:09.9496193Z    |
2019-07-22T17:35:09.9496239Z LL | / fn test4() {
2019-07-22T17:35:09.9496287Z LL | |     panic!();
2019-07-22T17:35:09.9496391Z    | |_^
2019-07-22T17:35:09.9496434Z    |
2019-07-22T17:35:09.9496434Z    |
2019-07-22T17:35:09.9496777Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9496870Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9496907Z 
2019-07-22T17:35:09.9497327Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9497665Z    |
2019-07-22T17:35:09.9497665Z    |
2019-07-22T17:35:09.9497709Z LL | / fn test5() {
2019-07-22T17:35:09.9497772Z LL | |     panic!();
2019-07-22T17:35:09.9497871Z    | |_^
2019-07-22T17:35:09.9497913Z    |
2019-07-22T17:35:09.9497913Z    |
2019-07-22T17:35:09.9498233Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9498294Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9498329Z 
2019-07-22T17:35:09.9498788Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9498948Z    |
2019-07-22T17:35:09.9499302Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9499365Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9499448Z error: aborting due to 6 previous errors
2019-07-22T17:35:09.9499496Z 
2019-07-22T17:35:09.9499770Z For more information about this error, try `rustc --explain E0658`.
2019-07-22T17:35:09.9499818Z 
2019-07-22T17:35:09.9499818Z 
2019-07-22T17:35:09.9500053Z ------------------------------------------
2019-07-22T17:35:09.9500103Z 
2019-07-22T17:35:09.9500132Z 
2019-07-22T17:35:09.9500389Z ---- [ui] ui/test-shadowing/test-cant-be-shadowed.rs stdout ----
2019-07-22T17:35:09.9500425Z 
2019-07-22T17:35:09.9501190Z error: test compilation failed although it shouldn't!
2019-07-22T17:35:09.9501253Z status: exit code: 1
2019-07-22T17:35:09.9502122Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-shadowing/test-cant-be-shadowed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-shadowing/test-cant-be-shadowed" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-shadowing/test-cant-be-shadowed/auxiliary" "-A" "unused"
2019-07-22T17:35:09.9502489Z ------------------------------------------
2019-07-22T17:35:09.9502540Z 
2019-07-22T17:35:09.9502781Z ------------------------------------------
2019-07-22T17:35:09.9502830Z stderr:
2019-07-22T17:35:09.9502830Z stderr:
2019-07-22T17:35:09.9503063Z ------------------------------------------
2019-07-22T17:35:09.9503493Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9503865Z    |
2019-07-22T17:35:09.9503911Z LL | fn foo(){}
2019-07-22T17:35:09.9503958Z    | ^^^^^^^^^^
2019-07-22T17:35:09.9504001Z    |
2019-07-22T17:35:09.9504001Z    |
2019-07-22T17:35:09.9504395Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9504462Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9504618Z 
2019-07-22T17:35:09.9505123Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9505492Z    |
2019-07-22T17:35:09.9505492Z    |
2019-07-22T17:35:09.9505571Z LL | fn bar() {}
2019-07-22T17:35:09.9505686Z    |
2019-07-22T17:35:09.9505686Z    |
2019-07-22T17:35:09.9506042Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9506108Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9506146Z 
2019-07-22T17:35:09.9506605Z error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9506672Z    |
2019-07-22T17:35:09.9507009Z    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9507091Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9507179Z error: aborting due to 3 previous errors
2019-07-22T17:35:09.9507229Z 
2019-07-22T17:35:09.9507531Z For more information about this error, try `rustc --explain E0658`.
2019-07-22T17:35:09.9507686Z 
2019-07-22T17:35:09.9507686Z 
2019-07-22T17:35:09.9507974Z ------------------------------------------
2019-07-22T17:35:09.9508026Z 
2019-07-22T17:35:09.9508056Z 
2019-07-22T17:35:09.9508320Z ---- [ui] ui/test-warns-dead-code.rs stdout ----
2019-07-22T17:35:09.9508376Z diff of stderr:
2019-07-22T17:35:09.9508408Z 
2019-07-22T17:35:09.9508674Z - error: function is never used: `dead`
2019-07-22T17:35:09.9508933Z -   --> $DIR/test-warns-dead-code.rs:5:1
2019-07-22T17:35:09.9509383Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libtest" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-22T17:35:09.9509470Z 3    |
2019-07-22T17:35:09.9509706Z - LL | fn dead() {}
2019-07-22T17:35:09.9510183Z -    |
2019-07-22T17:35:09.9510843Z - note: lint level defined here
2019-07-22T17:35:09.9511102Z -   --> $DIR/test-warns-dead-code.rs:3:9
2019-07-22T17:35:09.9511345Z -    |
2019-07-22T17:35:09.9511345Z -    |
2019-07-22T17:35:09.9511565Z - LL | #![deny(dead_code)]
2019-07-22T17:35:09.9511779Z -    |         ^^^^^^^^^
2019-07-22T17:35:09.9512122Z +    = note: for more information, see ***/issues/27812
2019-07-22T17:35:09.9512187Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-22T17:35:09.9512310Z 13 error: aborting due to previous error
2019-07-22T17:35:09.9512355Z 14 
2019-07-22T17:35:09.9512385Z 
---
2019-07-22T17:35:09.9523201Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-22T17:35:09.9523294Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-22T17:35:09.9523331Z 
2019-07-22T17:35:09.9523359Z 
2019-07-22T17:35:09.9525133Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-22T17:35:09.9525436Z 
2019-07-22T17:35:09.9525469Z 
2019-07-22T17:35:09.9525520Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-22T17:35:09.9525592Z Build completed unsuccessfully in 0:58:11
2019-07-22T17:35:09.9525592Z Build completed unsuccessfully in 0:58:11
2019-07-22T17:35:10.8911791Z ##[error]Bash exited with code '1'.
2019-07-22T17:35:10.8951698Z ##[section]Starting: Checkout
2019-07-22T17:35:10.8953594Z ==============================================================================
2019-07-22T17:35:10.8953671Z Task         : Get sources
2019-07-22T17:35:10.8953719Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
