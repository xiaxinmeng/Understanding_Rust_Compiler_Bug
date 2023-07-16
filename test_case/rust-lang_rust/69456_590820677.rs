plain
2020-02-25T10:19:14.3244543Z ========================== Starting Command Output ===========================
2020-02-25T10:19:14.3246936Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c71bc457-f443-4963-b790-7c93f158b4d7.sh
2020-02-25T10:19:14.3247793Z 
2020-02-25T10:19:14.3250737Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-25T10:19:14.3271281Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69456/merge to s
2020-02-25T10:19:14.3274624Z Task         : Get sources
2020-02-25T10:19:14.3274844Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-25T10:19:14.3275531Z Version      : 1.0.0
2020-02-25T10:19:14.3275682Z Author       : Microsoft
---
2020-02-25T10:19:15.3160694Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-25T10:19:15.3165017Z ##[command]git config gc.auto 0
2020-02-25T10:19:15.3168010Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-25T10:19:15.3170739Z ##[command]git config --get-all http.proxy
2020-02-25T10:19:15.3175826Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69456/merge:refs/remotes/pull/69456/merge
---
2020-02-25T11:16:40.9481843Z .................................................................................................... 1700/9704
2020-02-25T11:16:45.1477208Z .................................................................................................... 1800/9704
2020-02-25T11:16:55.4042056Z ...........................................i........................................................ 1900/9704
2020-02-25T11:17:02.5338772Z .................................................................................................... 2000/9704
2020-02-25T11:17:15.3648893Z .................................iiiii.............................................................. 2100/9704
2020-02-25T11:17:24.2239979Z .................................................................................................... 2300/9704
2020-02-25T11:17:26.4283713Z .................................................................................................... 2400/9704
2020-02-25T11:17:30.4027201Z .................................................................................................... 2500/9704
2020-02-25T11:17:49.6996163Z .................................................................................................... 2600/9704
---
2020-02-25T11:20:14.3004126Z .........i.......................................................................................... 5000/9704
2020-02-25T11:20:22.6954414Z .................................................................................................... 5100/9704
2020-02-25T11:20:26.6865861Z ....................................i......................................................F........ 5200/9704
2020-02-25T11:20:35.2671085Z .................................................................................................... 5300/9704
2020-02-25T11:20:40.3847083Z ............ii.ii........i...i...................................................................... 5400/9704
2020-02-25T11:20:47.5449896Z .................................................................................................... 5600/9704
2020-02-25T11:20:57.0756100Z .................................................................................................... 5700/9704
2020-02-25T11:21:03.1644261Z ...i................................................................................................ 5800/9704
2020-02-25T11:21:08.0079639Z .................................................................................................... 5900/9704
2020-02-25T11:21:08.0079639Z .................................................................................................... 5900/9704
2020-02-25T11:21:16.5265690Z ..............................................................................................ii...i 6000/9704
2020-02-25T11:21:26.7404201Z ..ii...........i.................................................................................... 6100/9704
2020-02-25T11:21:41.0885200Z .................................................................................................... 6300/9704
2020-02-25T11:21:47.4070600Z .................................................................................................... 6400/9704
2020-02-25T11:21:47.4070600Z .................................................................................................... 6400/9704
2020-02-25T11:22:02.2547123Z .........................i..ii...................................................................... 6500/9704
2020-02-25T11:22:21.5261424Z .................................................................................................... 6700/9704
2020-02-25T11:22:23.5385096Z .................i.................................................................................. 6800/9704
2020-02-25T11:22:25.4889205Z .................................................................................................... 6900/9704
2020-02-25T11:22:27.6321805Z ...............................................i.................................................... 7000/9704
---
2020-02-25T11:23:59.4369707Z .................................................................................................... 7700/9704
2020-02-25T11:24:03.5278955Z .................................................................................................... 7800/9704
2020-02-25T11:24:09.2421520Z ...........................................................................................i........ 7900/9704
2020-02-25T11:24:16.3554061Z .................................................................................................... 8000/9704
2020-02-25T11:24:22.5927790Z ........................................iiiiiii.i................................................... 8100/9704
2020-02-25T11:24:34.8786489Z ..........................................................................F......................... 8300/9704
2020-02-25T11:24:40.3811809Z .................................................................................................... 8400/9704
2020-02-25T11:24:53.2039170Z .................................................................................................... 8500/9704
2020-02-25T11:24:59.1914896Z .................................................................................................... 8600/9704
---
2020-02-25T11:26:42.9818173Z diff of stderr:
2020-02-25T11:26:42.9818425Z 
2020-02-25T11:26:42.9818931Z 2   --> $DIR/issue-12028.rs:27:14
2020-02-25T11:26:42.9819282Z 3    |
2020-02-25T11:26:42.9819616Z 4 LL |         self.input_stream(&mut stream);
2020-02-25T11:26:42.9820314Z -    |              ^^^^^^^^^^^^ cannot infer type for type parameter `H` declared on the trait `StreamHash`
2020-02-25T11:26:42.9821403Z +    |         |    |
2020-02-25T11:26:42.9821403Z +    |         |    |
2020-02-25T11:26:42.9821796Z +    |         |    cannot infer type for type parameter `H` declared on the trait `StreamHash`
2020-02-25T11:26:42.9822237Z +    |         this method call resolves to `()`
2020-02-25T11:26:42.9822531Z 6    |
2020-02-25T11:26:42.9822913Z 7    = note: cannot resolve `<_ as StreamHasher>::S == <H as StreamHasher>::S`
2020-02-25T11:26:42.9823480Z 
2020-02-25T11:26:42.9823687Z 
2020-02-25T11:26:42.9824008Z The actual stderr differed from the expected stderr.
2020-02-25T11:26:42.9824711Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12028/issue-12028.stderr
2020-02-25T11:26:42.9824711Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12028/issue-12028.stderr
2020-02-25T11:26:42.9825431Z To update references, rerun the tests and pass the `--bless` flag
2020-02-25T11:26:42.9826164Z To only update this specific test, also pass `--test-args issues/issue-12028.rs`
2020-02-25T11:26:42.9826830Z error: 1 errors occurred comparing output.
2020-02-25T11:26:42.9827170Z status: exit code: 1
2020-02-25T11:26:42.9827170Z status: exit code: 1
2020-02-25T11:26:42.9829208Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12028.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12028" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12028/auxiliary"
2020-02-25T11:26:42.9831769Z ------------------------------------------
2020-02-25T11:26:42.9832271Z 
2020-02-25T11:26:42.9832944Z ------------------------------------------
2020-02-25T11:26:42.9833956Z stderr:
2020-02-25T11:26:42.9833956Z stderr:
2020-02-25T11:26:42.9834827Z ------------------------------------------
2020-02-25T11:26:42.9835560Z error[E0284]: type annotations needed
2020-02-25T11:26:42.9836838Z   --> /checkout/src/test/ui/issues/issue-12028.rs:27:14
2020-02-25T11:26:42.9837258Z    |
2020-02-25T11:26:42.9837897Z LL |         self.input_stream(&mut stream); //~ ERROR type annotations needed
2020-02-25T11:26:42.9838759Z    |         -----^^^^^^^^^^^^-------------
2020-02-25T11:26:42.9839278Z    |         |    |
2020-02-25T11:26:42.9839644Z    |         |    cannot infer type for type parameter `H` declared on the trait `StreamHash`
2020-02-25T11:26:42.9840070Z    |         this method call resolves to `()`
2020-02-25T11:26:42.9840340Z    |
2020-02-25T11:26:42.9840713Z    = note: cannot resolve `<_ as StreamHasher>::S == <H as StreamHasher>::S`
2020-02-25T11:26:42.9841295Z error: aborting due to previous error
2020-02-25T11:26:42.9841680Z 
2020-02-25T11:26:42.9842425Z For more information about this error, try `rustc --explain E0284`.
2020-02-25T11:26:42.9842741Z 
---
2020-02-25T11:26:42.9845910Z - error[E0282]: type annotations needed for `&(_,)`
2020-02-25T11:26:42.9846288Z + error[E0282]: type annotations needed
2020-02-25T11:26:42.9846810Z 2   --> $DIR/issue-20261.rs:4:11
2020-02-25T11:26:42.9847256Z 3    |
2020-02-25T11:26:42.9847858Z - LL |     for (ref i,) in [].iter() {
2020-02-25T11:26:42.9848987Z 6 LL |         i.clone();
2020-02-25T11:26:42.9849265Z 7    |           ^^^^^ cannot infer type
2020-02-25T11:26:42.9849508Z 8    |
2020-02-25T11:26:42.9849668Z 
2020-02-25T11:26:42.9849668Z 
2020-02-25T11:26:42.9849821Z 
2020-02-25T11:26:42.9850079Z The actual stderr differed from the expected stderr.
2020-02-25T11:26:42.9850760Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20261/issue-20261.stderr
2020-02-25T11:26:42.9851818Z To update references, rerun the tests and pass the `--bless` flag
2020-02-25T11:26:42.9854431Z To only update this specific test, also pass `--test-args issues/issue-20261.rs`
2020-02-25T11:26:42.9855223Z error: 1 errors occurred comparing output.
2020-02-25T11:26:42.9855541Z status: exit code: 1
2020-02-25T11:26:42.9855541Z status: exit code: 1
2020-02-25T11:26:42.9857467Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20261.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20261" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20261/auxiliary"
2020-02-25T11:26:42.9859285Z ------------------------------------------
2020-02-25T11:26:42.9859598Z 
2020-02-25T11:26:42.9860265Z ------------------------------------------
2020-02-25T11:26:42.9860558Z stderr:
---
2020-02-25T11:26:42.9870760Z 1 error[E0282]: type annotations needed
2020-02-25T11:26:42.9871324Z -   --> $DIR/issue-65611.rs:59:20
2020-02-25T11:26:42.9872081Z +   --> $DIR/issue-65611.rs:59:27
2020-02-25T11:26:42.9872371Z 3    |
2020-02-25T11:26:42.9872639Z 4 LL |     let x = buffer.last().unwrap().0.clone();
2020-02-25T11:26:42.9874696Z +    |             --------------^^^^^^--
2020-02-25T11:26:42.9875365Z 6    |             |      |
2020-02-25T11:26:42.9877053Z 7    |             |      cannot infer type for type parameter `T`
2020-02-25T11:26:42.9877817Z -    |             this method call resolves to `std::option::Option<&T>`
---
2020-02-25T11:26:42.9878738Z 
2020-02-25T11:26:42.9878814Z 
2020-02-25T11:26:42.9878975Z The actual stderr differed from the expected stderr.
2020-02-25T11:26:42.9879527Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65611/issue-65611.stderr
2020-02-25T11:26:42.9880033Z To update references, rerun the tests and pass the `--bless` flag
2020-02-25T11:26:42.9880509Z To only update this specific test, also pass `--test-args issues/issue-65611.rs`
2020-02-25T11:26:42.9880868Z error: 1 errors occurred comparing output.
2020-02-25T11:26:42.9881059Z status: exit code: 1
2020-02-25T11:26:42.9881059Z status: exit code: 1
2020-02-25T11:26:42.9882707Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-65611.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65611" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65611/auxiliary"
2020-02-25T11:26:42.9883899Z ------------------------------------------
2020-02-25T11:26:42.9884030Z 
2020-02-25T11:26:42.9884307Z ------------------------------------------
2020-02-25T11:26:42.9884479Z stderr:
2020-02-25T11:26:42.9884479Z stderr:
2020-02-25T11:26:42.9884765Z ------------------------------------------
2020-02-25T11:26:42.9884967Z error[E0282]: type annotations needed
2020-02-25T11:26:42.9885357Z   --> /checkout/src/test/ui/issues/issue-65611.rs:59:27
2020-02-25T11:26:42.9885532Z    |
2020-02-25T11:26:42.9885690Z LL |     let x = buffer.last().unwrap().0.clone();
2020-02-25T11:26:42.9886219Z    |             |      |
2020-02-25T11:26:42.9886424Z    |             |      cannot infer type for type parameter `T`
2020-02-25T11:26:42.9886656Z    |             this method call resolves to `T`
2020-02-25T11:26:42.9886824Z    |
2020-02-25T11:26:42.9886824Z    |
2020-02-25T11:26:42.9886975Z    = note: type must be known at this point
2020-02-25T11:26:42.9887107Z 
2020-02-25T11:26:42.9887265Z error[E0609]: no field `0` on type `&_`
2020-02-25T11:26:42.9887643Z   --> /checkout/src/test/ui/issues/issue-65611.rs:59:36
2020-02-25T11:26:42.9887814Z    |
2020-02-25T11:26:42.9887988Z LL |     let x = buffer.last().unwrap().0.clone();
2020-02-25T11:26:42.9888320Z 
2020-02-25T11:26:42.9888457Z error: aborting due to 2 previous errors
2020-02-25T11:26:42.9888600Z 
2020-02-25T11:26:42.9888767Z Some errors have detailed explanations: E0282, E0609.
---
2020-02-25T11:26:42.9890614Z 
2020-02-25T11:26:42.9890808Z 1 error[E0282]: type annotations needed for `std::option::Option<_>`
2020-02-25T11:26:42.9891255Z 2   --> $DIR/issue-42234-unknown-receiver-type.rs:7:7
2020-02-25T11:26:42.9891435Z 3    |
2020-02-25T11:26:42.9891720Z - LL |     let x: Option<_> = None;
2020-02-25T11:26:42.9892255Z -    |         - consider giving `x` the explicit type `std::option::Option<_>`, where the type parameter `T` is specified
2020-02-25T11:26:42.9892685Z 6 LL |     x.unwrap().method_that_could_exist_on_some_type();
2020-02-25T11:26:42.9893444Z +    |     --^^^^^^--
2020-02-25T11:26:42.9893579Z +    |     | |
2020-02-25T11:26:42.9893756Z +    |     | cannot infer type for type parameter `T`
2020-02-25T11:26:42.9893982Z +    |     this method call resolves to `T`
2020-02-25T11:26:42.9893982Z +    |     this method call resolves to `T`
2020-02-25T11:26:42.9894130Z 8    |
2020-02-25T11:26:42.9894286Z 9    = note: type must be known at this point
2020-02-25T11:26:42.9894437Z 10 
2020-02-25T11:26:42.9894531Z 
2020-02-25T11:26:42.9894603Z 
2020-02-25T11:26:42.9894755Z The actual stderr differed from the expected stderr.
2020-02-25T11:26:42.9895342Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-42234-unknown-receiver-type/issue-42234-unknown-receiver-type.stderr
2020-02-25T11:26:42.9895888Z To update references, rerun the tests and pass the `--bless` flag
2020-02-25T11:26:42.9896380Z To only update this specific test, also pass `--test-args span/issue-42234-unknown-receiver-type.rs`
2020-02-25T11:26:42.9896753Z error: 1 errors occurred comparing output.
2020-02-25T11:26:42.9896926Z status: exit code: 1
2020-02-25T11:26:42.9896926Z status: exit code: 1
2020-02-25T11:26:42.9898508Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-42234-unknown-receiver-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-42234-unknown-receiver-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-42234-unknown-receiver-type/auxiliary"
2020-02-25T11:26:42.9899984Z ------------------------------------------
2020-02-25T11:26:42.9900123Z 
2020-02-25T11:26:42.9900416Z ------------------------------------------
2020-02-25T11:26:42.9900596Z stderr:
2020-02-25T11:26:42.9900596Z stderr:
2020-02-25T11:26:42.9900895Z ------------------------------------------
2020-02-25T11:26:42.9901164Z error[E0282]: type annotations needed for `std::option::Option<_>`
2020-02-25T11:26:42.9901678Z   --> /checkout/src/test/ui/span/issue-42234-unknown-receiver-type.rs:7:7
2020-02-25T11:26:42.9901895Z    |
2020-02-25T11:26:42.9902072Z LL |     x.unwrap().method_that_could_exist_on_some_type();
2020-02-25T11:26:42.9902418Z    |     --^^^^^^--
2020-02-25T11:26:42.9902738Z    |     | cannot infer type for type parameter `T`
2020-02-25T11:26:42.9902973Z    |     this method call resolves to `T`
2020-02-25T11:26:42.9903124Z    |
2020-02-25T11:26:42.9903284Z    = note: type must be known at this point
2020-02-25T11:26:42.9903284Z    = note: type must be known at this point
2020-02-25T11:26:42.9903424Z 
2020-02-25T11:26:42.9903591Z error[E0282]: type annotations needed
2020-02-25T11:26:42.9904033Z   --> /checkout/src/test/ui/span/issue-42234-unknown-receiver-type.rs:13:10
2020-02-25T11:26:42.9904251Z    |
2020-02-25T11:26:42.9904453Z LL |         .sum::<_>() //~ ERROR type annotations needed
2020-02-25T11:26:42.9904821Z    |
2020-02-25T11:26:42.9905066Z    = note: type must be known at this point
2020-02-25T11:26:42.9905215Z 
2020-02-25T11:26:42.9905363Z error: aborting due to 2 previous errors
---
2020-02-25T11:26:42.9907105Z diff of stderr:
2020-02-25T11:26:42.9907202Z 
2020-02-25T11:26:42.9907499Z 2   --> $DIR/sort_by_key.rs:3:9
2020-02-25T11:26:42.9907721Z 3    |
2020-02-25T11:26:42.9910092Z 4 LL |     lst.sort_by_key(|&(v, _)| v.iter().sum());
2020-02-25T11:26:42.9910950Z -    |         ^^^^^^^^^^^                    --- help: consider specifying the type argument in the method call: `sum::<S>`
2020-02-25T11:26:42.9911432Z 6    |         |
2020-02-25T11:26:42.9911432Z 6    |         |
2020-02-25T11:26:42.9911676Z 7    |         cannot infer type for type parameter `K` declared on the method `sort_by_key`
2020-02-25T11:26:42.9912044Z +    |         help: consider specifying the type arguments in the method call: `sort_by_key::<K, F>`
2020-02-25T11:26:42.9912456Z 9 error: aborting due to previous error
2020-02-25T11:26:42.9912597Z 10 
2020-02-25T11:26:42.9912673Z 
2020-02-25T11:26:42.9912745Z 
2020-02-25T11:26:42.9912745Z 
2020-02-25T11:26:42.9912913Z The actual stderr differed from the expected stderr.
2020-02-25T11:26:42.9913432Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-inference/sort_by_key/sort_by_key.stderr
2020-02-25T11:26:42.9913921Z To update references, rerun the tests and pass the `--bless` flag
2020-02-25T11:26:42.9914398Z To only update this specific test, also pass `--test-args type-inference/sort_by_key.rs`
2020-02-25T11:26:42.9914731Z error: 1 errors occurred comparing output.
2020-02-25T11:26:42.9914928Z status: exit code: 1
2020-02-25T11:26:42.9914928Z status: exit code: 1
2020-02-25T11:26:42.9919695Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-inference/sort_by_key.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-inference/sort_by_key" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-inference/sort_by_key/auxiliary"
2020-02-25T11:26:42.9921320Z ------------------------------------------
2020-02-25T11:26:42.9921483Z 
2020-02-25T11:26:42.9922334Z ------------------------------------------
2020-02-25T11:26:42.9922658Z stderr:
2020-02-25T11:26:42.9922658Z stderr:
2020-02-25T11:26:42.9922961Z ------------------------------------------
2020-02-25T11:26:42.9923176Z error[E0282]: type annotations needed
2020-02-25T11:26:42.9923605Z   --> /checkout/src/test/ui/type-inference/sort_by_key.rs:3:9
2020-02-25T11:26:42.9923797Z    |
2020-02-25T11:26:42.9924034Z LL |     lst.sort_by_key(|&(v, _)| v.iter().sum()); //~ ERROR type annotations needed
2020-02-25T11:26:42.9924420Z    |         |
2020-02-25T11:26:42.9924420Z    |         |
2020-02-25T11:26:42.9924850Z    |         cannot infer type for type parameter `K` declared on the method `sort_by_key`
2020-02-25T11:26:42.9925768Z    |         help: consider specifying the type arguments in the method call: `sort_by_key::<K, F>`
2020-02-25T11:26:42.9926652Z error: aborting due to previous error
2020-02-25T11:26:42.9927011Z 
2020-02-25T11:26:42.9927468Z For more information about this error, try `rustc --explain E0282`.
2020-02-25T11:26:42.9927664Z 
---
2020-02-25T11:26:42.9929907Z 
2020-02-25T11:26:42.9930060Z 1 error[E0282]: type annotations needed
2020-02-25T11:26:42.9930460Z 2   --> $DIR/unbounded-associated-type.rs:15:5
2020-02-25T11:26:42.9930663Z 3    |
2020-02-25T11:26:42.9930927Z - LL |     type A;
2020-02-25T11:26:42.9931284Z -    |     ------- `<Self as T>::A` defined here
2020-02-25T11:26:42.9931605Z - ...
2020-02-25T11:26:42.9931787Z 7 LL |     S(std::marker::PhantomData).foo();
2020-02-25T11:26:42.9933143Z -    |     |
2020-02-25T11:26:42.9933572Z -    |     this method call resolves to `<Self as T>::A`
2020-02-25T11:26:42.9934213Z -    |     cannot infer type for type parameter `X` declared on the struct `S`
2020-02-25T11:26:42.9934524Z +    |     ^ cannot infer type for type parameter `X` declared on the struct `S`
2020-02-25T11:26:42.9934524Z +    |     ^ cannot infer type for type parameter `X` declared on the struct `S`
2020-02-25T11:26:42.9934750Z 12 
2020-02-25T11:26:42.9934904Z 13 error: aborting due to previous error
2020-02-25T11:26:42.9935055Z 14 
2020-02-25T11:26:42.9935154Z 
2020-02-25T11:26:42.9935405Z 
2020-02-25T11:26:42.9937494Z The actual stderr differed from the expected stderr.
2020-02-25T11:26:42.9939318Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-inference/unbounded-associated-type/unbounded-associated-type.stderr
2020-02-25T11:26:42.9939895Z To update references, rerun the tests and pass the `--bless` flag
2020-02-25T11:26:42.9940391Z To only update this specific test, also pass `--test-args type-inference/unbounded-associated-type.rs`
2020-02-25T11:26:42.9940775Z error: 1 errors occurred comparing output.
2020-02-25T11:26:42.9945818Z status: exit code: 1
2020-02-25T11:26:42.9945818Z status: exit code: 1
2020-02-25T11:26:42.9948469Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-inference/unbounded-associated-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-inference/unbounded-associated-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-inference/unbounded-associated-type/auxiliary"
2020-02-25T11:26:42.9950273Z ------------------------------------------
2020-02-25T11:26:42.9950433Z 
2020-02-25T11:26:42.9950757Z ------------------------------------------
2020-02-25T11:26:42.9950929Z stderr:
2020-02-25T11:26:42.9950929Z stderr:
2020-02-25T11:26:42.9951267Z ------------------------------------------
2020-02-25T11:26:42.9951489Z error[E0282]: type annotations needed
2020-02-25T11:26:42.9952292Z   --> /checkout/src/test/ui/type-inference/unbounded-associated-type.rs:15:5
2020-02-25T11:26:42.9952518Z    |
2020-02-25T11:26:42.9952726Z LL |     S(std::marker::PhantomData).foo(); //~ ERROR type annotations needed
2020-02-25T11:26:42.9953023Z    |     ^ cannot infer type for type parameter `X` declared on the struct `S`
2020-02-25T11:26:42.9953346Z error: aborting due to previous error
2020-02-25T11:26:42.9953468Z 
2020-02-25T11:26:42.9953806Z For more information about this error, try `rustc --explain E0282`.
2020-02-25T11:26:42.9953991Z 
---
2020-02-25T11:26:42.9958974Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-25T11:26:42.9959320Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-25T11:26:42.9959515Z 
2020-02-25T11:26:42.9959613Z 
2020-02-25T11:26:42.9962954Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-25T11:26:42.9965317Z 
2020-02-25T11:26:42.9965397Z 
2020-02-25T11:26:42.9965589Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-25T11:26:42.9965853Z Build completed unsuccessfully in 1:00:57
2020-02-25T11:26:42.9965853Z Build completed unsuccessfully in 1:00:57
2020-02-25T11:26:42.9966265Z == clock drift check ==
2020-02-25T11:26:42.9966457Z   local time: Tue Feb 25 11:26:42 UTC 2020
2020-02-25T11:26:43.1546911Z   network time: Tue, 25 Feb 2020 11:26:43 GMT
2020-02-25T11:26:43.1547132Z == end clock drift check ==
2020-02-25T11:26:43.7006329Z 
2020-02-25T11:26:43.7069018Z ##[error]Bash exited with code '1'.
2020-02-25T11:26:43.7080168Z ##[section]Finishing: Run build
2020-02-25T11:26:43.7122792Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69456/merge to s
2020-02-25T11:26:43.7126784Z Task         : Get sources
2020-02-25T11:26:43.7127047Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-25T11:26:43.7127313Z Version      : 1.0.0
2020-02-25T11:26:43.7127484Z Author       : Microsoft
2020-02-25T11:26:43.7127484Z Author       : Microsoft
2020-02-25T11:26:43.7127753Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-25T11:26:43.7128085Z ==============================================================================
2020-02-25T11:26:43.9950012Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-25T11:26:43.9987265Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69456/merge to s
2020-02-25T11:26:44.0064381Z Cleaning up task key
2020-02-25T11:26:44.0065341Z Start cleaning up orphan processes.
2020-02-25T11:26:44.0197007Z Terminate orphan process: pid (4659) (python)
2020-02-25T11:26:44.0398991Z ##[section]Finishing: Finalize Job
