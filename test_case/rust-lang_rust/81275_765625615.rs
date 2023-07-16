plain
.................................................................................................... 3400/11277
.................................................................................................... 3500/11277
.................................................................................................... 3600/11277
.................................................................................................... 3700/11277
..............................F.F................................................................... 3800/11277
............................................................................................F....... 4000/11277
.................................................................................................... 4100/11277
.................................................................................................... 4200/11277
...............ii..................................................................................i 4300/11277
---
normalized stderr:
error[E0433]: failed to resolve: use of undeclared type `Foo`
  --> $DIR/in-trait-impl.rs:7:27
   |
LL |     type Quux<'a> = <T as Foo>::Bar<'a, 'static>;
   |                           ^^^ use of undeclared type `Foo`

error[E0405]: cannot find trait `Baz` in this scope
  --> $DIR/in-trait-impl.rs:6:9
   |
LL | impl<T> Baz for T where T: Foo {

error[E0405]: cannot find trait `Foo` in this scope
  --> $DIR/in-trait-impl.rs:6:28
   |
   |
LL | impl<T> Baz for T where T: Foo {


warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes
  --> $DIR/in-trait-impl.rs:4:12
LL | #![feature(generic_associated_types)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parse/in-trait-impl/in-trait-impl.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/parse/in-trait-impl.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/parse/in-trait-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parse/in-trait-impl" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parse/in-trait-impl/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0433]: failed to resolve: use of undeclared type `Foo`
  --> /checkout/src/test/ui/generic-associated-types/parse/in-trait-impl.rs:7:27
   |
LL |     type Quux<'a> = <T as Foo>::Bar<'a, 'static>;
   |                           ^^^ use of undeclared type `Foo`

error[E0405]: cannot find trait `Baz` in this scope
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
LL | impl<T> Baz for T where T: Foo {

error[E0405]: cannot find trait `Foo` in this scope
  --> /checkout/src/test/ui/generic-associated-types/parse/in-trait-impl.rs:6:28
   |
   |
LL | impl<T> Baz for T where T: Foo {


warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(generic_associated_types)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^
   |
---


---- [ui] ui/generic-associated-types/parse/in-trait.rs stdout ----
normalized stderr:
error[E0428]: the name `Bar` is defined multiple times
  --> $DIR/in-trait.rs:11:5
   |
LL |     type Bar<'a>;
   |     ------------- previous definition of the type `Bar` here
LL |     type Bar<'a, 'b>;
   |     ^^^^^^^^^^^^^^^^^ `Bar` redefined here
   |
   = note: `Bar` must be defined only once in the type namespace of this trait

error[E0428]: the name `Bar` is defined multiple times
  --> $DIR/in-trait.rs:12:5
   |
LL |     type Bar<'a>;
   |     ------------- previous definition of the type `Bar` here
LL |     type Bar<'a, 'b>;
LL |     type Bar<'a, 'b,>;
   |     ^^^^^^^^^^^^^^^^^^ `Bar` redefined here
   |
   = note: `Bar` must be defined only once in the type namespace of this trait

error[E0428]: the name `Bar` is defined multiple times
  --> $DIR/in-trait.rs:13:5
   |
LL |     type Bar<'a>;
   |     ------------- previous definition of the type `Bar` here
...
LL |     type Bar<'a, 'b, T>;
   |     ^^^^^^^^^^^^^^^^^^^^ `Bar` redefined here
   |
   = note: `Bar` must be defined only once in the type namespace of this trait

error[E0428]: the name `Bar` is defined multiple times
  --> $DIR/in-trait.rs:14:5
   |
LL |     type Bar<'a>;
   |     ------------- previous definition of the type `Bar` here
...
LL |     type Bar<'a, 'b, T, U>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^ `Bar` redefined here
   |
   = note: `Bar` must be defined only once in the type namespace of this trait

error[E0428]: the name `Bar` is defined multiple times
  --> $DIR/in-trait.rs:15:5
   |
LL |     type Bar<'a>;
   |     ------------- previous definition of the type `Bar` here
...
LL |     type Bar<'a, 'b, T, U,>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ `Bar` redefined here
   |
   = note: `Bar` must be defined only once in the type namespace of this trait

error[E0428]: the name `Bar` is defined multiple times
  --> $DIR/in-trait.rs:16:5
   |
LL |     type Bar<'a>;
   |     ------------- previous definition of the type `Bar` here
...
LL |     type Bar<'a, 'b, T: Debug, U,>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Bar` redefined here
   |
   = note: `Bar` must be defined only once in the type namespace of this trait

error[E0428]: the name `Bar` is defined multiple times
  --> $DIR/in-trait.rs:17:5
   |
LL |     type Bar<'a>;
   |     ------------- previous definition of the type `Bar` here
...
LL |     type Bar<'a, 'b, T: Debug, U,>: Debug;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Bar` redefined here
   |
   = note: `Bar` must be defined only once in the type namespace of this trait

error[E0428]: the name `Bar` is defined multiple times
  --> $DIR/in-trait.rs:18:5
   |
LL |     type Bar<'a>;
   |     ------------- previous definition of the type `Bar` here
...
LL |     type Bar<'a, 'b, T: Debug, U,>: Deref<Target = T> + Into<U>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Bar` redefined here
   |
   = note: `Bar` must be defined only once in the type namespace of this trait

error[E0428]: the name `Bar` is defined multiple times
  --> $DIR/in-trait.rs:19:5
   |
LL |     type Bar<'a>;
   |     ------------- previous definition of the type `Bar` here
...
LL |     type Bar<'a, 'b, T: Debug, U,> where T: Deref<Target = U>, U: Into<T>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Bar` redefined here
   |
   = note: `Bar` must be defined only once in the type namespace of this trait

error[E0428]: the name `Bar` is defined multiple times
  --> $DIR/in-trait.rs:20:5
   |
LL |       type Bar<'a>;
   |       ------------- previous definition of the type `Bar` here
...
LL | /     type Bar<'a, 'b, T: Debug, U,>: Deref<Target = T> + Into<U>
LL | |         where T: Deref<Target = U>, U: Into<T>;
   | |_______________________________________________^ `Bar` redefined here
   |
   = note: `Bar` must be defined only once in the type namespace of this trait

warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes
  --> $DIR/in-trait.rs:4:12
LL | #![feature(generic_associated_types)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parse/in-trait/in-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/parse/in-trait.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/parse/in-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parse/in-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parse/in-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0428]: the name `Bar` is defined multiple times
   |
   |
LL |     type Bar<'a>;
   |     ------------- previous definition of the type `Bar` here
LL |     type Bar<'a, 'b>;
   |     ^^^^^^^^^^^^^^^^^ `Bar` redefined here
   |
   = note: `Bar` must be defined only once in the type namespace of this trait

error[E0428]: the name `Bar` is defined multiple times
   |
   |
LL |     type Bar<'a>;
   |     ------------- previous definition of the type `Bar` here
LL |     type Bar<'a, 'b>;
LL |     type Bar<'a, 'b,>;
   |     ^^^^^^^^^^^^^^^^^^ `Bar` redefined here
   |
   = note: `Bar` must be defined only once in the type namespace of this trait

error[E0428]: the name `Bar` is defined multiple times
   |
   |
LL |     type Bar<'a>;
   |     ------------- previous definition of the type `Bar` here
...
LL |     type Bar<'a, 'b, T>;
   |     ^^^^^^^^^^^^^^^^^^^^ `Bar` redefined here
   |
   = note: `Bar` must be defined only once in the type namespace of this trait

error[E0428]: the name `Bar` is defined multiple times
   |
   |
LL |     type Bar<'a>;
   |     ------------- previous definition of the type `Bar` here
...
LL |     type Bar<'a, 'b, T, U>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^ `Bar` redefined here
   |
   = note: `Bar` must be defined only once in the type namespace of this trait

error[E0428]: the name `Bar` is defined multiple times
   |
   |
LL |     type Bar<'a>;
   |     ------------- previous definition of the type `Bar` here
...
LL |     type Bar<'a, 'b, T, U,>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ `Bar` redefined here
   |
   = note: `Bar` must be defined only once in the type namespace of this trait

error[E0428]: the name `Bar` is defined multiple times
   |
   |
LL |     type Bar<'a>;
   |     ------------- previous definition of the type `Bar` here
...
LL |     type Bar<'a, 'b, T: Debug, U,>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Bar` redefined here
   |
   = note: `Bar` must be defined only once in the type namespace of this trait

error[E0428]: the name `Bar` is defined multiple times
   |
   |
LL |     type Bar<'a>;
   |     ------------- previous definition of the type `Bar` here
...
LL |     type Bar<'a, 'b, T: Debug, U,>: Debug;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Bar` redefined here
   |
   = note: `Bar` must be defined only once in the type namespace of this trait

error[E0428]: the name `Bar` is defined multiple times
   |
   |
LL |     type Bar<'a>;
   |     ------------- previous definition of the type `Bar` here
...
LL |     type Bar<'a, 'b, T: Debug, U,>: Deref<Target = T> + Into<U>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Bar` redefined here
   |
   = note: `Bar` must be defined only once in the type namespace of this trait

error[E0428]: the name `Bar` is defined multiple times
   |
   |
LL |     type Bar<'a>;
   |     ------------- previous definition of the type `Bar` here
...
LL |     type Bar<'a, 'b, T: Debug, U,> where T: Deref<Target = U>, U: Into<T>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Bar` redefined here
   |
   = note: `Bar` must be defined only once in the type namespace of this trait

error[E0428]: the name `Bar` is defined multiple times
   |
   |
LL |       type Bar<'a>;
   |       ------------- previous definition of the type `Bar` here
...
LL | /     type Bar<'a, 'b, T: Debug, U,>: Deref<Target = T> + Into<U>
LL | |         where T: Deref<Target = U>, U: Into<T>;
   | |_______________________________________________^ `Bar` redefined here
   |
   = note: `Bar` must be defined only once in the type namespace of this trait

warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(generic_associated_types)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^
   |
---

---- [ui] ui/impl-trait/impl-trait-plus-priority.rs stdout ----
diff of stderr:

64 LL | type A = &A + B;
65    |          ^^^^^^ help: try adding parentheses: `&(A + B)`
- error: aborting due to 11 previous errors
- error: aborting due to 11 previous errors
+ error[E0428]: the name `f` is defined multiple times
+    |
+    |
+ LL | fn f() -> impl A + {} // OK
+    | ------------------ previous definition of the value `f` here
+ LL | fn f() -> impl A + B {} // OK
+    | ^^^^^^^^^^^^^^^^^^^^ `f` redefined here
+    |
+    = note: `f` must be defined only once in the value namespace of this module
- For more information about this error, try `rustc --explain E0178`.
- For more information about this error, try `rustc --explain E0178`.
+ error[E0428]: the name `f` is defined multiple times
+    |
+    |
+ LL | fn f() -> impl A + {} // OK
+    | ------------------ previous definition of the value `f` here
+ LL | fn f() -> impl A + B {} // OK
+ LL | fn f() -> dyn A + B {} // OK
+    | ^^^^^^^^^^^^^^^^^^^ `f` redefined here
+    |
+    = note: `f` must be defined only once in the value namespace of this module
+ 
+ error[E0428]: the name `f` is defined multiple times
+    |
+    |
+ LL | fn f() -> impl A + {} // OK
+    | ------------------ previous definition of the value `f` here
+ ...
+ LL | fn f() -> A + B {} // OK
+    | ^^^^^^^^^^^^^^^ `f` redefined here
+    |
+    = note: `f` must be defined only once in the value namespace of this module
+ 
+ error[E0428]: the name `A` is defined multiple times
+    |
+    |
+ LL | type A = fn() -> impl A +;
+    | -------------------------- previous definition of the type `A` here
+ LL |
+ LL | type A = fn() -> impl A + B;
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `A` redefined here
+    |
+    = note: `A` must be defined only once in the type namespace of this module
+ 
+ error[E0428]: the name `A` is defined multiple times
+    |
+    |
+ LL | type A = fn() -> impl A +;
+    | -------------------------- previous definition of the type `A` here
+ ...
+ LL | type A = fn() -> dyn A + B;
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `A` redefined here
+    |
+    = note: `A` must be defined only once in the type namespace of this module
+ 
+ error[E0428]: the name `A` is defined multiple times
+    |
+    |
+ LL | type A = fn() -> impl A +;
+    | -------------------------- previous definition of the type `A` here
+ ...
+ LL | type A = fn() -> A + B;
+    | ^^^^^^^^^^^^^^^^^^^^^^^ `A` redefined here
+    |
+    = note: `A` must be defined only once in the type namespace of this module
+ 
+ error[E0428]: the name `A` is defined multiple times
+    |
+    |
+ LL | type A = fn() -> impl A +;
+    | -------------------------- previous definition of the type `A` here
+ ...
+ LL | type A = Fn() -> impl A +;
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ `A` redefined here
+    |
+    = note: `A` must be defined only once in the type namespace of this module
+ 
+ error[E0428]: the name `A` is defined multiple times
+    |
+    |
+ LL | type A = fn() -> impl A +;
+    | -------------------------- previous definition of the type `A` here
+ ...
+ LL | type A = Fn() -> impl A + B;
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `A` redefined here
+    |
+    = note: `A` must be defined only once in the type namespace of this module
+ 
+ error[E0428]: the name `A` is defined multiple times
+    |
+    |
+ LL | type A = fn() -> impl A +;
+    | -------------------------- previous definition of the type `A` here
+ ...
+ LL | type A = Fn() -> dyn A + B;
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `A` redefined here
+    |
+    = note: `A` must be defined only once in the type namespace of this module
+ 
+ error[E0428]: the name `A` is defined multiple times
+    |
+    |
+ LL | type A = fn() -> impl A +;
+    | -------------------------- previous definition of the type `A` here
+ ...
+ LL | type A = Fn() -> A + B; // OK, interpreted as `(Fn() -> A) + B` for compatibility
+    | ^^^^^^^^^^^^^^^^^^^^^^^ `A` redefined here
+    |
+    = note: `A` must be defined only once in the type namespace of this module
+ 
+ error[E0428]: the name `A` is defined multiple times
+    |
+    |
+ LL | type A = fn() -> impl A +;
+    | -------------------------- previous definition of the type `A` here
+ ...
+ LL | type A = &impl A +;
+    | ^^^^^^^^^^^^^^^^^^^ `A` redefined here
+    |
+    = note: `A` must be defined only once in the type namespace of this module
+ 
+ error[E0428]: the name `A` is defined multiple times
+    |
+    |
+ LL | type A = fn() -> impl A +;
+    | -------------------------- previous definition of the type `A` here
+ ...
+ LL | type A = &impl A + B;
+    | ^^^^^^^^^^^^^^^^^^^^^ `A` redefined here
+    |
+    = note: `A` must be defined only once in the type namespace of this module
+ 
+ error[E0428]: the name `A` is defined multiple times
+    |
+    |
+ LL | type A = fn() -> impl A +;
+    | -------------------------- previous definition of the type `A` here
+ ...
+ LL | type A = &dyn A + B;
+    | ^^^^^^^^^^^^^^^^^^^^ `A` redefined here
+    |
+    = note: `A` must be defined only once in the type namespace of this module
+ 
+ error[E0428]: the name `A` is defined multiple times
+    |
+    |
+ LL | type A = fn() -> impl A +;
+    | -------------------------- previous definition of the type `A` here
+ ...
+ LL | type A = &A + B;
+    | ^^^^^^^^^^^^^^^^ `A` redefined here
+    |
+    = note: `A` must be defined only once in the type namespace of this module
+ 
+ error[E0404]: expected trait, found type alias `A`
+    |
+    |
+ LL | fn f() -> impl A + {} // OK
+    |                ^ type aliases cannot be used as traits
+    |
+ help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
+    |
+    |
+ LL | type A = fn() -> impl A +;
+ 
+ 
+ error[E0404]: expected trait, found type alias `A`
+    |
+    |
+ LL | fn f() -> impl A + B {} // OK
+    |                ^ type aliases cannot be used as traits
+    |
+ help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
+    |
+    |
+ LL | type A = fn() -> impl A +;
+ 
+ 
+ error[E0405]: cannot find trait `B` in this scope
+    |
+    |
+ LL | fn f() -> impl A + B {} // OK
+    |     -              ^ not found in this scope
+    |     help: you might be missing a type parameter: `<B>`
+ 
+ 
+ error[E0404]: expected trait, found type alias `A`
+    |
+    |
+ LL | fn f() -> dyn A + B {} // OK
+    |               ^ type aliases cannot be used as traits
+    |
+ help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
+    |
+    |
+ LL | type A = fn() -> impl A +;
+ 
+ 
+ error[E0405]: cannot find trait `B` in this scope
+    |
+    |
+ LL | fn f() -> dyn A + B {} // OK
+    |     -             ^ not found in this scope
+    |     help: you might be missing a type parameter: `<B>`
+ 
+ 
+ error[E0404]: expected trait, found type alias `A`
+    |
+    |
+ LL | fn f() -> A + B {} // OK
+    |           ^ type aliases cannot be used as traits
+    |
+ help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
+    |
+    |
+ LL | type A = fn() -> impl A +;
+ 
+ 
+ error[E0405]: cannot find trait `B` in this scope
+    |
+    |
+ LL | fn f() -> A + B {} // OK
+    |     -         ^ not found in this scope
+    |     help: you might be missing a type parameter: `<B>`
+ 
+ error[E0412]: cannot find type `S` in this scope
+   --> $DIR/impl-trait-plus-priority.rs:8:6
+   --> $DIR/impl-trait-plus-priority.rs:8:6
+    |
+ LL | impl S {
+    |      ^ help: a type alias with a similar name exists: `A`
+ ...
+ LL | type A = fn() -> impl A +;
+    | -------------------------- similarly named type alias `A` defined here
+ 
+ error[E0404]: expected trait, found type alias `A`
+    |
+    |
+ LL |     fn f(self) -> impl A + { // OK
+    |                        ^ type aliases cannot be used as traits
+    |
+ help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
+    |
+    |
+ LL | type A = fn() -> impl A +;
+ 
+ 
+ error[E0404]: expected trait, found type alias `A`
+    |
+    |
+ LL |         let _ = |a, b| -> impl A + {}; // OK
+    |                                ^ type aliases cannot be used as traits
+    |
+ help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
+    |
+    |
+ LL | type A = fn() -> impl A +;
+ 
+ 
+ error[E0404]: expected trait, found type alias `A`
+    |
+    |
+ LL |     fn f(self) -> impl A + B { // OK
+    |                        ^ type aliases cannot be used as traits
+    |
+ help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
+    |
+    |
+ LL | type A = fn() -> impl A +;
+ 
+ 
+ error[E0405]: cannot find trait `B` in this scope
+    |
+    |
+ LL |     fn f(self) -> impl A + B { // OK
+    |                            ^ not found in this scope
+ 
+ error[E0404]: expected trait, found type alias `A`
+    |
+    |
+ LL |         let _ = |a, b| -> impl A + B {}; // OK
+    |                                ^ type aliases cannot be used as traits
+    |
+ help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
+    |
+    |
+ LL | type A = fn() -> impl A +;
+ 
+ 
+ error[E0405]: cannot find trait `B` in this scope
+    |
+    |
+ LL |         let _ = |a, b| -> impl A + B {}; // OK
+    |                                    ^ not found in this scope
+ 
+ error[E0404]: expected trait, found type alias `A`
+    |
+    |
+ LL |     fn f(self) -> dyn A + B { // OK
+    |                       ^ type aliases cannot be used as traits
+    |
+ help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
+    |
+    |
+ LL | type A = fn() -> impl A +;
+ 
+ 
+ error[E0405]: cannot find trait `B` in this scope
+    |
+    |
+ LL |     fn f(self) -> dyn A + B { // OK
+    |                           ^ not found in this scope
+ 
+ error[E0404]: expected trait, found type alias `A`
+    |
+    |
+ LL |         let _ = |a, b| -> dyn A + B {}; // OK
+    |                               ^ type aliases cannot be used as traits
+    |
+ help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
+    |
+    |
+ LL | type A = fn() -> impl A +;
+ 
+ 
+ error[E0405]: cannot find trait `B` in this scope
+    |
+    |
+ LL |         let _ = |a, b| -> dyn A + B {}; // OK
+    |                                   ^ not found in this scope
+ 
+ error[E0404]: expected trait, found type alias `A`
+    |
+    |
+ LL |     fn f(self) -> A + B { // OK
+    |                   ^ type aliases cannot be used as traits
+    |
+ help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
+    |
+    |
+ LL | type A = fn() -> impl A +;
+ 
+ 
+ error[E0405]: cannot find trait `B` in this scope
+    |
+    |
+ LL |     fn f(self) -> A + B { // OK
+    |                       ^ not found in this scope
+ 
+ error[E0404]: expected trait, found type alias `A`
+    |
+    |
+ LL |         let _ = |a, b| -> A + B {}; // OK
+    |                           ^ type aliases cannot be used as traits
+    |
+ help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
+    |
+    |
+ LL | type A = fn() -> impl A +;
+ 
+ 
+ error[E0405]: cannot find trait `B` in this scope
+    |
+    |
+ LL |         let _ = |a, b| -> A + B {}; // OK
+    |                               ^ not found in this scope
+ 
+ error[E0404]: expected trait, found type alias `A`
+    |
+    |
+ LL | type A = fn() -> impl A +;
+    |                       ^ type aliases cannot be used as traits
+    |
+ help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
+    |
+    |
+ LL | type A = fn() -> impl A +;
+ 
+ 
+ error[E0404]: expected trait, found type alias `A`
+    |
+    |
+ LL | type A = fn() -> impl A + B;
+    |                       ^ type aliases cannot be used as traits
+    |
+ help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
+    |
+    |
+ LL | type A = fn() -> impl A +;
+ 
+ 
+ error[E0405]: cannot find trait `B` in this scope
+    |
+    |
+ LL | type A = fn() -> impl A + B;
+    |                           ^ not found in this scope
+ 
+ error[E0404]: expected trait, found type alias `A`
+    |
+    |
+ LL | type A = fn() -> dyn A + B;
+    |                      ^ type aliases cannot be used as traits
+    |
+ help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
+    |
+    |
+ LL | type A = fn() -> impl A +;
+ 
+ 
+ error[E0405]: cannot find trait `B` in this scope
+    |
+    |
+ LL | type A = fn() -> dyn A + B;
+    |                          ^ not found in this scope
+ 
+ error[E0404]: expected trait, found type alias `A`
+    |
+    |
+ LL | type A = Fn() -> impl A +;
+    |                       ^ type aliases cannot be used as traits
+    |
+ help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
+    |
+    |
+ LL | type A = fn() -> impl A +;
+ 
+ 
+ error[E0404]: expected trait, found type alias `A`
+    |
+    |
+ LL | type A = Fn() -> impl A + B;
+    |                       ^ type aliases cannot be used as traits
+    |
+ help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
+    |
+    |
+ LL | type A = fn() -> impl A +;
+ 
+ 
+ error[E0405]: cannot find trait `B` in this scope
+    |
+    |
+ LL | type A = Fn() -> impl A + B;
+    |                           ^ not found in this scope
+ 
+ error[E0404]: expected trait, found type alias `A`
+    |
+    |
+ LL | type A = Fn() -> dyn A + B;
+    |                      ^ type aliases cannot be used as traits
+    |
+ help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
+    |
+    |
+ LL | type A = fn() -> impl A +;
+ 
+ 
+ error[E0405]: cannot find trait `B` in this scope
+    |
+    |
+ LL | type A = Fn() -> dyn A + B;
+    |                          ^ not found in this scope
+ 
+ error[E0405]: cannot find trait `B` in this scope
+    |
+    |
+ LL | type A = Fn() -> A + B; // OK, interpreted as `(Fn() -> A) + B` for compatibility
+    |                      ^ not found in this scope
+ 
+ error[E0404]: expected trait, found type alias `A`
+    |
+    |
+ LL | type A = &impl A +;
+    |                ^ type aliases cannot be used as traits
+    |
+ help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
+    |
+    |
+ LL | type A = fn() -> impl A +;
+ 
+ 
+ error[E0404]: expected trait, found type alias `A`
+    |
+    |
+ LL | type A = &impl A + B;
+    |                ^ type aliases cannot be used as traits
+    |
+ help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
+    |
+    |
+ LL | type A = fn() -> impl A +;
+ 
---
15 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/bounds-type/bounds-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/bounds-type.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/bounds-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/bounds-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/bounds-type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `?` may only modify trait bounds, not lifetime bounds
   |
   |
LL |     T: ?'a, //~ ERROR `?` may only modify trait bounds, not lifetime bounds


error: `?const` may only modify trait bounds, not lifetime bounds
   |
   |
LL |     T: ?const 'a, //~ ERROR `?const` may only modify trait bounds, not lifetime bounds


error: `?const` and `?` are mutually exclusive
   |
   |
LL |     T: ?const ?Tr, // OK


error[E0403]: the name `T` is already used for a generic parameter in this item's generic parameters
   |
   |
LL |     T: 'a + Tr, // OK
   |     - first use of `T`
LL |     T: Tr + 'a, // OK
   |     ^ already used

error[E0403]: the name `T` is already used for a generic parameter in this item's generic parameters
   |
   |
LL |     T: 'a + Tr, // OK
   |     - first use of `T`
LL |     T: Tr + 'a, // OK
LL |     T: 'a, // OK
   |     ^ already used

error[E0403]: the name `T` is already used for a generic parameter in this item's generic parameters
   |
   |
LL |     T: 'a + Tr, // OK
   |     - first use of `T`
...
LL |     T:, // OK
   |     ^ already used

error[E0403]: the name `T` is already used for a generic parameter in this item's generic parameters
   |
   |
LL |     T: 'a + Tr, // OK
   |     - first use of `T`
...
LL |     T: ?for<'a> Trait, // OK
   |     ^ already used

error[E0403]: the name `T` is already used for a generic parameter in this item's generic parameters
   |
   |
LL |     T: 'a + Tr, // OK
   |     - first use of `T`
...
LL |     T: Tr +, // OK
   |     ^ already used

error[E0403]: the name `T` is already used for a generic parameter in this item's generic parameters
   |
   |
LL |     T: 'a + Tr, // OK
   |     - first use of `T`
...
LL |     T: ?'a, //~ ERROR `?` may only modify trait bounds, not lifetime bounds
   |     ^ already used

error[E0403]: the name `T` is already used for a generic parameter in this item's generic parameters
   |
   |
LL |     T: 'a + Tr, // OK
   |     - first use of `T`
...
LL |     T: ?const Tr, // OK
   |     ^ already used

error[E0403]: the name `T` is already used for a generic parameter in this item's generic parameters
   |
   |
LL |     T: 'a + Tr, // OK
   |     - first use of `T`
...
LL |     T: ?const ?Tr, // OK
   |     ^ already used

error[E0403]: the name `T` is already used for a generic parameter in this item's generic parameters
   |
   |
LL |     T: 'a + Tr, // OK
   |     - first use of `T`
...
LL |     T: ?const Tr + 'a, // OK
   |     ^ already used

error[E0403]: the name `T` is already used for a generic parameter in this item's generic parameters
   |
   |
LL |     T: 'a + Tr, // OK
   |     - first use of `T`
...
LL |     T: ?const 'a, //~ ERROR `?const` may only modify trait bounds, not lifetime bounds
   |     ^ already used

error[E0405]: cannot find trait `Tr` in this scope
   |
   |
LL |     T: 'a + Tr, // OK
   |             ^^ not found in this scope

error[E0405]: cannot find trait `Tr` in this scope
   |
   |
LL |     T: Tr + 'a, // OK
   |        ^^ not found in this scope

error[E0405]: cannot find trait `Trait` in this scope
   |
   |
LL |     T: ?for<'a> Trait, // OK


error[E0405]: cannot find trait `Tr` in this scope
   |
   |
LL |     T: Tr +, // OK
   |        ^^ not found in this scope

error[E0405]: cannot find trait `Tr` in this scope
   |
   |
LL |     T: ?const Tr, // OK
   |               ^^ not found in this scope

error[E0405]: cannot find trait `Tr` in this scope
   |
   |
LL |     T: ?const ?Tr, // OK
   |                ^^ not found in this scope

error[E0405]: cannot find trait `Tr` in this scope
   |
   |
LL |     T: ?const Tr + 'a, // OK
   |               ^^ not found in this scope

error[E0658]: `?const` on trait bounds is experimental
   |
   |
LL |     T: ?const Tr, // OK
   |
   = note: see issue #67794 <https://github.com/rust-lang/rust/issues/67794> for more information
   = help: add `#![feature(const_trait_bound_opt_out)]` to the crate attributes to enable


error[E0658]: `?const` on trait bounds is experimental
   |
   |
LL |     T: ?const ?Tr, // OK
   |
   = note: see issue #67794 <https://github.com/rust-lang/rust/issues/67794> for more information
   = help: add `#![feature(const_trait_bound_opt_out)]` to the crate attributes to enable


error[E0658]: `?const` on trait bounds is experimental
   |
   |
LL |     T: ?const Tr + 'a, // OK
   |
   = note: see issue #67794 <https://github.com/rust-lang/rust/issues/67794> for more information
   = help: add `#![feature(const_trait_bound_opt_out)]` to the crate attributes to enable


error[E0658]: `?const` on trait bounds is experimental
   |
   |
LL |     T: ?const 'a, //~ ERROR `?const` may only modify trait bounds, not lifetime bounds
   |
   = note: see issue #67794 <https://github.com/rust-lang/rust/issues/67794> for more information
   = help: add `#![feature(const_trait_bound_opt_out)]` to the crate attributes to enable

---
normalized stderr:
error[E0433]: failed to resolve: use of undeclared type `Trait`
  --> $DIR/impl-qpath.rs:5:15
   |
LL | impl <Type as Trait>::AssocTy {} // OK
   |               ^^^^^ use of undeclared type `Trait`
error[E0412]: cannot find type `Type` in this scope
  --> $DIR/impl-qpath.rs:5:7
   |
   |
LL | impl <Type as Trait>::AssocTy {} // OK


error[E0405]: cannot find trait `Trait` in this scope
  --> $DIR/impl-qpath.rs:6:12
   |
LL | impl <'a + Trait>::AssocTy {} // OK

error[E0412]: cannot find type `Type` in this scope
  --> $DIR/impl-qpath.rs:7:8
   |
   |
LL | impl <<Type>::AssocTy>::AssocTy {} // OK

warning: trait objects without an explicit `dyn` are deprecated
  --> $DIR/impl-qpath.rs:6:7
   |
   |
LL | impl <'a + Trait>::AssocTy {} // OK
   |       ^^^^^^^^^^ help: use `dyn`: `dyn 'a + Trait`
   = note: `#[warn(bare_trait_objects)]` on by default

error: aborting due to 4 previous errors; 1 warning emitted


Some errors have detailed explanations: E0405, E0412, E0433.
For more information about an error, try `rustc --explain E0405`.



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/impl-qpath/impl-qpath.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/impl-qpath.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/impl-qpath.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/impl-qpath" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/impl-qpath/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0433]: failed to resolve: use of undeclared type `Trait`
  --> /checkout/src/test/ui/parser/impl-qpath.rs:5:15
   |
LL | impl <Type as Trait>::AssocTy {} // OK
   |               ^^^^^ use of undeclared type `Trait`
error[E0412]: cannot find type `Type` in this scope
  --> /checkout/src/test/ui/parser/impl-qpath.rs:5:7
   |
   |
LL | impl <Type as Trait>::AssocTy {} // OK


error[E0405]: cannot find trait `Trait` in this scope
  --> /checkout/src/test/ui/parser/impl-qpath.rs:6:12
   |
LL | impl <'a + Trait>::AssocTy {} // OK

error[E0412]: cannot find type `Type` in this scope
  --> /checkout/src/test/ui/parser/impl-qpath.rs:7:8
   |
   |
LL | impl <<Type>::AssocTy>::AssocTy {} // OK

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/parser/impl-qpath.rs:6:7
   |
   |
LL | impl <'a + Trait>::AssocTy {} // OK
   |       ^^^^^^^^^^ help: use `dyn`: `dyn 'a + Trait`
   = note: `#[warn(bare_trait_objects)]` on by default

error: aborting due to 4 previous errors; 1 warning emitted

---


---- [ui] ui/rfc-2632-const-trait-impl/const-trait-bound-opt-out/syntax.rs stdout ----
normalized stderr:
error: `?const` and `?` are mutually exclusive
   |
   |
LL |     T: ?const ?for<'a> Tr<'a> + 'static + ?const std::ops::Add,


error: `?const` and `?` are mutually exclusive
   |
   |
LL |     T: ?const ?for<'a: 'b> m::Trait<'a>,

error: lifetime bounds cannot be used in this context
  --> $DIR/syntax.rs:9:24
   |
   |
LL |     T: ?const ?for<'a: 'b> m::Trait<'a>,


error[E0403]: the name `T` is already used for a generic parameter in this item's generic parameters
   |
   |
LL |     T: ?const ?for<'a> Tr<'a> + 'static + ?const std::ops::Add,
   |     - first use of `T`
LL |     T: ?const ?for<'a: 'b> m::Trait<'a>,
   |     ^ already used
error[E0433]: failed to resolve: use of undeclared crate or module `m`
  --> $DIR/syntax.rs:9:28
   |
   |
LL |     T: ?const ?for<'a: 'b> m::Trait<'a>,
   |                            ^ use of undeclared crate or module `m`

error[E0405]: cannot find trait `Tr` in this scope
   |
   |
LL |     T: ?const ?for<'a> Tr<'a> + 'static + ?const std::ops::Add,
   |                        ^^ not found in this scope
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0403, E0405, E0433.
For more information about an error, try `rustc --explain E0403`.
For more information about an error, try `rustc --explain E0403`.



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-trait-bound-opt-out/syntax/syntax.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/const-trait-bound-opt-out/syntax.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/const-trait-bound-opt-out/syntax.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-trait-bound-opt-out/syntax" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-trait-bound-opt-out/syntax/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `?const` and `?` are mutually exclusive
   |
   |
LL |     T: ?const ?for<'a> Tr<'a> + 'static + ?const std::ops::Add,


error: `?const` and `?` are mutually exclusive
   |
   |
LL |     T: ?const ?for<'a: 'b> m::Trait<'a>,

error: lifetime bounds cannot be used in this context
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/const-trait-bound-opt-out/syntax.rs:9:24
   |
   |
LL |     T: ?const ?for<'a: 'b> m::Trait<'a>,


error[E0403]: the name `T` is already used for a generic parameter in this item's generic parameters
   |
   |
LL |     T: ?const ?for<'a> Tr<'a> + 'static + ?const std::ops::Add,
   |     - first use of `T`
LL |     T: ?const ?for<'a: 'b> m::Trait<'a>,
   |     ^ already used
error[E0433]: failed to resolve: use of undeclared crate or module `m`
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/const-trait-bound-opt-out/syntax.rs:9:28
   |
   |
LL |     T: ?const ?for<'a: 'b> m::Trait<'a>,
   |                            ^ use of undeclared crate or module `m`

error[E0405]: cannot find trait `Tr` in this scope
   |
   |
LL |     T: ?const ?for<'a> Tr<'a> + 'static + ?const std::ops::Add,
   |                        ^^ not found in this scope
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0403, E0405, E0433.
For more information about an error, try `rustc --explain E0403`.
For more information about an error, try `rustc --explain E0403`.

------------------------------------------


---- [ui] ui/rfc-2632-const-trait-impl/syntax.rs stdout ----
normalized stderr:
error: `?const` is not permitted in trait objects
   |
   |
LL | impl ?const T {}


error[E0405]: cannot find trait `T` in this scope
   |
   |
LL | impl ?const T {}
   |             ^ not found in this scope
warning: trait objects without an explicit `dyn` are deprecated
  --> $DIR/syntax.rs:9:6
   |
   |
LL | impl ?const T {}
   |      ^^^^^^^^ help: use `dyn`: `dyn ?const T`
   = note: `#[warn(bare_trait_objects)]` on by default

error: aborting due to 2 previous errors; 1 warning emitted


For more information about this error, try `rustc --explain E0405`.



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/syntax/syntax.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/syntax.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/syntax.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/syntax" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/syntax/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `?const` is not permitted in trait objects
   |
   |
LL | impl ?const T {}


error[E0405]: cannot find trait `T` in this scope
   |
   |
LL | impl ?const T {}
   |             ^ not found in this scope
warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/syntax.rs:9:6
   |
   |
LL | impl ?const T {}
   |      ^^^^^^^^ help: use `dyn`: `dyn ?const T`
   = note: `#[warn(bare_trait_objects)]` on by default

error: aborting due to 2 previous errors; 1 warning emitted

---
test result: FAILED. 11182 passed; 8 failed; 87 ignored; 0 measured; 0 filtered out; finished in 134.41s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:23
