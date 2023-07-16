plain
2020-01-01T01:14:33.8513576Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-01T01:14:33.8668331Z ##[command]git config gc.auto 0
2020-01-01T01:14:33.8719670Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-01T01:14:33.8768245Z ##[command]git config --get-all http.proxy
2020-01-01T01:14:33.8928663Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67767/merge:refs/remotes/pull/67767/merge
---
2020-01-01T02:02:27.5659992Z .................................................................................................... 1500/9467
2020-01-01T02:02:32.3541893Z .................................................................................................... 1600/9467
2020-01-01T02:02:36.3217416Z .................................................................................................... 1700/9467
2020-01-01T02:02:43.7151677Z .................................................................................................... 1800/9467
2020-01-01T02:02:50.2484008Z i................................................................................................... 1900/9467
2020-01-01T02:02:55.5640032Z ......................................................................................iiiii......... 2000/9467
2020-01-01T02:03:12.7708953Z .................................................................................................... 2200/9467
2020-01-01T02:03:14.6949452Z .................................................................................................... 2300/9467
2020-01-01T02:03:16.7003892Z .................................................................................................... 2400/9467
2020-01-01T02:03:21.7829791Z .................................................................................................... 2500/9467
---
2020-01-01T02:05:46.7360657Z .................i...............i.................................................................. 4900/9467
2020-01-01T02:05:54.8231516Z .................................................................................................... 5000/9467
2020-01-01T02:05:59.6589962Z ...........................................................F..i..................................... 5100/9467
2020-01-01T02:06:06.7689918Z .................................................................................................... 5200/9467
2020-01-01T02:06:13.2640818Z .............................ii.ii...........i...................................................... 5300/9467
2020-01-01T02:06:21.3235797Z .................................................................................................... 5500/9467
2020-01-01T02:06:30.0140293Z .................................................................................................... 5600/9467
2020-01-01T02:06:36.2687095Z ............i....................................................................................... 5700/9467
2020-01-01T02:06:41.6062952Z .................................................................................................... 5800/9467
2020-01-01T02:06:41.6062952Z .................................................................................................... 5800/9467
2020-01-01T02:06:50.6009913Z .................................................................................................... 5900/9467
2020-01-01T02:07:00.7615590Z ii...i..ii...........i.............................................................................. 6000/9467
2020-01-01T02:07:15.8340802Z .................................................................................................... 6200/9467
2020-01-01T02:07:21.8201631Z .................................................................................................... 6300/9467
2020-01-01T02:07:21.8201631Z .................................................................................................... 6300/9467
2020-01-01T02:07:34.9816316Z ...........................i..ii.................................................................... 6400/9467
2020-01-01T02:07:51.0031224Z .................................................................................................... 6600/9467
2020-01-01T02:07:52.8173674Z ..i................................................................................................. 6700/9467
2020-01-01T02:07:54.8528537Z .................................................................................................... 6800/9467
2020-01-01T02:07:57.0917573Z ..i................................................................................................. 6900/9467
---
2020-01-01T02:09:17.0711047Z .................................................................................................... 7500/9467
2020-01-01T02:09:21.2691663Z .................................................................................................... 7600/9467
2020-01-01T02:09:26.3844887Z ...........................................................................................F........ 7700/9467
2020-01-01T02:09:35.2255341Z .................................................................................................... 7800/9467
2020-01-01T02:09:41.8599899Z ....................................iiii............................................................ 7900/9467
2020-01-01T02:09:54.9923564Z .................................................................................................... 8100/9467
2020-01-01T02:10:02.2795597Z .................................................................................................... 8200/9467
2020-01-01T02:10:14.1930861Z .................................................................................................... 8300/9467
2020-01-01T02:10:21.0069380Z .................................................................................................... 8400/9467
---
2020-01-01T02:11:54.6568394Z ---- [ui] ui/const-generics/array-impls/core-traits-no-impls-length-33.rs stdout ----
2020-01-01T02:11:54.6568910Z diff of stderr:
2020-01-01T02:11:54.6569052Z 
2020-01-01T02:11:54.6569167Z 3    |
2020-01-01T02:11:54.6569303Z 4 LL |     println!("{:?}", [0_usize; 33]);
2020-01-01T02:11:54.6569571Z 5    |                      ^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2020-01-01T02:11:54.6569700Z +    | 
2020-01-01T02:11:54.6569840Z +   ::: $SRC_DIR/libcore/fmt/mod.rs:LL:COL
2020-01-01T02:11:54.6569949Z 6    |
2020-01-01T02:11:54.6570329Z + LL |     pub fn new_debug<T: Debug>(x: &T) -> ArgumentV1<'_> {
2020-01-01T02:11:54.6570846Z +    |                         ----- required by this bound in `std::fmt::ArgumentV1::<'a>::new_debug`
2020-01-01T02:11:54.6571168Z 7    = note: required because of the requirements on the impl of `std::fmt::Debug` for `[usize; 33]`
2020-01-01T02:11:54.6571631Z -    = note: required by `std::fmt::Debug::fmt`
2020-01-01T02:11:54.6571765Z 9 
2020-01-01T02:11:54.6571898Z 10 error[E0277]: arrays only have std trait implementations for lengths 0..=32
2020-01-01T02:11:54.6571898Z 10 error[E0277]: arrays only have std trait implementations for lengths 0..=32
2020-01-01T02:11:54.6572169Z 11   --> $DIR/core-traits-no-impls-length-33.rs:10:16
2020-01-01T02:11:54.6572288Z 
2020-01-01T02:11:54.6572399Z 
2020-01-01T02:11:54.6572507Z The actual stderr differed from the expected stderr.
2020-01-01T02:11:54.6572865Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33/core-traits-no-impls-length-33.stderr
2020-01-01T02:11:54.6573191Z To update references, rerun the tests and pass the `--bless` flag
2020-01-01T02:11:54.6573688Z To only update this specific test, also pass `--test-args const-generics/array-impls/core-traits-no-impls-length-33.rs`
2020-01-01T02:11:54.6573945Z error: 1 errors occurred comparing output.
2020-01-01T02:11:54.6574050Z status: exit code: 1
2020-01-01T02:11:54.6574050Z status: exit code: 1
2020-01-01T02:11:54.6574870Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33/auxiliary" "-A" "unused"
2020-01-01T02:11:54.6575327Z ------------------------------------------
2020-01-01T02:11:54.6575446Z 
2020-01-01T02:11:54.6575728Z ------------------------------------------
2020-01-01T02:11:54.6575859Z stderr:
2020-01-01T02:11:54.6575859Z stderr:
2020-01-01T02:11:54.6576108Z ------------------------------------------
2020-01-01T02:11:54.6576263Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2020-01-01T02:11:54.6576562Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:2:22
2020-01-01T02:11:54.6576715Z    |
2020-01-01T02:11:54.6576839Z LL |     println!("{:?}", [0_usize; 33]);
2020-01-01T02:11:54.6576953Z    |                      ^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2020-01-01T02:11:54.6577192Z   ::: /checkout/src/libcore/fmt/mod.rs:295:25
2020-01-01T02:11:54.6577310Z    |
2020-01-01T02:11:54.6577310Z    |
2020-01-01T02:11:54.6577600Z LL |     pub fn new_debug<T: Debug>(x: &T) -> ArgumentV1<'_> {
2020-01-01T02:11:54.6577926Z    |                         ----- required by this bound in `std::fmt::ArgumentV1::<'a>::new_debug`
2020-01-01T02:11:54.6578193Z    = note: required because of the requirements on the impl of `std::fmt::Debug` for `[usize; 33]`
2020-01-01T02:11:54.6578289Z 
2020-01-01T02:11:54.6578413Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2020-01-01T02:11:54.6578713Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:10:16
2020-01-01T02:11:54.6578713Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:10:16
2020-01-01T02:11:54.6578845Z    |
2020-01-01T02:11:54.6578975Z LL |     set.insert([0_usize; 33]);
2020-01-01T02:11:54.6579091Z    |                ^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2020-01-01T02:11:54.6579198Z    |
2020-01-01T02:11:54.6579326Z    = note: required because of the requirements on the impl of `std::cmp::Eq` for `[usize; 33]`
2020-01-01T02:11:54.6579616Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2020-01-01T02:11:54.6581975Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:8:19
2020-01-01T02:11:54.6582256Z    |
2020-01-01T02:11:54.6582393Z LL |     let mut set = HashSet::new();
2020-01-01T02:11:54.6582393Z LL |     let mut set = HashSet::new();
2020-01-01T02:11:54.6582511Z    |                   ^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2020-01-01T02:11:54.6582618Z    |
2020-01-01T02:11:54.6582748Z    = note: required because of the requirements on the impl of `std::cmp::Eq` for `[usize; 33]`
2020-01-01T02:11:54.6582862Z    = note: required by `std::collections::HashSet::<T>::new`
2020-01-01T02:11:54.6583082Z error[E0369]: binary operation `==` cannot be applied to type `[usize; 33]`
2020-01-01T02:11:54.6583403Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:15:19
2020-01-01T02:11:54.6583678Z    |
2020-01-01T02:11:54.6583678Z    |
2020-01-01T02:11:54.6583816Z LL |     [0_usize; 33] == [1_usize; 33]
2020-01-01T02:11:54.6584107Z    |     ------------- ^^ ------------- [usize; 33]
2020-01-01T02:11:54.6584365Z    |     [usize; 33]
2020-01-01T02:11:54.6584468Z    |
2020-01-01T02:11:54.6584652Z    = note: an implementation of `std::cmp::PartialEq` might be missing for `[usize; 33]`
2020-01-01T02:11:54.6584773Z 
2020-01-01T02:11:54.6584773Z 
2020-01-01T02:11:54.6584880Z error[E0369]: binary operation `<` cannot be applied to type `[usize; 33]`
2020-01-01T02:11:54.6585189Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:20:19
2020-01-01T02:11:54.6585342Z    |
2020-01-01T02:11:54.6585465Z LL |     [0_usize; 33] < [1_usize; 33]
2020-01-01T02:11:54.6585745Z    |     ------------- ^ ------------- [usize; 33]
2020-01-01T02:11:54.6585980Z    |     [usize; 33]
2020-01-01T02:11:54.6586101Z    |
2020-01-01T02:11:54.6586101Z    |
2020-01-01T02:11:54.6586221Z    = note: an implementation of `std::cmp::PartialOrd` might be missing for `[usize; 33]`
2020-01-01T02:11:54.6586324Z 
2020-01-01T02:11:54.6586452Z error[E0277]: the trait bound `&[usize; 33]: std::iter::IntoIterator` is not satisfied
2020-01-01T02:11:54.6586760Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:25:14
2020-01-01T02:11:54.6586892Z    |
2020-01-01T02:11:54.6587012Z LL |     for _ in &[0_usize; 33] {
2020-01-01T02:11:54.6587125Z    |              ^^^^^^^^^^^^^^ the trait `std::iter::IntoIterator` is not implemented for `&[usize; 33]`
2020-01-01T02:11:54.6587540Z    = help: the following implementations were found:
2020-01-01T02:11:54.6587540Z    = help: the following implementations were found:
2020-01-01T02:11:54.6587815Z              <&'a [T; _] as std::iter::IntoIterator>
2020-01-01T02:11:54.6588107Z              <&'a [T] as std::iter::IntoIterator>
2020-01-01T02:11:54.6588423Z              <&'a mut [T; _] as std::iter::IntoIterator>
2020-01-01T02:11:54.6588719Z              <&'a mut [T] as std::iter::IntoIterator>
2020-01-01T02:11:54.6589605Z 
2020-01-01T02:11:54.6589926Z error: aborting due to 6 previous errors
2020-01-01T02:11:54.6590038Z 
2020-01-01T02:11:54.6590173Z Some errors have detailed explanations: E0277, E0369.
---
2020-01-01T02:11:54.6591472Z ---- [ui] ui/if/ifmt-unimpl.rs stdout ----
2020-01-01T02:11:54.6591613Z diff of stderr:
2020-01-01T02:11:54.6591710Z 
2020-01-01T02:11:54.6591818Z 3    |
2020-01-01T02:11:54.6591947Z 4 LL |     format!("{:X}", "3");
2020-01-01T02:11:54.6592064Z 5    |                     ^^^ the trait `std::fmt::UpperHex` is not implemented for `str`
2020-01-01T02:11:54.6592191Z +    | 
2020-01-01T02:11:54.6592303Z +   ::: $SRC_DIR/libcore/fmt/mod.rs:LL:COL
2020-01-01T02:11:54.6592529Z 6    |
2020-01-01T02:11:54.6592898Z + LL |     pub fn new_upperhex<T: UpperHex>(x: &T) -> ArgumentV1<'_> {
2020-01-01T02:11:54.6593252Z +    |                            -------- required by this bound in `std::fmt::ArgumentV1::<'a>::new_upperhex`
2020-01-01T02:11:54.6594782Z 7    = note: required because of the requirements on the impl of `std::fmt::UpperHex` for `&str`
2020-01-01T02:11:54.6595222Z -    = note: required by `std::fmt::UpperHex::fmt`
2020-01-01T02:11:54.6595402Z 9 
2020-01-01T02:11:54.6595568Z 10 error: aborting due to previous error
2020-01-01T02:11:54.6595568Z 10 error: aborting due to previous error
2020-01-01T02:11:54.6595711Z 11 
2020-01-01T02:11:54.6595834Z 
2020-01-01T02:11:54.6595972Z 
2020-01-01T02:11:54.6596119Z The actual stderr differed from the expected stderr.
2020-01-01T02:11:54.6596917Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/ifmt-unimpl/ifmt-unimpl.stderr
2020-01-01T02:11:54.6598319Z To update references, rerun the tests and pass the `--bless` flag
2020-01-01T02:11:54.6599378Z To only update this specific test, also pass `--test-args if/ifmt-unimpl.rs`
2020-01-01T02:11:54.6599632Z error: 1 errors occurred comparing output.
2020-01-01T02:11:54.6599764Z status: exit code: 1
2020-01-01T02:11:54.6599764Z status: exit code: 1
2020-01-01T02:11:54.6600506Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/if/ifmt-unimpl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/ifmt-unimpl" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/ifmt-unimpl/auxiliary" "-A" "unused"
2020-01-01T02:11:54.6600981Z ------------------------------------------
2020-01-01T02:11:54.6601138Z 
2020-01-01T02:11:54.6601417Z ------------------------------------------
2020-01-01T02:11:54.6601553Z stderr:
2020-01-01T02:11:54.6601553Z stderr:
2020-01-01T02:11:54.6601832Z ------------------------------------------
2020-01-01T02:11:54.6601974Z error[E0277]: the trait bound `str: std::fmt::UpperHex` is not satisfied
2020-01-01T02:11:54.6602430Z   --> /checkout/src/test/ui/if/ifmt-unimpl.rs:2:21
2020-01-01T02:11:54.6602593Z    |
2020-01-01T02:11:54.6602931Z LL |     format!("{:X}", "3");
2020-01-01T02:11:54.6603058Z    |                     ^^^ the trait `std::fmt::UpperHex` is not implemented for `str`
2020-01-01T02:11:54.6603313Z   ::: /checkout/src/libcore/fmt/mod.rs:372:28
2020-01-01T02:11:54.6603700Z    |
2020-01-01T02:11:54.6603700Z    |
2020-01-01T02:11:54.6604021Z LL |     pub fn new_upperhex<T: UpperHex>(x: &T) -> ArgumentV1<'_> {
2020-01-01T02:11:54.6604391Z    |                            -------- required by this bound in `std::fmt::ArgumentV1::<'a>::new_upperhex`
2020-01-01T02:11:54.6604712Z    = note: required because of the requirements on the impl of `std::fmt::UpperHex` for `&str`
2020-01-01T02:11:54.6604823Z 
2020-01-01T02:11:54.6604964Z error: aborting due to previous error
2020-01-01T02:11:54.6605070Z 
---
2020-01-01T02:11:54.6606953Z ---- [ui] ui/issues/issue-59488.rs stdout ----
2020-01-01T02:11:54.6607087Z diff of stderr:
2020-01-01T02:11:54.6607179Z 
2020-01-01T02:11:54.6607299Z 87    |
2020-01-01T02:11:54.6607405Z 88 LL |     assert_eq!(Foo::Bar, i);
2020-01-01T02:11:54.6607740Z 89    |     ^^^^^^^^^^^^^^^^^^^^^^^^ `fn(usize) -> Foo {Foo::Bar}` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2020-01-01T02:11:54.6607897Z +    | 
2020-01-01T02:11:54.6608107Z +   ::: $SRC_DIR/libcore/fmt/mod.rs:LL:COL
2020-01-01T02:11:54.6608302Z 90    |
2020-01-01T02:11:54.6608662Z + LL |     pub fn new_debug<T: Debug>(x: &T) -> ArgumentV1<'_> {
2020-01-01T02:11:54.6608986Z +    |                         ----- required by this bound in `std::fmt::ArgumentV1::<'a>::new_debug`
2020-01-01T02:11:54.6609140Z +    |
2020-01-01T02:11:54.6609433Z 91    = help: the trait `std::fmt::Debug` is not implemented for `fn(usize) -> Foo {Foo::Bar}`
2020-01-01T02:11:54.6609764Z 92    = note: required because of the requirements on the impl of `std::fmt::Debug` for `&fn(usize) -> Foo {Foo::Bar}`
2020-01-01T02:11:54.6610419Z 94    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2020-01-01T02:11:54.6610571Z 95 
2020-01-01T02:11:54.6610571Z 95 
2020-01-01T02:11:54.6610855Z 96 error[E0277]: `fn(usize) -> Foo {Foo::Bar}` doesn't implement `std::fmt::Debug`
2020-01-01T02:11:54.6611189Z 98    |
2020-01-01T02:11:54.6611189Z 98    |
2020-01-01T02:11:54.6611318Z 99 LL |     assert_eq!(Foo::Bar, i);
2020-01-01T02:11:54.6611673Z 100    |     ^^^^^^^^^^^^^^^^^^^^^^^^ `fn(usize) -> Foo {Foo::Bar}` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2020-01-01T02:11:54.6611830Z +    | 
2020-01-01T02:11:54.6611937Z +   ::: $SRC_DIR/libcore/fmt/mod.rs:LL:COL
2020-01-01T02:11:54.6612042Z 101    |
2020-01-01T02:11:54.6612331Z + LL |     pub fn new_debug<T: Debug>(x: &T) -> ArgumentV1<'_> {
2020-01-01T02:11:54.6612655Z +    |                         ----- required by this bound in `std::fmt::ArgumentV1::<'a>::new_debug`
2020-01-01T02:11:54.6612788Z +    |
2020-01-01T02:11:54.6613101Z 102    = help: the trait `std::fmt::Debug` is not implemented for `fn(usize) -> Foo {Foo::Bar}`
2020-01-01T02:11:54.6613436Z 103    = note: required because of the requirements on the impl of `std::fmt::Debug` for `&fn(usize) -> Foo {Foo::Bar}`
2020-01-01T02:11:54.6614120Z 105    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2020-01-01T02:11:54.6614440Z 106 
2020-01-01T02:11:54.6614573Z 107 error: aborting due to 10 previous errors
2020-01-01T02:11:54.6614669Z 
2020-01-01T02:11:54.6614669Z 
2020-01-01T02:11:54.6614765Z 
2020-01-01T02:11:54.6614892Z The actual stderr differed from the expected stderr.
2020-01-01T02:11:54.6615228Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488/issue-59488.stderr
2020-01-01T02:11:54.6615539Z To update references, rerun the tests and pass the `--bless` flag
2020-01-01T02:11:54.6616083Z To only update this specific test, also pass `--test-args issues/issue-59488.rs`
2020-01-01T02:11:54.6616328Z error: 1 errors occurred comparing output.
2020-01-01T02:11:54.6616463Z status: exit code: 1
2020-01-01T02:11:54.6616463Z status: exit code: 1
2020-01-01T02:11:54.6617262Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-59488.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488/auxiliary" "-A" "unused"
2020-01-01T02:11:54.6617754Z ------------------------------------------
2020-01-01T02:11:54.6617899Z 
2020-01-01T02:11:54.6618180Z ------------------------------------------
2020-01-01T02:11:54.6618317Z stderr:
2020-01-01T02:11:54.6618317Z stderr:
2020-01-01T02:11:54.6618606Z ------------------------------------------
2020-01-01T02:11:54.6618938Z error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {foo}`
2020-01-01T02:11:54.6619509Z   --> /checkout/src/test/ui/issues/issue-59488.rs:14:9
2020-01-01T02:11:54.6619808Z LL |     foo > 12;
2020-01-01T02:11:54.6619808Z LL |     foo > 12;
2020-01-01T02:11:54.6620084Z    |     --- ^ -- {integer}
2020-01-01T02:11:54.6620497Z    |     fn() -> i32 {foo}
2020-01-01T02:11:54.6620655Z    |     help: you might have forgotten to call this function: `foo()`
2020-01-01T02:11:54.6620758Z 
2020-01-01T02:11:54.6620868Z error[E0308]: mismatched types
---
2020-01-01T02:11:54.6621664Z    |
2020-01-01T02:11:54.6621935Z    = note: expected fn item `fn() -> i32 {foo}`
2020-01-01T02:11:54.6622251Z                  found type `i32`
2020-01-01T02:11:54.6622346Z 
2020-01-01T02:11:54.6622643Z error[E0369]: binary operation `>` cannot be applied to type `fn(i64) -> i64 {bar}`
2020-01-01T02:11:54.6623060Z   --> /checkout/src/test/ui/issues/issue-59488.rs:18:9
2020-01-01T02:11:54.6623300Z LL |     bar > 13;
2020-01-01T02:11:54.6623300Z LL |     bar > 13;
2020-01-01T02:11:54.6623562Z    |     --- ^ -- {integer}
2020-01-01T02:11:54.6623692Z    |     |
2020-01-01T02:11:54.6623939Z    |     fn(i64) -> i64 {bar}
2020-01-01T02:11:54.6624096Z    |     help: you might have forgotten to call this function: `bar( /* arguments */ )`
2020-01-01T02:11:54.6624360Z error[E0308]: mismatched types
2020-01-01T02:11:54.6624659Z   --> /checkout/src/test/ui/issues/issue-59488.rs:18:11
2020-01-01T02:11:54.6624791Z    |
2020-01-01T02:11:54.6624895Z LL |     bar > 13;
2020-01-01T02:11:54.6624895Z LL |     bar > 13;
2020-01-01T02:11:54.6625017Z    |           ^^ expected fn item, found integer
2020-01-01T02:11:54.6625122Z    |
2020-01-01T02:11:54.6625386Z    = note: expected fn item `fn(i64) -> i64 {bar}`
2020-01-01T02:11:54.6625647Z 
2020-01-01T02:11:54.6625647Z 
2020-01-01T02:11:54.6625943Z error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {foo}`
2020-01-01T02:11:54.6626276Z   --> /checkout/src/test/ui/issues/issue-59488.rs:22:9
2020-01-01T02:11:54.6626539Z LL |     foo > foo;
2020-01-01T02:11:54.6626539Z LL |     foo > foo;
2020-01-01T02:11:54.6626798Z    |     --- ^ --- fn() -> i32 {foo}
2020-01-01T02:11:54.6627274Z    |     fn() -> i32 {foo}
2020-01-01T02:11:54.6627410Z    |
2020-01-01T02:11:54.6627540Z help: you might have forgotten to call this function
2020-01-01T02:11:54.6627648Z    |
2020-01-01T02:11:54.6627648Z    |
2020-01-01T02:11:54.6627757Z LL |     foo() > foo;
2020-01-01T02:11:54.6627916Z    |     ^^^^^
2020-01-01T02:11:54.6628048Z help: you might have forgotten to call this function
2020-01-01T02:11:54.6628174Z    |
2020-01-01T02:11:54.6628278Z LL |     foo > foo();
2020-01-01T02:11:54.6628383Z    |           ^^^^^
2020-01-01T02:11:54.6628475Z 
2020-01-01T02:11:54.6628771Z error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {foo}`
2020-01-01T02:11:54.6629108Z   --> /checkout/src/test/ui/issues/issue-59488.rs:25:9
2020-01-01T02:11:54.6629266Z    |
2020-01-01T02:11:54.6629373Z LL |     foo > bar;
2020-01-01T02:11:54.6629632Z    |     --- ^ --- fn(i64) -> i64 {bar}
2020-01-01T02:11:54.6630008Z    |     fn() -> i32 {foo}
2020-01-01T02:11:54.6630160Z    |
2020-01-01T02:11:54.6630663Z    = note: an implementation of `std::cmp::PartialOrd` might be missing for `fn() -> i32 {foo}`
2020-01-01T02:11:54.6630793Z 
2020-01-01T02:11:54.6630793Z 
2020-01-01T02:11:54.6630908Z error[E0308]: mismatched types
2020-01-01T02:11:54.6631187Z   --> /checkout/src/test/ui/issues/issue-59488.rs:25:11
2020-01-01T02:11:54.6631345Z    |
2020-01-01T02:11:54.6631477Z LL |     foo > bar;
2020-01-01T02:11:54.6631594Z    |           ^^^ expected fn item, found a different fn item
2020-01-01T02:11:54.6631985Z    = note: expected fn item `fn() -> i32 {foo}`
2020-01-01T02:11:54.6631985Z    = note: expected fn item `fn() -> i32 {foo}`
2020-01-01T02:11:54.6632285Z               found fn item `fn(i64) -> i64 {bar}`
2020-01-01T02:11:54.6632495Z 
2020-01-01T02:11:54.6632902Z error[E0369]: binary operation `==` cannot be applied to type `fn(usize) -> Foo {Foo::Bar}`
2020-01-01T02:11:54.6633218Z   --> /checkout/src/test/ui/issues/issue-59488.rs:30:5
2020-01-01T02:11:54.6633357Z    |
2020-01-01T02:11:54.6633472Z LL |     assert_eq!(Foo::Bar, i);
2020-01-01T02:11:54.6633720Z    |     |
2020-01-01T02:11:54.6633720Z    |     |
2020-01-01T02:11:54.6634006Z    |     fn(usize) -> Foo {Foo::Bar}
2020-01-01T02:11:54.6634293Z    |     fn(usize) -> Foo {Foo::Bar}
2020-01-01T02:11:54.6634428Z    |
2020-01-01T02:11:54.6634743Z    = note: an implementation of `std::cmp::PartialEq` might be missing for `fn(usize) -> Foo {Foo::Bar}`
2020-01-01T02:11:54.6635285Z 
2020-01-01T02:11:54.6635285Z 
2020-01-01T02:11:54.6635800Z error[E0277]: `fn(usize) -> Foo {Foo::Bar}` doesn't implement `std::fmt::Debug`
2020-01-01T02:11:54.6638198Z   --> /checkout/src/test/ui/issues/issue-59488.rs:30:5
2020-01-01T02:11:54.6638356Z    |
2020-01-01T02:11:54.6638405Z LL |     assert_eq!(Foo::Bar, i);
2020-01-01T02:11:54.6638703Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^ `fn(usize) -> Foo {Foo::Bar}` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2020-01-01T02:11:54.6638799Z   ::: /checkout/src/libcore/fmt/mod.rs:295:25
2020-01-01T02:11:54.6638831Z    |
2020-01-01T02:11:54.6638831Z    |
2020-01-01T02:11:54.6639393Z LL |     pub fn new_debug<T: Debug>(x: &T) -> ArgumentV1<'_> {
2020-01-01T02:11:54.6639663Z    |                         ----- required by this bound in `std::fmt::ArgumentV1::<'a>::new_debug`
2020-01-01T02:11:54.6639701Z    |
2020-01-01T02:11:54.6639890Z    = help: the trait `std::fmt::Debug` is not implemented for `fn(usize) -> Foo {Foo::Bar}`
2020-01-01T02:11:54.6640412Z    = note: required because of the requirements on the impl of `std::fmt::Debug` for `&fn(usize) -> Foo {Foo::Bar}`
2020-01-01T02:11:54.6640698Z 
2020-01-01T02:11:54.6640698Z 
2020-01-01T02:11:54.6640899Z error[E0277]: `fn(usize) -> Foo {Foo::Bar}` doesn't implement `std::fmt::Debug`
2020-01-01T02:11:54.6641072Z   --> /checkout/src/test/ui/issues/issue-59488.rs:30:5
2020-01-01T02:11:54.6641106Z    |
2020-01-01T02:11:54.6641153Z LL |     assert_eq!(Foo::Bar, i);
2020-01-01T02:11:54.6641381Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^ `fn(usize) -> Foo {Foo::Bar}` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2020-01-01T02:11:54.6641467Z   ::: /checkout/src/libcore/fmt/mod.rs:295:25
2020-01-01T02:11:54.6641498Z    |
2020-01-01T02:11:54.6641498Z    |
2020-01-01T02:11:54.6641666Z LL |     pub fn new_debug<T: Debug>(x: &T) -> ArgumentV1<'_> {
2020-01-01T02:11:54.6641869Z    |                         ----- required by this bound in `std::fmt::ArgumentV1::<'a>::new_debug`
2020-01-01T02:11:54.6641927Z    |
2020-01-01T02:11:54.6642116Z    = help: the trait `std::fmt::Debug` is not implemented for `fn(usize) -> Foo {Foo::Bar}`
2020-01-01T02:11:54.6642325Z    = note: required because of the requirements on the impl of `std::fmt::Debug` for `&fn(usize) -> Foo {Foo::Bar}`
2020-01-01T02:11:54.6642607Z 
2020-01-01T02:11:54.6642642Z error: aborting due to 10 previous errors
2020-01-01T02:11:54.6642679Z 
2020-01-01T02:11:54.6642716Z Some errors have detailed explanations: E0277, E0308, E0369.
---
2020-01-01T02:11:54.6643571Z 3    |
2020-01-01T02:11:54.6643605Z 4 LL |     println!("{:?} {:?}", Foo, Bar);
2020-01-01T02:11:54.6643659Z 5    |                           ^^^ `Foo` cannot be formatted using `{:?}`
2020-01-01T02:11:54.6643694Z +    | 
2020-01-01T02:11:54.6643728Z +   ::: $SRC_DIR/libcore/fmt/mod.rs:LL:COL
2020-01-01T02:11:54.6643776Z 6    |
2020-01-01T02:11:54.6643993Z + LL |     pub fn new_debug<T: Debug>(x: &T) -> ArgumentV1<'_> {
2020-01-01T02:11:54.6644210Z +    |                         ----- required by this bound in `std::fmt::ArgumentV1::<'a>::new_debug`
2020-01-01T02:11:54.6644301Z 7    = help: the trait `std::fmt::Debug` is not implemented for `Foo`
2020-01-01T02:11:54.6644342Z 8    = note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
2020-01-01T02:11:54.6644535Z -    = note: required by `std::fmt::Debug::fmt`
2020-01-01T02:11:54.6644650Z 10 
2020-01-01T02:11:54.6644650Z 10 
2020-01-01T02:11:54.6644860Z 11 error[E0277]: `no_debug::Bar` doesn't implement `std::fmt::Debug`
2020-01-01T02:11:54.6645067Z 
2020-01-01T02:11:54.6645098Z 13    |
2020-01-01T02:11:54.6645132Z 14 LL |     println!("{:?} {:?}", Foo, Bar);
2020-01-01T02:11:54.6645132Z 14 LL |     println!("{:?} {:?}", Foo, Bar);
2020-01-01T02:11:54.6645390Z 15    |                                ^^^ `no_debug::Bar` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2020-01-01T02:11:54.6645430Z +    | 
2020-01-01T02:11:54.6645464Z +   ::: $SRC_DIR/libcore/fmt/mod.rs:LL:COL
2020-01-01T02:11:54.6645496Z 16    |
2020-01-01T02:11:54.6645695Z + LL |     pub fn new_debug<T: Debug>(x: &T) -> ArgumentV1<'_> {
2020-01-01T02:11:54.6645910Z +    |                         ----- required by this bound in `std::fmt::ArgumentV1::<'a>::new_debug`
2020-01-01T02:11:54.6646002Z 17    = help: the trait `std::fmt::Debug` is not implemented for `no_debug::Bar`
2020-01-01T02:11:54.6646191Z -    = note: required by `std::fmt::Debug::fmt`
2020-01-01T02:11:54.6646227Z 19 
2020-01-01T02:11:54.6646427Z 20 error[E0277]: `Foo` doesn't implement `std::fmt::Display`
2020-01-01T02:11:54.6646427Z 20 error[E0277]: `Foo` doesn't implement `std::fmt::Display`
2020-01-01T02:11:54.6646592Z 21   --> $DIR/no-debug.rs:11:23
2020-01-01T02:11:54.6646616Z 
2020-01-01T02:11:54.6646647Z 22    |
2020-01-01T02:11:54.6646869Z 23 LL |     println!("{} {}", Foo, Bar);
2020-01-01T02:11:54.6646913Z 24    |                       ^^^ `Foo` cannot be formatted with the default formatter
2020-01-01T02:11:54.6646948Z +    | 
2020-01-01T02:11:54.6646998Z +   ::: $SRC_DIR/libcore/fmt/mod.rs:LL:COL
2020-01-01T02:11:54.6647031Z 25    |
2020-01-01T02:11:54.6647227Z + LL |     pub fn new_display<T: Display>(x: &T) -> ArgumentV1<'_> {
2020-01-01T02:11:54.6647636Z +    |                           ------- required by this bound in `std::fmt::ArgumentV1::<'a>::new_display`
2020-01-01T02:11:54.6647716Z 26    = help: the trait `std::fmt::Display` is not implemented for `Foo`
2020-01-01T02:11:54.6647963Z 27    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
2020-01-01T02:11:54.6648146Z -    = note: required by `std::fmt::Display::fmt`
2020-01-01T02:11:54.6648182Z 29 
2020-01-01T02:11:54.6648182Z 29 
2020-01-01T02:11:54.6648389Z 30 error[E0277]: `no_debug::Bar` doesn't implement `std::fmt::Display`
2020-01-01T02:11:54.6648579Z 
2020-01-01T02:11:54.6648611Z 32    |
2020-01-01T02:11:54.6648662Z 33 LL |     println!("{} {}", Foo, Bar);
2020-01-01T02:11:54.6648662Z 33 LL |     println!("{} {}", Foo, Bar);
2020-01-01T02:11:54.6648704Z 34    |                            ^^^ `no_debug::Bar` cannot be formatted with the default formatter
2020-01-01T02:11:54.6648741Z +    | 
2020-01-01T02:11:54.6648792Z +   ::: $SRC_DIR/libcore/fmt/mod.rs:LL:COL
2020-01-01T02:11:54.6648825Z 35    |
2020-01-01T02:11:54.6649015Z + LL |     pub fn new_display<T: Display>(x: &T) -> ArgumentV1<'_> {
2020-01-01T02:11:54.6649312Z +    |                           ------- required by this bound in `std::fmt::ArgumentV1::<'a>::new_display`
2020-01-01T02:11:54.6649405Z 36    = help: the trait `std::fmt::Display` is not implemented for `no_debug::Bar`
2020-01-01T02:11:54.6649658Z 37    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
2020-01-01T02:11:54.6649841Z -    = note: required by `std::fmt::Display::fmt`
2020-01-01T02:11:54.6649877Z 39 
2020-01-01T02:11:54.6649877Z 39 
2020-01-01T02:11:54.6649912Z 40 error: aborting due to 4 previous errors
2020-01-01T02:11:54.6649961Z 41 
2020-01-01T02:11:54.6649982Z 
2020-01-01T02:11:54.6650003Z 
2020-01-01T02:11:54.6650039Z The actual stderr differed from the expected stderr.
2020-01-01T02:11:54.6650383Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/on-unimplemented/no-debug/no-debug.stderr
2020-01-01T02:11:54.6650576Z To update references, rerun the tests and pass the `--bless` flag
2020-01-01T02:11:54.6650783Z To only update this specific test, also pass `--test-args on-unimplemented/no-debug.rs`
2020-01-01T02:11:54.6650936Z error: 1 errors occurred comparing output.
2020-01-01T02:11:54.6650972Z status: exit code: 1
2020-01-01T02:11:54.6650972Z status: exit code: 1
2020-01-01T02:11:54.6651971Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/on-unimplemented/no-debug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/on-unimplemented/no-debug" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/on-unimplemented/no-debug/auxiliary" "-A" "unused"
2020-01-01T02:11:54.6652260Z ------------------------------------------
2020-01-01T02:11:54.6652287Z 
2020-01-01T02:11:54.6652471Z ------------------------------------------
2020-01-01T02:11:54.6652887Z stderr:
---
2020-01-01T02:11:54.6655484Z    |                           ^^^ `Foo` cannot be formatted using `{:?}`
2020-01-01T02:11:54.6655538Z    | 
2020-01-01T02:11:54.6655571Z   ::: /checkout/src/libcore/fmt/mod.rs:295:25
2020-01-01T02:11:54.6655602Z    |
2020-01-01T02:11:54.6655838Z LL |     pub fn new_debug<T: Debug>(x: &T) -> ArgumentV1<'_> {
2020-01-01T02:11:54.6656040Z    |                         ----- required by this bound in `std::fmt::ArgumentV1::<'a>::new_debug`
2020-01-01T02:11:54.6656131Z    = help: the trait `std::fmt::Debug` is not implemented for `Foo`
2020-01-01T02:11:54.6656187Z    = note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
2020-01-01T02:11:54.6656212Z 
2020-01-01T02:11:54.6656212Z 
2020-01-01T02:11:54.6656405Z error[E0277]: `no_debug::Bar` doesn't implement `std::fmt::Debug`
2020-01-01T02:11:54.6656614Z    |
2020-01-01T02:11:54.6656661Z LL |     println!("{:?} {:?}", Foo, Bar);
2020-01-01T02:11:54.6656661Z LL |     println!("{:?} {:?}", Foo, Bar);
2020-01-01T02:11:54.6656886Z    |                                ^^^ `no_debug::Bar` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2020-01-01T02:11:54.6657152Z   ::: /checkout/src/libcore/fmt/mod.rs:295:25
2020-01-01T02:11:54.6657185Z    |
2020-01-01T02:11:54.6657185Z    |
2020-01-01T02:11:54.6657364Z LL |     pub fn new_debug<T: Debug>(x: &T) -> ArgumentV1<'_> {
2020-01-01T02:11:54.6657592Z    |                         ----- required by this bound in `std::fmt::ArgumentV1::<'a>::new_debug`
2020-01-01T02:11:54.6657783Z    = help: the trait `std::fmt::Debug` is not implemented for `no_debug::Bar`
2020-01-01T02:11:54.6657818Z 
2020-01-01T02:11:54.6658040Z error[E0277]: `Foo` doesn't implement `std::fmt::Display`
2020-01-01T02:11:54.6658223Z   --> /checkout/src/test/ui/on-unimplemented/no-debug.rs:11:23
2020-01-01T02:11:54.6658223Z   --> /checkout/src/test/ui/on-unimplemented/no-debug.rs:11:23
2020-01-01T02:11:54.6658258Z    |
2020-01-01T02:11:54.6658307Z LL |     println!("{} {}", Foo, Bar);
2020-01-01T02:11:54.6658347Z    |                       ^^^ `Foo` cannot be formatted with the default formatter
2020-01-01T02:11:54.6658382Z    | 
2020-01-01T02:11:54.6658435Z   ::: /checkout/src/libcore/fmt/mod.rs:284:27
2020-01-01T02:11:54.6658467Z    |
2020-01-01T02:11:54.6658648Z LL |     pub fn new_display<T: Display>(x: &T) -> ArgumentV1<'_> {
2020-01-01T02:11:54.6658864Z    |                           ------- required by this bound in `std::fmt::ArgumentV1::<'a>::new_display`
2020-01-01T02:11:54.6658956Z    = help: the trait `std::fmt::Display` is not implemented for `Foo`
2020-01-01T02:11:54.6659808Z    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
2020-01-01T02:11:54.6659863Z 
2020-01-01T02:11:54.6659863Z 
2020-01-01T02:11:54.6660055Z error[E0277]: `no_debug::Bar` doesn't implement `std::fmt::Display`
2020-01-01T02:11:54.6660292Z    |
2020-01-01T02:11:54.6660324Z LL |     println!("{} {}", Foo, Bar);
2020-01-01T02:11:54.6660324Z LL |     println!("{} {}", Foo, Bar);
2020-01-01T02:11:54.6660366Z    |                            ^^^ `no_debug::Bar` cannot be formatted with the default formatter
2020-01-01T02:11:54.6660451Z   ::: /checkout/src/libcore/fmt/mod.rs:284:27
2020-01-01T02:11:54.6660484Z    |
2020-01-01T02:11:54.6660484Z    |
2020-01-01T02:11:54.6660819Z LL |     pub fn new_display<T: Display>(x: &T) -> ArgumentV1<'_> {
2020-01-01T02:11:54.6661042Z    |                           ------- required by this bound in `std::fmt::ArgumentV1::<'a>::new_display`
2020-01-01T02:11:54.6661118Z    = help: the trait `std::fmt::Display` is not implemented for `no_debug::Bar`
2020-01-01T02:11:54.6661338Z    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
2020-01-01T02:11:54.6661364Z 
2020-01-01T02:11:54.6661396Z error: aborting due to 4 previous errors
---
2020-01-01T02:11:54.6662011Z ---- [ui] ui/rfc-2361-dbg-macro/dbg-macro-requires-debug.rs stdout ----
2020-01-01T02:11:54.6662046Z diff of stderr:
2020-01-01T02:11:54.6662066Z 
2020-01-01T02:11:54.6662111Z 3    |
2020-01-01T02:11:54.6662144Z 4 LL |     let _: NotDebug = dbg!(NotDebug);
2020-01-01T02:11:54.6662182Z 5    |                       ^^^^^^^^^^^^^^ `NotDebug` cannot be formatted using `{:?}`
2020-01-01T02:11:54.6662231Z +    | 
2020-01-01T02:11:54.6662274Z +   ::: $SRC_DIR/libcore/fmt/mod.rs:LL:COL
2020-01-01T02:11:54.6662304Z 6    |
2020-01-01T02:11:54.6662496Z + LL |     pub fn new_debug<T: Debug>(x: &T) -> ArgumentV1<'_> {
2020-01-01T02:11:54.6663522Z +    |                         ----- required by this bound in `std::fmt::ArgumentV1::<'a>::new_debug`
2020-01-01T02:11:54.6663607Z 7    = help: the trait `std::fmt::Debug` is not implemented for `NotDebug`
2020-01-01T02:11:54.6663666Z 8    = note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
2020-01-01T02:11:54.6663666Z 8    = note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
2020-01-01T02:11:54.6663708Z 9    = note: required because of the requirements on the impl of `std::fmt::Debug` for `&NotDebug`
2020-01-01T02:11:54.6664580Z -    = note: required by `std::fmt::Debug::fmt`
2020-01-01T02:11:54.6664827Z 11    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2020-01-01T02:11:54.6664869Z 12 
2020-01-01T02:11:54.6665040Z 13 error: aborting due to previous error
2020-01-01T02:11:54.6665040Z 13 error: aborting due to previous error
2020-01-01T02:11:54.6665071Z 
2020-01-01T02:11:54.6665091Z 
2020-01-01T02:11:54.6665127Z The actual stderr differed from the expected stderr.
2020-01-01T02:11:54.6665430Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-requires-debug/dbg-macro-requires-debug.stderr
2020-01-01T02:11:54.6665627Z To update references, rerun the tests and pass the `--bless` flag
2020-01-01T02:11:54.6665845Z To only update this specific test, also pass `--test-args rfc-2361-dbg-macro/dbg-macro-requires-debug.rs`
2020-01-01T02:11:54.6665923Z error: 1 errors occurred comparing output.
2020-01-01T02:11:54.6665958Z status: exit code: 1
2020-01-01T02:11:54.6665958Z status: exit code: 1
2020-01-01T02:11:54.6666866Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-requires-debug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-requires-debug" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-requires-debug/auxiliary" "-A" "unused"
2020-01-01T02:11:54.6667236Z ------------------------------------------
2020-01-01T02:11:54.6667263Z 
2020-01-01T02:11:54.6667431Z ------------------------------------------
2020-01-01T02:11:54.6667483Z stderr:
2020-01-01T02:11:54.6667483Z stderr:
2020-01-01T02:11:54.6667646Z ------------------------------------------
2020-01-01T02:11:54.6667828Z error[E0277]: `NotDebug` doesn't implement `std::fmt::Debug`
2020-01-01T02:11:54.6668092Z    |
2020-01-01T02:11:54.6668092Z    |
2020-01-01T02:11:54.6668308Z LL |     let _: NotDebug = dbg!(NotDebug); //~ ERROR `NotDebug` doesn't implement `std::fmt::Debug`
2020-01-01T02:11:54.6668372Z    |                       ^^^^^^^^^^^^^^ `NotDebug` cannot be formatted using `{:?}`
2020-01-01T02:11:54.6668441Z   ::: /checkout/src/libcore/fmt/mod.rs:295:25
2020-01-01T02:11:54.6668492Z    |
2020-01-01T02:11:54.6668492Z    |
2020-01-01T02:11:54.6668675Z LL |     pub fn new_debug<T: Debug>(x: &T) -> ArgumentV1<'_> {
2020-01-01T02:11:54.6668888Z    |                         ----- required by this bound in `std::fmt::ArgumentV1::<'a>::new_debug`
2020-01-01T02:11:54.6668979Z    = help: the trait `std::fmt::Debug` is not implemented for `NotDebug`
2020-01-01T02:11:54.6669020Z    = note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
2020-01-01T02:11:54.6669020Z    = note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
2020-01-01T02:11:54.6669062Z    = note: required because of the requirements on the impl of `std::fmt::Debug` for `&NotDebug`
2020-01-01T02:11:54.6674522Z 
2020-01-01T02:11:54.6674570Z error: aborting due to previous error
2020-01-01T02:11:54.6674618Z 
2020-01-01T02:11:54.6674938Z For more information about this error, try `rustc --explain E0277`.
---
2020-01-01T02:11:54.6675404Z ---- [ui] ui/suggestions/path-display.rs stdout ----
2020-01-01T02:11:54.6675444Z diff of stderr:
2020-01-01T02:11:54.6675483Z 
2020-01-01T02:11:54.6675517Z 3    |
2020-01-01T02:11:54.6675552Z 4 LL |     println!("{}", path);
2020-01-01T02:11:54.6675598Z 5    |                    ^^^^ `std::path::Path` cannot be formatted with the default formatter; call `.display()` on it
2020-01-01T02:11:54.6675655Z +    | 
2020-01-01T02:11:54.6675691Z +   ::: $SRC_DIR/libcore/fmt/mod.rs:LL:COL
2020-01-01T02:11:54.6675854Z 6    |
2020-01-01T02:11:54.6676119Z + LL |     pub fn new_display<T: Display>(x: &T) -> ArgumentV1<'_> {
2020-01-01T02:11:54.6676745Z +    |                           ------- required by this bound in `std::fmt::ArgumentV1::<'a>::new_display`
2020-01-01T02:11:54.6676853Z 7    = help: the trait `std::fmt::Display` is not implemented for `std::path::Path`
2020-01-01T02:11:54.6676853Z 7    = help: the trait `std::fmt::Display` is not implemented for `std::path::Path`
2020-01-01T02:11:54.6677273Z 8    = note: call `.display()` or `.to_string_lossy()` to safely print paths, as they may contain non-Unicode data
2020-01-01T02:11:54.6677325Z 9    = note: required because of the requirements on the impl of `std::fmt::Display` for `&std::path::Path`
2020-01-01T02:11:54.6677557Z -    = note: required by `std::fmt::Display::fmt`
2020-01-01T02:11:54.6677594Z 11 
2020-01-01T02:11:54.6677630Z 12 error: aborting due to previous error
2020-01-01T02:11:54.6677681Z 13 
2020-01-01T02:11:54.6677681Z 13 
2020-01-01T02:11:54.6677702Z 
2020-01-01T02:11:54.6677723Z 
2020-01-01T02:11:54.6677869Z The actual stderr differed from the expected stderr.
2020-01-01T02:11:54.6678430Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/path-display/path-display.stderr
2020-01-01T02:11:54.6678691Z To update references, rerun the tests and pass the `--bless` flag
2020-01-01T02:11:54.6678903Z To only update this specific test, also pass `--test-args suggestions/path-display.rs`
2020-01-01T02:11:54.6678992Z error: 1 errors occurred comparing output.
2020-01-01T02:11:54.6679029Z status: exit code: 1
2020-01-01T02:11:54.6679029Z status: exit code: 1
2020-01-01T02:11:54.6679744Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/path-display.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/path-display" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/path-display/auxiliary" "-A" "unused"
2020-01-01T02:11:54.6680025Z ------------------------------------------
2020-01-01T02:11:54.6680053Z 
2020-01-01T02:11:54.6680384Z ------------------------------------------
2020-01-01T02:11:54.6680436Z stderr:
2020-01-01T02:11:54.6680436Z stderr:
2020-01-01T02:11:54.6680946Z ------------------------------------------
2020-01-01T02:11:54.6681189Z error[E0277]: `std::path::Path` doesn't implement `std::fmt::Display`
2020-01-01T02:11:54.6681387Z   --> /checkout/src/test/ui/suggestions/path-display.rs:5:20
2020-01-01T02:11:54.6681427Z    |
2020-01-01T02:11:54.6681460Z LL |     println!("{}", path);
2020-01-01T02:11:54.6681519Z    |                    ^^^^ `std::path::Path` cannot be formatted with the default formatter; call `.display()` on it
2020-01-01T02:11:54.6681606Z   ::: /checkout/src/libcore/fmt/mod.rs:284:27
2020-01-01T02:11:54.6681655Z    |
2020-01-01T02:11:54.6681655Z    |
2020-01-01T02:11:54.6681840Z LL |     pub fn new_display<T: Display>(x: &T) -> ArgumentV1<'_> {
2020-01-01T02:11:54.6682055Z    |                           ------- required by this bound in `std::fmt::ArgumentV1::<'a>::new_display`
2020-01-01T02:11:54.6682147Z    = help: the trait `std::fmt::Display` is not implemented for `std::path::Path`
2020-01-01T02:11:54.6682147Z    = help: the trait `std::fmt::Display` is not implemented for `std::path::Path`
2020-01-01T02:11:54.6682605Z    = note: call `.display()` or `.to_string_lossy()` to safely print paths, as they may contain non-Unicode data
2020-01-01T02:11:54.6682675Z    = note: required because of the requirements on the impl of `std::fmt::Display` for `&std::path::Path`
2020-01-01T02:11:54.6682735Z error: aborting due to previous error
2020-01-01T02:11:54.6682758Z 
2020-01-01T02:11:54.6682963Z For more information about this error, try `rustc --explain E0277`.
2020-01-01T02:11:54.6682998Z 
---
2020-01-01T02:11:54.6684706Z test result: FAILED. 9414 passed; 6 failed; 47 ignored; 0 measured; 0 filtered out
2020-01-01T02:11:54.6684735Z 
2020-01-01T02:11:54.6684819Z 
2020-01-01T02:11:54.6684841Z 
2020-01-01T02:11:54.6686482Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-01T02:11:54.6686774Z 
2020-01-01T02:11:54.6686797Z 
2020-01-01T02:11:54.6686833Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-01T02:11:54.6686890Z Build completed unsuccessfully in 0:51:42
2020-01-01T02:11:54.6686890Z Build completed unsuccessfully in 0:51:42
2020-01-01T02:11:54.6686927Z == clock drift check ==
2020-01-01T02:11:54.6687171Z   local time: thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2020-01-01T02:11:54.6687237Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-01T02:11:54.6687277Z Wed Jan  1 02:11:54 UTC 2020
2020-01-01T02:11:54.9560948Z   network time: Wed, 01 Jan 2020 02:11:54 GMT
2020-01-01T02:11:54.9561045Z == end clock drift check ==
2020-01-01T02:11:56.1834560Z 
2020-01-01T02:11:56.1918747Z ##[error]Bash exited with code '1'.
2020-01-01T02:11:56.1958449Z ##[section]Starting: Checkout
2020-01-01T02:11:56.1960060Z ==============================================================================
2020-01-01T02:11:56.1960119Z Task         : Get sources
2020-01-01T02:11:56.1960154Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
