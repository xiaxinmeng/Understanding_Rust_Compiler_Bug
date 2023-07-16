plain
2019-07-23T23:58:20.0552494Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-23T23:58:20.0732715Z ##[command]git config gc.auto 0
2019-07-23T23:58:20.0799759Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-23T23:58:20.0865443Z ##[command]git config --get-all http.proxy
2019-07-23T23:58:20.1001759Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62921/merge:refs/remotes/pull/62921/merge
---
2019-07-23T23:58:54.5288588Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-23T23:58:54.5288735Z 
2019-07-23T23:58:54.5289145Z   git checkout -b <new-branch-name>
2019-07-23T23:58:54.5289381Z 
2019-07-23T23:58:54.5289432Z HEAD is now at 8400f5b54 Merge 8b0b22eaa08cf8606d6b13f93c40e6dad5a0f0c8 into a7f28678bbf4e16893bb6a718e427504167a9494
2019-07-23T23:58:54.5423010Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-23T23:58:54.5426243Z ==============================================================================
2019-07-23T23:58:54.5426305Z Task         : Bash
2019-07-23T23:58:54.5426353Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-24T00:55:09.1690928Z .................................................................................................... 200/5850
2019-07-24T00:55:13.0909655Z .................................................................................................... 300/5850
2019-07-24T00:55:16.6987018Z .................................................................................................... 400/5850
2019-07-24T00:55:20.2201477Z .................................................................................................... 500/5850
2019-07-24T00:55:23.8788262Z ........................................................................i........................... 600/5850
2019-07-24T00:55:32.5753410Z .................................................................................................... 800/5850
2019-07-24T00:55:38.0790507Z .................................................................................................... 900/5850
2019-07-24T00:55:42.9229951Z ...................................................................................................i 1000/5850
2019-07-24T00:55:42.9229951Z ...................................................................................................i 1000/5850
2019-07-24T00:55:48.3231562Z ...........i........................................................................................ 1100/5850
2019-07-24T00:55:52.0268228Z .............................iiiii.................................................................. 1200/5850
2019-07-24T00:55:55.4851336Z ..............................................................F..................................... 1300/5850
2019-07-24T00:56:00.4468786Z .................................................................................................... 1500/5850
2019-07-24T00:56:04.2163337Z .................................................................................................... 1600/5850
2019-07-24T00:56:06.8575696Z .................................................................................................... 1700/5850
2019-07-24T00:56:06.8575696Z .................................................................................................... 1700/5850
2019-07-24T00:56:10.0287101Z .....................................................................i.............................. 1800/5850
2019-07-24T00:56:18.1433014Z .................................................................................................... 2000/5850
2019-07-24T00:56:22.3712356Z ..............................................................................F..................... 2100/5850
2019-07-24T00:56:25.8404315Z .................................................................................................... 2200/5850
2019-07-24T00:56:25.8404315Z .................................................................................................... 2200/5850
2019-07-24T00:56:29.3966726Z .....................................................i.............................................. 2300/5850
2019-07-24T00:56:38.1851215Z .................................................................................................... 2500/5850
2019-07-24T00:56:41.9356663Z .................................................................................................... 2600/5850
2019-07-24T00:56:46.5534920Z .................................................................................................... 2700/5850
2019-07-24T00:56:50.0914286Z .........................................................................................F.......... 2800/5850
2019-07-24T00:56:50.0914286Z .........................................................................................F.......... 2800/5850
2019-07-24T00:56:54.1206864Z .................................................................................................... 2900/5850
2019-07-24T00:56:58.7629675Z .................................................................................................... 3000/5850
2019-07-24T00:57:02.8907704Z .................................................................................................... 3100/5850
2019-07-24T00:57:07.7740662Z .................................................................................................... 3200/5850
2019-07-24T00:57:10.9204843Z .................................................................................................... 3300/5850
2019-07-24T00:57:14.3052564Z .................................................................................................... 3400/5850
2019-07-24T00:57:19.0785864Z .................................................................................................... 3500/5850
2019-07-24T00:57:22.4173681Z ....................i............................................................................... 3600/5850
2019-07-24T00:57:26.1907329Z .............................F.F..........F.....................................................ii.. 3700/5850
2019-07-24T00:57:29.6470669Z .i..ii.............................................................................................. 3800/5850
2019-07-24T00:57:37.5231920Z .................................................................................................... 4000/5850
2019-07-24T00:57:37.5231920Z .................................................................................................... 4000/5850
2019-07-24T00:57:41.0298484Z ..........ii........................................................................................ 4100/5850
2019-07-24T00:57:42.7698037Z ...............................i.................................................................... 4200/5850
2019-07-24T00:57:44.5551588Z .................................................................................................i.. 4300/5850
2019-07-24T00:57:50.4669562Z .................................................................................................... 4500/5850
2019-07-24T00:58:07.5605697Z .................................................................................................... 4600/5850
2019-07-24T00:58:10.3629813Z .................................................................................................... 4700/5850
2019-07-24T00:58:13.8595274Z .................................................................................................... 4800/5850
---
2019-07-24T00:58:46.9696504Z ..................................................................................F................. 5400/5850
2019-07-24T00:58:50.6260344Z .................................................................................................... 5500/5850
2019-07-24T00:58:54.4326551Z .................................................................................................... 5600/5850
2019-07-24T00:58:57.2153191Z .................................................................................................... 5700/5850
2019-07-24T00:58:59.9122207Z ..........................................................................................i......... 5800/5850
2019-07-24T00:59:01.6661152Z failures:
2019-07-24T00:59:01.6697590Z 
2019-07-24T00:59:01.6698513Z ---- [ui] ui/associated-const/associated-const-ambiguity-report.rs stdout ----
2019-07-24T00:59:01.6698820Z diff of stderr:
2019-07-24T00:59:01.6698820Z diff of stderr:
2019-07-24T00:59:01.6699044Z 
2019-07-24T00:59:01.6699230Z 9    |
2019-07-24T00:59:01.6699410Z 10 LL |     const ID: i32 = 1;
2019-07-24T00:59:01.6699615Z 11    |     ^^^^^^^^^^^^^^^^^^
2019-07-24T00:59:01.6699802Z +    = help: to disambiguate the method call, write `Foo::ID(...)` instead
2019-07-24T00:59:01.6700009Z 12 note: candidate #2 is defined in an impl of the trait `Bar` for the type `i32`
2019-07-24T00:59:01.6701653Z 14    |
2019-07-24T00:59:01.6701951Z 
2019-07-24T00:59:01.6701951Z 
2019-07-24T00:59:01.6702255Z 15 LL |     const ID: i32 = 3;
2019-07-24T00:59:01.6702534Z 16    |     ^^^^^^^^^^^^^^^^^^
2019-07-24T00:59:01.6702760Z +    = help: to disambiguate the method call, write `Bar::ID(...)` instead
2019-07-24T00:59:01.6703406Z 18 error: aborting due to previous error
2019-07-24T00:59:01.6703612Z 19 
2019-07-24T00:59:01.6703780Z 
2019-07-24T00:59:01.6703940Z 
2019-07-24T00:59:01.6703940Z 
2019-07-24T00:59:01.6704122Z The actual stderr differed from the expected stderr.
2019-07-24T00:59:01.6705309Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/associated-const-ambiguity-report/associated-const-ambiguity-report.stderr
2019-07-24T00:59:01.6705975Z To update references, rerun the tests and pass the `--bless` flag
2019-07-24T00:59:01.6706665Z To only update this specific test, also pass `--test-args associated-const/associated-const-ambiguity-report.rs`
2019-07-24T00:59:01.6707180Z error: 1 errors occurred comparing output.
2019-07-24T00:59:01.6707418Z status: exit code: 1
2019-07-24T00:59:01.6707418Z status: exit code: 1
2019-07-24T00:59:01.6708623Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-const/associated-const-ambiguity-report.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/associated-const-ambiguity-report" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/associated-const-ambiguity-report/auxiliary" "-A" "unused"
2019-07-24T00:59:01.6709511Z ------------------------------------------
2019-07-24T00:59:01.6711097Z 
2019-07-24T00:59:01.6711913Z ------------------------------------------
2019-07-24T00:59:01.6712267Z stderr:
2019-07-24T00:59:01.6712267Z stderr:
2019-07-24T00:59:01.6712713Z ------------------------------------------
2019-07-24T00:59:01.6712972Z error[E0034]: multiple applicable items in scope
2019-07-24T00:59:01.6713409Z   --> /checkout/src/test/ui/associated-const/associated-const-ambiguity-report.rs:17:16
2019-07-24T00:59:01.6713688Z    |
2019-07-24T00:59:01.6713898Z LL | const X: i32 = <i32>::ID; //~ ERROR E0034
2019-07-24T00:59:01.6714079Z    |                ^^^^^^^^^ multiple `ID` found
2019-07-24T00:59:01.6714278Z    |
2019-07-24T00:59:01.6714457Z note: candidate #1 is defined in an impl of the trait `Foo` for the type `i32`
2019-07-24T00:59:01.6715924Z    |
2019-07-24T00:59:01.6715924Z    |
2019-07-24T00:59:01.6716134Z LL |     const ID: i32 = 1;
2019-07-24T00:59:01.6716367Z    |     ^^^^^^^^^^^^^^^^^^
2019-07-24T00:59:01.6717116Z    = help: to disambiguate the method call, write `Foo::ID(...)` instead
2019-07-24T00:59:01.6717339Z note: candidate #2 is defined in an impl of the trait `Bar` for the type `i32`
2019-07-24T00:59:01.6718275Z    |
2019-07-24T00:59:01.6718275Z    |
2019-07-24T00:59:01.6718397Z LL |     const ID: i32 = 3;
2019-07-24T00:59:01.6718534Z    |     ^^^^^^^^^^^^^^^^^^
2019-07-24T00:59:01.6718692Z    = help: to disambiguate the method call, write `Bar::ID(...)` instead
2019-07-24T00:59:01.6718962Z error: aborting due to previous error
2019-07-24T00:59:01.6719071Z 
2019-07-24T00:59:01.6719468Z For more information about this error, try `rustc --explain E0034`.
2019-07-24T00:59:01.6719642Z 
---
2019-07-24T00:59:01.6720896Z 
2019-07-24T00:59:01.6721051Z 9    |
2019-07-24T00:59:01.6721172Z 10 LL |     fn foo() {}
2019-07-24T00:59:01.6721291Z 11    |     ^^^^^^^^
2019-07-24T00:59:01.6721438Z +    = help: to disambiguate the method call, write `Trait1::foo(...)` instead
2019-07-24T00:59:01.6721571Z 12 note: candidate #2 is defined in an impl of the trait `Trait2` for the type `Test`
2019-07-24T00:59:01.6722109Z 13   --> $DIR/E0034.rs:16:5
2019-07-24T00:59:01.6722412Z 
2019-07-24T00:59:01.6722710Z 15 LL |     fn foo() {}
2019-07-24T00:59:01.6723019Z 16    |     ^^^^^^^^
2019-07-24T00:59:01.6723019Z 16    |     ^^^^^^^^
2019-07-24T00:59:01.6723790Z +    = help: to disambiguate the method call, write `Trait2::foo(...)` instead
2019-07-24T00:59:01.6724379Z 18 error: aborting due to previous error
2019-07-24T00:59:01.6726194Z 19 
2019-07-24T00:59:01.6727633Z 
2019-07-24T00:59:01.6728109Z 
2019-07-24T00:59:01.6728109Z 
2019-07-24T00:59:01.6728304Z The actual stderr differed from the expected stderr.
2019-07-24T00:59:01.6729588Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0034/E0034.stderr
2019-07-24T00:59:01.6730015Z To update references, rerun the tests and pass the `--bless` flag
2019-07-24T00:59:01.6730397Z To only update this specific test, also pass `--test-args error-codes/E0034.rs`
2019-07-24T00:59:01.6731234Z error: 1 errors occurred comparing output.
2019-07-24T00:59:01.6731540Z status: exit code: 1
2019-07-24T00:59:01.6731540Z status: exit code: 1
2019-07-24T00:59:01.6732581Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0034.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0034" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0034/auxiliary" "-A" "unused"
2019-07-24T00:59:01.6733587Z ------------------------------------------
2019-07-24T00:59:01.6733754Z 
2019-07-24T00:59:01.6734121Z ------------------------------------------
2019-07-24T00:59:01.6734319Z stderr:
2019-07-24T00:59:01.6734319Z stderr:
2019-07-24T00:59:01.6734944Z ------------------------------------------
2019-07-24T00:59:01.6735179Z error[E0034]: multiple applicable items in scope
2019-07-24T00:59:01.6735611Z   --> /checkout/src/test/ui/error-codes/E0034.rs:20:5
2019-07-24T00:59:01.6735793Z    |
2019-07-24T00:59:01.6735938Z LL |     Test::foo() //~ ERROR multiple applicable items in scope
2019-07-24T00:59:01.6736105Z    |     ^^^^^^^^^ multiple `foo` found
2019-07-24T00:59:01.6736238Z    |
2019-07-24T00:59:01.6736397Z note: candidate #1 is defined in an impl of the trait `Trait1` for the type `Test`
2019-07-24T00:59:01.6736971Z    |
2019-07-24T00:59:01.6737129Z LL |     fn foo() {}
2019-07-24T00:59:01.6737262Z    |     ^^^^^^^^
2019-07-24T00:59:01.6737262Z    |     ^^^^^^^^
2019-07-24T00:59:01.6737401Z    = help: to disambiguate the method call, write `Trait1::foo(...)` instead
2019-07-24T00:59:01.6737567Z note: candidate #2 is defined in an impl of the trait `Trait2` for the type `Test`
2019-07-24T00:59:01.6738313Z    |
2019-07-24T00:59:01.6739358Z LL |     fn foo() {}
2019-07-24T00:59:01.6739672Z    |     ^^^^^^^^
2019-07-24T00:59:01.6739672Z    |     ^^^^^^^^
2019-07-24T00:59:01.6739970Z    = help: to disambiguate the method call, write `Trait2::foo(...)` instead
2019-07-24T00:59:01.6740214Z error: aborting due to previous error
2019-07-24T00:59:01.6740314Z 
2019-07-24T00:59:01.6740935Z For more information about this error, try `rustc --explain E0034`.
2019-07-24T00:59:01.6741093Z 
2019-07-24T00:59:01.6741093Z 
2019-07-24T00:59:01.6741421Z ------------------------------------------
2019-07-24T00:59:01.6741790Z 
2019-07-24T00:59:01.6741906Z 
2019-07-24T00:59:01.6742411Z ---- [ui] ui/inference/inference_unstable_featured.rs stdout ----
2019-07-24T00:59:01.6742796Z diff of stderr:
2019-07-24T00:59:01.6742928Z 
2019-07-24T00:59:01.6743054Z 5    |                    ^^^^^^^^^^^ multiple `ipu_flatten` found
2019-07-24T00:59:01.6743178Z 6    |
2019-07-24T00:59:01.6743695Z 7    = note: candidate #1 is defined in an impl of the trait `inference_unstable_iterator::IpuIterator` for the type `char`
2019-07-24T00:59:01.6744364Z +    = help: to disambiguate the method call, write `inference_unstable_iterator::IpuIterator::ipu_flatten('x')` instead
2019-07-24T00:59:01.6744601Z 8    = note: candidate #2 is defined in an impl of the trait `inference_unstable_itertools::IpuItertools` for the type `char`
2019-07-24T00:59:01.6745456Z +    = help: to disambiguate the method call, write `inference_unstable_itertools::IpuItertools::ipu_flatten('x')` instead
2019-07-24T00:59:01.6745848Z 10 error: aborting due to previous error
2019-07-24T00:59:01.6746007Z 11 
2019-07-24T00:59:01.6746119Z 
2019-07-24T00:59:01.6746232Z 
2019-07-24T00:59:01.6746232Z 
2019-07-24T00:59:01.6746370Z The actual stderr differed from the expected stderr.
2019-07-24T00:59:01.6746880Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/inference_unstable_featured/inference_unstable_featured.stderr
2019-07-24T00:59:01.6747332Z To update references, rerun the tests and pass the `--bless` flag
2019-07-24T00:59:01.6747804Z To only update this specific test, also pass `--test-args inference/inference_unstable_featured.rs`
2019-07-24T00:59:01.6748282Z error: 1 errors occurred comparing output.
2019-07-24T00:59:01.6748419Z status: exit code: 1
2019-07-24T00:59:01.6748419Z status: exit code: 1
2019-07-24T00:59:01.6749330Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/inference_unstable_featured.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/inference_unstable_featured" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/inference_unstable_featured/auxiliary" "-A" "unused"
2019-07-24T00:59:01.6749945Z ------------------------------------------
2019-07-24T00:59:01.6750137Z 
2019-07-24T00:59:01.6750460Z ------------------------------------------
2019-07-24T00:59:01.6750618Z stderr:
2019-07-24T00:59:01.6750618Z stderr:
2019-07-24T00:59:01.6750931Z ------------------------------------------
2019-07-24T00:59:01.6751094Z error[E0034]: multiple applicable items in scope
2019-07-24T00:59:01.6751469Z   --> /checkout/src/test/ui/inference/inference_unstable_featured.rs:16:20
2019-07-24T00:59:01.6751664Z    |
2019-07-24T00:59:01.6752006Z LL |     assert_eq!('x'.ipu_flatten(), 0);   //~ ERROR E0034
2019-07-24T00:59:01.6752183Z    |                    ^^^^^^^^^^^ multiple `ipu_flatten` found
2019-07-24T00:59:01.6752331Z    |
2019-07-24T00:59:01.6752465Z    = note: candidate #1 is defined in an impl of the trait `inference_unstable_iterator::IpuIterator` for the type `char`
2019-07-24T00:59:01.6752876Z    = help: to disambiguate the method call, write `inference_unstable_iterator::IpuIterator::ipu_flatten('x')` instead
2019-07-24T00:59:01.6753059Z    = note: candidate #2 is defined in an impl of the trait `inference_unstable_itertools::IpuItertools` for the type `char`
2019-07-24T00:59:01.6753466Z    = help: to disambiguate the method call, write `inference_unstable_itertools::IpuItertools::ipu_flatten('x')` instead
2019-07-24T00:59:01.6753762Z error: aborting due to previous error
2019-07-24T00:59:01.6753866Z 
2019-07-24T00:59:01.6754223Z For more information about this error, try `rustc --explain E0034`.
2019-07-24T00:59:01.6754370Z 
2019-07-24T00:59:01.6754370Z 
2019-07-24T00:59:01.6755134Z ------------------------------------------
2019-07-24T00:59:01.6755346Z 
2019-07-24T00:59:01.6755468Z 
2019-07-24T00:59:01.6755871Z ---- [ui] ui/issues/issue-3702-2.rs stdout ----
2019-07-24T00:59:01.6756079Z diff of stderr:
2019-07-24T00:59:01.6756199Z 
2019-07-24T00:59:01.6756330Z 9    |
2019-07-24T00:59:01.6756695Z 10 LL |     fn to_int(&self) -> isize { 0 }
2019-07-24T00:59:01.6756971Z 11    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-07-24T00:59:01.6757152Z +    = help: to disambiguate the method call, write `ToPrimitive::to_int(&self)` instead
2019-07-24T00:59:01.6757469Z 12 note: candidate #2 is defined in an impl of the trait `Add` for the type `isize`
2019-07-24T00:59:01.6757880Z 13   --> $DIR/issue-3702-2.rs:14:5
2019-07-24T00:59:01.6758181Z 
2019-07-24T00:59:01.6758181Z 
2019-07-24T00:59:01.6758701Z 15 LL |     fn to_int(&self) -> isize { *self }
2019-07-24T00:59:01.6758868Z 16    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-07-24T00:59:01.6759016Z +    = help: to disambiguate the method call, write `Add::to_int(&self)` instead
2019-07-24T00:59:01.6759286Z 18 error: aborting due to previous error
2019-07-24T00:59:01.6759404Z 19 
2019-07-24T00:59:01.6759502Z 
2019-07-24T00:59:01.6759604Z 
2019-07-24T00:59:01.6759604Z 
2019-07-24T00:59:01.6759741Z The actual stderr differed from the expected stderr.
2019-07-24T00:59:01.6760140Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3702-2/issue-3702-2.stderr
2019-07-24T00:59:01.6760524Z To update references, rerun the tests and pass the `--bless` flag
2019-07-24T00:59:01.6761347Z To only update this specific test, also pass `--test-args issues/issue-3702-2.rs`
2019-07-24T00:59:01.6764408Z error: 1 errors occurred comparing output.
2019-07-24T00:59:01.6764462Z status: exit code: 1
2019-07-24T00:59:01.6764462Z status: exit code: 1
2019-07-24T00:59:01.6765799Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3702-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3702-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3702-2/auxiliary" "-A" "unused"
2019-07-24T00:59:01.6766191Z ------------------------------------------
2019-07-24T00:59:01.6766226Z 
2019-07-24T00:59:01.6766465Z ------------------------------------------
2019-07-24T00:59:01.6766521Z stderr:
2019-07-24T00:59:01.6766521Z stderr:
2019-07-24T00:59:01.6766734Z ------------------------------------------
2019-07-24T00:59:01.6766802Z error[E0034]: multiple applicable items in scope
2019-07-24T00:59:01.6767038Z   --> /checkout/src/test/ui/issues/issue-3702-2.rs:16:14
2019-07-24T00:59:01.6767087Z    |
2019-07-24T00:59:01.6767158Z LL |         self.to_int() + other.to_int() //~ ERROR multiple applicable items in scope
2019-07-24T00:59:01.6767218Z    |              ^^^^^^ multiple `to_int` found
2019-07-24T00:59:01.6767262Z    |
2019-07-24T00:59:01.6767332Z note: candidate #1 is defined in an impl of the trait `ToPrimitive` for the type `isize`
2019-07-24T00:59:01.6767575Z   --> /checkout/src/test/ui/issues/issue-3702-2.rs:2:5
2019-07-24T00:59:01.6767622Z    |
2019-07-24T00:59:01.6767853Z LL |     fn to_int(&self) -> isize { 0 }
2019-07-24T00:59:01.6767903Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-07-24T00:59:01.6767954Z    = help: to disambiguate the method call, write `ToPrimitive::to_int(&self)` instead
2019-07-24T00:59:01.6768037Z note: candidate #2 is defined in an impl of the trait `Add` for the type `isize`
2019-07-24T00:59:01.6768435Z   --> /checkout/src/test/ui/issues/issue-3702-2.rs:14:5
2019-07-24T00:59:01.6768481Z    |
2019-07-24T00:59:01.6768704Z LL |     fn to_int(&self) -> isize { *self }
2019-07-24T00:59:01.6768752Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-07-24T00:59:01.6768964Z    = help: to disambiguate the method call, write `Add::to_int(&self)` instead
2019-07-24T00:59:01.6769060Z error: aborting due to previous error
2019-07-24T00:59:01.6769087Z 
2019-07-24T00:59:01.6769577Z For more information about this error, try `rustc --explain E0034`.
2019-07-24T00:59:01.6769837Z 
2019-07-24T00:59:01.6769837Z 
2019-07-24T00:59:01.6770226Z ------------------------------------------
2019-07-24T00:59:01.6770256Z 
2019-07-24T00:59:01.6770281Z 
2019-07-24T00:59:01.6770515Z ---- [ui] ui/methods/method-ambig-two-traits-cross-crate.rs stdout ----
2019-07-24T00:59:01.6770581Z diff of stderr:
2019-07-24T00:59:01.6770692Z 
2019-07-24T00:59:01.6770731Z 9    |
2019-07-24T00:59:01.6770986Z 10 LL | impl Me2 for usize { fn me(&self) -> usize { *self } }
2019-07-24T00:59:01.6771054Z 11    |                      ^^^^^^^^^^^^^^^^^^^^^
2019-07-24T00:59:01.6771105Z +    = help: to disambiguate the method call, write `Me2::me(1usize)` instead
2019-07-24T00:59:01.6771161Z 12    = note: candidate #2 is defined in an impl of the trait `ambig_impl_2_lib::Me` for the type `usize`
2019-07-24T00:59:01.6771245Z +    = help: to disambiguate the method call, write `ambig_impl_2_lib::Me::me(1usize)` instead
2019-07-24T00:59:01.6771334Z 14 error: aborting due to previous error
2019-07-24T00:59:01.6771392Z 15 
2019-07-24T00:59:01.6771418Z 
2019-07-24T00:59:01.6771442Z 
2019-07-24T00:59:01.6771442Z 
2019-07-24T00:59:01.6771484Z The actual stderr differed from the expected stderr.
2019-07-24T00:59:01.6771844Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-cross-crate/method-ambig-two-traits-cross-crate.stderr
2019-07-24T00:59:01.6772101Z To update references, rerun the tests and pass the `--bless` flag
2019-07-24T00:59:01.6772376Z To only update this specific test, also pass `--test-args methods/method-ambig-two-traits-cross-crate.rs`
2019-07-24T00:59:01.6772477Z error: 1 errors occurred comparing output.
2019-07-24T00:59:01.6772523Z status: exit code: 1
2019-07-24T00:59:01.6772523Z status: exit code: 1
2019-07-24T00:59:01.6773402Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-ambig-two-traits-cross-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-cross-crate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-cross-crate/auxiliary" "-A" "unused"
2019-07-24T00:59:01.6774118Z ------------------------------------------
2019-07-24T00:59:01.6774148Z 
2019-07-24T00:59:01.6774336Z ------------------------------------------
2019-07-24T00:59:01.6774392Z stderr:
2019-07-24T00:59:01.6774392Z stderr:
2019-07-24T00:59:01.6774578Z ------------------------------------------
2019-07-24T00:59:01.6774622Z error[E0034]: multiple applicable items in scope
2019-07-24T00:59:01.6775639Z   --> /checkout/src/test/ui/methods/method-ambig-two-traits-cross-crate.rs:11:21
2019-07-24T00:59:01.6775718Z    |
2019-07-24T00:59:01.6775817Z LL | fn main() { 1_usize.me(); } //~ ERROR E0034
2019-07-24T00:59:01.6775867Z    |                     ^^ multiple `me` found
2019-07-24T00:59:01.6775932Z    |
2019-07-24T00:59:01.6775981Z note: candidate #1 is defined in an impl of the trait `Me2` for the type `usize`
2019-07-24T00:59:01.6776330Z    |
2019-07-24T00:59:01.6776330Z    |
2019-07-24T00:59:01.6776562Z LL | impl Me2 for usize { fn me(&self) -> usize { *self } }
2019-07-24T00:59:01.6776615Z    |                      ^^^^^^^^^^^^^^^^^^^^^
2019-07-24T00:59:01.6776684Z    = help: to disambiguate the method call, write `Me2::me(1usize)` instead
2019-07-24T00:59:01.6776741Z    = note: candidate #2 is defined in an impl of the trait `ambig_impl_2_lib::Me` for the type `usize`
2019-07-24T00:59:01.6776807Z    = help: to disambiguate the method call, write `ambig_impl_2_lib::Me::me(1usize)` instead
2019-07-24T00:59:01.6776905Z error: aborting due to previous error
2019-07-24T00:59:01.6776934Z 
2019-07-24T00:59:01.6777177Z For more information about this error, try `rustc --explain E0034`.
2019-07-24T00:59:01.6777228Z 
2019-07-24T00:59:01.6777228Z 
2019-07-24T00:59:01.6777442Z ------------------------------------------
2019-07-24T00:59:01.6777473Z 
2019-07-24T00:59:01.6777499Z 
2019-07-24T00:59:01.6777763Z ---- [ui] ui/methods/method-ambig-two-traits-with-default-method.rs stdout ----
2019-07-24T00:59:01.6777918Z diff of stderr:
2019-07-24T00:59:01.6777947Z 
2019-07-24T00:59:01.6777985Z 9    |
2019-07-24T00:59:01.6778046Z 10 LL | trait Foo { fn method(&self) {} }
2019-07-24T00:59:01.6778093Z 11    |             ^^^^^^^^^^^^^^^^
2019-07-24T00:59:01.6778145Z +    = help: to disambiguate the method call, write `Foo::method(1usize)` instead
2019-07-24T00:59:01.6778220Z 12 note: candidate #2 is defined in an impl of the trait `Bar` for the type `usize`
2019-07-24T00:59:01.6778555Z 14    |
2019-07-24T00:59:01.6778581Z 
2019-07-24T00:59:01.6778581Z 
2019-07-24T00:59:01.6778641Z 15 LL | trait Bar { fn method(&self) {} }
2019-07-24T00:59:01.6778846Z 16    |             ^^^^^^^^^^^^^^^^
2019-07-24T00:59:01.6779053Z +    = help: to disambiguate the method call, write `Bar::method(1usize)` instead
2019-07-24T00:59:01.6779155Z 18 error: aborting due to previous error
2019-07-24T00:59:01.6779201Z 19 
2019-07-24T00:59:01.6779225Z 
2019-07-24T00:59:01.6779268Z 
2019-07-24T00:59:01.6779268Z 
2019-07-24T00:59:01.6779309Z The actual stderr differed from the expected stderr.
2019-07-24T00:59:01.6779644Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-with-default-method/method-ambig-two-traits-with-default-method.stderr
2019-07-24T00:59:01.6780051Z To update references, rerun the tests and pass the `--bless` flag
2019-07-24T00:59:01.6780400Z To only update this specific test, also pass `--test-args methods/method-ambig-two-traits-with-default-method.rs`
2019-07-24T00:59:01.6780499Z error: 1 errors occurred comparing output.
2019-07-24T00:59:01.6780539Z status: exit code: 1
2019-07-24T00:59:01.6780539Z status: exit code: 1
2019-07-24T00:59:01.6781293Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-ambig-two-traits-with-default-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-with-default-method" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-with-default-method/auxiliary" "-A" "unused"
2019-07-24T00:59:01.6781613Z ------------------------------------------
2019-07-24T00:59:01.6781660Z 
2019-07-24T00:59:01.6781855Z ------------------------------------------
2019-07-24T00:59:01.6781895Z stderr:
2019-07-24T00:59:01.6781895Z stderr:
2019-07-24T00:59:01.6782084Z ------------------------------------------
2019-07-24T00:59:01.6782145Z error[E0034]: multiple applicable items in scope
2019-07-24T00:59:01.6782382Z   --> /checkout/src/test/ui/methods/method-ambig-two-traits-with-default-method.rs:12:13
2019-07-24T00:59:01.6782429Z    |
2019-07-24T00:59:01.6782490Z LL |     1_usize.method(); //~ ERROR E0034
2019-07-24T00:59:01.6782540Z    |             ^^^^^^ multiple `method` found
2019-07-24T00:59:01.6782578Z    |
2019-07-24T00:59:01.6782639Z note: candidate #1 is defined in an impl of the trait `Foo` for the type `usize`
2019-07-24T00:59:01.6782927Z    |
2019-07-24T00:59:01.6782927Z    |
2019-07-24T00:59:01.6782984Z LL | trait Foo { fn method(&self) {} }
2019-07-24T00:59:01.6783032Z    |             ^^^^^^^^^^^^^^^^
2019-07-24T00:59:01.6783078Z    = help: to disambiguate the method call, write `Foo::method(1usize)` instead
2019-07-24T00:59:01.6783147Z note: candidate #2 is defined in an impl of the trait `Bar` for the type `usize`
2019-07-24T00:59:01.6783431Z    |
2019-07-24T00:59:01.6783431Z    |
2019-07-24T00:59:01.6783486Z LL | trait Bar { fn method(&self) {} }
2019-07-24T00:59:01.6783527Z    |             ^^^^^^^^^^^^^^^^
2019-07-24T00:59:01.6783647Z    = help: to disambiguate the method call, write `Bar::method(1usize)` instead
2019-07-24T00:59:01.6783731Z error: aborting due to previous error
2019-07-24T00:59:01.6783756Z 
2019-07-24T00:59:01.6784001Z For more information about this error, try `rustc --explain E0034`.
2019-07-24T00:59:01.6784032Z 
2019-07-24T00:59:01.6784032Z 
2019-07-24T00:59:01.6784241Z ------------------------------------------
2019-07-24T00:59:01.6784268Z 
2019-07-24T00:59:01.6784291Z 
2019-07-24T00:59:01.6784528Z ---- [ui] ui/methods/method-deref-to-same-trait-object-with-separate-params.rs stdout ----
2019-07-24T00:59:01.6784592Z diff of stderr:
2019-07-24T00:59:01.6784617Z 
2019-07-24T00:59:01.6784652Z 27    |
2019-07-24T00:59:01.6785357Z 28 LL |         fn foo(self: Smaht<Self, u64>) -> u64 {
2019-07-24T00:59:01.6785431Z 29    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-07-24T00:59:01.6785484Z +    = help: to disambiguate the method call, write `internal::X::foo(x)` instead
2019-07-24T00:59:01.6785552Z 30 note: candidate #2 is defined in an impl of the trait `nuisance_foo::NuisanceFoo` for the type `_`
2019-07-24T00:59:01.6785881Z 32    |
2019-07-24T00:59:01.6785907Z 
2019-07-24T00:59:01.6785967Z 33 LL |         fn foo(self) {}
2019-07-24T00:59:01.6786012Z 34    |         ^^^^^^^^^^^^
2019-07-24T00:59:01.6786012Z 34    |         ^^^^^^^^^^^^
2019-07-24T00:59:01.6786150Z +    = help: to disambiguate the method call, write `nuisance_foo::NuisanceFoo::foo(x)` instead
2019-07-24T00:59:01.6786229Z 35 note: candidate #3 is defined in the trait `FinalFoo`
2019-07-24T00:59:01.6786565Z 37    |
2019-07-24T00:59:01.6786592Z 
2019-07-24T00:59:01.6786635Z 
2019-07-24T00:59:01.6786679Z The actual stderr differed from the expected stderr.
2019-07-24T00:59:01.6786679Z The actual stderr differed from the expected stderr.
2019-07-24T00:59:01.6787058Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-deref-to-same-trait-object-with-separate-params/method-deref-to-same-trait-object-with-separate-params.stderr
2019-07-24T00:59:01.6787393Z To update references, rerun the tests and pass the `--bless` flag
2019-07-24T00:59:01.6787700Z To only update this specific test, also pass `--test-args methods/method-deref-to-same-trait-object-with-separate-params.rs`
2019-07-24T00:59:01.6787802Z error: 1 errors occurred comparing output.
2019-07-24T00:59:01.6787847Z status: exit code: 1
2019-07-24T00:59:01.6787847Z status: exit code: 1
2019-07-24T00:59:01.6788823Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-deref-to-same-trait-object-with-separate-params" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-deref-to-same-trait-object-with-separate-params/auxiliary" "-A" "unused"
2019-07-24T00:59:01.6789126Z ------------------------------------------
2019-07-24T00:59:01.6789172Z 
2019-07-24T00:59:01.6789361Z ------------------------------------------
2019-07-24T00:59:01.6789400Z stderr:
2019-07-24T00:59:01.6789400Z stderr:
2019-07-24T00:59:01.6789591Z ------------------------------------------
2019-07-24T00:59:01.6789649Z error[E0308]: mismatched types
2019-07-24T00:59:01.6789888Z   --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:85:24
2019-07-24T00:59:01.6789934Z    |
2019-07-24T00:59:01.6789993Z LL |     let _seetype: () = z; //~ ERROR mismatched types
2019-07-24T00:59:01.6790037Z    |                        ^ expected (), found u32
2019-07-24T00:59:01.6790130Z    = note: expected type `()`
2019-07-24T00:59:01.6790246Z               found type `u32`
2019-07-24T00:59:01.6790271Z 
2019-07-24T00:59:01.6790306Z error[E0308]: mismatched types
2019-07-24T00:59:01.6790306Z error[E0308]: mismatched types
2019-07-24T00:59:01.6790595Z   --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:102:24
2019-07-24T00:59:01.6790640Z    |
2019-07-24T00:59:01.6790680Z LL |     let _seetype: () = z; //~ ERROR mismatched types
2019-07-24T00:59:01.6790739Z    |                        ^ expected (), found u64
2019-07-24T00:59:01.6790819Z    = note: expected type `()`
2019-07-24T00:59:01.6790875Z               found type `u64`
2019-07-24T00:59:01.6790899Z 
2019-07-24T00:59:01.6790937Z error[E0034]: multiple applicable items in scope
2019-07-24T00:59:01.6790937Z error[E0034]: multiple applicable items in scope
2019-07-24T00:59:01.6799204Z   --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:120:15
2019-07-24T00:59:01.6799308Z    |
2019-07-24T00:59:01.6799355Z LL |     let z = x.foo(); //~ ERROR multiple applicable items in scope
2019-07-24T00:59:01.6799401Z    |               ^^^ multiple `foo` found
2019-07-24T00:59:01.6799476Z    |
2019-07-24T00:59:01.6799522Z note: candidate #1 is defined in an impl of the trait `internal::X` for the type `_`
2019-07-24T00:59:01.6800003Z    |
2019-07-24T00:59:01.6800003Z    |
2019-07-24T00:59:01.6803405Z LL |         fn foo(self: Smaht<Self, u64>) -> u64 {
2019-07-24T00:59:01.6803479Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-07-24T00:59:01.6803707Z    = help: to disambiguate the method call, write `internal::X::foo(x)` instead
2019-07-24T00:59:01.6803771Z note: candidate #2 is defined in an impl of the trait `nuisance_foo::NuisanceFoo` for the type `_`
2019-07-24T00:59:01.6804209Z    |
2019-07-24T00:59:01.6804248Z LL |         fn foo(self) {}
2019-07-24T00:59:01.6804287Z    |         ^^^^^^^^^^^^
2019-07-24T00:59:01.6804287Z    |         ^^^^^^^^^^^^
2019-07-24T00:59:01.6804359Z    = help: to disambiguate the method call, write `nuisance_foo::NuisanceFoo::foo(x)` instead
2019-07-24T00:59:01.6804407Z note: candidate #3 is defined in the trait `FinalFoo`
2019-07-24T00:59:01.6805187Z    |
2019-07-24T00:59:01.6805394Z LL |     fn foo(&self) -> u8;
2019-07-24T00:59:01.6805442Z    |     ^^^^^^^^^^^^^^^^^^^^
2019-07-24T00:59:01.6805442Z    |     ^^^^^^^^^^^^^^^^^^^^
2019-07-24T00:59:01.6805503Z    = help: to disambiguate the method call, write `FinalFoo::foo(x)` instead
2019-07-24T00:59:01.6805601Z error[E0308]: mismatched types
2019-07-24T00:59:01.6805879Z   --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:137:24
2019-07-24T00:59:01.6805947Z    |
2019-07-24T00:59:01.6805947Z    |
2019-07-24T00:59:01.6805995Z LL |     let _seetype: () = z; //~ ERROR mismatched types
2019-07-24T00:59:01.6806045Z    |                        ^ expected (), found u8
2019-07-24T00:59:01.6806158Z    = note: expected type `()`
2019-07-24T00:59:01.6806202Z               found type `u8`
2019-07-24T00:59:01.6806230Z 
2019-07-24T00:59:01.6806291Z error[E0308]: mismatched types
2019-07-24T00:59:01.6806291Z error[E0308]: mismatched types
2019-07-24T00:59:01.6806570Z   --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:155:24
2019-07-24T00:59:01.6806620Z    |
2019-07-24T00:59:01.6806684Z LL |     let _seetype: () = z; //~ ERROR mismatched types
2019-07-24T00:59:01.6806742Z    |                        ^ expected (), found u32
2019-07-24T00:59:01.6806826Z    = note: expected type `()`
2019-07-24T00:59:01.6806888Z               found type `u32`
2019-07-24T00:59:01.6806917Z 
2019-07-24T00:59:01.6806957Z error[E0308]: mismatched types
2019-07-24T00:59:01.6806957Z error[E0308]: mismatched types
2019-07-24T00:59:01.6807251Z   --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:172:24
2019-07-24T00:59:01.6807302Z    |
2019-07-24T00:59:01.6807348Z LL |     let _seetype: () = z; //~ ERROR mismatched types
2019-07-24T00:59:01.6807515Z    |                        ^ expected (), found u32
2019-07-24T00:59:01.6807618Z    = note: expected type `()`
2019-07-24T00:59:01.6807661Z               found type `u32`
2019-07-24T00:59:01.6807708Z 
2019-07-24T00:59:01.6807752Z error: aborting due to 6 previous errors
---
2019-07-24T00:59:01.6808753Z ---- [ui] ui/traits/trait-alias-ambiguous.rs stdout ----
2019-07-24T00:59:01.6808794Z diff of stderr:
2019-07-24T00:59:01.6808818Z 
2019-07-24T00:59:01.6808869Z 9    |
2019-07-24T00:59:01.6808906Z 10 LL |         fn foo(&self) {}
2019-07-24T00:59:01.6809122Z 11    |         ^^^^^^^^^^^^^
2019-07-24T00:59:01.6809184Z +    = help: to disambiguate the method call, write `inner::A::foo(t)` instead
2019-07-24T00:59:01.6809233Z 12 note: candidate #2 is defined in an impl of the trait `inner::B` for the type `u8`
2019-07-24T00:59:01.6809637Z 14    |
2019-07-24T00:59:01.6809683Z 
2019-07-24T00:59:01.6809683Z 
2019-07-24T00:59:01.6809718Z 15 LL |         fn foo(&self) {}
2019-07-24T00:59:01.6809755Z 16    |         ^^^^^^^^^^^^^
2019-07-24T00:59:01.6809888Z +    = help: to disambiguate the method call, write `inner::B::foo(t)` instead
2019-07-24T00:59:01.6810162Z 18 error: aborting due to previous error
2019-07-24T00:59:01.6810199Z 19 
2019-07-24T00:59:01.6810238Z 
2019-07-24T00:59:01.6810260Z 
2019-07-24T00:59:01.6810260Z 
2019-07-24T00:59:01.6810298Z The actual stderr differed from the expected stderr.
2019-07-24T00:59:01.6810604Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-ambiguous/trait-alias-ambiguous.stderr
2019-07-24T00:59:01.6810857Z To update references, rerun the tests and pass the `--bless` flag
2019-07-24T00:59:01.6811096Z To only update this specific test, also pass `--test-args traits/trait-alias-ambiguous.rs`
2019-07-24T00:59:01.6811181Z error: 1 errors occurred comparing output.
2019-07-24T00:59:01.6811220Z status: exit code: 1
2019-07-24T00:59:01.6811220Z status: exit code: 1
2019-07-24T00:59:01.6812122Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-alias-ambiguous.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-ambiguous" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-ambiguous/auxiliary" "-A" "unused"
2019-07-24T00:59:01.6812580Z ------------------------------------------
2019-07-24T00:59:01.6812626Z 
2019-07-24T00:59:01.6812815Z ------------------------------------------
2019-07-24T00:59:01.6812855Z stderr:
2019-07-24T00:59:01.6812855Z stderr:
2019-07-24T00:59:01.6813056Z ------------------------------------------
2019-07-24T00:59:01.6813100Z error[E0034]: multiple applicable items in scope
2019-07-24T00:59:01.6813307Z   --> /checkout/src/test/ui/traits/trait-alias-ambiguous.rs:21:7
2019-07-24T00:59:01.6813373Z    |
2019-07-24T00:59:01.6813415Z LL |     t.foo(); //~ ERROR E0034
2019-07-24T00:59:01.6813455Z    |       ^^^ multiple `foo` found
2019-07-24T00:59:01.6813491Z    |
2019-07-24T00:59:01.6813552Z note: candidate #1 is defined in an impl of the trait `inner::A` for the type `u8`
2019-07-24T00:59:01.6813810Z    |
2019-07-24T00:59:01.6813810Z    |
2019-07-24T00:59:01.6813864Z LL |         fn foo(&self) {}
2019-07-24T00:59:01.6813901Z    |         ^^^^^^^^^^^^^
2019-07-24T00:59:01.6814026Z    = help: to disambiguate the method call, write `inner::A::foo(t)` instead
2019-07-24T00:59:01.6814091Z note: candidate #2 is defined in an impl of the trait `inner::B` for the type `u8`
2019-07-24T00:59:01.6814375Z    |
2019-07-24T00:59:01.6814375Z    |
2019-07-24T00:59:01.6814429Z LL |         fn foo(&self) {}
2019-07-24T00:59:01.6814467Z    |         ^^^^^^^^^^^^^
2019-07-24T00:59:01.6814518Z    = help: to disambiguate the method call, write `inner::B::foo(t)` instead
2019-07-24T00:59:01.6814603Z error: aborting due to previous error
2019-07-24T00:59:01.6814629Z 
2019-07-24T00:59:01.6815239Z For more information about this error, try `rustc --explain E0034`.
2019-07-24T00:59:01.6815277Z 
---
2019-07-24T00:59:01.6818267Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-24T00:59:01.6818317Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-24T00:59:01.6818371Z 
2019-07-24T00:59:01.6818393Z 
2019-07-24T00:59:01.6819886Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-24T00:59:01.6820143Z 
2019-07-24T00:59:01.6820168Z 
2019-07-24T00:59:01.6820209Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-24T00:59:01.6820270Z Build completed unsuccessfully in 0:53:58
2019-07-24T00:59:01.6820270Z Build completed unsuccessfully in 0:53:58
2019-07-24T00:59:02.8666401Z ##[error]Bash exited with code '1'.
2019-07-24T00:59:02.8700735Z ##[section]Starting: Checkout
2019-07-24T00:59:02.8702762Z ==============================================================================
2019-07-24T00:59:02.8702970Z Task         : Get sources
2019-07-24T00:59:02.8703014Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
