plain
2020-01-29T01:26:41.5190140Z 
2020-01-29T01:26:41.5190290Z 1 error[E0277]: `T` cannot be sent between threads safely
2020-01-29T01:26:41.5190733Z 2   --> $DIR/kindck-impl-type-params.rs:18:13
2020-01-29T01:26:41.5190934Z 3    |
2020-01-29T01:26:41.5191299Z - LL | fn f<T>(val: T) {
2020-01-29T01:26:41.5191929Z -    |      - help: consider restricting this bound: `T: std::marker::Send`
2020-01-29T01:26:41.5192483Z - LL |     let t: S<T> = S(marker::PhantomData);
2020-01-29T01:26:41.5192714Z 7 LL |     let a = &t as &dyn Gettable<T>;
2020-01-29T01:26:41.5192869Z 8    |             ^^ `T` cannot be sent between threads safely
2020-01-29T01:26:41.5193336Z 
2020-01-29T01:26:41.5193659Z 10    = help: the trait `std::marker::Send` is not implemented for `T`
2020-01-29T01:26:41.5193659Z 10    = help: the trait `std::marker::Send` is not implemented for `T`
2020-01-29T01:26:41.5193928Z + help: consider restricting this type parameter with `T: std::marker::Send`
2020-01-29T01:26:41.5194384Z +   --> $DIR/kindck-impl-type-params.rs:16:6
2020-01-29T01:26:41.5194594Z +    |
2020-01-29T01:26:41.5194724Z + LL | fn f<T>(val: T) {
2020-01-29T01:26:41.5194873Z +    |      ^
2020-01-29T01:26:41.5195036Z 11    = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
2020-01-29T01:26:41.5195200Z 12    = note: required for the cast to the object type `dyn Gettable<T>`
2020-01-29T01:26:41.5195463Z 
2020-01-29T01:26:41.5195630Z 14 error[E0277]: the trait bound `T: std::marker::Copy` is not satisfied
2020-01-29T01:26:41.5196001Z 15   --> $DIR/kindck-impl-type-params.rs:18:13
2020-01-29T01:26:41.5196201Z 16    |
2020-01-29T01:26:41.5196201Z 16    |
2020-01-29T01:26:41.5196510Z - LL | fn f<T>(val: T) {
2020-01-29T01:26:41.5196926Z -    |      - help: consider restricting this bound: `T: std::marker::Copy`
2020-01-29T01:26:41.5197342Z - LL |     let t: S<T> = S(marker::PhantomData);
2020-01-29T01:26:41.5197532Z 20 LL |     let a = &t as &dyn Gettable<T>;
2020-01-29T01:26:41.5197700Z 21    |             ^^ the trait `std::marker::Copy` is not implemented for `T`
2020-01-29T01:26:41.5197966Z 
2020-01-29T01:26:41.5197966Z 
2020-01-29T01:26:41.5198108Z + help: consider restricting this type parameter with `T: std::marker::Copy`
2020-01-29T01:26:41.5198469Z +   --> $DIR/kindck-impl-type-params.rs:16:6
2020-01-29T01:26:41.5198666Z +    |
2020-01-29T01:26:41.5198798Z + LL | fn f<T>(val: T) {
2020-01-29T01:26:41.5198945Z +    |      ^
2020-01-29T01:26:41.5199306Z 23    = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
2020-01-29T01:26:41.5199545Z 24    = note: required for the cast to the object type `dyn Gettable<T>`
2020-01-29T01:26:41.5199814Z 
2020-01-29T01:26:41.5199964Z 26 error[E0277]: `T` cannot be sent between threads safely
2020-01-29T01:26:41.5200365Z 27   --> $DIR/kindck-impl-type-params.rs:25:31
2020-01-29T01:26:41.5200570Z 28    |
2020-01-29T01:26:41.5200570Z 28    |
2020-01-29T01:26:41.5200876Z - LL | fn g<T>(val: T) {
2020-01-29T01:26:41.5201293Z -    |      - help: consider restricting this bound: `T: std::marker::Send`
2020-01-29T01:26:41.5201672Z - LL |     let t: S<T> = S(marker::PhantomData);
2020-01-29T01:26:41.5201876Z 32 LL |     let a: &dyn Gettable<T> = &t;
2020-01-29T01:26:41.5202046Z 33    |                               ^^ `T` cannot be sent between threads safely
2020-01-29T01:26:41.5202315Z 
2020-01-29T01:26:41.5202478Z 35    = help: the trait `std::marker::Send` is not implemented for `T`
2020-01-29T01:26:41.5202478Z 35    = help: the trait `std::marker::Send` is not implemented for `T`
2020-01-29T01:26:41.5202661Z + help: consider restricting this type parameter with `T: std::marker::Send`
2020-01-29T01:26:41.5203020Z +   --> $DIR/kindck-impl-type-params.rs:23:6
2020-01-29T01:26:41.5203485Z +    |
2020-01-29T01:26:41.5203654Z + LL | fn g<T>(val: T) {
2020-01-29T01:26:41.5203981Z +    |      ^
2020-01-29T01:26:41.5204150Z 36    = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
2020-01-29T01:26:41.5204310Z 37    = note: required for the cast to the object type `dyn Gettable<T>`
2020-01-29T01:26:41.5204571Z 
2020-01-29T01:26:41.5204727Z 39 error[E0277]: the trait bound `T: std::marker::Copy` is not satisfied
2020-01-29T01:26:41.5205123Z 40   --> $DIR/kindck-impl-type-params.rs:25:31
2020-01-29T01:26:41.5205337Z 41    |
2020-01-29T01:26:41.5205337Z 41    |
2020-01-29T01:26:41.5205662Z - LL | fn g<T>(val: T) {
2020-01-29T01:26:41.5206222Z -    |      - help: consider restricting this bound: `T: std::marker::Copy`
2020-01-29T01:26:41.5206866Z - LL |     let t: S<T> = S(marker::PhantomData);
2020-01-29T01:26:41.5207064Z 45 LL |     let a: &dyn Gettable<T> = &t;
2020-01-29T01:26:41.5207239Z 46    |                               ^^ the trait `std::marker::Copy` is not implemented for `T`
2020-01-29T01:26:41.5207525Z 
2020-01-29T01:26:41.5207525Z 
2020-01-29T01:26:41.5207681Z + help: consider restricting this type parameter with `T: std::marker::Copy`
2020-01-29T01:26:41.5208022Z +   --> $DIR/kindck-impl-type-params.rs:23:6
2020-01-29T01:26:41.5208220Z +    |
2020-01-29T01:26:41.5208346Z + LL | fn g<T>(val: T) {
2020-01-29T01:26:41.5208492Z +    |      ^
2020-01-29T01:26:41.5208654Z 48    = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
2020-01-29T01:26:41.5208807Z 49    = note: required for the cast to the object type `dyn Gettable<T>`
2020-01-29T01:26:41.5209058Z 
2020-01-29T01:26:41.5209159Z 
2020-01-29T01:26:41.5209309Z The actual stderr differed from the expected stderr.
2020-01-29T01:26:41.5209309Z The actual stderr differed from the expected stderr.
2020-01-29T01:26:41.5209772Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-impl-type-params.nll/kindck-impl-type-params.nll.stderr
2020-01-29T01:26:41.5210189Z To update references, rerun the tests and pass the `--bless` flag
2020-01-29T01:26:41.5211165Z To only update this specific test, also pass `--test-args kindck/kindck-impl-type-params.rs`
2020-01-29T01:26:41.5213146Z error: 1 errors occurred comparing output.
2020-01-29T01:26:41.5213367Z status: exit code: 1
2020-01-29T01:26:41.5213367Z status: exit code: 1
2020-01-29T01:26:41.5215873Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/kindck/kindck-impl-type-params.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-impl-type-params.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-impl-type-params.nll/auxiliary" "-A" "unused"
2020-01-29T01:26:41.5218613Z ------------------------------------------
2020-01-29T01:26:41.5218866Z 
2020-01-29T01:26:41.5219076Z ------------------------------------------
2020-01-29T01:26:41.5219190Z stderr:
2020-01-29T01:26:41.5219190Z stderr:
2020-01-29T01:26:41.5219387Z ------------------------------------------
2020-01-29T01:26:41.5219472Z error[E0277]: `T` cannot be sent between threads safely
2020-01-29T01:26:41.5219714Z   --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:18:13
2020-01-29T01:26:41.5219800Z    |
2020-01-29T01:26:41.5219853Z LL |     let a = &t as &dyn Gettable<T>;
2020-01-29T01:26:41.5219941Z    |             ^^ `T` cannot be sent between threads safely
2020-01-29T01:26:41.5220082Z    = help: the trait `std::marker::Send` is not implemented for `T`
2020-01-29T01:26:41.5220182Z help: consider restricting this type parameter with `T: std::marker::Send`
2020-01-29T01:26:41.5220437Z   --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:16:6
2020-01-29T01:26:41.5220706Z    |
2020-01-29T01:26:41.5220706Z    |
2020-01-29T01:26:41.5220755Z LL | fn f<T>(val: T) {
2020-01-29T01:26:41.5222324Z    |      ^
2020-01-29T01:26:41.5222390Z    = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
2020-01-29T01:26:41.5222488Z    = note: required for the cast to the object type `dyn Gettable<T>`
2020-01-29T01:26:41.5222608Z error[E0277]: the trait bound `T: std::marker::Copy` is not satisfied
2020-01-29T01:26:41.5222921Z   --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:18:13
2020-01-29T01:26:41.5223003Z    |
2020-01-29T01:26:41.5223003Z    |
2020-01-29T01:26:41.5223072Z LL |     let a = &t as &dyn Gettable<T>;
2020-01-29T01:26:41.5223140Z    |             ^^ the trait `std::marker::Copy` is not implemented for `T`
2020-01-29T01:26:41.5223469Z help: consider restricting this type parameter with `T: std::marker::Copy`
2020-01-29T01:26:41.5224418Z   --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:16:6
2020-01-29T01:26:41.5224495Z    |
2020-01-29T01:26:41.5224495Z    |
2020-01-29T01:26:41.5224780Z LL | fn f<T>(val: T) {
2020-01-29T01:26:41.5225010Z    |      ^
2020-01-29T01:26:41.5225100Z    = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
2020-01-29T01:26:41.5225185Z    = note: required for the cast to the object type `dyn Gettable<T>`
2020-01-29T01:26:41.5225822Z error[E0277]: `T` cannot be sent between threads safely
2020-01-29T01:26:41.5226345Z   --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:25:31
2020-01-29T01:26:41.5226420Z    |
2020-01-29T01:26:41.5226420Z    |
2020-01-29T01:26:41.5226492Z LL |     let a: &dyn Gettable<T> = &t;
2020-01-29T01:26:41.5226569Z    |                               ^^ `T` cannot be sent between threads safely
2020-01-29T01:26:41.5226849Z    = help: the trait `std::marker::Send` is not implemented for `T`
2020-01-29T01:26:41.5226933Z help: consider restricting this type parameter with `T: std::marker::Send`
2020-01-29T01:26:41.5227220Z   --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:23:6
2020-01-29T01:26:41.5227300Z    |
2020-01-29T01:26:41.5227300Z    |
2020-01-29T01:26:41.5227374Z LL | fn g<T>(val: T) {
2020-01-29T01:26:41.5227434Z    |      ^
2020-01-29T01:26:41.5227524Z    = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
2020-01-29T01:26:41.5227611Z    = note: required for the cast to the object type `dyn Gettable<T>`
2020-01-29T01:26:41.5227743Z error[E0277]: the trait bound `T: std::marker::Copy` is not satisfied
2020-01-29T01:26:41.5228223Z   --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:25:31
2020-01-29T01:26:41.5228457Z    |
2020-01-29T01:26:41.5228457Z    |
2020-01-29T01:26:41.5228530Z LL |     let a: &dyn Gettable<T> = &t;
2020-01-29T01:26:41.5228612Z    |                               ^^ the trait `std::marker::Copy` is not implemented for `T`
2020-01-29T01:26:41.5229308Z help: consider restricting this type parameter with `T: std::marker::Copy`
2020-01-29T01:26:41.5230000Z   --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:23:6
2020-01-29T01:26:41.5230555Z    |
2020-01-29T01:26:41.5230555Z    |
2020-01-29T01:26:41.5230608Z LL | fn g<T>(val: T) {
2020-01-29T01:26:41.5230684Z    |      ^
2020-01-29T01:26:41.5230752Z    = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
2020-01-29T01:26:41.5230860Z    = note: required for the cast to the object type `dyn Gettable<T>`
2020-01-29T01:26:41.5230990Z error[E0277]: the trait bound `std::string::String: std::marker::Copy` is not satisfied
2020-01-29T01:26:41.5231438Z   --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:38:13
2020-01-29T01:26:41.5231793Z    |
2020-01-29T01:26:41.5231793Z    |
2020-01-29T01:26:41.5231848Z LL |     let a = t as Box<dyn Gettable<String>>;
2020-01-29T01:26:41.5231949Z    |             ^ the trait `std::marker::Copy` is not implemented for `std::string::String`
2020-01-29T01:26:41.5232016Z    |
2020-01-29T01:26:41.5232106Z    = note: required because of the requirements on the impl of `Gettable<std::string::String>` for `S<std::string::String>`
2020-01-29T01:26:41.5232301Z    = note: required for the cast to the object type `dyn Gettable<std::string::String>`
2020-01-29T01:26:41.5232352Z 
2020-01-29T01:26:41.5232443Z error[E0277]: the trait bound `foo3::Foo: std::marker::Copy` is not satisfied
2020-01-29T01:26:41.5232708Z   --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:46:37
2020-01-29T01:26:41.5232791Z    |
2020-01-29T01:26:41.5232842Z LL |     let a: Box<dyn Gettable<Foo>> = t;
2020-01-29T01:26:41.5232935Z    |                                     ^ the trait `std::marker::Copy` is not implemented for `foo3::Foo`
2020-01-29T01:26:41.5233189Z    |
2020-01-29T01:26:41.5233376Z    = note: required because of the requirements on the impl of `Gettable<foo3::Foo>` for `S<foo3::Foo>`
2020-01-29T01:26:41.5233469Z    = note: required for the cast to the object type `dyn Gettable<foo3::Foo>`
2020-01-29T01:26:41.5233585Z error: aborting due to 6 previous errors
2020-01-29T01:26:41.5233641Z 
2020-01-29T01:26:41.5234215Z For more information about this error, try `rustc --explain E0277`.
2020-01-29T01:26:41.5234269Z 
---
2020-01-29T01:26:41.5236260Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-01-29T01:26:41.5236356Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-29T01:26:41.5236435Z 
2020-01-29T01:26:41.5236469Z 
2020-01-29T01:26:41.5239368Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2020-01-29T01:26:41.5240760Z 
2020-01-29T01:26:41.5240796Z 
2020-01-29T01:26:41.5241024Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-29T01:26:41.5241296Z Build completed unsuccessfully in 1:37:29
2020-01-29T01:26:41.5241296Z Build completed unsuccessfully in 1:37:29
2020-01-29T01:26:41.5242013Z == clock drift check ==
2020-01-29T01:26:41.5242780Z   local time: Wed Jan 29 01:26:40 UTC 2020
2020-01-29T01:26:41.5243046Z   network time: Wed, 29 Jan 2020 01:26:41 GMT
2020-01-29T01:26:41.5243117Z == end clock drift check ==
2020-01-29T01:26:41.9437582Z 
2020-01-29T01:26:41.9530885Z ##[error]Bash exited with code '1'.
2020-01-29T01:26:41.9579116Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-01-29T01:26:41.9582186Z ==============================================================================
2020-01-29T01:26:41.9582269Z Task         : Get sources
2020-01-29T01:26:41.9582538Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
