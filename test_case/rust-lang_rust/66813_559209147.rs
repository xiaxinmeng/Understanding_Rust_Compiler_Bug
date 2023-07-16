plain
2019-11-27T17:35:10.5830120Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-27T17:35:10.6016713Z ##[command]git config gc.auto 0
2019-11-27T17:35:10.6097176Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-27T17:35:10.6186510Z ##[command]git config --get-all http.proxy
2019-11-27T17:35:10.6336324Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66813/merge:refs/remotes/pull/66813/merge
---
2019-11-27T18:35:22.3035055Z .................................................................................................... 1600/9298
2019-11-27T18:35:27.1486231Z .................................................................................................... 1700/9298
2019-11-27T18:35:39.9512103Z ...................................i................................................................ 1800/9298
2019-11-27T18:35:47.8394474Z .................................................................................................... 1900/9298
2019-11-27T18:36:01.7674052Z ....................iiiii........................................................................... 2000/9298
2019-11-27T18:36:12.0005331Z .................................................................................................... 2200/9298
2019-11-27T18:36:14.5676214Z .................................................................................................... 2300/9298
2019-11-27T18:36:19.6606434Z .................................................................................................... 2400/9298
2019-11-27T18:36:41.1421755Z .................................................................................................... 2500/9298
---
2019-11-27T18:39:26.0364033Z .....................i...............i.............................................................. 4800/9298
2019-11-27T18:39:36.8675314Z .................................................................................................... 4900/9298
2019-11-27T18:39:42.8039904Z .................................................................................................... 5000/9298
2019-11-27T18:39:51.5216132Z .................................................................................................... 5100/9298
2019-11-27T18:39:59.2055198Z .........................ii.ii...........i.......................................................... 5200/9298
2019-11-27T18:40:08.8648521Z .................................................................................................... 5400/9298
2019-11-27T18:40:20.0181749Z .................................................................................................... 5500/9298
2019-11-27T18:40:27.2031931Z .......i............................................................................................ 5600/9298
2019-11-27T18:40:33.7977816Z .................................................................................................... 5700/9298
2019-11-27T18:40:33.7977816Z .................................................................................................... 5700/9298
2019-11-27T18:40:44.6014395Z .............................................................................................ii...i. 5800/9298
2019-11-27T18:40:57.7269529Z .ii...........i..................................................................................... 5900/9298
2019-11-27T18:41:17.0467740Z .................................................................................................... 6100/9298
2019-11-27T18:41:22.0144677Z .................................................................................................... 6200/9298
2019-11-27T18:41:22.0144677Z .................................................................................................... 6200/9298
2019-11-27T18:41:36.5752147Z ................i..ii............................................................................... 6300/9298
2019-11-27T18:41:56.5339701Z ....................................................................................i............... 6500/9298
2019-11-27T18:41:59.0028832Z .................................................................................................... 6600/9298
2019-11-27T18:42:01.5032162Z ...........................................................................i........................ 6700/9298
2019-11-27T18:42:04.4245283Z .................................................................................................... 6800/9298
---
2019-11-27T18:47:01.6673671Z 
2019-11-27T18:47:01.6675156Z ---- [ui] ui/associated-types/associated-types-overridden-binding-2.rs stdout ----
2019-11-27T18:47:01.6675677Z diff of stderr:
2019-11-27T18:47:01.6675852Z 
2019-11-27T18:47:01.6676025Z 4 LL |     let _: &dyn I32Iterator<Item = u32> = &vec![42].into_iter();
2019-11-27T18:47:01.6676520Z 5    |                                           ^^^^^^^^^^^^^^^^^^^^^ expected `i32`, found `u32`
2019-11-27T18:47:01.6676744Z 6    |
2019-11-27T18:47:01.6677391Z -    = note: required for the cast to the object type `dyn std::iter::Iterator<Item = u32, Item = i32>`
2019-11-27T18:47:01.6677744Z +    = note: required for the cast to the object type `dyn I32Iterator<Item = u32, Item = i32>`
2019-11-27T18:47:01.6678222Z 9 error: aborting due to previous error
2019-11-27T18:47:01.6678484Z 10 
2019-11-27T18:47:01.6678606Z 
2019-11-27T18:47:01.6678740Z 
2019-11-27T18:47:01.6678740Z 
2019-11-27T18:47:01.6678876Z The actual stderr differed from the expected stderr.
2019-11-27T18:47:01.6679402Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding-2/associated-types-overridden-binding-2.stderr
2019-11-27T18:47:01.6679820Z To update references, rerun the tests and pass the `--bless` flag
2019-11-27T18:47:01.6680345Z To only update this specific test, also pass `--test-args associated-types/associated-types-overridden-binding-2.rs`
2019-11-27T18:47:01.6680898Z error: 1 errors occurred comparing output.
2019-11-27T18:47:01.6681137Z status: exit code: 1
2019-11-27T18:47:01.6681137Z status: exit code: 1
2019-11-27T18:47:01.6682315Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-overridden-binding-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding-2/auxiliary" "-A" "unused"
2019-11-27T18:47:01.6683158Z ------------------------------------------
2019-11-27T18:47:01.6683325Z 
2019-11-27T18:47:01.6684096Z ------------------------------------------
2019-11-27T18:47:01.6684540Z stderr:
2019-11-27T18:47:01.6684540Z stderr:
2019-11-27T18:47:01.6684966Z ------------------------------------------
2019-11-27T18:47:01.6685159Z error[E0271]: type mismatch resolving `<std::vec::IntoIter<u32> as std::iter::Iterator>::Item == i32`
2019-11-27T18:47:01.6685576Z   --> /checkout/src/test/ui/associated-types/associated-types-overridden-binding-2.rs:6:43
2019-11-27T18:47:01.6685780Z    |
2019-11-27T18:47:01.6685930Z LL |     let _: &dyn I32Iterator<Item = u32> = &vec![42].into_iter();
2019-11-27T18:47:01.6686086Z    |                                           ^^^^^^^^^^^^^^^^^^^^^ expected `i32`, found `u32`
2019-11-27T18:47:01.6686252Z    |
2019-11-27T18:47:01.6686405Z    = note: required for the cast to the object type `dyn I32Iterator<Item = u32, Item = i32>`
2019-11-27T18:47:01.6686701Z error: aborting due to previous error
2019-11-27T18:47:01.6686837Z 
2019-11-27T18:47:01.6687219Z For more information about this error, try `rustc --explain E0271`.
2019-11-27T18:47:01.6687373Z 
---
2019-11-27T18:47:01.6690503Z - error[E0277]: the size for values of type `dyn Trait` cannot be known at compilation time
2019-11-27T18:47:01.6690687Z + error[E0277]: the size for values of type `dyn std::marker::Sized` cannot be known at compilation time
2019-11-27T18:47:01.6691037Z 13   --> $DIR/bad-sized.rs:4:12
2019-11-27T18:47:01.6691226Z 14    |
2019-11-27T18:47:01.6691566Z 15 LL |     let x: Vec<dyn Trait + Sized> = Vec::new();
2019-11-27T18:47:01.6692119Z 16    |            ^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-11-27T18:47:01.6692292Z 17    |
2019-11-27T18:47:01.6692688Z -    = help: the trait `std::marker::Sized` is not implemented for `dyn Trait`
2019-11-27T18:47:01.6692897Z +    = help: the trait `std::marker::Sized` is not implemented for `dyn std::marker::Sized`
---
2019-11-27T18:47:01.6694695Z - error[E0277]: the size for values of type `dyn Trait` cannot be known at compilation time
2019-11-27T18:47:01.6695129Z + error[E0038]: the trait `std::marker::Sized` cannot be made into an object
2019-11-27T18:47:01.6695553Z +   --> $DIR/bad-sized.rs:4:12
2019-11-27T18:47:01.6695898Z +    |
2019-11-27T18:47:01.6696075Z + LL |     let x: Vec<dyn Trait + Sized> = Vec::new();
2019-11-27T18:47:01.6696406Z +    |
2019-11-27T18:47:01.6696406Z +    |
2019-11-27T18:47:01.6696557Z +    = note: the trait cannot require that `Self : Sized`
2019-11-27T18:47:01.6696874Z + error[E0277]: the size for values of type `dyn std::marker::Sized` cannot be known at compilation time
2019-11-27T18:47:01.6697254Z 23   --> $DIR/bad-sized.rs:4:37
2019-11-27T18:47:01.6697455Z 24    |
2019-11-27T18:47:01.6697455Z 24    |
2019-11-27T18:47:01.6697605Z 25 LL |     let x: Vec<dyn Trait + Sized> = Vec::new();
2019-11-27T18:47:01.6698286Z 26    |                                     ^^^^^^^^ doesn't have a size known at compile-time
2019-11-27T18:47:01.6698571Z 27    |
2019-11-27T18:47:01.6699000Z -    = help: the trait `std::marker::Sized` is not implemented for `dyn Trait`
2019-11-27T18:47:01.6699203Z +    = help: the trait `std::marker::Sized` is not implemented for `dyn std::marker::Sized`
---
2019-11-27T18:47:01.6700611Z - error: aborting due to 3 previous errors
2019-11-27T18:47:01.6700801Z + error[E0038]: the trait `std::marker::Sized` cannot be made into an object
2019-11-27T18:47:01.6701142Z +   --> $DIR/bad-sized.rs:4:37
2019-11-27T18:47:01.6701325Z +    |
2019-11-27T18:47:01.6701461Z + LL |     let x: Vec<dyn Trait + Sized> = Vec::new();
2019-11-27T18:47:01.6701949Z +    |
2019-11-27T18:47:01.6701949Z +    |
2019-11-27T18:47:01.6702090Z +    = note: the trait cannot require that `Self : Sized`
2019-11-27T18:47:01.6702955Z - Some errors have detailed explanations: E0225, E0277.
2019-11-27T18:47:01.6703379Z - For more information about an error, try `rustc --explain E0225`.
2019-11-27T18:47:01.6703547Z + error: aborting due to 5 previous errors
2019-11-27T18:47:01.6703681Z + 
2019-11-27T18:47:01.6703681Z + 
2019-11-27T18:47:01.6703857Z + Some errors have detailed explanations: E0038, E0225, E0277.
2019-11-27T18:47:01.6704650Z + For more information about an error, try `rustc --explain E0038`.
2019-11-27T18:47:01.6704868Z 36 
2019-11-27T18:47:01.6704992Z 
2019-11-27T18:47:01.6705120Z 
2019-11-27T18:47:01.6705288Z The actual stderr differed from the expected stderr.
2019-11-27T18:47:01.6705716Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bad/bad-sized/bad-sized.stderr
2019-11-27T18:47:01.6706126Z To update references, rerun the tests and pass the `--bless` flag
2019-11-27T18:47:01.6706597Z To only update this specific test, also pass `--test-args bad/bad-sized.rs`
2019-11-27T18:47:01.6706966Z error: 1 errors occurred comparing output.
2019-11-27T18:47:01.6707141Z status: exit code: 1
2019-11-27T18:47:01.6707141Z status: exit code: 1
2019-11-27T18:47:01.6708357Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/bad/bad-sized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bad/bad-sized" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bad/bad-sized/auxiliary" "-A" "unused"
2019-11-27T18:47:01.6708932Z ------------------------------------------
2019-11-27T18:47:01.6709075Z 
2019-11-27T18:47:01.6709400Z ------------------------------------------
2019-11-27T18:47:01.6709575Z stderr:
2019-11-27T18:47:01.6709575Z stderr:
2019-11-27T18:47:01.6710090Z ------------------------------------------
2019-11-27T18:47:01.6710272Z error[E0225]: only auto traits can be used as additional traits in a trait object
2019-11-27T18:47:01.6710637Z   --> /checkout/src/test/ui/bad/bad-sized.rs:4:28
2019-11-27T18:47:01.6710829Z    |
2019-11-27T18:47:01.6710985Z LL |     let x: Vec<dyn Trait + Sized> = Vec::new();
2019-11-27T18:47:01.6711492Z    |                    -----   ^^^^^
2019-11-27T18:47:01.6712021Z    |                    |       additional non-auto trait
2019-11-27T18:47:01.6712213Z    |                    |       trait alias used in trait object type (additional use)
2019-11-27T18:47:01.6712566Z    |                    first non-auto trait
2019-11-27T18:47:01.6712762Z    |                    trait alias used in trait object type (first use)
2019-11-27T18:47:01.6712762Z    |                    trait alias used in trait object type (first use)
2019-11-27T18:47:01.6712891Z 
2019-11-27T18:47:01.6713057Z error[E0277]: the size for values of type `dyn std::marker::Sized` cannot be known at compilation time
2019-11-27T18:47:01.6713581Z   --> /checkout/src/test/ui/bad/bad-sized.rs:4:12
2019-11-27T18:47:01.6713737Z    |
2019-11-27T18:47:01.6713881Z LL |     let x: Vec<dyn Trait + Sized> = Vec::new();
2019-11-27T18:47:01.6714906Z    |
2019-11-27T18:47:01.6715079Z    = help: the trait `std::marker::Sized` is not implemented for `dyn std::marker::Sized`
2019-11-27T18:47:01.6715549Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-11-27T18:47:01.6715749Z    = note: required by `std::vec::Vec`
2019-11-27T18:47:01.6715749Z    = note: required by `std::vec::Vec`
2019-11-27T18:47:01.6715880Z 
2019-11-27T18:47:01.6716032Z error[E0038]: the trait `std::marker::Sized` cannot be made into an object
2019-11-27T18:47:01.6720237Z   --> /checkout/src/test/ui/bad/bad-sized.rs:4:12
2019-11-27T18:47:01.6720334Z    |
2019-11-27T18:47:01.6722739Z LL |     let x: Vec<dyn Trait + Sized> = Vec::new();
2019-11-27T18:47:01.6723995Z    |
2019-11-27T18:47:01.6723995Z    |
2019-11-27T18:47:01.6724047Z    = note: the trait cannot require that `Self : Sized`
2019-11-27T18:47:01.6724618Z error[E0277]: the size for values of type `dyn std::marker::Sized` cannot be known at compilation time
2019-11-27T18:47:01.6725198Z   --> /checkout/src/test/ui/bad/bad-sized.rs:4:37
2019-11-27T18:47:01.6725255Z    |
2019-11-27T18:47:01.6725255Z    |
2019-11-27T18:47:01.6725301Z LL |     let x: Vec<dyn Trait + Sized> = Vec::new();
2019-11-27T18:47:01.6725663Z    |
2019-11-27T18:47:01.6725713Z    = help: the trait `std::marker::Sized` is not implemented for `dyn std::marker::Sized`
2019-11-27T18:47:01.6726057Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-11-27T18:47:01.6726132Z    = note: required by `std::vec::Vec::<T>::new`
2019-11-27T18:47:01.6726132Z    = note: required by `std::vec::Vec::<T>::new`
2019-11-27T18:47:01.6726175Z 
2019-11-27T18:47:01.6726242Z error[E0038]: the trait `std::marker::Sized` cannot be made into an object
2019-11-27T18:47:01.6726477Z   --> /checkout/src/test/ui/bad/bad-sized.rs:4:37
2019-11-27T18:47:01.6726524Z    |
2019-11-27T18:47:01.6726585Z LL |     let x: Vec<dyn Trait + Sized> = Vec::new();
2019-11-27T18:47:01.6726689Z    |
2019-11-27T18:47:01.6726689Z    |
2019-11-27T18:47:01.6726751Z    = note: the trait cannot require that `Self : Sized`
2019-11-27T18:47:01.6726827Z error: aborting due to 5 previous errors
2019-11-27T18:47:01.6726856Z 
2019-11-27T18:47:01.6726920Z Some errors have detailed explanations: E0038, E0225, E0277.
2019-11-27T18:47:01.6727170Z For more information about an error, try `rustc --explain E0038`.
---
2019-11-27T18:47:01.6728704Z - error[E0277]: the trait bound `dyn Misc: std::marker::Copy` is not satisfied
2019-11-27T18:47:01.6728758Z + error[E0277]: the trait bound `dyn std::marker::Copy: std::marker::Copy` is not satisfied
2019-11-27T18:47:01.6728962Z 13   --> $DIR/issue-32963.rs:8:5
2019-11-27T18:47:01.6729021Z 14    |
2019-11-27T18:47:01.6729251Z 15 LL | fn size_of_copy<T: Copy+?Sized>() -> usize { mem::size_of::<T>() }
2019-11-27T18:47:01.6729523Z 16    |    ------------    ---- required by this bound in `size_of_copy`
2019-11-27T18:47:01.6729577Z 17 ...
2019-11-27T18:47:01.6729577Z 17 ...
2019-11-27T18:47:01.6729625Z 18 LL |     size_of_copy::<dyn Misc + Copy>();
2019-11-27T18:47:01.6729891Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::marker::Copy` is not implemented for `dyn Misc`
2019-11-27T18:47:01.6729969Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::marker::Copy` is not implemented for `dyn std::marker::Copy`
2019-11-27T18:47:01.6730216Z - error: aborting due to 2 previous errors
2019-11-27T18:47:01.6730287Z + error[E0038]: the trait `std::marker::Copy` cannot be made into an object
2019-11-27T18:47:01.6730489Z +   --> $DIR/issue-32963.rs:8:20
2019-11-27T18:47:01.6730531Z +    |
2019-11-27T18:47:01.6730531Z +    |
2019-11-27T18:47:01.6730590Z + LL |     size_of_copy::<dyn Misc + Copy>();
2019-11-27T18:47:01.6730641Z +    |                    ^^^^^^^^^^^^^^^ the trait `std::marker::Copy` cannot be made into an object
2019-11-27T18:47:01.6730684Z +    |
2019-11-27T18:47:01.6730753Z +    = note: the trait cannot require that `Self : Sized`
2019-11-27T18:47:01.6731087Z - Some errors have detailed explanations: E0225, E0277.
2019-11-27T18:47:01.6731363Z - For more information about an error, try `rustc --explain E0225`.
2019-11-27T18:47:01.6731413Z + error: aborting due to 3 previous errors
2019-11-27T18:47:01.6731452Z + 
2019-11-27T18:47:01.6731452Z + 
2019-11-27T18:47:01.6731496Z + Some errors have detailed explanations: E0038, E0225, E0277.
2019-11-27T18:47:01.6731925Z + For more information about an error, try `rustc --explain E0038`.
2019-11-27T18:47:01.6731968Z 25 
2019-11-27T18:47:01.6731993Z 
2019-11-27T18:47:01.6732018Z 
2019-11-27T18:47:01.6732079Z The actual stderr differed from the expected stderr.
2019-11-27T18:47:01.6732365Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32963/issue-32963.stderr
2019-11-27T18:47:01.6732603Z To update references, rerun the tests and pass the `--bless` flag
2019-11-27T18:47:01.6732873Z To only update this specific test, also pass `--test-args issues/issue-32963.rs`
2019-11-27T18:47:01.6732964Z error: 1 errors occurred comparing output.
2019-11-27T18:47:01.6733025Z status: exit code: 1
2019-11-27T18:47:01.6733025Z status: exit code: 1
2019-11-27T18:47:01.6733751Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-32963.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32963" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32963/auxiliary" "-A" "unused"
2019-11-27T18:47:01.6734063Z ------------------------------------------
2019-11-27T18:47:01.6734094Z 
2019-11-27T18:47:01.6734757Z ------------------------------------------
2019-11-27T18:47:01.6734921Z stderr:
2019-11-27T18:47:01.6734921Z stderr:
2019-11-27T18:47:01.6735159Z ------------------------------------------
2019-11-27T18:47:01.6735223Z error[E0225]: only auto traits can be used as additional traits in a trait object
2019-11-27T18:47:01.6735490Z   --> /checkout/src/test/ui/issues/issue-32963.rs:8:31
2019-11-27T18:47:01.6735539Z    |
2019-11-27T18:47:01.6735583Z LL |     size_of_copy::<dyn Misc + Copy>();
2019-11-27T18:47:01.6735815Z    |                        ----   ^^^^
2019-11-27T18:47:01.6736095Z    |                        |      additional non-auto trait
2019-11-27T18:47:01.6736169Z    |                        |      trait alias used in trait object type (additional use)
2019-11-27T18:47:01.6736395Z    |                        first non-auto trait
2019-11-27T18:47:01.6736450Z    |                        trait alias used in trait object type (first use)
2019-11-27T18:47:01.6736450Z    |                        trait alias used in trait object type (first use)
2019-11-27T18:47:01.6736498Z 
2019-11-27T18:47:01.6736548Z error[E0277]: the trait bound `dyn std::marker::Copy: std::marker::Copy` is not satisfied
2019-11-27T18:47:01.6736800Z   --> /checkout/src/test/ui/issues/issue-32963.rs:8:5
2019-11-27T18:47:01.6736863Z    |
2019-11-27T18:47:01.6737107Z LL | fn size_of_copy<T: Copy+?Sized>() -> usize { mem::size_of::<T>() }
2019-11-27T18:47:01.6737351Z    |    ------------    ---- required by this bound in `size_of_copy`
2019-11-27T18:47:01.6737396Z ...
2019-11-27T18:47:01.6737497Z LL |     size_of_copy::<dyn Misc + Copy>();
2019-11-27T18:47:01.6737554Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::marker::Copy` is not implemented for `dyn std::marker::Copy`
2019-11-27T18:47:01.6737824Z error[E0038]: the trait `std::marker::Copy` cannot be made into an object
2019-11-27T18:47:01.6738063Z   --> /checkout/src/test/ui/issues/issue-32963.rs:8:20
2019-11-27T18:47:01.6738109Z    |
2019-11-27T18:47:01.6738109Z    |
2019-11-27T18:47:01.6738169Z LL |     size_of_copy::<dyn Misc + Copy>();
2019-11-27T18:47:01.6738219Z    |                    ^^^^^^^^^^^^^^^ the trait `std::marker::Copy` cannot be made into an object
2019-11-27T18:47:01.6738273Z    |
2019-11-27T18:47:01.6738413Z    = note: the trait cannot require that `Self : Sized`
2019-11-27T18:47:01.6738492Z error: aborting due to 3 previous errors
2019-11-27T18:47:01.6738521Z 
2019-11-27T18:47:01.6738584Z Some errors have detailed explanations: E0038, E0225, E0277.
2019-11-27T18:47:01.6738844Z For more information about an error, try `rustc --explain E0038`.
---
2019-11-27T18:47:01.6740539Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-27T18:47:01.6745497Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-27T18:47:01.6751180Z 
2019-11-27T18:47:01.6751245Z 
2019-11-27T18:47:01.6752915Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-27T18:47:01.6753278Z 
2019-11-27T18:47:01.6753305Z 
2019-11-27T18:47:01.6761100Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-27T18:47:01.6761172Z Build completed unsuccessfully in 1:05:40
2019-11-27T18:47:01.6761172Z Build completed unsuccessfully in 1:05:40
2019-11-27T18:47:01.6812482Z == clock drift check ==
2019-11-27T18:47:01.6831889Z   local time: Wed Nov 27 18:47:01 UTC 2019
2019-11-27T18:47:02.2215268Z   network time: Wed, 27 Nov 2019 18:47:02 GMT
2019-11-27T18:47:02.2215438Z == end clock drift check ==
2019-11-27T18:47:03.0039935Z 
2019-11-27T18:47:03.0143957Z ##[error]Bash exited with code '1'.
2019-11-27T18:47:03.0181102Z ##[section]Starting: Checkout
2019-11-27T18:47:03.0183405Z ==============================================================================
2019-11-27T18:47:03.0183457Z Task         : Get sources
2019-11-27T18:47:03.0183505Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
