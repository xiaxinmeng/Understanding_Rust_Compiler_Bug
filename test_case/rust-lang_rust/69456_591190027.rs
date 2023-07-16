plain
2020-02-26T00:26:28.0864332Z ========================== Starting Command Output ===========================
2020-02-26T00:26:28.0866786Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8eb1bcd6-53a2-4f10-8dab-d17a9eab52e6.sh
2020-02-26T00:26:28.0867064Z 
2020-02-26T00:26:28.0870078Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-26T00:26:28.0889355Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69456/merge to s
2020-02-26T00:26:28.0892561Z Task         : Get sources
2020-02-26T00:26:28.0893138Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-26T00:26:28.0893459Z Version      : 1.0.0
2020-02-26T00:26:28.0893658Z Author       : Microsoft
---
2020-02-26T00:26:29.0840306Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-26T00:26:29.0846384Z ##[command]git config gc.auto 0
2020-02-26T00:26:29.0850139Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-26T00:26:29.0853427Z ##[command]git config --get-all http.proxy
2020-02-26T00:26:29.0859556Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69456/merge:refs/remotes/pull/69456/merge
---
2020-02-26T01:25:26.4048113Z .................................................................................................... 1700/9705
2020-02-26T01:25:30.7797689Z .................................................................................................... 1800/9705
2020-02-26T01:25:41.7692197Z ...........................................i........................................................ 1900/9705
2020-02-26T01:25:49.2930919Z .................................................................................................... 2000/9705
2020-02-26T01:26:02.2994467Z .................................iiiii.............................................................. 2100/9705
2020-02-26T01:26:11.6258691Z .................................................................................................... 2300/9705
2020-02-26T01:26:13.8682935Z ................F................................................................................... 2400/9705
2020-02-26T01:26:17.9894926Z .................................................................................................... 2500/9705
2020-02-26T01:26:37.5924467Z .................................................................................................... 2600/9705
---
2020-02-26T01:29:07.2459071Z .........i.......................................................................................... 5000/9705
2020-02-26T01:29:15.7304427Z .................................................................................................... 5100/9705
2020-02-26T01:29:20.0410679Z ....................................i.......................................................F....... 5200/9705
2020-02-26T01:29:29.1818368Z .................F.....................F............................................................ 5300/9705
2020-02-26T01:29:34.6946930Z .............ii.ii........i...i..................................................................... 5400/9705
2020-02-26T01:29:42.5330067Z .................................................................................................... 5600/9705
2020-02-26T01:29:52.4239604Z .................................................................................................... 5700/9705
2020-02-26T01:29:58.9362772Z ....i............................................................................................... 5800/9705
2020-02-26T01:30:04.2154812Z .................................................................................................... 5900/9705
2020-02-26T01:30:04.2154812Z .................................................................................................... 5900/9705
2020-02-26T01:30:13.6807497Z ...............................................................................................ii... 6000/9705
2020-02-26T01:30:24.8298073Z i..ii...........i................................................................................... 6100/9705
2020-02-26T01:30:37.7246478Z .................................................................................................... 6300/9705
2020-02-26T01:30:41.1617481Z .................................................................................................... 6400/9705
2020-02-26T01:30:41.1617481Z .................................................................................................... 6400/9705
2020-02-26T01:30:53.1578725Z ..........................i..ii..................................................................... 6500/9705
2020-02-26T01:31:12.1347182Z .................................................................................................... 6700/9705
2020-02-26T01:31:14.2035994Z ..................i................................................................................. 6800/9705
2020-02-26T01:31:16.2312094Z .................................................................................................... 6900/9705
2020-02-26T01:31:18.3156080Z ................................................i................................................... 7000/9705
---
2020-02-26T01:32:49.9670772Z .................................................................................................... 7700/9705
2020-02-26T01:32:54.4414820Z .................................................................................................... 7800/9705
2020-02-26T01:33:00.6122653Z ............................................................................................i....... 7900/9705
2020-02-26T01:33:08.3206153Z .................................................................................................... 8000/9705
2020-02-26T01:33:15.1605111Z .........................................iiiiiii.i.................................................. 8100/9705
2020-02-26T01:33:28.5172094Z ...........................................................................F........................ 8300/9705
2020-02-26T01:33:34.2260054Z .................................................................................................... 8400/9705
2020-02-26T01:33:48.2117828Z .................................................................................................... 8500/9705
2020-02-26T01:33:54.9010210Z .................................................................................................... 8600/9705
---
2020-02-26T01:35:41.3705646Z diff of stderr:
2020-02-26T01:35:41.3705965Z 
2020-02-26T01:35:41.3706581Z 2   --> $DIR/E0282.rs:2:9
2020-02-26T01:35:41.3706954Z 3    |
2020-02-26T01:35:41.3707374Z 4 LL |     let x = "hello".chars().rev().collect();
2020-02-26T01:35:41.3708099Z -    |         ^ consider giving `x` a type
2020-02-26T01:35:41.3709606Z +    |         ^ cannot infer type       ------- help: consider specifying the type argument in the method call: `collect::<B>`
2020-02-26T01:35:41.3711121Z 7 error: aborting due to previous error
2020-02-26T01:35:41.3711580Z 8 
2020-02-26T01:35:41.3711837Z 
2020-02-26T01:35:41.3712086Z 
2020-02-26T01:35:41.3712086Z 
2020-02-26T01:35:41.3712474Z The actual stderr differed from the expected stderr.
2020-02-26T01:35:41.3713451Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0282/E0282.stderr
2020-02-26T01:35:41.3716900Z To update references, rerun the tests and pass the `--bless` flag
2020-02-26T01:35:41.3717942Z To only update this specific test, also pass `--test-args error-codes/E0282.rs`
2020-02-26T01:35:41.3718784Z error: 1 errors occurred comparing output.
2020-02-26T01:35:41.3719225Z status: exit code: 1
2020-02-26T01:35:41.3719225Z status: exit code: 1
2020-02-26T01:35:41.3721708Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0282.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0282" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0282/auxiliary"
2020-02-26T01:35:41.3726615Z ------------------------------------------
2020-02-26T01:35:41.3726986Z 
2020-02-26T01:35:41.3727644Z ------------------------------------------
2020-02-26T01:35:41.3728521Z stderr:
2020-02-26T01:35:41.3728521Z stderr:
2020-02-26T01:35:41.3731446Z ------------------------------------------
2020-02-26T01:35:41.3731782Z error[E0282]: type annotations needed
2020-02-26T01:35:41.3732550Z   --> /checkout/src/test/ui/error-codes/E0282.rs:2:9
2020-02-26T01:35:41.3732849Z    |
2020-02-26T01:35:41.3736141Z LL |     let x = "hello".chars().rev().collect(); //~ ERROR E0282
2020-02-26T01:35:41.3737142Z    |         ^ cannot infer type       ------- help: consider specifying the type argument in the method call: `collect::<B>`
2020-02-26T01:35:41.3737755Z error: aborting due to previous error
2020-02-26T01:35:41.3737927Z 
2020-02-26T01:35:41.3738380Z For more information about this error, try `rustc --explain E0282`.
2020-02-26T01:35:41.3738625Z 
---
2020-02-26T01:35:41.3739916Z diff of stderr:
2020-02-26T01:35:41.3740046Z 
2020-02-26T01:35:41.3740590Z 2   --> $DIR/issue-20261.rs:4:11
2020-02-26T01:35:41.3740807Z 3    |
2020-02-26T01:35:41.3741011Z 4 LL |     for (ref i,) in [].iter() {
2020-02-26T01:35:41.3742308Z +    |                     --------- this method call resolves to `std::slice::Iter<'_, T>`
2020-02-26T01:35:41.3742681Z 6 LL |         i.clone();
2020-02-26T01:35:41.3742920Z 7    |           ^^^^^ cannot infer type
2020-02-26T01:35:41.3743136Z 8    |
2020-02-26T01:35:41.3743136Z 8    |
2020-02-26T01:35:41.3743250Z 
2020-02-26T01:35:41.3743351Z 
2020-02-26T01:35:41.3743562Z The actual stderr differed from the expected stderr.
2020-02-26T01:35:41.3744241Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20261/issue-20261.stderr
2020-02-26T01:35:41.3744879Z To update references, rerun the tests and pass the `--bless` flag
2020-02-26T01:35:41.3745477Z To only update this specific test, also pass `--test-args issues/issue-20261.rs`
2020-02-26T01:35:41.3745945Z error: 1 errors occurred comparing output.
2020-02-26T01:35:41.3746194Z status: exit code: 1
2020-02-26T01:35:41.3746194Z status: exit code: 1
2020-02-26T01:35:41.3748172Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20261.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20261" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20261/auxiliary"
2020-02-26T01:35:41.3749777Z ------------------------------------------
2020-02-26T01:35:41.3749961Z 
2020-02-26T01:35:41.3750335Z ------------------------------------------
2020-02-26T01:35:41.3750550Z stderr:
2020-02-26T01:35:41.3750550Z stderr:
2020-02-26T01:35:41.3750951Z ------------------------------------------
2020-02-26T01:35:41.3751247Z error[E0282]: type annotations needed for `&(_,)`
2020-02-26T01:35:41.3751768Z   --> /checkout/src/test/ui/issues/issue-20261.rs:4:11
2020-02-26T01:35:41.3752021Z    |
2020-02-26T01:35:41.3752216Z LL |     for (ref i,) in [].iter() {
2020-02-26T01:35:41.3752825Z    |                     --------- this method call resolves to `std::slice::Iter<'_, T>`
2020-02-26T01:35:41.3753204Z LL |         i.clone();
2020-02-26T01:35:41.3753628Z    |
2020-02-26T01:35:41.3753857Z    = note: type must be known at this point
2020-02-26T01:35:41.3754042Z 
2020-02-26T01:35:41.3754232Z error: aborting due to previous error
---
2020-02-26T01:35:41.3755751Z 
2020-02-26T01:35:41.3756140Z ---- [ui] ui/issues/issue-25368.rs stdout ----
2020-02-26T01:35:41.3756443Z diff of stderr:
2020-02-26T01:35:41.3756575Z 
2020-02-26T01:35:41.3756966Z 1 error[E0282]: type annotations needed for `(std::sync::mpsc::Sender<Foo<T>>, std::sync::mpsc::Receiver<Foo<T>>)`
2020-02-26T01:35:41.3757800Z 3    |
2020-02-26T01:35:41.3757800Z 3    |
2020-02-26T01:35:41.3758187Z - LL |     let (tx, rx) = channel();
2020-02-26T01:35:41.3759073Z -    |         -------- consider giving this pattern the explicit type `(std::sync::mpsc::Sender<Foo<T>>, std::sync::mpsc::Receiver<Foo<T>>)`, where the type parameter `T` is specified
2020-02-26T01:35:41.3759751Z - ...
2020-02-26T01:35:41.3759991Z 7 LL |         tx.send(Foo{ foo: PhantomData });
2020-02-26T01:35:41.3760611Z -    |                 ^^^ cannot infer type for type parameter `T` declared on the struct `Foo`
2020-02-26T01:35:41.3761184Z +    |         --------^^^---------------------
2020-02-26T01:35:41.3761817Z +    |         |       cannot infer type for type parameter `T` declared on the struct `Foo`
2020-02-26T01:35:41.3762343Z +    |         this method call resolves to `std::result::Result<(), std::sync::mpsc::SendError<T>>`
2020-02-26T01:35:41.3762687Z 9 
2020-02-26T01:35:41.3762888Z 10 error: aborting due to previous error
2020-02-26T01:35:41.3762888Z 10 error: aborting due to previous error
2020-02-26T01:35:41.3763107Z 11 
2020-02-26T01:35:41.3763216Z 
2020-02-26T01:35:41.3763317Z 
2020-02-26T01:35:41.3763529Z The actual stderr differed from the expected stderr.
2020-02-26T01:35:41.3764214Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-25368/issue-25368.stderr
2020-02-26T01:35:41.3764854Z To update references, rerun the tests and pass the `--bless` flag
2020-02-26T01:35:41.3765446Z To only update this specific test, also pass `--test-args issues/issue-25368.rs`
2020-02-26T01:35:41.3765915Z error: 1 errors occurred comparing output.
2020-02-26T01:35:41.3766157Z status: exit code: 1
2020-02-26T01:35:41.3766157Z status: exit code: 1
2020-02-26T01:35:41.3768123Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-25368.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-25368" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-25368/auxiliary"
2020-02-26T01:35:41.3777860Z ------------------------------------------
2020-02-26T01:35:41.3778047Z 
2020-02-26T01:35:41.3778433Z ------------------------------------------
2020-02-26T01:35:41.3778641Z stderr:
2020-02-26T01:35:41.3778641Z stderr:
2020-02-26T01:35:41.3779040Z ------------------------------------------
2020-02-26T01:35:41.3779509Z error[E0282]: type annotations needed for `(std::sync::mpsc::Sender<Foo<T>>, std::sync::mpsc::Receiver<Foo<T>>)`
2020-02-26T01:35:41.3780454Z    |
2020-02-26T01:35:41.3780454Z    |
2020-02-26T01:35:41.3781335Z LL |         tx.send(Foo{ foo: PhantomData }); //~ ERROR E0282
2020-02-26T01:35:41.3782083Z    |         --------^^^---------------------
2020-02-26T01:35:41.3782665Z    |         |       cannot infer type for type parameter `T` declared on the struct `Foo`
2020-02-26T01:35:41.3783168Z    |         this method call resolves to `std::result::Result<(), std::sync::mpsc::SendError<T>>`
2020-02-26T01:35:41.3783503Z 
2020-02-26T01:35:41.3783693Z error: aborting due to previous error
---
2020-02-26T01:35:41.3786333Z 1 error[E0282]: type annotations needed
2020-02-26T01:35:41.3786802Z -   --> $DIR/issue-65611.rs:59:20
2020-02-26T01:35:41.3787236Z +   --> $DIR/issue-65611.rs:59:27
2020-02-26T01:35:41.3787437Z 3    |
2020-02-26T01:35:41.3787679Z 4 LL |     let x = buffer.last().unwrap().0.clone();
2020-02-26T01:35:41.3792782Z +    |             --------------^^^^^^--
2020-02-26T01:35:41.3793056Z 6    |             |      |
2020-02-26T01:35:41.3793348Z 7    |             |      cannot infer type for type parameter `T`
2020-02-26T01:35:41.3793960Z -    |             this method call resolves to `std::option::Option<&T>`
---
2020-02-26T01:35:41.3796349Z 
2020-02-26T01:35:41.3796451Z 
2020-02-26T01:35:41.3796663Z The actual stderr differed from the expected stderr.
2020-02-26T01:35:41.3797403Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65611/issue-65611.stderr
2020-02-26T01:35:41.3798048Z To update references, rerun the tests and pass the `--bless` flag
2020-02-26T01:35:41.3798643Z To only update this specific test, also pass `--test-args issues/issue-65611.rs`
2020-02-26T01:35:41.3799105Z error: 1 errors occurred comparing output.
2020-02-26T01:35:41.3799354Z status: exit code: 1
2020-02-26T01:35:41.3799354Z status: exit code: 1
2020-02-26T01:35:41.3801312Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-65611.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65611" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65611/auxiliary"
2020-02-26T01:35:41.3802938Z ------------------------------------------
2020-02-26T01:35:41.3803122Z 
2020-02-26T01:35:41.3803531Z ------------------------------------------
2020-02-26T01:35:41.3803754Z stderr:
2020-02-26T01:35:41.3803754Z stderr:
2020-02-26T01:35:41.3804138Z ------------------------------------------
2020-02-26T01:35:41.3804606Z error[E0282]: type annotations needed
2020-02-26T01:35:41.3805976Z   --> /checkout/src/test/ui/issues/issue-65611.rs:59:27
2020-02-26T01:35:41.3806252Z    |
2020-02-26T01:35:41.3806480Z LL |     let x = buffer.last().unwrap().0.clone();
2020-02-26T01:35:41.3807744Z    |             |      |
2020-02-26T01:35:41.3808195Z    |             |      cannot infer type for type parameter `T`
2020-02-26T01:35:41.3808522Z    |             this method call resolves to `T`
2020-02-26T01:35:41.3808753Z    |
2020-02-26T01:35:41.3808753Z    |
2020-02-26T01:35:41.3808963Z    = note: type must be known at this point
2020-02-26T01:35:41.3809149Z 
2020-02-26T01:35:41.3809360Z error[E0609]: no field `0` on type `&_`
2020-02-26T01:35:41.3815758Z   --> /checkout/src/test/ui/issues/issue-65611.rs:59:36
2020-02-26T01:35:41.3816063Z    |
2020-02-26T01:35:41.3816290Z LL |     let x = buffer.last().unwrap().0.clone();
2020-02-26T01:35:41.3816782Z 
2020-02-26T01:35:41.3816978Z error: aborting due to 2 previous errors
2020-02-26T01:35:41.3817170Z 
2020-02-26T01:35:41.3817405Z Some errors have detailed explanations: E0282, E0609.
---
2020-02-26T01:35:41.3819667Z ---- [ui] ui/issues/issue-69455.rs stdout ----
2020-02-26T01:35:41.3819918Z diff of stderr:
2020-02-26T01:35:41.3820048Z 
2020-02-26T01:35:41.3820246Z 1 error[E0284]: type annotations needed
2020-02-26T01:35:41.3820785Z -   --> /home/e/Local/rust/src/test/ui/issues/issue-69455.rs:29:26
2020-02-26T01:35:41.3821290Z +   --> $DIR/issue-69455.rs:29:26
2020-02-26T01:35:41.3821826Z - 6  |     type Output;
2020-02-26T01:35:41.3822050Z + LL |     type Output;
2020-02-26T01:35:41.3822050Z + LL |     type Output;
2020-02-26T01:35:41.3822562Z 5    |     ------------ `<Self as Test<Rhs>>::Output` defined here
2020-02-26T01:35:41.3822839Z 6 ...
2020-02-26T01:35:41.3823459Z - 29 |     println!("{}", 23u64.test(xs.iter().sum())); //~ ERROR: type annotation needed for the method test
2020-02-26T01:35:41.3823899Z + LL |     println!("{}", 23u64.test(xs.iter().sum()));
2020-02-26T01:35:41.3824779Z 9    |                    |     |
2020-02-26T01:35:41.3825077Z 10    |                    |     cannot infer type for type `u64`
2020-02-26T01:35:41.3825295Z 
2020-02-26T01:35:41.3825586Z 
2020-02-26T01:35:41.3825586Z 
2020-02-26T01:35:41.3825916Z The actual stderr differed from the expected stderr.
2020-02-26T01:35:41.3826599Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69455/issue-69455.stderr
2020-02-26T01:35:41.3827239Z To update references, rerun the tests and pass the `--bless` flag
2020-02-26T01:35:41.3827849Z To only update this specific test, also pass `--test-args issues/issue-69455.rs`
2020-02-26T01:35:41.3828298Z error: 1 errors occurred comparing output.
2020-02-26T01:35:41.3828561Z status: exit code: 1
2020-02-26T01:35:41.3828561Z status: exit code: 1
2020-02-26T01:35:41.3830526Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-69455.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69455" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69455/auxiliary"
2020-02-26T01:35:41.3832380Z ------------------------------------------
2020-02-26T01:35:41.3832563Z 
2020-02-26T01:35:41.3832954Z ------------------------------------------
2020-02-26T01:35:41.3833163Z stderr:
2020-02-26T01:35:41.3833163Z stderr:
2020-02-26T01:35:41.3833545Z ------------------------------------------
2020-02-26T01:35:41.3833829Z error[E0284]: type annotations needed
2020-02-26T01:35:41.3834326Z   --> /checkout/src/test/ui/issues/issue-69455.rs:29:26
2020-02-26T01:35:41.3834571Z    |
2020-02-26T01:35:41.3834755Z LL |     type Output;
2020-02-26T01:35:41.3835258Z    |     ------------ `<Self as Test<Rhs>>::Output` defined here
2020-02-26T01:35:41.3835526Z ...
2020-02-26T01:35:41.3835876Z LL |     println!("{}", 23u64.test(xs.iter().sum())); //~ ERROR: type annotation needed for `test`
2020-02-26T01:35:41.3836962Z    |                    |     |
2020-02-26T01:35:41.3837263Z    |                    |     cannot infer type for type `u64`
2020-02-26T01:35:41.3837263Z    |                    |     cannot infer type for type `u64`
2020-02-26T01:35:41.3837680Z    |                    this method call resolves to `<Self as Test<Rhs>>::Output`
2020-02-26T01:35:41.3837990Z    |
2020-02-26T01:35:41.3838262Z    = note: cannot resolve `<u64 as Test<_>>::Output == _`
2020-02-26T01:35:41.3838679Z error: aborting due to previous error
2020-02-26T01:35:41.3838853Z 
2020-02-26T01:35:41.3839323Z For more information about this error, try `rustc --explain E0284`.
2020-02-26T01:35:41.3839552Z 
---
2020-02-26T01:35:41.3841612Z 3    |
2020-02-26T01:35:41.3841786Z 4 LL |     let v = &[];
2020-02-26T01:35:41.3842212Z -    |         -   ^^^ cannot infer type
2020-02-26T01:35:41.3842587Z -    |         |
2020-02-26T01:35:41.3843145Z -    |         consider giving `v` the explicit type `&[_; 0]`, with the type parameters specified
2020-02-26T01:35:41.3843977Z + LL |     let it = v.iter();
2020-02-26T01:35:41.3845001Z +    |              -------- this method call resolves to `std::slice::Iter<'_, T>`
2020-02-26T01:35:41.3845328Z 8 
2020-02-26T01:35:41.3845530Z 9 error: aborting due to previous error
2020-02-26T01:35:41.3845530Z 9 error: aborting due to previous error
2020-02-26T01:35:41.3845808Z 10 
2020-02-26T01:35:41.3845915Z 
2020-02-26T01:35:41.3846015Z 
2020-02-26T01:35:41.3846246Z The actual stderr differed from the expected stderr.
2020-02-26T01:35:41.3846915Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-7813/issue-7813.stderr
2020-02-26T01:35:41.3847548Z To update references, rerun the tests and pass the `--bless` flag
2020-02-26T01:35:41.3848626Z To only update this specific test, also pass `--test-args issues/issue-7813.rs`
2020-02-26T01:35:41.3849070Z error: 1 errors occurred comparing output.
2020-02-26T01:35:41.3849332Z status: exit code: 1
2020-02-26T01:35:41.3849332Z status: exit code: 1
2020-02-26T01:35:41.3851287Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-7813.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-7813" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-7813/auxiliary"
2020-02-26T01:35:41.3853181Z ------------------------------------------
2020-02-26T01:35:41.3853362Z 
2020-02-26T01:35:41.3853750Z ------------------------------------------
2020-02-26T01:35:41.3853959Z stderr:
2020-02-26T01:35:41.3853959Z stderr:
2020-02-26T01:35:41.3854595Z ------------------------------------------
2020-02-26T01:35:41.3855024Z error[E0282]: type annotations needed for `&[_; 0]`
2020-02-26T01:35:41.3855566Z   --> /checkout/src/test/ui/issues/issue-7813.rs:2:13
2020-02-26T01:35:41.3855800Z    |
2020-02-26T01:35:41.3856045Z LL |     let v = &[]; //~ ERROR type annotations needed
2020-02-26T01:35:41.3856577Z LL |     let it = v.iter();
2020-02-26T01:35:41.3857168Z    |              -------- this method call resolves to `std::slice::Iter<'_, T>`
2020-02-26T01:35:41.3857454Z 
2020-02-26T01:35:41.3857774Z error: aborting due to previous error
---
2020-02-26T01:35:41.3860142Z 
2020-02-26T01:35:41.3860427Z 1 error[E0282]: type annotations needed for `std::option::Option<_>`
2020-02-26T01:35:41.3861253Z 2   --> $DIR/issue-42234-unknown-receiver-type.rs:7:7
2020-02-26T01:35:41.3861502Z 3    |
2020-02-26T01:35:41.3861895Z - LL |     let x: Option<_> = None;
2020-02-26T01:35:41.3862597Z -    |         - consider giving `x` the explicit type `std::option::Option<_>`, where the type parameter `T` is specified
2020-02-26T01:35:41.3863322Z 6 LL |     x.unwrap().method_that_could_exist_on_some_type();
2020-02-26T01:35:41.3864318Z +    |     --^^^^^^--
2020-02-26T01:35:41.3864507Z +    |     | |
2020-02-26T01:35:41.3864762Z +    |     | cannot infer type for type parameter `T`
2020-02-26T01:35:41.3865057Z +    |     this method call resolves to `T`
2020-02-26T01:35:41.3865057Z +    |     this method call resolves to `T`
2020-02-26T01:35:41.3865266Z 8    |
2020-02-26T01:35:41.3865620Z 9    = note: type must be known at this point
2020-02-26T01:35:41.3865838Z 10 
2020-02-26T01:35:41.3865946Z 
2020-02-26T01:35:41.3866046Z 
2020-02-26T01:35:41.3866275Z The actual stderr differed from the expected stderr.
2020-02-26T01:35:41.3867066Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-42234-unknown-receiver-type/issue-42234-unknown-receiver-type.stderr
2020-02-26T01:35:41.3867889Z To update references, rerun the tests and pass the `--bless` flag
2020-02-26T01:35:41.3868649Z To only update this specific test, also pass `--test-args span/issue-42234-unknown-receiver-type.rs`
2020-02-26T01:35:41.3869136Z error: 1 errors occurred comparing output.
2020-02-26T01:35:41.3869398Z status: exit code: 1
2020-02-26T01:35:41.3869398Z status: exit code: 1
2020-02-26T01:35:41.3871507Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-42234-unknown-receiver-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-42234-unknown-receiver-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-42234-unknown-receiver-type/auxiliary"
2020-02-26T01:35:41.3873564Z ------------------------------------------
2020-02-26T01:35:41.3873767Z 
2020-02-26T01:35:41.3874163Z ------------------------------------------
2020-02-26T01:35:41.3874373Z stderr:
2020-02-26T01:35:41.3874373Z stderr:
2020-02-26T01:35:41.3874750Z ------------------------------------------
2020-02-26T01:35:41.3875113Z error[E0282]: type annotations needed for `std::option::Option<_>`
2020-02-26T01:35:41.3875746Z   --> /checkout/src/test/ui/span/issue-42234-unknown-receiver-type.rs:7:7
2020-02-26T01:35:41.3876043Z    |
2020-02-26T01:35:41.3876278Z LL |     x.unwrap().method_that_could_exist_on_some_type();
2020-02-26T01:35:41.3876693Z    |     --^^^^^^--
2020-02-26T01:35:41.3877126Z    |     | cannot infer type for type parameter `T`
2020-02-26T01:35:41.3877409Z    |     this method call resolves to `T`
2020-02-26T01:35:41.3877609Z    |
2020-02-26T01:35:41.3877834Z    = note: type must be known at this point
2020-02-26T01:35:41.3877834Z    = note: type must be known at this point
2020-02-26T01:35:41.3878026Z 
2020-02-26T01:35:41.3878219Z error[E0282]: type annotations needed
2020-02-26T01:35:41.3878800Z   --> /checkout/src/test/ui/span/issue-42234-unknown-receiver-type.rs:13:10
2020-02-26T01:35:41.3879082Z    |
2020-02-26T01:35:41.3879326Z LL |         .sum::<_>() //~ ERROR type annotations needed
2020-02-26T01:35:41.3879829Z    |
2020-02-26T01:35:41.3880038Z    = note: type must be known at this point
2020-02-26T01:35:41.3880226Z 
2020-02-26T01:35:41.3880434Z error: aborting due to 2 previous errors
---
2020-02-26T01:35:41.3887864Z 9    |
2020-02-26T01:35:41.3887975Z 
2020-02-26T01:35:41.3888135Z 
2020-02-26T01:35:41.3888359Z The actual stderr differed from the expected stderr.
2020-02-26T01:35:41.3889199Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-2/unboxed-closures-failed-recursive-fn-2.stderr
2020-02-26T01:35:41.3889948Z To update references, rerun the tests and pass the `--bless` flag
2020-02-26T01:35:41.3890657Z To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closures-failed-recursive-fn-2.rs`
2020-02-26T01:35:41.3891169Z error: 1 errors occurred comparing output.
2020-02-26T01:35:41.3891601Z status: exit code: 1
2020-02-26T01:35:41.3891601Z status: exit code: 1
2020-02-26T01:35:41.3894716Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-2/auxiliary"
2020-02-26T01:35:41.3896549Z ------------------------------------------
2020-02-26T01:35:41.3896732Z 
2020-02-26T01:35:41.3897121Z ------------------------------------------
2020-02-26T01:35:41.3897329Z stderr:
---
2020-02-26T01:35:41.3908621Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-26T01:35:41.3909046Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-26T01:35:41.3909286Z 
2020-02-26T01:35:41.3909402Z 
2020-02-26T01:35:41.3913161Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-26T01:35:41.3915956Z 
2020-02-26T01:35:41.3916058Z 
2020-02-26T01:35:41.3917976Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-26T01:35:41.3918391Z Build completed unsuccessfully in 1:02:02
2020-02-26T01:35:41.3918391Z Build completed unsuccessfully in 1:02:02
2020-02-26T01:35:41.3918752Z == clock drift check ==
2020-02-26T01:35:41.3919035Z   local time: Wed Feb 26 01:35:41 UTC 2020
2020-02-26T01:35:41.6729805Z   network time: Wed, 26 Feb 2020 01:35:41 GMT
2020-02-26T01:35:41.6734372Z == end clock drift check ==
2020-02-26T01:35:42.1213410Z 
2020-02-26T01:35:42.1284160Z ##[error]Bash exited with code '1'.
2020-02-26T01:35:42.1296750Z ##[section]Finishing: Run build
2020-02-26T01:35:42.1344433Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69456/merge to s
2020-02-26T01:35:42.1349220Z Task         : Get sources
2020-02-26T01:35:42.1349590Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-26T01:35:42.1349920Z Version      : 1.0.0
2020-02-26T01:35:42.1350151Z Author       : Microsoft
2020-02-26T01:35:42.1350151Z Author       : Microsoft
2020-02-26T01:35:42.1350533Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-26T01:35:42.1350954Z ==============================================================================
2020-02-26T01:35:42.4465036Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-26T01:35:42.4469239Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69456/merge to s
2020-02-26T01:35:42.4552058Z Cleaning up task key
2020-02-26T01:35:42.4553326Z Start cleaning up orphan processes.
2020-02-26T01:35:42.4708743Z Terminate orphan process: pid (4143) (python)
2020-02-26T01:35:42.4919356Z ##[section]Finishing: Finalize Job
