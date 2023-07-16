
2020-07-13T21:53:24.5772669Z failures:
2020-07-13T21:53:24.5880703Z 
2020-07-13T21:53:24.5882233Z ---- [ui (nll)] ui/kindck/kindck-impl-type-params.rs stdout ----
2020-07-13T21:53:24.5882642Z diff of stderr:
2020-07-13T21:53:24.5882803Z 
2020-07-13T21:53:24.5883062Z 4	LL |     let a = &t as &dyn Gettable<T>;
2020-07-13T21:53:24.5883808Z 5	   |             ^^ `T` cannot be sent between threads safely
2020-07-13T21:53:24.5884084Z 6	   |
2020-07-13T21:53:24.5890649Z -	   = help: the trait `std::marker::Send` is not implemented for `T`
2020-07-13T21:53:24.5891497Z 8	   = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
2020-07-13T21:53:24.5891844Z 9	   = note: required for the cast to the object type `dyn Gettable<T>`
2020-07-13T21:53:24.5892159Z 10	help: consider restricting type parameter `T`
2020-07-13T21:53:24.5892326Z 
2020-07-13T21:53:24.5892575Z 31	LL |     let a: &dyn Gettable<T> = &t;
2020-07-13T21:53:24.5892867Z 32	   |                               ^^ `T` cannot be sent between threads safely
2020-07-13T21:53:24.5893144Z 33	   |
2020-07-13T21:53:24.5894121Z -	   = help: the trait `std::marker::Send` is not implemented for `T`
2020-07-13T21:53:24.5894460Z 35	   = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
2020-07-13T21:53:24.5894778Z 36	   = note: required for the cast to the object type `dyn Gettable<T>`
2020-07-13T21:53:24.5895082Z 37	help: consider restricting type parameter `T`
2020-07-13T21:53:24.5895248Z 
2020-07-13T21:53:24.5895366Z 
2020-07-13T21:53:24.5895620Z The actual stderr differed from the expected stderr.
2020-07-13T21:53:24.5896440Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-impl-type-params.nll/kindck-impl-type-params.nll.stderr
2020-07-13T21:53:24.5897133Z To update references, rerun the tests and pass the `--bless` flag
2020-07-13T21:53:24.5897807Z To only update this specific test, also pass `--test-args kindck/kindck-impl-type-params.rs`
2020-07-13T21:53:24.5897987Z 
2020-07-13T21:53:24.5898244Z error: 1 errors occurred comparing output.
2020-07-13T21:53:24.5898521Z status: exit code: 1
2020-07-13T21:53:24.5900416Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/kindck/kindck-impl-type-params.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-impl-type-params.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-impl-type-params.nll/auxiliary"
2020-07-13T21:53:24.5901106Z stdout:
2020-07-13T21:53:24.5901640Z ------------------------------------------
2020-07-13T21:53:24.5901808Z 
2020-07-13T21:53:24.5902313Z ------------------------------------------
2020-07-13T21:53:24.5902578Z stderr:
2020-07-13T21:53:24.5903300Z ------------------------------------------
2020-07-13T21:53:24.5903596Z error[E0277]: `T` cannot be sent between threads safely
2020-07-13T21:53:24.5904216Z   --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:18:13
2020-07-13T21:53:24.5904499Z    |
2020-07-13T21:53:24.5904748Z LL |     let a = &t as &dyn Gettable<T>;
2020-07-13T21:53:24.5905052Z    |             ^^ `T` cannot be sent between threads safely
2020-07-13T21:53:24.5905320Z    |
2020-07-13T21:53:24.5905595Z    = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
2020-07-13T21:53:24.5905928Z    = note: required for the cast to the object type `dyn Gettable<T>`
2020-07-13T21:53:24.5906228Z help: consider restricting type parameter `T`
2020-07-13T21:53:24.5906483Z    |
2020-07-13T21:53:24.5906805Z LL | fn f<T: std::marker::Send>(val: T) {
2020-07-13T21:53:24.5907061Z    |       ^^^^^^^^^^^^^^^^^^^
2020-07-13T21:53:24.5907219Z 
2020-07-13T21:53:24.5907541Z error[E0277]: the trait bound `T: std::marker::Copy` is not satisfied
2020-07-13T21:53:24.5908172Z   --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:18:13
2020-07-13T21:53:24.5908451Z    |
2020-07-13T21:53:24.5908693Z LL |     let a = &t as &dyn Gettable<T>;
2020-07-13T21:53:24.5909051Z    |             ^^ the trait `std::marker::Copy` is not implemented for `T`
2020-07-13T21:53:24.5909331Z    |
2020-07-13T21:53:24.5909608Z    = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
2020-07-13T21:53:24.5909932Z    = note: required for the cast to the object type `dyn Gettable<T>`
2020-07-13T21:53:24.5910358Z help: consider restricting type parameter `T`
2020-07-13T21:53:24.5910630Z    |
2020-07-13T21:53:24.5910950Z LL | fn f<T: std::marker::Copy>(val: T) {
2020-07-13T21:53:24.5911211Z    |       ^^^^^^^^^^^^^^^^^^^
2020-07-13T21:53:24.5911352Z 
2020-07-13T21:53:24.5911606Z error[E0277]: `T` cannot be sent between threads safely
2020-07-13T21:53:24.5912249Z   --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:25:31
2020-07-13T21:53:24.5912527Z    |
2020-07-13T21:53:24.5912771Z LL |     let a: &dyn Gettable<T> = &t;
2020-07-13T21:53:24.5913059Z    |                               ^^ `T` cannot be sent between threads safely
2020-07-13T21:53:24.5913358Z    |
2020-07-13T21:53:24.5913631Z    = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
2020-07-13T21:53:24.5913950Z    = note: required for the cast to the object type `dyn Gettable<T>`
2020-07-13T21:53:24.5914240Z help: consider restricting type parameter `T`
2020-07-13T21:53:24.5914493Z    |
2020-07-13T21:53:24.5914805Z LL | fn g<T: std::marker::Send>(val: T) {
2020-07-13T21:53:24.5915071Z    |       ^^^^^^^^^^^^^^^^^^^
2020-07-13T21:53:24.5915226Z 
2020-07-13T21:53:24.5915538Z error[E0277]: the trait bound `T: std::marker::Copy` is not satisfied
2020-07-13T21:53:24.5916176Z   --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:25:31
2020-07-13T21:53:24.5916447Z    |
2020-07-13T21:53:24.5916688Z LL |     let a: &dyn Gettable<T> = &t;
2020-07-13T21:53:24.5917051Z    |                               ^^ the trait `std::marker::Copy` is not implemented for `T`
2020-07-13T21:53:24.5917368Z    |
2020-07-13T21:53:24.5917642Z    = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
2020-07-13T21:53:24.5917954Z    = note: required for the cast to the object type `dyn Gettable<T>`
2020-07-13T21:53:24.5918243Z help: consider restricting type parameter `T`
2020-07-13T21:53:24.5918502Z    |
2020-07-13T21:53:24.5918800Z LL | fn g<T: std::marker::Copy>(val: T) {
2020-07-13T21:53:24.5919071Z    |       ^^^^^^^^^^^^^^^^^^^
2020-07-13T21:53:24.5919227Z 
2020-07-13T21:53:24.5919673Z error[E0277]: the trait bound `std::string::String: std::marker::Copy` is not satisfied
2020-07-13T21:53:24.5920342Z   --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:38:13
2020-07-13T21:53:24.5920620Z    |
2020-07-13T21:53:24.5920874Z LL |     let a = t as Box<dyn Gettable<String>>;
2020-07-13T21:53:24.5921242Z    |             ^ the trait `std::marker::Copy` is not implemented for `std::string::String`
2020-07-13T21:53:24.5921544Z    |
2020-07-13T21:53:24.5922242Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:344:22
2020-07-13T21:53:24.5922570Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-07-13T21:53:24.5922987Z    = note: required because of the requirements on the impl of `Gettable<std::string::String>` for `S<std::string::String>`
2020-07-13T21:53:24.5923411Z    = note: required for the cast to the object type `dyn Gettable<std::string::String>`
2020-07-13T21:53:24.5923605Z 
2020-07-13T21:53:24.5924222Z error[E0277]: the trait bound `foo3::Foo: std::marker::Copy` is not satisfied
2020-07-13T21:53:24.5924887Z   --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:46:37
2020-07-13T21:53:24.5925161Z    |
2020-07-13T21:53:24.5925409Z LL |     let a: Box<dyn Gettable<Foo>> = t;
2020-07-13T21:53:24.5925806Z    |                                     ^ the trait `std::marker::Copy` is not implemented for `foo3::Foo`
2020-07-13T21:53:24.5926091Z    |
2020-07-13T21:53:24.5926423Z    = note: required because of the requirements on the impl of `Gettable<foo3::Foo>` for `S<foo3::Foo>`
2020-07-13T21:53:24.5926804Z    = note: required for the cast to the object type `dyn Gettable<foo3::Foo>`
2020-07-13T21:53:24.5926972Z 
2020-07-13T21:53:24.5927220Z error: aborting due to 6 previous errors
2020-07-13T21:53:24.5927380Z 
2020-07-13T21:53:24.5927976Z For more information about this error, try `rustc --explain E0277`.
2020-07-13T21:53:24.5928266Z 
2020-07-13T21:53:24.5928794Z ------------------------------------------
