plain
2019-11-28T17:00:27.2073289Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-28T17:00:27.2089492Z ##[command]git config gc.auto 0
2019-11-28T17:00:27.2092096Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-28T17:00:27.2093751Z ##[command]git config --get-all http.proxy
2019-11-28T17:00:27.2097699Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66813/merge:refs/remotes/pull/66813/merge
---
2019-11-28T17:53:26.8216067Z .................................................................................................... 1600/9303
2019-11-28T17:53:31.1350013Z .................................................................................................... 1700/9303
2019-11-28T17:53:42.3631547Z ...................................i................................................................ 1800/9303
2019-11-28T17:53:49.3001941Z .................................................................................................... 1900/9303
2019-11-28T17:54:01.7294669Z ....................iiiii........................................................................... 2000/9303
2019-11-28T17:54:10.7169485Z .................................................................................................... 2200/9303
2019-11-28T17:54:12.9847787Z .................................................................................................... 2300/9303
2019-11-28T17:54:17.4745120Z .................................................................................................... 2400/9303
2019-11-28T17:54:36.7617882Z .................................................................................................... 2500/9303
---
2019-11-28T17:57:01.7951238Z .....................i...............i.............................................................. 4800/9303
2019-11-28T17:57:11.2228727Z .................................................................................................... 4900/9303
2019-11-28T17:57:16.4739428Z .................................................................................................... 5000/9303
2019-11-28T17:57:24.1615806Z .................................................................................................... 5100/9303
2019-11-28T17:57:30.8475277Z .........................ii.ii...........i.......................................................... 5200/9303
2019-11-28T17:57:39.0758698Z .................................................................................................... 5400/9303
2019-11-28T17:57:48.8862179Z .................................................................................................... 5500/9303
2019-11-28T17:57:55.1885475Z .......i............................................................................................ 5600/9303
2019-11-28T17:58:00.8798803Z .................................................................................................... 5700/9303
2019-11-28T17:58:00.8798803Z .................................................................................................... 5700/9303
2019-11-28T17:58:10.4496476Z .............................................................................................ii...i. 5800/9303
2019-11-28T17:58:22.1409179Z .ii...........i..................................................................................... 5900/9303
2019-11-28T17:58:38.3101529Z .................................................................................................... 6100/9303
2019-11-28T17:58:41.6439730Z .................................................................................................... 6200/9303
2019-11-28T17:58:41.6439730Z .................................................................................................... 6200/9303
2019-11-28T17:58:54.2771056Z ................i..ii............................................................................... 6300/9303
2019-11-28T17:59:11.7088163Z ....................................................................................i............... 6500/9303
2019-11-28T17:59:13.7478747Z .................................................................................................... 6600/9303
2019-11-28T17:59:15.7408740Z ...........................................................................i........................ 6700/9303
2019-11-28T17:59:18.2234268Z .................................................................................................... 6800/9303
---
2019-11-28T18:03:37.1810063Z 
2019-11-28T18:03:37.1810613Z ---- [ui] ui/associated-types/associated-types-overridden-binding-2.rs stdout ----
2019-11-28T18:03:37.1810741Z diff of stderr:
2019-11-28T18:03:37.1810772Z 
2019-11-28T18:03:37.1810823Z 4 LL |     let _: &dyn I32Iterator<Item = u32> = &vec![42].into_iter();
2019-11-28T18:03:37.1810878Z 5    |                                           ^^^^^^^^^^^^^^^^^^^^^ expected `i32`, found `u32`
2019-11-28T18:03:37.1810945Z 6    |
2019-11-28T18:03:37.1811244Z -    = note: required for the cast to the object type `dyn std::iter::Iterator<Item = u32, Item = i32>`
2019-11-28T18:03:37.1811328Z +    = note: required for the cast to the object type `dyn I32Iterator<Item = u32, Item = i32>`
2019-11-28T18:03:37.1811426Z 9 error: aborting due to previous error
2019-11-28T18:03:37.1811468Z 10 
2019-11-28T18:03:37.1811514Z 
2019-11-28T18:03:37.1811540Z 
2019-11-28T18:03:37.1811540Z 
2019-11-28T18:03:37.1811587Z The actual stderr differed from the expected stderr.
2019-11-28T18:03:37.1811964Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding-2/associated-types-overridden-binding-2.stderr
2019-11-28T18:03:37.1812374Z To update references, rerun the tests and pass the `--bless` flag
2019-11-28T18:03:37.1812593Z To only update this specific test, also pass `--test-args associated-types/associated-types-overridden-binding-2.rs`
2019-11-28T18:03:37.1812671Z error: 1 errors occurred comparing output.
2019-11-28T18:03:37.1812704Z status: exit code: 1
2019-11-28T18:03:37.1812704Z status: exit code: 1
2019-11-28T18:03:37.1813348Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-overridden-binding-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding-2/auxiliary" "-A" "unused"
2019-11-28T18:03:37.1813610Z ------------------------------------------
2019-11-28T18:03:37.1813634Z 
2019-11-28T18:03:37.1813791Z ------------------------------------------
2019-11-28T18:03:37.1813826Z stderr:
2019-11-28T18:03:37.1813826Z stderr:
2019-11-28T18:03:37.1813996Z ------------------------------------------
2019-11-28T18:03:37.1814036Z error[E0271]: type mismatch resolving `<std::vec::IntoIter<u32> as std::iter::Iterator>::Item == i32`
2019-11-28T18:03:37.1814236Z   --> /checkout/src/test/ui/associated-types/associated-types-overridden-binding-2.rs:6:43
2019-11-28T18:03:37.1814297Z    |
2019-11-28T18:03:37.1814332Z LL |     let _: &dyn I32Iterator<Item = u32> = &vec![42].into_iter();
2019-11-28T18:03:37.1814372Z    |                                           ^^^^^^^^^^^^^^^^^^^^^ expected `i32`, found `u32`
2019-11-28T18:03:37.1814575Z    |
2019-11-28T18:03:37.1814682Z    = note: required for the cast to the object type `dyn I32Iterator<Item = u32, Item = i32>`
2019-11-28T18:03:37.1814759Z error: aborting due to previous error
2019-11-28T18:03:37.1814780Z 
2019-11-28T18:03:37.1814995Z For more information about this error, try `rustc --explain E0271`.
2019-11-28T18:03:37.1815020Z 
---
2019-11-28T18:03:37.1815904Z - error[E0277]: the size for values of type `dyn Trait` cannot be known at compilation time
2019-11-28T18:03:37.1815962Z + error[E0277]: the size for values of type `dyn std::marker::Sized` cannot be known at compilation time
2019-11-28T18:03:37.1816139Z 13   --> $DIR/bad-sized.rs:4:12
2019-11-28T18:03:37.1816173Z 14    |
2019-11-28T18:03:37.1816206Z 15 LL |     let x: Vec<dyn Trait + Sized> = Vec::new();
2019-11-28T18:03:37.1816434Z 16    |            ^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-11-28T18:03:37.1816470Z 17    |
2019-11-28T18:03:37.1816651Z -    = help: the trait `std::marker::Sized` is not implemented for `dyn Trait`
2019-11-28T18:03:37.1816710Z +    = help: the trait `std::marker::Sized` is not implemented for `dyn std::marker::Sized`
---
2019-11-28T18:03:37.1817278Z - error[E0277]: the size for values of type `dyn Trait` cannot be known at compilation time
2019-11-28T18:03:37.1817338Z + error[E0038]: the trait `std::marker::Sized` cannot be made into an object
2019-11-28T18:03:37.1817493Z +   --> $DIR/bad-sized.rs:4:12
2019-11-28T18:03:37.1817526Z +    |
2019-11-28T18:03:37.1817575Z + LL |     let x: Vec<dyn Trait + Sized> = Vec::new();
2019-11-28T18:03:37.1817649Z +    |
2019-11-28T18:03:37.1817649Z +    |
2019-11-28T18:03:37.1817683Z +    = note: the trait cannot require that `Self : Sized`
2019-11-28T18:03:37.1817768Z + error[E0277]: the size for values of type `dyn std::marker::Sized` cannot be known at compilation time
2019-11-28T18:03:37.1817927Z 23   --> $DIR/bad-sized.rs:4:37
2019-11-28T18:03:37.1817976Z 24    |
2019-11-28T18:03:37.1817976Z 24    |
2019-11-28T18:03:37.1818009Z 25 LL |     let x: Vec<dyn Trait + Sized> = Vec::new();
2019-11-28T18:03:37.1818754Z 26    |                                     ^^^^^^^^ doesn't have a size known at compile-time
2019-11-28T18:03:37.1818812Z 27    |
2019-11-28T18:03:37.1819072Z -    = help: the trait `std::marker::Sized` is not implemented for `dyn Trait`
2019-11-28T18:03:37.1819149Z +    = help: the trait `std::marker::Sized` is not implemented for `dyn std::marker::Sized`
---
2019-11-28T18:03:37.1819834Z - error: aborting due to 3 previous errors
2019-11-28T18:03:37.1819887Z + error[E0038]: the trait `std::marker::Sized` cannot be made into an object
2019-11-28T18:03:37.1820110Z +   --> $DIR/bad-sized.rs:4:37
2019-11-28T18:03:37.1820154Z +    |
2019-11-28T18:03:37.1820199Z + LL |     let x: Vec<dyn Trait + Sized> = Vec::new();
2019-11-28T18:03:37.1820491Z +    |
2019-11-28T18:03:37.1820491Z +    |
2019-11-28T18:03:37.1820538Z +    = note: the trait cannot require that `Self : Sized`
2019-11-28T18:03:37.1820860Z - Some errors have detailed explanations: E0225, E0277.
2019-11-28T18:03:37.1821363Z - For more information about an error, try `rustc --explain E0225`.
2019-11-28T18:03:37.1821422Z + error: aborting due to 5 previous errors
2019-11-28T18:03:37.1821485Z + 
2019-11-28T18:03:37.1821485Z + 
2019-11-28T18:03:37.1821530Z + Some errors have detailed explanations: E0038, E0225, E0277.
2019-11-28T18:03:37.1821944Z + For more information about an error, try `rustc --explain E0038`.
2019-11-28T18:03:37.1822174Z 36 
2019-11-28T18:03:37.1822195Z 
2019-11-28T18:03:37.1822214Z 
2019-11-28T18:03:37.1822246Z The actual stderr differed from the expected stderr.
2019-11-28T18:03:37.1822481Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bad/bad-sized/bad-sized.stderr
2019-11-28T18:03:37.1822669Z To update references, rerun the tests and pass the `--bless` flag
2019-11-28T18:03:37.1822895Z To only update this specific test, also pass `--test-args bad/bad-sized.rs`
2019-11-28T18:03:37.1822969Z error: 1 errors occurred comparing output.
2019-11-28T18:03:37.1823001Z status: exit code: 1
2019-11-28T18:03:37.1823001Z status: exit code: 1
2019-11-28T18:03:37.1823547Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/bad/bad-sized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bad/bad-sized" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bad/bad-sized/auxiliary" "-A" "unused"
2019-11-28T18:03:37.1823799Z ------------------------------------------
2019-11-28T18:03:37.1823824Z 
2019-11-28T18:03:37.1823988Z ------------------------------------------
2019-11-28T18:03:37.1824020Z stderr:
2019-11-28T18:03:37.1824020Z stderr:
2019-11-28T18:03:37.1824190Z ------------------------------------------
2019-11-28T18:03:37.1824229Z error[E0225]: only auto traits can be used as additional traits in a trait object
2019-11-28T18:03:37.1824396Z   --> /checkout/src/test/ui/bad/bad-sized.rs:4:28
2019-11-28T18:03:37.1824448Z    |
2019-11-28T18:03:37.1824482Z LL |     let x: Vec<dyn Trait + Sized> = Vec::new();
2019-11-28T18:03:37.1824638Z    |                    -----   ^^^^^
2019-11-28T18:03:37.1824857Z    |                    |       additional non-auto trait
2019-11-28T18:03:37.1824897Z    |                    |       trait alias used in trait object type (additional use)
2019-11-28T18:03:37.1825078Z    |                    first non-auto trait
2019-11-28T18:03:37.1825128Z    |                    trait alias used in trait object type (first use)
2019-11-28T18:03:37.1825128Z    |                    trait alias used in trait object type (first use)
2019-11-28T18:03:37.1825152Z 
2019-11-28T18:03:37.1825188Z error[E0277]: the size for values of type `dyn std::marker::Sized` cannot be known at compilation time
2019-11-28T18:03:37.1825381Z   --> /checkout/src/test/ui/bad/bad-sized.rs:4:12
2019-11-28T18:03:37.1825415Z    |
2019-11-28T18:03:37.1825447Z LL |     let x: Vec<dyn Trait + Sized> = Vec::new();
2019-11-28T18:03:37.1825685Z    |
2019-11-28T18:03:37.1825721Z    = help: the trait `std::marker::Sized` is not implemented for `dyn std::marker::Sized`
2019-11-28T18:03:37.1825972Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-11-28T18:03:37.1826015Z    = note: required by `std::vec::Vec`
2019-11-28T18:03:37.1826015Z    = note: required by `std::vec::Vec`
2019-11-28T18:03:37.1826038Z 
2019-11-28T18:03:37.1826183Z error[E0038]: the trait `std::marker::Sized` cannot be made into an object
2019-11-28T18:03:37.1826436Z   --> /checkout/src/test/ui/bad/bad-sized.rs:4:12
2019-11-28T18:03:37.1826475Z    |
2019-11-28T18:03:37.1826508Z LL |     let x: Vec<dyn Trait + Sized> = Vec::new();
2019-11-28T18:03:37.1826599Z    |
2019-11-28T18:03:37.1826599Z    |
2019-11-28T18:03:37.1826632Z    = note: the trait cannot require that `Self : Sized`
2019-11-28T18:03:37.1826707Z error[E0277]: the size for values of type `dyn std::marker::Sized` cannot be known at compilation time
2019-11-28T18:03:37.1826897Z   --> /checkout/src/test/ui/bad/bad-sized.rs:4:37
2019-11-28T18:03:37.1826946Z    |
2019-11-28T18:03:37.1826946Z    |
2019-11-28T18:03:37.1826978Z LL |     let x: Vec<dyn Trait + Sized> = Vec::new();
2019-11-28T18:03:37.1827232Z    |
2019-11-28T18:03:37.1827274Z    = help: the trait `std::marker::Sized` is not implemented for `dyn std::marker::Sized`
2019-11-28T18:03:37.1827514Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-11-28T18:03:37.1827573Z    = note: required by `std::vec::Vec::<T>::new`
2019-11-28T18:03:37.1827573Z    = note: required by `std::vec::Vec::<T>::new`
2019-11-28T18:03:37.1827595Z 
2019-11-28T18:03:37.1827628Z error[E0038]: the trait `std::marker::Sized` cannot be made into an object
2019-11-28T18:03:37.1827796Z   --> /checkout/src/test/ui/bad/bad-sized.rs:4:37
2019-11-28T18:03:37.1827839Z    |
2019-11-28T18:03:37.1827871Z LL |     let x: Vec<dyn Trait + Sized> = Vec::new();
2019-11-28T18:03:37.1827964Z    |
2019-11-28T18:03:37.1827964Z    |
2019-11-28T18:03:37.1827997Z    = note: the trait cannot require that `Self : Sized`
2019-11-28T18:03:37.1828068Z error: aborting due to 5 previous errors
2019-11-28T18:03:37.1828095Z 
2019-11-28T18:03:37.1828133Z Some errors have detailed explanations: E0038, E0225, E0277.
2019-11-28T18:03:37.1828900Z For more information about an error, try `rustc --explain E0038`.
---
2019-11-28T18:03:37.1830151Z - error[E0277]: the trait bound `dyn Misc: std::marker::Copy` is not satisfied
2019-11-28T18:03:37.1830225Z + error[E0277]: the trait bound `dyn std::marker::Copy: std::marker::Copy` is not satisfied
2019-11-28T18:03:37.1830453Z 13   --> $DIR/issue-32963.rs:8:5
2019-11-28T18:03:37.1830506Z 14    |
2019-11-28T18:03:37.1830773Z 15 LL | fn size_of_copy<T: Copy+?Sized>() -> usize { mem::size_of::<T>() }
2019-11-28T18:03:37.1831049Z 16    |    ------------    ---- required by this bound in `size_of_copy`
2019-11-28T18:03:37.1831112Z 17 ...
2019-11-28T18:03:37.1831112Z 17 ...
2019-11-28T18:03:37.1831158Z 18 LL |     size_of_copy::<dyn Misc + Copy>();
2019-11-28T18:03:37.1831439Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::marker::Copy` is not implemented for `dyn Misc`
2019-11-28T18:03:37.1831519Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::marker::Copy` is not implemented for `dyn std::marker::Copy`
2019-11-28T18:03:37.1831783Z - error: aborting due to 2 previous errors
2019-11-28T18:03:37.1831852Z + error[E0038]: the trait `std::marker::Copy` cannot be made into an object
2019-11-28T18:03:37.1832068Z +   --> $DIR/issue-32963.rs:8:20
2019-11-28T18:03:37.1832113Z +    |
2019-11-28T18:03:37.1832113Z +    |
2019-11-28T18:03:37.1832479Z + LL |     size_of_copy::<dyn Misc + Copy>();
2019-11-28T18:03:37.1832541Z +    |                    ^^^^^^^^^^^^^^^ the trait `std::marker::Copy` cannot be made into an object
2019-11-28T18:03:37.1832738Z +    |
2019-11-28T18:03:37.1832772Z +    = note: the trait cannot require that `Self : Sized`
2019-11-28T18:03:37.1833019Z - Some errors have detailed explanations: E0225, E0277.
2019-11-28T18:03:37.1833196Z - For more information about an error, try `rustc --explain E0225`.
2019-11-28T18:03:37.1833248Z + error: aborting due to 3 previous errors
2019-11-28T18:03:37.1833279Z + 
2019-11-28T18:03:37.1833279Z + 
2019-11-28T18:03:37.1833313Z + Some errors have detailed explanations: E0038, E0225, E0277.
2019-11-28T18:03:37.1833506Z + For more information about an error, try `rustc --explain E0038`.
2019-11-28T18:03:37.1833540Z 25 
2019-11-28T18:03:37.1833560Z 
2019-11-28T18:03:37.1833578Z 
2019-11-28T18:03:37.1833610Z The actual stderr differed from the expected stderr.
2019-11-28T18:03:37.1833859Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32963/issue-32963.stderr
2019-11-28T18:03:37.1834042Z To update references, rerun the tests and pass the `--bless` flag
2019-11-28T18:03:37.1834244Z To only update this specific test, also pass `--test-args issues/issue-32963.rs`
2019-11-28T18:03:37.1834302Z error: 1 errors occurred comparing output.
2019-11-28T18:03:37.1834334Z status: exit code: 1
2019-11-28T18:03:37.1834334Z status: exit code: 1
2019-11-28T18:03:37.1834893Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-32963.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32963" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32963/auxiliary" "-A" "unused"
2019-11-28T18:03:37.1835148Z ------------------------------------------
2019-11-28T18:03:37.1835172Z 
2019-11-28T18:03:37.1835330Z ------------------------------------------
2019-11-28T18:03:37.1835377Z stderr:
2019-11-28T18:03:37.1835377Z stderr:
2019-11-28T18:03:37.1835531Z ------------------------------------------
2019-11-28T18:03:37.1835569Z error[E0225]: only auto traits can be used as additional traits in a trait object
2019-11-28T18:03:37.1835758Z   --> /checkout/src/test/ui/issues/issue-32963.rs:8:31
2019-11-28T18:03:37.1835794Z    |
2019-11-28T18:03:37.1835825Z LL |     size_of_copy::<dyn Misc + Copy>();
2019-11-28T18:03:37.1835985Z    |                        ----   ^^^^
2019-11-28T18:03:37.1836205Z    |                        |      additional non-auto trait
2019-11-28T18:03:37.1836246Z    |                        |      trait alias used in trait object type (additional use)
2019-11-28T18:03:37.1836433Z    |                        first non-auto trait
2019-11-28T18:03:37.1836478Z    |                        trait alias used in trait object type (first use)
2019-11-28T18:03:37.1836478Z    |                        trait alias used in trait object type (first use)
2019-11-28T18:03:37.1836502Z 
2019-11-28T18:03:37.1836553Z error[E0277]: the trait bound `dyn std::marker::Copy: std::marker::Copy` is not satisfied
2019-11-28T18:03:37.1836726Z   --> /checkout/src/test/ui/issues/issue-32963.rs:8:5
2019-11-28T18:03:37.1836760Z    |
2019-11-28T18:03:37.1836951Z LL | fn size_of_copy<T: Copy+?Sized>() -> usize { mem::size_of::<T>() }
2019-11-28T18:03:37.1837132Z    |    ------------    ---- required by this bound in `size_of_copy`
2019-11-28T18:03:37.1837166Z ...
2019-11-28T18:03:37.1837198Z LL |     size_of_copy::<dyn Misc + Copy>();
2019-11-28T18:03:37.1837252Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::marker::Copy` is not implemented for `dyn std::marker::Copy`
2019-11-28T18:03:37.1837316Z error[E0038]: the trait `std::marker::Copy` cannot be made into an object
2019-11-28T18:03:37.1837649Z   --> /checkout/src/test/ui/issues/issue-32963.rs:8:20
2019-11-28T18:03:37.1837690Z    |
2019-11-28T18:03:37.1837690Z    |
2019-11-28T18:03:37.1837722Z LL |     size_of_copy::<dyn Misc + Copy>();
2019-11-28T18:03:37.1837777Z    |                    ^^^^^^^^^^^^^^^ the trait `std::marker::Copy` cannot be made into an object
2019-11-28T18:03:37.1837811Z    |
2019-11-28T18:03:37.1837844Z    = note: the trait cannot require that `Self : Sized`
2019-11-28T18:03:37.1837915Z error: aborting due to 3 previous errors
2019-11-28T18:03:37.1837936Z 
2019-11-28T18:03:37.1837969Z Some errors have detailed explanations: E0038, E0225, E0277.
2019-11-28T18:03:37.1838561Z For more information about an error, try `rustc --explain E0038`.
---
2019-11-28T18:03:37.1844461Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-28T18:03:37.1844522Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-28T18:03:37.1847569Z 
2019-11-28T18:03:37.1847624Z 
2019-11-28T18:03:37.1849636Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-28T18:03:37.1849914Z 
2019-11-28T18:03:37.1849944Z 
2019-11-28T18:03:37.1854471Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-28T18:03:37.1854549Z Build completed unsuccessfully in 0:57:41
2019-11-28T18:03:37.1854549Z Build completed unsuccessfully in 0:57:41
2019-11-28T18:03:37.1902775Z == clock drift check ==
2019-11-28T18:03:37.1917366Z   local time: Thu Nov 28 18:03:37 UTC 2019
2019-11-28T18:03:37.2767647Z   network time: Thu, 28 Nov 2019 18:03:37 GMT
2019-11-28T18:03:37.2771290Z == end clock drift check ==
2019-11-28T18:03:38.0501221Z 
2019-11-28T18:03:38.0558128Z ##[error]Bash exited with code '1'.
2019-11-28T18:03:38.0597998Z ##[section]Starting: Checkout
2019-11-28T18:03:38.0600060Z ==============================================================================
2019-11-28T18:03:38.0600114Z Task         : Get sources
2019-11-28T18:03:38.0600163Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
