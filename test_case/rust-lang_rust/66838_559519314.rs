plain
2019-11-28T13:29:11.9457469Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-28T13:29:11.9671289Z ##[command]git config gc.auto 0
2019-11-28T13:29:11.9754916Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-28T13:29:11.9819657Z ##[command]git config --get-all http.proxy
2019-11-28T13:29:11.9974490Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66838/merge:refs/remotes/pull/66838/merge
---
2019-11-28T14:30:35.2718949Z .................................................................................................... 1600/9303
2019-11-28T14:30:40.3008313Z .................................................................................................... 1700/9303
2019-11-28T14:30:53.3853580Z ...................................i................................................................ 1800/9303
2019-11-28T14:31:01.5063117Z .................................................................................................... 1900/9303
2019-11-28T14:31:16.0162908Z ....................iiiii........................................................................... 2000/9303
2019-11-28T14:31:26.5843190Z .................................................................................................... 2200/9303
2019-11-28T14:31:29.2802251Z .................................................................................................... 2300/9303
2019-11-28T14:31:34.3835429Z .................................................................................................... 2400/9303
2019-11-28T14:31:56.7156223Z .................................................................................................... 2500/9303
---
2019-11-28T14:34:44.9833305Z .....................i...............i.............................................................. 4800/9303
2019-11-28T14:34:55.8572567Z .................................................................................................... 4900/9303
2019-11-28T14:35:01.9740497Z .................................................................................................... 5000/9303
2019-11-28T14:35:10.9687567Z .................................................................................................... 5100/9303
2019-11-28T14:35:18.8258709Z ..........................ii.ii...........i......................................................... 5200/9303
2019-11-28T14:35:28.7135992Z .................................................................................................... 5400/9303
2019-11-28T14:35:39.9009948Z .................................................................................................... 5500/9303
2019-11-28T14:35:47.3748218Z ........i........................................................................................... 5600/9303
2019-11-28T14:35:54.0660069Z .................................................................................................... 5700/9303
2019-11-28T14:35:54.0660069Z .................................................................................................... 5700/9303
2019-11-28T14:36:05.3276740Z ..............................................................................................ii...i 5800/9303
2019-11-28T14:36:18.8441007Z ..ii...........i.................................................................................... 5900/9303
2019-11-28T14:36:38.1673976Z .................................................................................................... 6100/9303
2019-11-28T14:36:44.6096813Z .................................................................................................... 6200/9303
2019-11-28T14:36:44.6096813Z .................................................................................................... 6200/9303
2019-11-28T14:36:59.3088443Z .................i..ii.............................................................................. 6300/9303
2019-11-28T14:37:19.8113316Z .....................................................................................i.............. 6500/9303
2019-11-28T14:37:22.3367059Z .................................................................................................... 6600/9303
2019-11-28T14:37:24.7075542Z ............................................................................i....................... 6700/9303
2019-11-28T14:37:27.7057082Z .................................................................................................... 6800/9303
---
2019-11-28T14:42:30.7125124Z error: /checkout/src/test/ui/associated-types/associated-types-overridden-binding.rs:7: expected error not found: type annotations needed
2019-11-28T14:42:30.7125332Z 
2019-11-28T14:42:30.7125497Z error: 0 unexpected errors found, 1 expected errors not found
2019-11-28T14:42:30.7125667Z status: exit code: 1
2019-11-28T14:42:30.7126898Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-overridden-binding.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding/auxiliary" "-A" "unused"
2019-11-28T14:42:30.7127160Z not found errors (from test file): [
2019-11-28T14:42:30.7127586Z         line_num: 7,
2019-11-28T14:42:30.7127905Z         kind: Some(
2019-11-28T14:42:30.7128043Z             Error,
2019-11-28T14:42:30.7128198Z         ),
---
2019-11-28T14:42:30.7129472Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-28T14:42:30.7129626Z 
2019-11-28T14:42:30.7130026Z ---- [ui] ui/issues/issue-20831-debruijn.rs stdout ----
2019-11-28T14:42:30.7130191Z 
2019-11-28T14:42:30.7130620Z error: /checkout/src/test/ui/issues/issue-20831-debruijn.rs:28: expected error not found: mismatched types
2019-11-28T14:42:30.7130783Z 
2019-11-28T14:42:30.7131219Z error: /checkout/src/test/ui/issues/issue-20831-debruijn.rs:28: expected error not found: mismatched types
2019-11-28T14:42:30.7132986Z error: 0 unexpected errors found, 2 expected errors not found
2019-11-28T14:42:30.7133033Z status: exit code: 1
2019-11-28T14:42:30.7133033Z status: exit code: 1
2019-11-28T14:42:30.7133939Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20831-debruijn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20831-debruijn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20831-debruijn/auxiliary" "-A" "unused"
2019-11-28T14:42:30.7134052Z not found errors (from test file): [
2019-11-28T14:42:30.7134146Z         line_num: 28,
2019-11-28T14:42:30.7134197Z         kind: Some(
2019-11-28T14:42:30.7134256Z             Error,
2019-11-28T14:42:30.7134297Z         ),
---
2019-11-28T14:42:30.7135550Z ---- [ui] ui/issues/issue-38091.rs stdout ----
2019-11-28T14:42:30.7135584Z 
2019-11-28T14:42:30.7135826Z error: test compilation failed although it shouldn't!
2019-11-28T14:42:30.7135886Z status: exit code: 101
2019-11-28T14:42:30.7136620Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-38091.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-38091/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-38091/auxiliary"
2019-11-28T14:42:30.7136950Z ------------------------------------------
2019-11-28T14:42:30.7136983Z 
2019-11-28T14:42:30.7137379Z ------------------------------------------
2019-11-28T14:42:30.7137453Z stderr:
2019-11-28T14:42:30.7137453Z stderr:
2019-11-28T14:42:30.7137736Z ------------------------------------------
2019-11-28T14:42:30.7138346Z error: internal compiler error: src/librustc/traits/codegen/mod.rs:127: Encountered errors `[FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<() as Valid>)), depth=2),Unimplemented)]` resolving bounds after type-checking
2019-11-28T14:42:30.7138654Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:892:9
2019-11-28T14:42:30.7138727Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-28T14:42:30.7138759Z 
2019-11-28T14:42:30.7138805Z note: the compiler unexpectedly panicked. this is a bug.
2019-11-28T14:42:30.7138805Z note: the compiler unexpectedly panicked. this is a bug.
2019-11-28T14:42:30.7138835Z 
2019-11-28T14:42:30.7139255Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-11-28T14:42:30.7139539Z note: rustc 1.41.0-dev running on x86_64-unknown-linux-gnu
2019-11-28T14:42:30.7139597Z 
2019-11-28T14:42:30.7139597Z 
2019-11-28T14:42:30.7139885Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-11-28T14:42:30.7139967Z error: aborting due to previous error
2019-11-28T14:42:30.7140109Z 
2019-11-28T14:42:30.7140134Z 
2019-11-28T14:42:30.7140380Z ------------------------------------------
2019-11-28T14:42:30.7140380Z ------------------------------------------
2019-11-28T14:42:30.7140412Z 
2019-11-28T14:42:30.7140437Z 
2019-11-28T14:42:30.7140703Z ---- [ui] ui/suggestions/missing-assoc-type-bound-restriction.rs stdout ----
2019-11-28T14:42:30.7140753Z diff of stderr:
2019-11-28T14:42:30.7140781Z 
2019-11-28T14:42:30.7140836Z 32    |
2019-11-28T14:42:30.7140891Z 33    = note: required because of the requirements on the impl of `Child<A>` for `ChildWrapper<<T as Parent>::Assoc>`
2019-11-28T14:42:30.7140940Z 34 
2019-11-28T14:42:30.7141388Z - error[E0277]: the trait bound `<T as Parent>::Assoc: Child<A>` is not satisfied
2019-11-28T14:42:30.7141805Z -    |
2019-11-28T14:42:30.7141805Z -    |
2019-11-28T14:42:30.7141991Z - LL | trait Parent {
2019-11-28T14:42:30.7142216Z -    | ------------ required by `Parent`
2019-11-28T14:42:30.7142388Z - ...
2019-11-28T14:42:30.7142617Z - LL | impl<A, T: Parent<Ty = A>> Parent for ParentWrapper<T> {
2019-11-28T14:42:30.7142975Z -    |                                                       - help: consider further restricting the associated type: `where <T as Parent>::Assoc: Child<A>`
2019-11-28T14:42:30.7143158Z - ...
2019-11-28T14:42:30.7143372Z - LL |     type Assoc = ChildWrapper<T::Assoc>;
2019-11-28T14:42:30.7143671Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Child<A>` is not implemented for `<T as Parent>::Assoc`
2019-11-28T14:42:30.7144058Z - error: aborting due to 3 previous errors
2019-11-28T14:42:30.7144121Z + error: aborting due to 2 previous errors
2019-11-28T14:42:30.7144162Z 48 
2019-11-28T14:42:30.7144472Z 49 For more information about this error, try `rustc --explain E0277`.
2019-11-28T14:42:30.7144472Z 49 For more information about this error, try `rustc --explain E0277`.
2019-11-28T14:42:30.7144544Z 50 
2019-11-28T14:42:30.7144570Z 
2019-11-28T14:42:30.7144595Z 
2019-11-28T14:42:30.7144639Z The actual stderr differed from the expected stderr.
2019-11-28T14:42:30.7145018Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-assoc-type-bound-restriction/missing-assoc-type-bound-restriction.stderr
2019-11-28T14:42:30.7145274Z To update references, rerun the tests and pass the `--bless` flag
2019-11-28T14:42:30.7145555Z To only update this specific test, also pass `--test-args suggestions/missing-assoc-type-bound-restriction.rs`
2019-11-28T14:42:30.7145650Z error: 1 errors occurred comparing output.
2019-11-28T14:42:30.7145693Z status: exit code: 1
2019-11-28T14:42:30.7145693Z status: exit code: 1
2019-11-28T14:42:30.7146523Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/missing-assoc-type-bound-restriction.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-assoc-type-bound-restriction" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-assoc-type-bound-restriction/auxiliary" "-A" "unused"
2019-11-28T14:42:30.7146853Z ------------------------------------------
2019-11-28T14:42:30.7146886Z 
2019-11-28T14:42:30.7147096Z ------------------------------------------
2019-11-28T14:42:30.7147141Z stderr:
2019-11-28T14:42:30.7147141Z stderr:
2019-11-28T14:42:30.7147364Z ------------------------------------------
2019-11-28T14:42:30.7147416Z error[E0277]: the trait bound `<T as Parent>::Assoc: Child<A>` is not satisfied
2019-11-28T14:42:30.7148062Z    |
2019-11-28T14:42:30.7148114Z LL |   trait Parent {
2019-11-28T14:42:30.7148330Z    |   ------------ required by `Parent`
2019-11-28T14:42:30.7148391Z ...
2019-11-28T14:42:30.7148391Z ...
2019-11-28T14:42:30.7148438Z LL |   impl<A, T: Parent<Ty = A>> Parent for ParentWrapper<T> {
2019-11-28T14:42:30.7148887Z    |   ^                                                     - help: consider further restricting the associated type: `where <T as Parent>::Assoc: Child<A>`
2019-11-28T14:42:30.7148963Z    |  _|
2019-11-28T14:42:30.7149005Z    | |
2019-11-28T14:42:30.7149055Z LL | |     //~^ ERROR the trait bound `<T as Parent>::Assoc: Child<A>` is not satisfied
2019-11-28T14:42:30.7149121Z LL | |     type Ty = A;
2019-11-28T14:42:30.7149167Z LL | |     type Assoc = ChildWrapper<T::Assoc>;
2019-11-28T14:42:30.7149219Z LL | |     //~^ ERROR the trait bound `<T as Parent>::Assoc: Child<A>` is not satisfied
2019-11-28T14:42:30.7149292Z LL | |     //~| ERROR the trait bound `<T as Parent>::Assoc: Child<A>` is not satisfied
2019-11-28T14:42:30.7149340Z LL | | }
2019-11-28T14:42:30.7149396Z    | |_^ the trait `Child<A>` is not implemented for `<T as Parent>::Assoc`
2019-11-28T14:42:30.7149429Z 
2019-11-28T14:42:30.7149495Z error[E0277]: the trait bound `<T as Parent>::Assoc: Child<A>` is not satisfied
2019-11-28T14:42:30.7149824Z    |
2019-11-28T14:42:30.7149824Z    |
2019-11-28T14:42:30.7149884Z LL |     type Assoc: Child<Self::Ty>;
2019-11-28T14:42:30.7150109Z    |          ----- associated type defined here
2019-11-28T14:42:30.7150154Z ...
2019-11-28T14:42:30.7150216Z LL | impl<A, T: Parent<Ty = A>> Parent for ParentWrapper<T> {
2019-11-28T14:42:30.7150552Z    | ------------------------------------------------------- help: consider further restricting the associated type: `where <T as Parent>::Assoc: Child<A>`
2019-11-28T14:42:30.7150665Z    | in this `impl` item
2019-11-28T14:42:30.7150705Z ...
2019-11-28T14:42:30.7150705Z ...
2019-11-28T14:42:30.7150841Z LL |     type Assoc = ChildWrapper<T::Assoc>;
2019-11-28T14:42:30.7150905Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Child<A>` is not implemented for `<T as Parent>::Assoc`
2019-11-28T14:42:30.7150970Z    |
2019-11-28T14:42:30.7151022Z    = note: required because of the requirements on the impl of `Child<A>` for `ChildWrapper<<T as Parent>::Assoc>`
2019-11-28T14:42:30.7151282Z error: aborting due to 2 previous errors
2019-11-28T14:42:30.7151311Z 
2019-11-28T14:42:30.7151579Z For more information about this error, try `rustc --explain E0277`.
2019-11-28T14:42:30.7151613Z 
2019-11-28T14:42:30.7151613Z 
2019-11-28T14:42:30.7151837Z ------------------------------------------
2019-11-28T14:42:30.7151868Z 
2019-11-28T14:42:30.7151892Z 
2019-11-28T14:42:30.7152124Z ---- [ui] ui/type-alias-impl-trait/bound_reduction2.rs stdout ----
2019-11-28T14:42:30.7152186Z diff of stderr:
2019-11-28T14:42:30.7152213Z 
2019-11-28T14:42:30.7152471Z + error[E0277]: the trait bound `T: TraitWithAssoc` is not satisfied
2019-11-28T14:42:30.7152865Z +   --> $DIR/bound_reduction2.rs:10:1
2019-11-28T14:42:30.7152926Z +    |
2019-11-28T14:42:30.7152968Z + LL | type Foo<V> = impl Trait<V>;
2019-11-28T14:42:30.7153019Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `TraitWithAssoc` is not implemented for `T`
2019-11-28T14:42:30.7153088Z + ...
2019-11-28T14:42:30.7153323Z + LL | fn foo_desugared<T: TraitWithAssoc>(_: T) -> Foo<T::Assoc> {
2019-11-28T14:42:30.7153589Z +    |                  -- help: consider further restricting this bound: `T: TraitWithAssoc +`
2019-11-28T14:42:30.7153652Z + 
2019-11-28T14:42:30.7153707Z 1 error: defining opaque type use does not fully define opaque type: generic parameter `V` is specified as concrete type `<T as TraitWithAssoc>::Assoc`
2019-11-28T14:42:30.7153983Z 3    |
2019-11-28T14:42:30.7154008Z 
2019-11-28T14:42:30.7154008Z 
2019-11-28T14:42:30.7154049Z 12 LL | type Foo<V> = impl Trait<V>;
2019-11-28T14:42:30.7154151Z 14 
2019-11-28T14:42:30.7154366Z - error: aborting due to 2 previous errors
2019-11-28T14:42:30.7154415Z + error: aborting due to 3 previous errors
2019-11-28T14:42:30.7154470Z 16 
2019-11-28T14:42:30.7154470Z 16 
2019-11-28T14:42:30.7154709Z + For more information about this error, try `rustc --explain E0277`.
2019-11-28T14:42:30.7154835Z 17 
2019-11-28T14:42:30.7154861Z 
2019-11-28T14:42:30.7154902Z 
2019-11-28T14:42:30.7154946Z The actual stderr differed from the expected stderr.
2019-11-28T14:42:30.7155280Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/bound_reduction2/bound_reduction2.stderr
2019-11-28T14:42:30.7155525Z To update references, rerun the tests and pass the `--bless` flag
2019-11-28T14:42:30.7155853Z To only update this specific test, also pass `--test-args type-alias-impl-trait/bound_reduction2.rs`
2019-11-28T14:42:30.7155937Z error: 1 errors occurred comparing output.
2019-11-28T14:42:30.7155999Z status: exit code: 1
2019-11-28T14:42:30.7155999Z status: exit code: 1
2019-11-28T14:42:30.7156806Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/bound_reduction2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/bound_reduction2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/bound_reduction2/auxiliary" "-A" "unused"
2019-11-28T14:42:30.7157463Z ------------------------------------------
2019-11-28T14:42:30.7157499Z 
2019-11-28T14:42:30.7158278Z ------------------------------------------
2019-11-28T14:42:30.7158334Z stderr:
2019-11-28T14:42:30.7158334Z stderr:
2019-11-28T14:42:30.7158558Z ------------------------------------------
2019-11-28T14:42:30.7158627Z error[E0277]: the trait bound `T: TraitWithAssoc` is not satisfied
2019-11-28T14:42:30.7159073Z    |
2019-11-28T14:42:30.7159073Z    |
2019-11-28T14:42:30.7159135Z LL | type Foo<V> = impl Trait<V>;
2019-11-28T14:42:30.7159188Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `TraitWithAssoc` is not implemented for `T`
2019-11-28T14:42:30.7159245Z ...
2019-11-28T14:42:30.7159575Z LL | fn foo_desugared<T: TraitWithAssoc>(_: T) -> Foo<T::Assoc> { //~ ERROR does not fully define
2019-11-28T14:42:30.7159854Z    |                  -- help: consider further restricting this bound: `T: TraitWithAssoc +`
2019-11-28T14:42:30.7159892Z 
2019-11-28T14:42:30.7159949Z error: defining opaque type use does not fully define opaque type: generic parameter `V` is specified as concrete type `<T as TraitWithAssoc>::Assoc`
2019-11-28T14:42:30.7160280Z    |
2019-11-28T14:42:30.7160280Z    |
2019-11-28T14:42:30.7160574Z LL | / fn foo_desugared<T: TraitWithAssoc>(_: T) -> Foo<T::Assoc> { //~ ERROR does not fully define
2019-11-28T14:42:30.7160670Z LL | | }
2019-11-28T14:42:30.7160710Z    | |_^
2019-11-28T14:42:30.7160754Z 
2019-11-28T14:42:30.7160797Z error: could not find defining uses
2019-11-28T14:42:30.7160797Z error: could not find defining uses
2019-11-28T14:42:30.7161428Z   --> /checkout/src/test/ui/type-alias-impl-trait/bound_reduction2.rs:10:1
2019-11-28T14:42:30.7161503Z    |
2019-11-28T14:42:30.7161544Z LL | type Foo<V> = impl Trait<V>;
2019-11-28T14:42:30.7161617Z 
2019-11-28T14:42:30.7161675Z error: aborting due to 3 previous errors
2019-11-28T14:42:30.7161702Z 
2019-11-28T14:42:30.7162361Z For more information about this error, try `rustc --explain E0277`.
---
2019-11-28T14:42:30.7165156Z 
2019-11-28T14:42:30.7177788Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-28T14:42:30.7196204Z 
2019-11-28T14:42:30.7196752Z 
2019-11-28T14:42:30.7199710Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-28T14:42:30.7200322Z 
2019-11-28T14:42:30.7200472Z 
2019-11-28T14:42:30.7206659Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-28T14:42:30.7206936Z Build completed unsuccessfully in 1:07:01
2019-11-28T14:42:30.7206936Z Build completed unsuccessfully in 1:07:01
2019-11-28T14:42:30.7279508Z == clock drift check ==
2019-11-28T14:42:30.7296701Z   local time: Thu Nov 28 14:42:30 UTC 2019
2019-11-28T14:42:31.0109540Z   network time: Thu, 28 Nov 2019 14:42:31 GMT
2019-11-28T14:42:31.0109807Z == end clock drift check ==
2019-11-28T14:42:31.8013306Z 
2019-11-28T14:42:31.8119988Z ##[error]Bash exited with code '1'.
2019-11-28T14:42:31.8185613Z ##[section]Starting: Checkout
2019-11-28T14:42:31.8187215Z ==============================================================================
2019-11-28T14:42:31.8187263Z Task         : Get sources
2019-11-28T14:42:31.8187307Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
