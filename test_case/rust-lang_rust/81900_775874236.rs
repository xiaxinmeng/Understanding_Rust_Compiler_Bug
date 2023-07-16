plain
.................................................................................................... 10100/11437
.................................................................................................... 10200/11437
.................................................................................................... 10300/11437
........................i........................................................................... 10400/11437
.............F.F............F.....F............................................................F.... 10500/11437
.......F........F...............................................F................................... 10600/11437
.....................F.......................F...................................................... 10700/11437
.................................................................................................... 10900/11437
.................................................................................................... 11000/11437
.................................................................................................... 11100/11437
.................................................................................................... 11200/11437
.................................................................................................... 11200/11437
.................................................................................................... 11300/11437
...........................i.i...................................................................... 11400/11437
.....................................
failures:

---- [ui] ui/traits/alias/ambiguous.rs stdout ----

1 error[E0034]: multiple applicable items in scope
-   --> $DIR/alias-ambiguous.rs:21:7
+   --> $DIR/ambiguous.rs:21:7
+   --> $DIR/ambiguous.rs:21:7
3    |
4 LL |     t.foo();
5    |       ^^^ multiple `foo` found
6    |
6    |
7 note: candidate #1 is defined in an impl of the trait `A` for the type `u8`
-   --> $DIR/alias-ambiguous.rs:8:9
+   --> $DIR/ambiguous.rs:8:9
9    |
10 LL |         fn foo(&self) {}


12 note: candidate #2 is defined in an impl of the trait `B` for the type `u8`
-   --> $DIR/alias-ambiguous.rs:11:9
+   --> $DIR/ambiguous.rs:11:9
14    |
15 LL |         fn foo(&self) {}


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/ambiguous/ambiguous.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/alias/ambiguous.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/alias/ambiguous.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/ambiguous" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/ambiguous/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0034]: multiple applicable items in scope
  --> /checkout/src/test/ui/traits/alias/ambiguous.rs:21:7
   |
LL |     t.foo(); //~ ERROR E0034
   |       ^^^ multiple `foo` found
   |
note: candidate #1 is defined in an impl of the trait `A` for the type `u8`
  --> /checkout/src/test/ui/traits/alias/ambiguous.rs:8:9
   |
LL |         fn foo(&self) {}
   |         ^^^^^^^^^^^^^
note: candidate #2 is defined in an impl of the trait `B` for the type `u8`
  --> /checkout/src/test/ui/traits/alias/ambiguous.rs:11:9
   |
LL |         fn foo(&self) {}
help: disambiguate the associated function for candidate #1
   |
   |
LL |     A::foo(&t); //~ ERROR E0034
help: disambiguate the associated function for candidate #2
   |
   |
LL |     B::foo(&t); //~ ERROR E0034

error: aborting due to previous error

For more information about this error, try `rustc --explain E0034`.
For more information about this error, try `rustc --explain E0034`.

------------------------------------------


---- [ui] ui/traits/alias/impl.rs stdout ----
diff of stderr:

1 error[E0404]: expected trait, found trait alias `DefaultAlias`
-   --> $DIR/alias-impl.rs:5:6
3    |
3    |
4 LL | impl DefaultAlias for () {}
5    |      ^^^^^^^^^^^^ not a trait

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/impl/impl.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/impl/impl.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/alias/impl.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/alias/impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/impl" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/impl/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0404]: expected trait, found trait alias `DefaultAlias`
   |
   |
LL | impl DefaultAlias for () {} //~ ERROR expected trait, found trait alias
   |      ^^^^^^^^^^^^ not a trait
error: aborting due to previous error

For more information about this error, try `rustc --explain E0404`.


------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


---- [ui] ui/traits/alias/only-maybe-bound.rs stdout ----

1 error[E0224]: at least one trait is required for an object type
-   --> $DIR/alias-only-maybe-bound.rs:13:12
+   --> $DIR/only-maybe-bound.rs:13:12
+   --> $DIR/only-maybe-bound.rs:13:12
3    |
4 LL | type _T0 = dyn _1;

6 
7 error[E0224]: at least one trait is required for an object type
-   --> $DIR/alias-only-maybe-bound.rs:19:12
-   --> $DIR/alias-only-maybe-bound.rs:19:12
+   --> $DIR/only-maybe-bound.rs:19:12
9    |
10 LL | type _T1 = dyn _2;


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/only-maybe-bound/only-maybe-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/alias/only-maybe-bound.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/alias/only-maybe-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/only-maybe-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/only-maybe-bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0224]: at least one trait is required for an object type
  --> /checkout/src/test/ui/traits/alias/only-maybe-bound.rs:13:12
   |
LL | type _T0 = dyn _1;

error[E0224]: at least one trait is required for an object type
  --> /checkout/src/test/ui/traits/alias/only-maybe-bound.rs:19:12
   |
   |
LL | type _T1 = dyn _2;

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0224`.
For more information about this error, try `rustc --explain E0224`.

------------------------------------------


---- [ui] ui/traits/assoc-type-in-superbad.rs stdout ----


1 error[E0271]: type mismatch resolving `<std::vec::IntoIter<i32> as Iterator>::Item == u32`
-   --> $DIR/assoc-type-in-supertrait-bad.rs:12:16
+   --> $DIR/assoc-type-in-superbad.rs:12:16
4 LL |     type Key = u32;
4 LL |     type Key = u32;
5    |                ^^^ expected `i32`, found `u32`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/assoc-type-in-superbad/assoc-type-in-superbad.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/assoc-type-in-superbad.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/assoc-type-in-superbad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/assoc-type-in-superbad" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/assoc-type-in-superbad/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<std::vec::IntoIter<i32> as Iterator>::Item == u32`
  --> /checkout/src/test/ui/traits/assoc-type-in-superbad.rs:12:16
   |
LL |     type Key = u32; //~ ERROR type mismatch
   |                ^^^ expected `i32`, found `u32`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.


------------------------------------------


---- [ui] ui/traits/default-method/xc.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/default-method/xc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/default-method/xc/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/default-method/xc/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0463]: can't find crate for `trait_default_method_xc_aux`
  --> /checkout/src/test/ui/traits/default-method/xc.rs:8:1
   |
LL | extern crate trait_default_method_xc_aux as aux;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.


------------------------------------------


---- [ui] ui/traits/impl-of-supertrait-has-wrong-lifetime-parameters.rs stdout ----
diff of stderr:

1 error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'b` due to conflicting requirements
-   --> $DIR/impl-of-superhas-wrong-lifetime-parameters.rs:24:13
3    |
3    |
4 LL | impl<'a,'b> T2<'a, 'b> for S<'a, 'b> {

6    |
6    |
7 note: first, the lifetime cannot outlive the lifetime `'a` as defined on the impl at 24:6...
-   --> $DIR/impl-of-superhas-wrong-lifetime-parameters.rs:24:6
9    |
9    |
10 LL | impl<'a,'b> T2<'a, 'b> for S<'a, 'b> {


12 note: ...but the lifetime must also be valid for the lifetime `'b` as defined on the impl at 24:9...
-   --> $DIR/impl-of-superhas-wrong-lifetime-parameters.rs:24:9
14    |
14    |
15 LL | impl<'a,'b> T2<'a, 'b> for S<'a, 'b> {


17 note: ...so that the types are compatible
-   --> $DIR/impl-of-superhas-wrong-lifetime-parameters.rs:24:13
19    |
19    |
20 LL | impl<'a,'b> T2<'a, 'b> for S<'a, 'b> {


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-of-supertrait-has-wrong-lifetime-parameters/impl-of-supertrait-has-wrong-lifetime-parameters.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-of-supertrait-has-wrong-lifetime-parameters/impl-of-supertrait-has-wrong-lifetime-parameters.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/impl-of-supertrait-has-wrong-lifetime-parameters.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/impl-of-supertrait-has-wrong-lifetime-parameters.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-of-supertrait-has-wrong-lifetime-parameters" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/impl-of-supertrait-has-wrong-lifetime-parameters/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'b` due to conflicting requirements
   |
   |
LL | impl<'a,'b> T2<'a, 'b> for S<'a, 'b> { //~ ERROR cannot infer an appropriate lifetime
   |
   |
note: first, the lifetime cannot outlive the lifetime `'a` as defined on the impl at 24:6...
   |
   |
LL | impl<'a,'b> T2<'a, 'b> for S<'a, 'b> { //~ ERROR cannot infer an appropriate lifetime
   |      ^^
note: ...but the lifetime must also be valid for the lifetime `'b` as defined on the impl at 24:9...
   |
   |
LL | impl<'a,'b> T2<'a, 'b> for S<'a, 'b> { //~ ERROR cannot infer an appropriate lifetime
   |         ^^
note: ...so that the types are compatible
   |
   |
LL | impl<'a,'b> T2<'a, 'b> for S<'a, 'b> { //~ ERROR cannot infer an appropriate lifetime
   |             ^^^^^^^^^^
   = note: expected `T1<'a>`
              found `T1<'_>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0495`.


------------------------------------------


---- [ui] ui/traits/inductive-overflow/superauto-trait.rs stdout ----

1 error[E0568]: auto traits cannot have super traits
-   --> $DIR/supertrait-auto-trait.rs:8:19
+   --> $DIR/superauto-trait.rs:8:19
+   --> $DIR/superauto-trait.rs:8:19
3    |
4 LL | auto trait Magic: Copy {}
5    |            -----  ^^^^ help: remove the super traits
7    |            auto trait cannot have super traits
8 
8 
9 error[E0277]: the trait bound `NoClone: Copy` is not satisfied
-   --> $DIR/supertrait-auto-trait.rs:16:23
+   --> $DIR/superauto-trait.rs:16:23
11    |
12 LL | fn copy<T: Magic>(x: T) -> (T, T) { (x, x) }
13    |            ----- required by this bound in `copy`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/superauto-trait/superauto-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/inductive-overflow/superauto-trait.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/inductive-overflow/superauto-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/superauto-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/superauto-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0568]: auto traits cannot have super traits
  --> /checkout/src/test/ui/traits/inductive-overflow/superauto-trait.rs:8:19
   |
LL | auto trait Magic: Copy {} //~ ERROR E0568
   |            -----  ^^^^ help: remove the super traits
   |            auto trait cannot have super traits


error[E0277]: the trait bound `NoClone: Copy` is not satisfied
  --> /checkout/src/test/ui/traits/inductive-overflow/superauto-trait.rs:16:23
   |
LL | fn copy<T: Magic>(x: T) -> (T, T) { (x, x) }
   |            ----- required by this bound in `copy`
...
LL |     let (a, b) = copy(NoClone); //~ ERROR
   |                       ^^^^^^^ the trait `Copy` is not implemented for `NoClone`
   |
   = note: required because of the requirements on the impl of `Magic` for `NoClone`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0568.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/traits/issue-65284-suggest-generic-bound.rs stdout ----
diff of stderr:

1 error[E0599]: no method named `foo` found for type parameter `T` in the current scope
-   --> $DIR/issue-65284-suggest-generic-trait-bound.rs:8:7
3    |
3    |
4 LL |     t.foo()
5    |       ^^^ method not found in `T`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-65284-suggest-generic-bound/issue-65284-suggest-generic-bound.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-65284-suggest-generic-bound/issue-65284-suggest-generic-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/issue-65284-suggest-generic-bound.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-65284-suggest-generic-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-65284-suggest-generic-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-65284-suggest-generic-bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: no method named `foo` found for type parameter `T` in the current scope
   |
   |
LL |     t.foo() //~ ERROR no method named `foo` found
   |       ^^^ method not found in `T`
   = help: items from traits can only be used if the type parameter is bounded by the trait
   = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following trait defines an item `foo`, perhaps you need to restrict type parameter `T` with it:
   |
LL | fn do_stuff<T: Foo + Bar>(t : T) {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.

------------------------------------------


---- [ui] ui/traits/repeated-superambig.rs stdout ----


1 error[E0277]: the trait bound `dyn CompareToInts: CompareTo<i32>` is not satisfied
-   --> $DIR/repeated-supertrait-ambig.rs:26:7
+   --> $DIR/repeated-superambig.rs:26:7
3    |
4 LL |     c.same_as(22)
5    |       ^^^^^^^ the trait `CompareTo<i32>` is not implemented for `dyn CompareToInts`
6 
6 
7 error[E0277]: the trait bound `C: CompareTo<i32>` is not satisfied
-   --> $DIR/repeated-supertrait-ambig.rs:30:7
+   --> $DIR/repeated-superambig.rs:30:7
9    |
10 LL |     c.same_as(22)
11    |       ^^^^^^^ the trait `CompareTo<i32>` is not implemented for `C`
16    |                               ^^^^^^^^^^^^^^^^
17 
17 
18 error[E0277]: the trait bound `dyn CompareToInts: CompareTo<i32>` is not satisfied
-   --> $DIR/repeated-supertrait-ambig.rs:34:5
+   --> $DIR/repeated-superambig.rs:34:5
20    |
21 LL |     fn same_as(&self, t: T) -> bool;
22    |     -------------------------------- required by `CompareTo::same_as`

25    |     ^^^^^^^^^^^^^^^^^^^^^^ the trait `CompareTo<i32>` is not implemented for `dyn CompareToInts`
26 
27 error[E0277]: the trait bound `C: CompareTo<i32>` is not satisfied
-   --> $DIR/repeated-supertrait-ambig.rs:38:5
+   --> $DIR/repeated-superambig.rs:38:5
29    |
30 LL |     fn same_as(&self, t: T) -> bool;
31    |     -------------------------------- required by `CompareTo::same_as`
39    |                               ^^^^^^^^^^^^^^^^
40 
40 
41 error[E0277]: the trait bound `i64: CompareTo<i32>` is not satisfied
-   --> $DIR/repeated-supertrait-ambig.rs:42:23
+   --> $DIR/repeated-superambig.rs:42:23
43    |
44 LL |     assert_eq!(22_i64.same_as(22), true);
45    |                       ^^^^^^^ the trait `CompareTo<i32>` is not implemented for `i64`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/repeated-superambig/repeated-superambig.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/repeated-superambig.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/repeated-superambig.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/repeated-superambig" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/repeated-superambig/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `dyn CompareToInts: CompareTo<i32>` is not satisfied
  --> /checkout/src/test/ui/traits/repeated-superambig.rs:26:7
   |
LL |     c.same_as(22) //~ ERROR `dyn CompareToInts: CompareTo<i32>` is not satisfied
   |       ^^^^^^^ the trait `CompareTo<i32>` is not implemented for `dyn CompareToInts`

error[E0277]: the trait bound `C: CompareTo<i32>` is not satisfied
  --> /checkout/src/test/ui/traits/repeated-superambig.rs:30:7
   |
LL |     c.same_as(22) //~ ERROR `C: CompareTo<i32>` is not satisfied
   |       ^^^^^^^ the trait `CompareTo<i32>` is not implemented for `C`
help: consider further restricting this bound
   |
   |
LL | fn with_trait<C:CompareToInts + CompareTo<i32>>(c: &C) -> bool {
---
    [ui] ui/traits/alias/only-maybe-bound.rs
    [ui] ui/traits/assoc-type-in-superbad.rs
    [ui] ui/traits/default-method/xc.rs
    [ui] ui/traits/impl-of-supertrait-has-wrong-lifetime-parameters.rs
    [ui] ui/traits/inductive-overflow/superauto-trait.rs
    [ui] ui/traits/repeated-superambig.rs
    [ui] ui/traits/suggest-deferences/issue-39029.rs

test result: FAILED. 11335 passed; 10 failed; 92 ignored; 0 measured; 0 filtered out; finished in 137.25s
test result: FAILED. 11335 passed; 10 failed; 92 ignored; 0 measured; 0 filtered out; finished in 137.25s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:36
