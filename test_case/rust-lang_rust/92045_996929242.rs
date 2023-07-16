plain
.................................................................................................... 11300/12489
.................................................................................................... 11400/12489
.................................................................................................... 11500/12489
.................................................................................................... 11600/12489
.............................................................FF..................F.F.FFFF.....F..F.. 11700/12489
..............i....F......................F........F................................................ 11800/12489
.................................................................................................... 12000/12489
.................................................................................................... 12100/12489
.................................................................................................... 12200/12489
.................................................................................................... 12300/12489
---
---- [ui] ui/impl-trait/issues/issue-70877.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/issue-70877.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-70877" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-70877/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error: ui test compiled successfully!
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/bounds-are-checked.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/bounds-are-checked" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/bounds-are-checked/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unnecessary lifetime parameter `'a`
   |
   |
LL | fn f<'a: 'static>(t: &'a str) -> X<'a> {
   |
   |
   = help: you can use the `'static` lifetime directly, in place of `'a`
warning: 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/type-alias-impl-trait/bounds-are-checked-2.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/bounds-are-checked-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/bounds-are-checked-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/bounds-are-checked-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/type-alias-impl-trait/generic_duplicate_param_use2.rs stdout ----
diff of stderr:

10 LL | type Two<T, U> = impl Debug;
11    |          ^  ^
- error[E0277]: `T` doesn't implement `Debug`
-   --> $DIR/generic_duplicate_param_use2.rs:8:18
-    |
-    |
- LL | type Two<T, U> = impl Debug;
-    |                  ^^^^^^^^^^ `T` cannot be formatted using `{:?}` because it doesn't implement `Debug`
- help: consider restricting type parameter `T`
-    |
-    |
- LL | type Two<T: std::fmt::Debug, U> = impl Debug;
+ error: aborting due to previous error
23 
- error: aborting due to 2 previous errors
- 
---
To only update this specific test, also pass `--test-args type-alias-impl-trait/generic_duplicate_param_use2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_duplicate_param_use2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_duplicate_param_use2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: non-defining opaque type use in defining scope
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use2.rs:11:27
   |
LL | fn one<T: Debug>(t: T) -> Two<T, T> {
   |
note: type used multiple times
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use2.rs:8:10
   |
   |
LL | type Two<T, U> = impl Debug;
   |          ^  ^
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/type-alias-impl-trait/generic_duplicate_param_use3.rs stdout ----
diff of stderr:

22 LL | fn two<T: Debug, U>(t: T, _: U) -> Two<T, U> {
24 
- error[E0277]: `T` doesn't implement `Debug`
-   --> $DIR/generic_duplicate_param_use3.rs:8:18
-    |
-    |
- LL | type Two<T, U> = impl Debug;
-    |                  ^^^^^^^^^^ `T` cannot be formatted using `{:?}` because it doesn't implement `Debug`
- help: consider restricting type parameter `T`
-    |
-    |
- LL | type Two<T: std::fmt::Debug, U> = impl Debug;
+ error: aborting due to 2 previous errors
35 
- error: aborting due to 3 previous errors
- 
---
To only update this specific test, also pass `--test-args type-alias-impl-trait/generic_duplicate_param_use3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_duplicate_param_use3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_duplicate_param_use3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: non-defining opaque type use in defining scope
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use3.rs:11:27
   |
LL | fn one<T: Debug>(t: T) -> Two<T, T> {
   |
note: type used multiple times
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use3.rs:8:10
   |
   |
LL | type Two<T, U> = impl Debug;
   |          ^  ^
error: concrete type differs from previous defining opaque type use
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use3.rs:20:1
   |
   |
LL | fn three<T, U: Debug>(_: T, u: U) -> Two<T, U> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `T`, got `U`
note: previous use here
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use3.rs:16:1
   |
   |
LL | fn two<T: Debug, U>(t: T, _: U) -> Two<T, U> {

error: aborting due to 2 previous errors



------------------------------------------


---- [ui] ui/type-alias-impl-trait/generic_duplicate_param_use5.rs stdout ----
diff of stderr:

10 LL | fn two<T: Debug, U: Debug>(t: T, u: U) -> Two<T, U> {
12 
- error[E0277]: `T` doesn't implement `Debug`
-   --> $DIR/generic_duplicate_param_use5.rs:8:18
-    |
-    |
- LL | type Two<T, U> = impl Debug;
-    |                  ^^^^^^^^^^ `T` cannot be formatted using `{:?}` because it doesn't implement `Debug`
-    |
-    = note: required because of the requirements on the impl of `Debug` for `(T, U)`
- help: consider restricting type parameter `T`
-    |
- LL | type Two<T: std::fmt::Debug, U> = impl Debug;
+ error: aborting due to previous error
24 
- error[E0277]: `U` doesn't implement `Debug`
-   --> $DIR/generic_duplicate_param_use5.rs:8:18
-   --> $DIR/generic_duplicate_param_use5.rs:8:18
-    |
- LL | type Two<T, U> = impl Debug;
-    |                  ^^^^^^^^^^ `U` cannot be formatted using `{:?}` because it doesn't implement `Debug`
-    |
-    = note: required because of the requirements on the impl of `Debug` for `(T, U)`
- help: consider restricting type parameter `U`
-    |
- LL | type Two<T, U: std::fmt::Debug> = impl Debug;
- 
- error: aborting due to 3 previous errors
- 
- For more information about this error, try `rustc --explain E0277`.
---
To only update this specific test, also pass `--test-args type-alias-impl-trait/generic_duplicate_param_use5.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_duplicate_param_use5" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_duplicate_param_use5/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: concrete type differs from previous defining opaque type use
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use5.rs:16:1
   |
LL | fn three<T: Debug, U: Debug>(t: T, u: U) -> Two<T, U> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `(T, U)`, got `(U, T)`
note: previous use here
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use5.rs:12:1
   |
   |
LL | fn two<T: Debug, U: Debug>(t: T, u: U) -> Two<T, U> {

error: aborting due to previous error



------------------------------------------


---- [ui] ui/type-alias-impl-trait/generic_duplicate_param_use4.rs stdout ----
diff of stderr:

10 LL | type Two<T, U> = impl Debug;
11    |          ^  ^
- error[E0277]: `U` doesn't implement `Debug`
-   --> $DIR/generic_duplicate_param_use4.rs:8:18
-    |
-    |
- LL | type Two<T, U> = impl Debug;
-    |                  ^^^^^^^^^^ `U` cannot be formatted using `{:?}` because it doesn't implement `Debug`
- help: consider restricting type parameter `U`
-    |
-    |
- LL | type Two<T, U: std::fmt::Debug> = impl Debug;
+ error: aborting due to previous error
23 
- error: aborting due to 2 previous errors
- 
---
To only update this specific test, also pass `--test-args type-alias-impl-trait/generic_duplicate_param_use4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_duplicate_param_use4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_duplicate_param_use4/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: non-defining opaque type use in defining scope
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use4.rs:11:27
   |
LL | fn one<T: Debug>(t: T) -> Two<T, T> {
   |
note: type used multiple times
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use4.rs:8:10
   |
   |
LL | type Two<T, U> = impl Debug;
   |          ^  ^
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/type-alias-impl-trait/generic_duplicate_param_use8.rs stdout ----
diff of stderr:

10 LL | fn two<T: Debug, U: Debug>(t: T, _: U) -> Two<T, U> {
12 
- error[E0277]: `T` doesn't implement `Debug`
-   --> $DIR/generic_duplicate_param_use8.rs:7:18
-    |
-    |
- LL | type Two<T, U> = impl Debug;
-    |                  ^^^^^^^^^^ `T` cannot be formatted using `{:?}` because it doesn't implement `Debug`
-    |
-    = note: required because of the requirements on the impl of `Debug` for `(T, u32)`
- help: consider restricting type parameter `T`
-    |
- LL | type Two<T: std::fmt::Debug, U> = impl Debug;
+ error: aborting due to previous error
24 
- error: aborting due to 2 previous errors
- 
---
To only update this specific test, also pass `--test-args type-alias-impl-trait/generic_duplicate_param_use8.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use8.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_duplicate_param_use8" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_duplicate_param_use8/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: concrete type differs from previous defining opaque type use
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use8.rs:14:1
   |
LL | fn three<T: Debug, U: Debug>(_: T, u: U) -> Two<T, U> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `(T, u32)`, got `(U, u32)`
note: previous use here
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use8.rs:10:1
   |
   |
LL | fn two<T: Debug, U: Debug>(t: T, _: U) -> Two<T, U> {

error: aborting due to previous error



------------------------------------------


---- [ui] ui/type-alias-impl-trait/generic_duplicate_param_use6.rs stdout ----
diff of stderr:

10 LL | fn two<T: Copy + Debug, U: Debug>(t: T, u: U) -> Two<T, U> {
12 
- error[E0277]: `T` doesn't implement `Debug`
-   --> $DIR/generic_duplicate_param_use6.rs:8:18
-    |
-    |
- LL | type Two<T, U> = impl Debug;
-    |                  ^^^^^^^^^^ `T` cannot be formatted using `{:?}` because it doesn't implement `Debug`
-    |
-    = note: required because of the requirements on the impl of `Debug` for `(T, T)`
- help: consider restricting type parameter `T`
-    |
- LL | type Two<T: std::fmt::Debug, U> = impl Debug;
+ error: aborting due to previous error
24 
- error: aborting due to 2 previous errors
- 
---
To only update this specific test, also pass `--test-args type-alias-impl-trait/generic_duplicate_param_use6.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use6.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_duplicate_param_use6" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_duplicate_param_use6/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: concrete type differs from previous defining opaque type use
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use6.rs:15:1
   |
LL | fn three<T: Copy + Debug, U: Debug>(t: T, u: U) -> Two<T, U> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `(T, T)`, got `(U, T)`
note: previous use here
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use6.rs:11:1
   |
   |
LL | fn two<T: Copy + Debug, U: Debug>(t: T, u: U) -> Two<T, U> {

error: aborting due to previous error



------------------------------------------


---- [ui] ui/type-alias-impl-trait/generic_duplicate_param_use9.rs stdout ----
diff of stderr:

10 LL | fn two<T: Debug + Foo, U: Debug>(t: T, u: U) -> Two<T, U> {
12 
12 
- error[E0277]: the trait bound `A: Foo` is not satisfied in `(A, B, <A as Foo>::Bar)`
-   --> $DIR/generic_duplicate_param_use9.rs:7:18
-    |
- LL | type Two<A, B> = impl Debug;
-    |                  ^^^^^^^^^^ within `(A, B, <A as Foo>::Bar)`, the trait `Foo` is not implemented for `A`
-    |
-    = note: required because it appears within the type `(A, B, <A as Foo>::Bar)`
- help: consider restricting type parameter `A`
-    |
- LL | type Two<A: Foo, B> = impl Debug;
+ error: aborting due to previous error
24 
24 
- error[E0277]: `A` doesn't implement `Debug`
-   --> $DIR/generic_duplicate_param_use9.rs:7:18
-    |
- LL | type Two<A, B> = impl Debug;
-    |                  ^^^^^^^^^^ `A` cannot be formatted using `{:?}` because it doesn't implement `Debug`
-    |
-    = note: required because of the requirements on the impl of `Debug` for `(A, B, <A as Foo>::Bar)`
- help: consider restricting type parameter `A`
-    |
- LL | type Two<A: std::fmt::Debug, B> = impl Debug;
- 
- 
- error[E0277]: `B` doesn't implement `Debug`
-   --> $DIR/generic_duplicate_param_use9.rs:7:18
-    |
- LL | type Two<A, B> = impl Debug;
-    |                  ^^^^^^^^^^ `B` cannot be formatted using `{:?}` because it doesn't implement `Debug`
-    |
-    = note: required because of the requirements on the impl of `Debug` for `(A, B, <A as Foo>::Bar)`
- help: consider restricting type parameter `B`
-    |
- LL | type Two<A, B: std::fmt::Debug> = impl Debug;
- 
- error: aborting due to 4 previous errors
- 
- For more information about this error, try `rustc --explain E0277`.
---
To only update this specific test, also pass `--test-args type-alias-impl-trait/generic_duplicate_param_use9.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use9.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_duplicate_param_use9" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_duplicate_param_use9/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: concrete type differs from previous defining opaque type use
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use9.rs:21:1
   |
LL | fn three<T: Debug, U: Debug>(t: T, u: U) -> Two<T, U> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `(A, B, <A as Foo>::Bar)`, got `(A, B, i32)`
note: previous use here
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use9.rs:17:1
   |
   |
LL | fn two<T: Debug + Foo, U: Debug>(t: T, u: U) -> Two<T, U> {

error: aborting due to previous error



------------------------------------------


---- [ui] ui/type-alias-impl-trait/issue-52843.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-52843.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-52843" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-52843/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/type-alias-impl-trait/issue-63355.rs stdout ----
diff of stderr:

- error[E0271]: type mismatch resolving `<() as Bar>::Foo == ()`
-   --> $DIR/issue-63355.rs:34:20
+ error[E0636]: the feature `type_alias_impl_trait` has already been declared
3    |
- LL | pub type FooImpl = impl Foo;
-    |                    -------- the found opaque type
-    |                    -------- the found opaque type
- LL | pub type BarImpl = impl Bar<Foo = FooImpl>;
-    |                    ^^^^^^^^^^^^^^^^^^^^^^^ type mismatch resolving `<() as Bar>::Foo == ()`
- note: expected this to be `()`
-   --> $DIR/issue-63355.rs:24:16
-    |
- LL |     type Foo = FooImpl;
- LL |     type Foo = FooImpl;
-    |                ^^^^^^^
-    = note: expected unit type `()`
-             found opaque type `impl Foo`
+ LL | #![feature(type_alias_impl_trait)]
16 
17 error: aborting due to previous error
18 

---
To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-63355.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-63355.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-63355" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-63355/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0636]: the feature `type_alias_impl_trait` has already been declared
   |
LL | #![feature(type_alias_impl_trait)]
   |            ^^^^^^^^^^^^^^^^^^^^^

---
---- [ui] ui/type-alias-impl-trait/issue-89686.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-89686.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-89686" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-89686/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/type-alias-impl-trait/not_a_defining_use.rs stdout ----
diff of stderr:

22 LL | fn three<T: Debug, U>(t: T) -> Two<T, U> {
24 
- error[E0277]: `T` doesn't implement `Debug`
-   --> $DIR/not_a_defining_use.rs:7:18
-    |
-    |
- LL | type Two<T, U> = impl Debug;
-    |                  ^^^^^^^^^^ `T` cannot be formatted using `{:?}` because it doesn't implement `Debug`
-    |
-    = note: required because of the requirements on the impl of `Debug` for `(T, i8)`
- help: consider restricting type parameter `T`
-    |
- LL | type Two<T: std::fmt::Debug, U> = impl Debug;
+ error: aborting due to 2 previous errors
36 
- error: aborting due to 3 previous errors
- 
---
To only update this specific test, also pass `--test-args type-alias-impl-trait/not_a_defining_use.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/not_a_defining_use.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/not_a_defining_use" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/not_a_defining_use/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: non-defining opaque type use in defining scope
  --> /checkout/src/test/ui/type-alias-impl-trait/not_a_defining_use.rs:10:27
   |
LL | fn two<T: Debug>(t: T) -> Two<T, u32> {
   |
   |
note: used non-generic type `u32` for generic parameter
   |
   |
LL | type Two<T, U> = impl Debug;

error: concrete type differs from previous defining opaque type use
  --> /checkout/src/test/ui/type-alias-impl-trait/not_a_defining_use.rs:29:1
   |
   |
LL | fn four<T: Debug, U: Bar>(t: T) -> Two<T, U> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `(T, i8)`, got `(T, <U as Bar>::Blub)`
note: previous use here
  --> /checkout/src/test/ui/type-alias-impl-trait/not_a_defining_use.rs:15:1
   |
   |
LL | fn three<T: Debug, U>(t: T) -> Two<T, U> {

error: aborting due to 2 previous errors


---
test result: FAILED. 12356 passed; 14 failed; 119 ignored; 0 measured; 0 filtered out; finished in 164.78s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:03
