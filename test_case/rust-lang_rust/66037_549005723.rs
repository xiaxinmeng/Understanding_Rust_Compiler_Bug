plain
2019-11-02T02:07:50.4417606Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-02T02:07:50.4628299Z ##[command]git config gc.auto 0
2019-11-02T02:07:50.4696322Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-02T02:07:50.4751202Z ##[command]git config --get-all http.proxy
2019-11-02T02:07:50.4918867Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66037/merge:refs/remotes/pull/66037/merge
---
2019-11-02T03:09:32.3065883Z .................................................................................................... 1600/9265
2019-11-02T03:09:38.2733036Z .................................................................................................... 1700/9265
2019-11-02T03:09:51.0151292Z ............................................................i...............i....................... 1800/9265
2019-11-02T03:09:59.0391211Z .................................................................................................... 1900/9265
2019-11-02T03:10:14.2589803Z ..................................................iiiii............................................. 2000/9265
2019-11-02T03:10:25.3093074Z .................................................................................................... 2200/9265
2019-11-02T03:10:27.9913859Z .................................................................................................... 2300/9265
2019-11-02T03:10:31.7395556Z .................................................................................................... 2400/9265
2019-11-02T03:10:55.6890813Z .................................................................................................... 2500/9265
---
2019-11-02T03:13:52.3603103Z ..................................................i...............i................................. 4800/9265
2019-11-02T03:14:01.4834229Z .................................................................................................... 4900/9265
2019-11-02T03:14:10.5156715Z .................................................................................................... 5000/9265
2019-11-02T03:14:16.8907896Z .................................................................................................... 5100/9265
2019-11-02T03:14:27.5567807Z ...................................................ii.ii...........i................................ 5200/9265
2019-11-02T03:14:37.8423411Z .................................................................................................... 5400/9265
2019-11-02T03:14:48.1430146Z .................................................................................................... 5500/9265
2019-11-02T03:14:55.8648747Z ........................i........................................................................... 5600/9265
2019-11-02T03:15:02.6017646Z .................................................................................................... 5700/9265
2019-11-02T03:15:02.6017646Z .................................................................................................... 5700/9265
2019-11-02T03:15:14.8618036Z .................................................................................................... 5800/9265
2019-11-02T03:15:27.1586717Z .........ii...i..ii...........i..................................................................... 5900/9265
2019-11-02T03:15:49.7816537Z .................................................................................................... 6100/9265
2019-11-02T03:15:55.8835575Z .................................................................................................... 6200/9265
2019-11-02T03:15:55.8835575Z .................................................................................................... 6200/9265
2019-11-02T03:16:10.3253608Z ............................i..ii................................................................... 6300/9265
2019-11-02T03:16:31.0238528Z ...............................................................................................i.... 6500/9265
2019-11-02T03:16:33.3975395Z .................................................................................................... 6600/9265
2019-11-02T03:16:35.8548098Z .....................................................................i.............................. 6700/9265
2019-11-02T03:16:38.9310927Z .................................................................................................... 6800/9265
---
2019-11-02T03:21:40.0602018Z ---- [ui] ui/codegen-object-shim.rs stdout ----
2019-11-02T03:21:40.0602290Z 
2019-11-02T03:21:40.0605672Z error: test compilation failed although it shouldn't!
2019-11-02T03:21:40.0605746Z status: exit code: 1
2019-11-02T03:21:40.0607166Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/codegen-object-shim.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codegen-object-shim/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codegen-object-shim/auxiliary"
2019-11-02T03:21:40.0612799Z ------------------------------------------
2019-11-02T03:21:40.0612871Z 
2019-11-02T03:21:40.0613226Z ------------------------------------------
2019-11-02T03:21:40.0613278Z stderr:
2019-11-02T03:21:40.0613278Z stderr:
2019-11-02T03:21:40.0613570Z ------------------------------------------
2019-11-02T03:21:40.0613644Z warning: impl_potentially_overlapping_dyn_trait
2019-11-02T03:21:40.0613962Z    |
2019-11-02T03:21:40.0613962Z    |
2019-11-02T03:21:40.0614008Z LL | / impl<T: fmt::Display + ?Sized> ToString for T {
2019-11-02T03:21:40.0614054Z LL | |     #[inline]
2019-11-02T03:21:40.0614306Z LL | |     default fn to_string(&self) -> String {
2019-11-02T03:21:40.0614365Z LL | |         use fmt::Write;
2019-11-02T03:21:40.0614447Z LL | |     }
2019-11-02T03:21:40.0614504Z LL | | }
2019-11-02T03:21:40.0614542Z    | |_^
2019-11-02T03:21:40.0614570Z 
2019-11-02T03:21:40.0614570Z 
2019-11-02T03:21:40.0614618Z error[E0038]: the trait `std::string::ToString` cannot be made into an object
2019-11-02T03:21:40.0615234Z   --> /checkout/src/test/ui/codegen-object-shim.rs:4:79
2019-11-02T03:21:40.0615327Z    |
2019-11-02T03:21:40.0615744Z LL |     assert_eq!((ToString::to_string as fn(&(dyn ToString+'static)) -> String)(&"foo"),
2019-11-02T03:21:40.0615888Z    |
2019-11-02T03:21:40.0615888Z    |
2019-11-02T03:21:40.0616246Z    = note: required because of the requirements on the impl of `std::ops::CoerceUnsized<&dyn std::string::ToString>` for `&&'static str`
2019-11-02T03:21:40.0616530Z    = note: required by cast to type `&(dyn std::string::ToString + 'static)`
2019-11-02T03:21:40.0616635Z error[E0038]: the trait `std::string::ToString` cannot be made into an object
2019-11-02T03:21:40.0616886Z   --> /checkout/src/test/ui/codegen-object-shim.rs:4:40
2019-11-02T03:21:40.0616936Z    |
2019-11-02T03:21:40.0616936Z    |
2019-11-02T03:21:40.0617202Z LL |     assert_eq!((ToString::to_string as fn(&(dyn ToString+'static)) -> String)(&"foo"),
2019-11-02T03:21:40.0617322Z 
2019-11-02T03:21:40.0617375Z error: aborting due to 2 previous errors
2019-11-02T03:21:40.0617419Z 
2019-11-02T03:21:40.0617669Z For more information about this error, try `rustc --explain E0038`.
---
2019-11-02T03:21:40.0618235Z ---- [ui] ui/issues/issue-24010.rs stdout ----
2019-11-02T03:21:40.0618268Z 
2019-11-02T03:21:40.0618499Z error: test compilation failed although it shouldn't!
2019-11-02T03:21:40.0618565Z status: exit code: 1
2019-11-02T03:21:40.0619706Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-24010.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24010/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24010/auxiliary"
2019-11-02T03:21:40.0620326Z ------------------------------------------
2019-11-02T03:21:40.0620363Z 
2019-11-02T03:21:40.0620611Z ------------------------------------------
2019-11-02T03:21:40.0620656Z stderr:
2019-11-02T03:21:40.0620656Z stderr:
2019-11-02T03:21:40.0620978Z ------------------------------------------
2019-11-02T03:21:40.0621058Z warning: impl_potentially_overlapping_dyn_trait
2019-11-02T03:21:40.0621326Z   --> /checkout/src/test/ui/issues/issue-24010.rs:5:1
2019-11-02T03:21:40.0621374Z    |
2019-11-02T03:21:40.0621628Z LL | impl<T: ?Sized + Fn(i32) -> i32 + Send> Foo for T {}
2019-11-02T03:21:40.0621720Z 
2019-11-02T03:21:40.0621765Z error[E0038]: the trait `Foo` cannot be made into an object
2019-11-02T03:21:40.0622022Z   --> /checkout/src/test/ui/issues/issue-24010.rs:7:1
2019-11-02T03:21:40.0622077Z    |
2019-11-02T03:21:40.0622077Z    |
2019-11-02T03:21:40.0622298Z LL | fn wants_foo(f: Box<dyn Foo>) -> i32 {
2019-11-02T03:21:40.0622373Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Foo` cannot be made into an object
2019-11-02T03:21:40.0622453Z error: aborting due to previous error
2019-11-02T03:21:40.0622481Z 
2019-11-02T03:21:40.0622748Z For more information about this error, try `rustc --explain E0038`.
2019-11-02T03:21:40.0622783Z 
2019-11-02T03:21:40.0622783Z 
2019-11-02T03:21:40.0623001Z ------------------------------------------
2019-11-02T03:21:40.0623058Z 
2019-11-02T03:21:40.0623083Z 
2019-11-02T03:21:40.0623326Z ---- [ui] ui/issues/issue-33140-traitobject-crate.rs stdout ----
2019-11-02T03:21:40.0623359Z 
2019-11-02T03:21:40.0623908Z error: test compilation failed although it shouldn't!
2019-11-02T03:21:40.0623972Z status: exit code: 1
2019-11-02T03:21:40.0624785Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-33140-traitobject-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-33140-traitobject-crate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-33140-traitobject-crate/auxiliary" "-A" "unused"
2019-11-02T03:21:40.0625142Z ------------------------------------------
2019-11-02T03:21:40.0625175Z 
2019-11-02T03:21:40.0625416Z ------------------------------------------
2019-11-02T03:21:40.0625460Z stderr:
2019-11-02T03:21:40.0625460Z stderr:
2019-11-02T03:21:40.0625677Z ------------------------------------------
2019-11-02T03:21:40.0626004Z warning: conflicting implementations of trait `Trait` for type `(dyn std::marker::Send + std::marker::Sync + 'static)`: (E0119)
2019-11-02T03:21:40.0626893Z    |
2019-11-02T03:21:40.0626893Z    |
2019-11-02T03:21:40.0626974Z LL | unsafe impl Trait for dyn (::std::marker::Send) + Sync { }
2019-11-02T03:21:40.0627767Z    | ------------------------------------------------------ first implementation here
2019-11-02T03:21:40.0627833Z LL | unsafe impl Trait for dyn (::std::marker::Send) + Send + Sync { }
2019-11-02T03:21:40.0628203Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + std::marker::Sync + 'static)`
2019-11-02T03:21:40.0628355Z note: lint level defined here
2019-11-02T03:21:40.0628620Z   --> /checkout/src/test/ui/issues/issue-33140-traitobject-crate.rs:3:9
2019-11-02T03:21:40.0628668Z    |
2019-11-02T03:21:40.0628712Z LL | #![warn(order_dependent_trait_objects)]
2019-11-02T03:21:40.0628712Z LL | #![warn(order_dependent_trait_objects)]
2019-11-02T03:21:40.0628775Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-11-02T03:21:40.0628832Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2019-11-02T03:21:40.0630637Z    = note: for more information, see issue #56484 <***/issues/56484>
2019-11-02T03:21:40.0630688Z 
2019-11-02T03:21:40.0631865Z warning: conflicting implementations of trait `Trait` for type `(dyn std::marker::Send + std::marker::Sync + 'static)`: (E0119)
2019-11-02T03:21:40.0632381Z    |
2019-11-02T03:21:40.0632381Z    |
2019-11-02T03:21:40.0632427Z LL | unsafe impl Trait for dyn (::std::marker::Send) + Send + Sync { }
2019-11-02T03:21:40.0632740Z    | ------------------------------------------------------------- first implementation here
2019-11-02T03:21:40.0632817Z LL | unsafe impl Trait for dyn (::std::marker::Sync) + Send { }
2019-11-02T03:21:40.0633965Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + std::marker::Sync + 'static)`
2019-11-02T03:21:40.0634660Z    |
2019-11-02T03:21:40.0634756Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2019-11-02T03:21:40.0635208Z    = note: for more information, see issue #56484 <***/issues/56484>
2019-11-02T03:21:40.0635269Z 
2019-11-02T03:21:40.0635601Z warning: conflicting implementations of trait `Trait` for type `(dyn std::marker::Send + std::marker::Sync + 'static)`: (E0119)
2019-11-02T03:21:40.0636378Z    |
2019-11-02T03:21:40.0636378Z    |
2019-11-02T03:21:40.0636426Z LL | unsafe impl Trait for dyn (::std::marker::Sync) + Send { }
2019-11-02T03:21:40.0636697Z    | ------------------------------------------------------ first implementation here
2019-11-02T03:21:40.0636767Z LL | unsafe impl Trait for dyn (::std::marker::Sync) + Sync { }
2019-11-02T03:21:40.0636818Z LL | unsafe impl Trait for dyn (::std::marker::Sync) + Send + Sync { }
2019-11-02T03:21:40.0637147Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + std::marker::Sync + 'static)`
2019-11-02T03:21:40.0637233Z    |
2019-11-02T03:21:40.0637287Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2019-11-02T03:21:40.0638247Z    = note: for more information, see issue #56484 <***/issues/56484>
2019-11-02T03:21:40.0638317Z 
2019-11-02T03:21:40.0638363Z warning: impl_potentially_overlapping_dyn_trait
2019-11-02T03:21:40.0638668Z    |
2019-11-02T03:21:40.0638668Z    |
2019-11-02T03:21:40.0638729Z LL | / impl<T: fmt::Display + ?Sized> ToString for T {
2019-11-02T03:21:40.0638775Z LL | |     #[inline]
2019-11-02T03:21:40.0639768Z LL | |     default fn to_string(&self) -> String {
2019-11-02T03:21:40.0639872Z LL | |         use fmt::Write;
2019-11-02T03:21:40.0640384Z LL | |     }
2019-11-02T03:21:40.0640464Z LL | | }
2019-11-02T03:21:40.0640504Z    | |_^
2019-11-02T03:21:40.0640531Z 
2019-11-02T03:21:40.0640531Z 
2019-11-02T03:21:40.0640580Z error[E0038]: the trait `std::string::ToString` cannot be made into an object
2019-11-02T03:21:40.0641285Z   --> /checkout/src/test/ui/issues/issue-33140-traitobject-crate.rs:92:13
2019-11-02T03:21:40.0641350Z    |
2019-11-02T03:21:40.0641408Z LL | unsafe impl Trait for dyn (::std::string::ToString) + Send { }
2019-11-02T03:21:40.0641517Z 
2019-11-02T03:21:40.0641563Z error[E0038]: the trait `std::string::ToString` cannot be made into an object
2019-11-02T03:21:40.0642230Z   --> /checkout/src/test/ui/issues/issue-33140-traitobject-crate.rs:93:13
2019-11-02T03:21:40.0642311Z    |
2019-11-02T03:21:40.0642311Z    |
2019-11-02T03:21:40.0642357Z LL | unsafe impl Trait for dyn (::std::string::ToString) + Sync { }
2019-11-02T03:21:40.0642619Z 
2019-11-02T03:21:40.0642668Z error[E0038]: the trait `std::string::ToString` cannot be made into an object
2019-11-02T03:21:40.0642985Z   --> /checkout/src/test/ui/issues/issue-33140-traitobject-crate.rs:94:13
2019-11-02T03:21:40.0643049Z    |
2019-11-02T03:21:40.0643049Z    |
2019-11-02T03:21:40.0643096Z LL | unsafe impl Trait for dyn (::std::string::ToString) + Send + Sync { }
2019-11-02T03:21:40.0643273Z 
2019-11-02T03:21:40.0643331Z error: aborting due to 3 previous errors
2019-11-02T03:21:40.0643360Z 
2019-11-02T03:21:40.0643930Z For more information about this error, try `rustc --explain E0038`.
2019-11-02T03:21:40.0643930Z For more information about this error, try `rustc --explain E0038`.
2019-11-02T03:21:40.0643975Z 
2019-11-02T03:21:40.0644224Z ------------------------------------------
2019-11-02T03:21:40.0644256Z 
2019-11-02T03:21:40.0644281Z 
2019-11-02T03:21:40.0644504Z ---- [ui] ui/issues/issue-42312.rs stdout ----
2019-11-02T03:21:40.0644569Z diff of stderr:
2019-11-02T03:21:40.0644608Z 
2019-11-02T03:21:40.0644905Z - error[E0277]: the size for values of type `<Self as std::ops::Deref>::Target` cannot be known at compilation time
2019-11-02T03:21:40.0645145Z -   --> $DIR/issue-42312.rs:4:29
2019-11-02T03:21:40.0645196Z + warning: impl_potentially_overlapping_dyn_trait
2019-11-02T03:21:40.0645419Z +   --> $SRC_DIR/liballoc/string.rs:LL:COL
2019-11-02T03:21:40.0645472Z 3    |
2019-11-02T03:21:40.0645723Z - LL |     fn baz(_: Self::Target) where Self: Deref {}
2019-11-02T03:21:40.0646068Z -    |                             ^                - help: consider further restricting the associated type: `, <Self as std::ops::Deref>::Target: std::marker::Sized`
2019-11-02T03:21:40.0646568Z -    |                             doesn't have a size known at compile-time
2019-11-02T03:21:40.0646762Z -    |
2019-11-02T03:21:40.0646762Z -    |
2019-11-02T03:21:40.0647038Z -    = help: the trait `std::marker::Sized` is not implemented for `<Self as std::ops::Deref>::Target`
2019-11-02T03:21:40.0647651Z -    = note: all function arguments must have a statically known size
2019-11-02T03:21:40.0647894Z -    = help: unsized locals are gated as an unstable feature
2019-11-02T03:21:40.0647894Z -    = help: unsized locals are gated as an unstable feature
2019-11-02T03:21:40.0647971Z + LL | / impl<T: fmt::Display + ?Sized> ToString for T {
2019-11-02T03:21:40.0648017Z + LL | |     #[inline]
2019-11-02T03:21:40.0648253Z + LL | |     default fn to_string(&self) -> String {
2019-11-02T03:21:40.0648318Z + LL | |         use fmt::Write;
2019-11-02T03:21:40.0648361Z + ...  |
2019-11-02T03:21:40.0648401Z + LL | |     }
2019-11-02T03:21:40.0648460Z + LL | | }
2019-11-02T03:21:40.0648538Z 13 
2019-11-02T03:21:40.0648538Z 13 
2019-11-02T03:21:40.0648836Z - error[E0277]: the size for values of type `(dyn std::string::ToString + 'static)` cannot be known at compilation time
2019-11-02T03:21:40.0649086Z -   --> $DIR/issue-42312.rs:8:27
2019-11-02T03:21:40.0649371Z +   --> $DIR/issue-42312.rs:8:1
2019-11-02T03:21:40.0649432Z 16    |
2019-11-02T03:21:40.0649432Z 16    |
2019-11-02T03:21:40.0649475Z 17 LL | pub fn f(_: dyn ToString) {}
2019-11-02T03:21:40.0649737Z -    |                           ^ doesn't have a size known at compile-time
2019-11-02T03:21:40.0649947Z -    |
2019-11-02T03:21:40.0650226Z -    = help: the trait `std::marker::Sized` is not implemented for `(dyn std::string::ToString + 'static)`
2019-11-02T03:21:40.0650822Z -    = note: all function arguments must have a statically known size
2019-11-02T03:21:40.0651066Z -    = help: unsized locals are gated as an unstable feature
2019-11-02T03:21:40.0651124Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::string::ToString` cannot be made into an object
2019-11-02T03:21:40.0651320Z 24 
---
2019-11-02T03:21:40.0652367Z 
2019-11-02T03:21:40.0652393Z 
2019-11-02T03:21:40.0652437Z The actual stderr differed from the expected stderr.
2019-11-02T03:21:40.0652761Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42312/issue-42312.stderr
2019-11-02T03:21:40.0653030Z To update references, rerun the tests and pass the `--bless` flag
2019-11-02T03:21:40.0653292Z To only update this specific test, also pass `--test-args issues/issue-42312.rs`
2019-11-02T03:21:40.0653385Z error: 1 errors occurred comparing output.
2019-11-02T03:21:40.0653438Z status: exit code: 1
2019-11-02T03:21:40.0653438Z status: exit code: 1
2019-11-02T03:21:40.0656911Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-42312.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42312" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42312/auxiliary" "-A" "unused"
2019-11-02T03:21:40.0657292Z ------------------------------------------
2019-11-02T03:21:40.0657344Z 
2019-11-02T03:21:40.0657573Z ------------------------------------------
2019-11-02T03:21:40.0657619Z stderr:
2019-11-02T03:21:40.0657619Z stderr:
2019-11-02T03:21:40.0657837Z ------------------------------------------
2019-11-02T03:21:40.0657903Z warning: impl_potentially_overlapping_dyn_trait
2019-11-02T03:21:40.0658194Z    |
2019-11-02T03:21:40.0658194Z    |
2019-11-02T03:21:40.0658255Z LL | / impl<T: fmt::Display + ?Sized> ToString for T {
2019-11-02T03:21:40.0658302Z LL | |     #[inline]
2019-11-02T03:21:40.0659372Z LL | |     default fn to_string(&self) -> String {
2019-11-02T03:21:40.0659469Z LL | |         use fmt::Write;
2019-11-02T03:21:40.0659553Z LL | |     }
2019-11-02T03:21:40.0659610Z LL | | }
2019-11-02T03:21:40.0659651Z    | |_^
2019-11-02T03:21:40.0659678Z 
2019-11-02T03:21:40.0659678Z 
2019-11-02T03:21:40.0659726Z error[E0038]: the trait `std::string::ToString` cannot be made into an object
2019-11-02T03:21:40.0660005Z   --> /checkout/src/test/ui/issues/issue-42312.rs:8:1
2019-11-02T03:21:40.0660056Z    |
2019-11-02T03:21:40.0660100Z LL | pub fn f(_: dyn ToString) {}
2019-11-02T03:21:40.0660214Z 
2019-11-02T03:21:40.0660767Z error: aborting due to previous error
2019-11-02T03:21:40.0660809Z 
2019-11-02T03:21:40.0661173Z For more information about this error, try `rustc --explain E0038`.
---
2019-11-02T03:21:40.0662374Z ---- [ui] ui/structs-enums/class-cast-to-trait-cross-crate-2.rs stdout ----
2019-11-02T03:21:40.0663239Z 
2019-11-02T03:21:40.0663996Z error: test compilation failed although it shouldn't!
2019-11-02T03:21:40.0664085Z status: exit code: 1
2019-11-02T03:21:40.0666303Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/structs-enums/class-cast-to-trait-cross-crate-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs-enums/class-cast-to-trait-cross-crate-2/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs-enums/class-cast-to-trait-cross-crate-2/auxiliary"
2019-11-02T03:21:40.0666977Z ------------------------------------------
2019-11-02T03:21:40.0667187Z 
2019-11-02T03:21:40.0667471Z ------------------------------------------
2019-11-02T03:21:40.0667519Z stderr:
2019-11-02T03:21:40.0667519Z stderr:
2019-11-02T03:21:40.0667740Z ------------------------------------------
2019-11-02T03:21:40.0667806Z warning: impl_potentially_overlapping_dyn_trait
2019-11-02T03:21:40.0668087Z    |
2019-11-02T03:21:40.0668087Z    |
2019-11-02T03:21:40.0668133Z LL | / impl<T: fmt::Display + ?Sized> ToString for T {
2019-11-02T03:21:40.0668196Z LL | |     #[inline]
2019-11-02T03:21:40.0668425Z LL | |     default fn to_string(&self) -> String {
2019-11-02T03:21:40.0668485Z LL | |         use fmt::Write;
2019-11-02T03:21:40.0668583Z LL | |     }
2019-11-02T03:21:40.0668623Z LL | | }
2019-11-02T03:21:40.0668677Z    | |_^
2019-11-02T03:21:40.0668705Z 
2019-11-02T03:21:40.0668705Z 
2019-11-02T03:21:40.0668752Z error[E0038]: the trait `std::string::ToString` cannot be made into an object
2019-11-02T03:21:40.0669043Z   --> /checkout/src/test/ui/structs-enums/class-cast-to-trait-cross-crate-2.rs:11:1
2019-11-02T03:21:40.0669112Z    |
2019-11-02T03:21:40.0669161Z LL | fn print_out(thing: Box<dyn ToString>, expected: String) {
2019-11-02T03:21:40.0669271Z 
2019-11-02T03:21:40.0669316Z error: aborting due to previous error
2019-11-02T03:21:40.0669344Z 
2019-11-02T03:21:40.0669597Z For more information about this error, try `rustc --explain E0038`.
---
2019-11-02T03:21:40.0670186Z ---- [ui] ui/structs-enums/class-separate-impl.rs stdout ----
2019-11-02T03:21:40.0670219Z 
2019-11-02T03:21:40.0670451Z error: test compilation failed although it shouldn't!
2019-11-02T03:21:40.0670500Z status: exit code: 1
2019-11-02T03:21:40.0671254Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/structs-enums/class-separate-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs-enums/class-separate-impl/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs-enums/class-separate-impl/auxiliary"
2019-11-02T03:21:40.0671584Z ------------------------------------------
2019-11-02T03:21:40.0671626Z 
2019-11-02T03:21:40.0671848Z ------------------------------------------
2019-11-02T03:21:40.0672162Z stderr:
2019-11-02T03:21:40.0672162Z stderr:
2019-11-02T03:21:40.0672452Z ------------------------------------------
2019-11-02T03:21:40.0679799Z warning: impl_potentially_overlapping_dyn_trait
2019-11-02T03:21:40.0684060Z    |
2019-11-02T03:21:40.0684060Z    |
2019-11-02T03:21:40.0684161Z LL | / impl<T: fmt::Display + ?Sized> ToString for T {
2019-11-02T03:21:40.0684209Z LL | |     #[inline]
2019-11-02T03:21:40.0688188Z LL | |     default fn to_string(&self) -> String {
2019-11-02T03:21:40.0688262Z LL | |         use fmt::Write;
2019-11-02T03:21:40.0688370Z LL | |     }
2019-11-02T03:21:40.0688412Z LL | | }
2019-11-02T03:21:40.0688452Z    | |_^
2019-11-02T03:21:40.0688480Z 
2019-11-02T03:21:40.0688480Z 
2019-11-02T03:21:40.0688547Z error[E0038]: the trait `std::string::ToString` cannot be made into an object
2019-11-02T03:21:40.0688874Z   --> /checkout/src/test/ui/structs-enums/class-separate-impl.rs:56:1
2019-11-02T03:21:40.0689085Z    |
2019-11-02T03:21:40.0689152Z LL | fn print_out(thing: Box<dyn ToString>, expected: String) {
2019-11-02T03:21:40.0689258Z 
2019-11-02T03:21:40.0689402Z error: aborting due to previous error
2019-11-02T03:21:40.0689440Z 
2019-11-02T03:21:40.0689772Z For more information about this error, try `rustc --explain E0038`.
2019-11-02T03:21:40.0689772Z For more information about this error, try `rustc --explain E0038`.
2019-11-02T03:21:40.0689810Z 
2019-11-02T03:21:40.0690074Z ------------------------------------------
2019-11-02T03:21:40.0690108Z 
2019-11-02T03:21:40.0690135Z 
2019-11-02T03:21:40.0690403Z ---- [ui] ui/use/use-after-move-implicity-coerced-object.rs stdout ----
2019-11-02T03:21:40.0690473Z diff of stderr:
2019-11-02T03:21:40.0690502Z 
2019-11-02T03:21:40.0690740Z - error[E0382]: borrow of moved value: `n`
2019-11-02T03:21:40.0691010Z -   --> $DIR/use-after-move-implicity-coerced-object.rs:28:13
2019-11-02T03:21:40.0691093Z + warning: impl_potentially_overlapping_dyn_trait
2019-11-02T03:21:40.0691343Z +   --> $SRC_DIR/liballoc/string.rs:LL:COL
2019-11-02T03:21:40.0691390Z 3    |
2019-11-02T03:21:40.0691655Z - LL |     let n: Box<_> = box Number { n: 42 };
2019-11-02T03:21:40.0691983Z -    |         - move occurs because `n` has type `std::boxed::Box<Number>`, which does not implement the `Copy` trait
2019-11-02T03:21:40.0692255Z - LL |     let mut l: Box<_> = box List { list: Vec::new() };
2019-11-02T03:21:40.0692494Z - LL |     l.push(n);
2019-11-02T03:21:40.0692731Z -    |            - value moved here
2019-11-02T03:21:40.0693037Z - LL |     let x = n.to_string();
2019-11-02T03:21:40.0693307Z -    |             ^ value borrowed here after move
2019-11-02T03:21:40.0693363Z + LL | / impl<T: fmt::Display + ?Sized> ToString for T {
2019-11-02T03:21:40.0693411Z + LL | |     #[inline]
2019-11-02T03:21:40.0694041Z + LL | |     default fn to_string(&self) -> String {
2019-11-02T03:21:40.0694119Z + LL | |         use fmt::Write;
2019-11-02T03:21:40.0694162Z + ...  |
2019-11-02T03:21:40.0694204Z + LL | |     }
2019-11-02T03:21:40.0694262Z + LL | | }
2019-11-02T03:21:40.0694342Z 11 
2019-11-02T03:21:40.0694590Z - error: aborting due to previous error
2019-11-02T03:21:40.0694655Z + error[E0038]: the trait `std::string::ToString` cannot be made into an object
2019-11-02T03:21:40.0694909Z +   --> $DIR/use-after-move-implicity-coerced-object.rs:16:5
2019-11-02T03:21:40.0694909Z +   --> $DIR/use-after-move-implicity-coerced-object.rs:16:5
2019-11-02T03:21:40.0694972Z +    |
2019-11-02T03:21:40.0695205Z + LL |     list: Vec<Box<dyn ToString + 'static>> }
2019-11-02T03:21:40.0695311Z 13 
2019-11-02T03:21:40.0695579Z - For more information about this error, try `rustc --explain E0382`.
2019-11-02T03:21:40.0695636Z + error[E0038]: the trait `std::string::ToString` cannot be made into an object
2019-11-02T03:21:40.0695894Z +   --> $DIR/use-after-move-implicity-coerced-object.rs:19:5
2019-11-02T03:21:40.0695894Z +   --> $DIR/use-after-move-implicity-coerced-object.rs:19:5
2019-11-02T03:21:40.0695964Z +    |
2019-11-02T03:21:40.0696211Z + LL |     fn push(&mut self, n: Box<dyn ToString + 'static>) {
2019-11-02T03:21:40.0696344Z + 
2019-11-02T03:21:40.0696388Z + error: aborting due to 2 previous errors
2019-11-02T03:21:40.0696429Z + 
2019-11-02T03:21:40.0696698Z + For more information about this error, try `rustc --explain E0038`.
2019-11-02T03:21:40.0696698Z + For more information about this error, try `rustc --explain E0038`.
2019-11-02T03:21:40.0696747Z 15 
2019-11-02T03:21:40.0696776Z 
2019-11-02T03:21:40.0696802Z 
2019-11-02T03:21:40.0696864Z The actual stderr differed from the expected stderr.
2019-11-02T03:21:40.0697216Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/use/use-after-move-implicity-coerced-object/use-after-move-implicity-coerced-object.stderr
2019-11-02T03:21:40.0697481Z To update references, rerun the tests and pass the `--bless` flag
2019-11-02T03:21:40.0697963Z To only update this specific test, also pass `--test-args use/use-after-move-implicity-coerced-object.rs`
2019-11-02T03:21:40.0698056Z error: 1 errors occurred comparing output.
2019-11-02T03:21:40.0698115Z status: exit code: 1
2019-11-02T03:21:40.0698115Z status: exit code: 1
2019-11-02T03:21:40.0698991Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/use/use-after-move-implicity-coerced-object.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/use/use-after-move-implicity-coerced-object" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/use/use-after-move-implicity-coerced-object/auxiliary" "-A" "unused"
2019-11-02T03:21:40.0699366Z ------------------------------------------
2019-11-02T03:21:40.0699401Z 
2019-11-02T03:21:40.0699643Z ------------------------------------------
2019-11-02T03:21:40.0699689Z stderr:
2019-11-02T03:21:40.0699689Z stderr:
2019-11-02T03:21:40.0699910Z ------------------------------------------
2019-11-02T03:21:40.0699975Z warning: impl_potentially_overlapping_dyn_trait
2019-11-02T03:21:40.0700262Z    |
2019-11-02T03:21:40.0700262Z    |
2019-11-02T03:21:40.0700308Z LL | / impl<T: fmt::Display + ?Sized> ToString for T {
2019-11-02T03:21:40.0700370Z LL | |     #[inline]
2019-11-02T03:21:40.0700604Z LL | |     default fn to_string(&self) -> String {
2019-11-02T03:21:40.0700653Z LL | |         use fmt::Write;
2019-11-02T03:21:40.0700751Z LL | |     }
2019-11-02T03:21:40.0700791Z LL | | }
2019-11-02T03:21:40.0700847Z    | |_^
2019-11-02T03:21:40.0700873Z 
2019-11-02T03:21:40.0700873Z 
2019-11-02T03:21:40.0700921Z error[E0038]: the trait `std::string::ToString` cannot be made into an object
2019-11-02T03:21:40.0701200Z   --> /checkout/src/test/ui/use/use-after-move-implicity-coerced-object.rs:16:5
2019-11-02T03:21:40.0701266Z    |
2019-11-02T03:21:40.0701496Z LL |     list: Vec<Box<dyn ToString + 'static>> }
2019-11-02T03:21:40.0701615Z 
2019-11-02T03:21:40.0701664Z error[E0038]: the trait `std::string::ToString` cannot be made into an object
2019-11-02T03:21:40.0701928Z   --> /checkout/src/test/ui/use/use-after-move-implicity-coerced-object.rs:19:5
2019-11-02T03:21:40.0701992Z    |
2019-11-02T03:21:40.0701992Z    |
2019-11-02T03:21:40.0702232Z LL |     fn push(&mut self, n: Box<dyn ToString + 'static>) {
2019-11-02T03:21:40.0702344Z 
2019-11-02T03:21:40.0702388Z error: aborting due to 2 previous errors
2019-11-02T03:21:40.0702424Z 
2019-11-02T03:21:40.0702677Z For more information about this error, try `rustc --explain E0038`.
---
2019-11-02T03:21:40.0722392Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-02T03:21:40.0722451Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-02T03:21:40.0722531Z 
2019-11-02T03:21:40.0722577Z 
2019-11-02T03:21:40.0724995Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-02T03:21:40.0725283Z 
2019-11-02T03:21:40.0725313Z 
2019-11-02T03:21:40.0737633Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-02T03:21:40.0737736Z Build completed unsuccessfully in 1:06:50
2019-11-02T03:21:40.0737736Z Build completed unsuccessfully in 1:06:50
2019-11-02T03:21:40.0789120Z == clock drift check ==
2019-11-02T03:21:40.0805616Z   local time: Sat Nov  2 03:21:40 UTC 2019
2019-11-02T03:21:40.3595299Z   network time: Sat, 02 Nov 2019 03:21:40 GMT
2019-11-02T03:21:40.3601238Z == end clock drift check ==
2019-11-02T03:21:41.5383087Z 
2019-11-02T03:21:41.5507587Z ##[error]Bash exited with code '1'.
2019-11-02T03:21:41.5567774Z ##[section]Starting: Checkout
2019-11-02T03:21:41.5569645Z ==============================================================================
2019-11-02T03:21:41.5569708Z Task         : Get sources
2019-11-02T03:21:41.5569773Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
