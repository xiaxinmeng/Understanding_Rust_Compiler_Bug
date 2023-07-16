plain
.................................................................................................... 100/12541
............................................iiiiiiiiiiii..........i.i................i...i.......... 200/12541
.................................................................................................... 300/12541
.................................................................................................... 400/12541
..............................................................................F.......F............. 500/12541
..F..............FF................................................................................. 600/12541
...i............................................................................i................... 800/12541
.................................................................................................... 900/12541
.................................................................................................... 1000/12541
.................................................................................................... 1100/12541
---
................................F...........................................ii...................... 3800/12541
.................................................................................................... 3900/12541
.i.................................................................................................. 4000/12541
.................................................................................................... 4100/12541
..F....F.......F.F.F..FF..F.F........F....................F....F..F......................F.......... 4200/12541
.................................................................................................... 4400/12541
.................................................................................................... 4500/12541
.................................................................................................... 4600/12541
.................................................................................................... 4700/12541
---
.............i...................................................................................... 10300/12541
.........iiiiii.i..iiiiii.i......................................................................... 10400/12541
.................................................................................................... 10500/12541
.................................................................................................... 10600/12541
........................FF.F.........F..F.......F....F.............................................. 10700/12541
.................................................................................................... 10900/12541
.................................................................................................... 11000/12541
.................................................................................................... 11100/12541
........................................................................ii.......................... 11200/12541
........................................................................ii.......................... 11200/12541
...i................................................................................................ 11300/12541
.................................................................................................... 11400/12541
.....................F......................................................F....................... 11500/12541
.................................................................................................... 11700/12541
..................................................................i................................. 11800/12541
.................................................................................................... 11900/12541
.................................................................................................... 12000/12541
---

---- [ui] ui/associated-types/issue-43784-associated-type.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `T: Copy` is not satisfied
-   --> $DIR/issue-43784-associated-type.rs:14:18
3    |
4 LL |     type Assoc = T;
-    |                  ^ the trait `Copy` is not implemented for `T`
+    |     ^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `T`
---
To only update this specific test, also pass `--test-args associated-types/issue-43784-associated-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-43784-associated-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-43784-associated-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-43784-associated-type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `T: Copy` is not satisfied
   |
   |
LL |     type Assoc = T; //~ ERROR the trait bound `T: Copy` is not satisfied
   |     ^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `T`
note: required by a bound in `Complete::Assoc`
  --> /checkout/src/test/ui/associated-types/issue-43784-associated-type.rs:5:17
   |
   |
LL |     type Assoc: Partial<Self>;
   |                 ^^^^^^^^^^^^^ required by this bound in `Complete::Assoc`
help: consider restricting type parameter `T`
   |
LL | impl<T: std::marker::Copy> Complete for T {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/associated-types/issue-54108.rs stdout ----
diff of stderr:

1 error[E0277]: cannot add `<T as SubEncoder>::ActualSize` to `<T as SubEncoder>::ActualSize`
-   --> $DIR/issue-54108.rs:19:17
3    |
3    |
4 LL |     type Size = <Self as SubEncoder>::ActualSize;
-    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `<T as SubEncoder>::ActualSize + <T as SubEncoder>::ActualSize`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `<T as SubEncoder>::ActualSize + <T as SubEncoder>::ActualSize`
6    |
7    = help: the trait `Add` is not implemented for `<T as SubEncoder>::ActualSize`
8 note: required by a bound in `Encoder::Size`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-54108/issue-54108.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/issue-54108.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-54108.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-54108" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-54108/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: cannot add `<T as SubEncoder>::ActualSize` to `<T as SubEncoder>::ActualSize`
   |
   |
LL |     type Size = <Self as SubEncoder>::ActualSize;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `<T as SubEncoder>::ActualSize + <T as SubEncoder>::ActualSize`
   |
   = help: the trait `Add` is not implemented for `<T as SubEncoder>::ActualSize`
note: required by a bound in `Encoder::Size`
   |
   |
LL |     type Size: Add<Output = Self::Size>;
   |                ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Encoder::Size`
   |
   |
LL |     T: SubEncoder, <T as SubEncoder>::ActualSize: Add

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/associated-types/issue-72806.rs stdout ----
diff of stderr:

1 error[E0271]: type mismatch resolving `<Foo2 as Bar2>::Ok == char`
-   --> $DIR/issue-72806.rs:14:20
3    |
4 LL |     type Sibling = Foo2;
4 LL |     type Sibling = Foo2;
-    |                    ^^^^ type mismatch resolving `<Foo2 as Bar2>::Ok == char`
+    |     ^^^^^^^^^^^^^^^^^^^^ type mismatch resolving `<Foo2 as Bar2>::Ok == char`
7 note: expected this to be `char`
8   --> $DIR/issue-72806.rs:18:15



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-72806/issue-72806.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/issue-72806.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-72806.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-72806" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-72806/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<Foo2 as Bar2>::Ok == char`
   |
LL |     type Sibling = Foo2;
LL |     type Sibling = Foo2;
   |     ^^^^^^^^^^^^^^^^^^^^ type mismatch resolving `<Foo2 as Bar2>::Ok == char`
note: expected this to be `char`
  --> /checkout/src/test/ui/associated-types/issue-72806.rs:18:15
   |
LL |     type Ok = u32;
LL |     type Ok = u32;
   |               ^^^
note: required by a bound in `Bar::Sibling`
   |
   |
LL |     type Sibling: Bar2<Ok=char>;
   |                        ^^^^^^^ required by this bound in `Bar::Sibling`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.


------------------------------------------


---- [ui] ui/associated-types/point-at-type-on-obligation-failure-2.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `bool: Bar` is not satisfied
-   --> $DIR/point-at-type-on-obligation-failure-2.rs:8:18
3    |
4 LL |     type Assoc = bool;
4 LL |     type Assoc = bool;
-    |                  ^^^^ the trait `Bar` is not implemented for `bool`
+    |     ^^^^^^^^^^^^^^^^^^ the trait `Bar` is not implemented for `bool`
7 note: required by a bound in `Foo::Assoc`
8   --> $DIR/point-at-type-on-obligation-failure-2.rs:4:17


11    |                 ^^^ required by this bound in `Foo::Assoc`
12 
13 error[E0277]: the trait bound `bool: Bar` is not satisfied
-   --> $DIR/point-at-type-on-obligation-failure-2.rs:19:18
15    |
16 LL |     type Assoc = bool;
16 LL |     type Assoc = bool;
-    |                  ^^^^ the trait `Bar` is not implemented for `bool`
+    |     ^^^^^^^^^^^^^^^^^^ the trait `Bar` is not implemented for `bool`
18    |
19 note: required by a bound in `Baz::Assoc`

26    |          ----- required by a bound in this
27 
27 
28 error[E0277]: the trait bound `bool: Bar` is not satisfied
-   --> $DIR/point-at-type-on-obligation-failure-2.rs:30:18
30    |
31 LL |     type Assoc = bool;
31 LL |     type Assoc = bool;
-    |                  ^^^^ the trait `Bar` is not implemented for `bool`
+    |     ^^^^^^^^^^^^^^^^^^ the trait `Bar` is not implemented for `bool`
33    |
34 note: required by a bound in `Bat::Assoc`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/point-at-type-on-obligation-failure-2/point-at-type-on-obligation-failure-2.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/point-at-type-on-obligation-failure-2/point-at-type-on-obligation-failure-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/point-at-type-on-obligation-failure-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/point-at-type-on-obligation-failure-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/point-at-type-on-obligation-failure-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/point-at-type-on-obligation-failure-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `bool: Bar` is not satisfied
   |
   |
LL |     type Assoc = bool; //~ ERROR the trait bound `bool: Bar` is not satisfied
   |     ^^^^^^^^^^^^^^^^^^ the trait `Bar` is not implemented for `bool`
note: required by a bound in `Foo::Assoc`
  --> /checkout/src/test/ui/associated-types/point-at-type-on-obligation-failure-2.rs:4:17
   |
   |
LL |     type Assoc: Bar;
   |                 ^^^ required by this bound in `Foo::Assoc`

error[E0277]: the trait bound `bool: Bar` is not satisfied
   |
   |
LL |     type Assoc = bool; //~ ERROR the trait bound `bool: Bar` is not satisfied
   |     ^^^^^^^^^^^^^^^^^^ the trait `Bar` is not implemented for `bool`
   |
note: required by a bound in `Baz::Assoc`
   |
   |
LL |     Self::Assoc: Bar,
   |                  ^^^ required by this bound in `Baz::Assoc`
LL | {
LL |     type Assoc;
   |          ----- required by a bound in this

error[E0277]: the trait bound `bool: Bar` is not satisfied
   |
   |
LL |     type Assoc = bool; //~ ERROR the trait bound `bool: Bar` is not satisfied
   |     ^^^^^^^^^^^^^^^^^^ the trait `Bar` is not implemented for `bool`
   |
note: required by a bound in `Bat::Assoc`
   |
   |
LL |     <Self as Bat>::Assoc: Bar,
   |                           ^^^ required by this bound in `Bat::Assoc`
LL | {
LL |     type Assoc;
   |          ----- required by a bound in this
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/associated-types/point-at-type-on-obligation-failure.rs stdout ----
diff of stderr:

1 error[E0271]: type mismatch resolving `<Foo2 as Bar2>::Ok == ()`
-   --> $DIR/point-at-type-on-obligation-failure.rs:14:20
3    |
4 LL |     type Sibling = Foo2;
4 LL |     type Sibling = Foo2;
-    |                    ^^^^ type mismatch resolving `<Foo2 as Bar2>::Ok == ()`
+    |     ^^^^^^^^^^^^^^^^^^^^ type mismatch resolving `<Foo2 as Bar2>::Ok == ()`
7 note: expected this to be `()`
8   --> $DIR/point-at-type-on-obligation-failure.rs:18:15



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/point-at-type-on-obligation-failure/point-at-type-on-obligation-failure.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/point-at-type-on-obligation-failure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/point-at-type-on-obligation-failure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/point-at-type-on-obligation-failure" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/point-at-type-on-obligation-failure/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<Foo2 as Bar2>::Ok == ()`
   |
LL |     type Sibling = Foo2;
LL |     type Sibling = Foo2;
   |     ^^^^^^^^^^^^^^^^^^^^ type mismatch resolving `<Foo2 as Bar2>::Ok == ()`
note: expected this to be `()`
  --> /checkout/src/test/ui/associated-types/point-at-type-on-obligation-failure.rs:18:15
   |
LL |     type Ok = u32;
LL |     type Ok = u32;
   |               ^^^
note: required by a bound in `Bar::Sibling`
   |
   |
LL |     type Sibling: Bar2<Ok=Self::Ok>;
   |                        ^^^^^^^^^^^ required by this bound in `Bar::Sibling`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.


------------------------------------------


---- [ui] ui/chalkify/impl_wf_2.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `f32: Foo` is not satisfied
-   --> $DIR/impl_wf_2.rs:25:17
3    |
4 LL |     type Item = f32;
-    |                 ^^^ the trait `Foo` is not implemented for `f32`
+    |     ^^^^^^^^^^^^^^^^ the trait `Foo` is not implemented for `f32`
+    |     ^^^^^^^^^^^^^^^^ the trait `Foo` is not implemented for `f32`
6    |
7 note: required by a bound in `Bar::Item`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/impl_wf_2/impl_wf_2.stderr
To only update this specific test, also pass `--test-args chalkify/impl_wf_2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/chalkify/impl_wf_2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/impl_wf_2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "chalk" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/impl_wf_2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `f32: Foo` is not satisfied
   |
LL |     type Item = f32;
   |     ^^^^^^^^^^^^^^^^ the trait `Foo` is not implemented for `f32`
   |
   |
note: required by a bound in `Bar::Item`
   |
LL |     type Item: Foo;
LL |     type Item: Foo;
   |                ^^^ required by this bound in `Bar::Item`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/const-generics/associated-type-bound-fail.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `u16: Bar<N>` is not satisfied
-   --> $DIR/associated-type-bound-fail.rs:9:18
3    |
4 LL |     type Assoc = u16;
4 LL |     type Assoc = u16;
-    |                  ^^^ the trait `Bar<N>` is not implemented for `u16`
+    |     ^^^^^^^^^^^^^^^^^ the trait `Bar<N>` is not implemented for `u16`
7    = help: the following implementations were found:
7    = help: the following implementations were found:
8              <u16 as Bar<3_usize>>

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/associated-type-bound-fail/associated-type-bound-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/associated-type-bound-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/associated-type-bound-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/associated-type-bound-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/associated-type-bound-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error[E0277]: the trait bound `u16: Bar<N>` is not satisfied
   |
   |
LL |     type Assoc = u16; //~ ERROR the trait bound `u16: Bar<N>`
   |     ^^^^^^^^^^^^^^^^^ the trait `Bar<N>` is not implemented for `u16`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <u16 as Bar<3_usize>>
note: required by a bound in `Foo::Assoc`
   |
   |
LL |     type Assoc: Bar<N>;
   |                 ^^^^^^ required by this bound in `Foo::Assoc`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.

---
diff of stderr:

62    = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable
63 
64 error[E0277]: the trait bound `U32: Clone` is not satisfied
-   --> $DIR/feature-gate-generic_associated_types.rs:16:26
66    |
66    |
67 LL |     type Pointer2<U32> = Box<U32>;
-    |                          ^^^^^^^^ the trait `Clone` is not implemented for `U32`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `U32`
70 help: consider restricting type parameter `U32`
71    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-generic_associated_types/feature-gate-generic_associated_types.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/feature-gate-generic_associated_types.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-generic_associated_types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-generic_associated_types" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-generic_associated_types/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: generic associated types are unstable
   |
   |
LL |     type Pointer<T>: Deref<Target = T>;
   |
   = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
   = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable


error[E0658]: generic associated types are unstable
   |
   |
LL |     type Pointer2<T>: Deref<Target = T> where T: Clone, U: Clone;
   |
   = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
   = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable


error[E0658]: where clauses on associated types are unstable
   |
   |
LL |     type Pointer2<T>: Deref<Target = T> where T: Clone, U: Clone;
   |
   = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
   = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable


error[E0658]: generic associated types are unstable
   |
   |
LL |     type Pointer<Usize> = Box<Usize>;
   |
   = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
   = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable


error[E0658]: generic associated types are unstable
   |
   |
LL |     type Pointer2<U32> = Box<U32>;
   |
   = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
   = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable


error[E0658]: where clauses on associated types are unstable
   |
   |
LL |     type Assoc where Self: Sized;
   |
   = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
   = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable


error[E0658]: where clauses on associated types are unstable
   |
   |
LL |     type Assoc where Self: Sized = Foo;
   |
   = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
   = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable


error[E0277]: the trait bound `U32: Clone` is not satisfied
   |
   |
LL |     type Pointer2<U32> = Box<U32>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `U32`
help: consider restricting type parameter `U32`
   |
   |
LL |     type Pointer2<U32: std::clone::Clone> = Box<U32>;

error: aborting due to 8 previous errors

Some errors have detailed explanations: E0277, E0658.
---

---- [ui] ui/generic-associated-types/cross-crate-bounds.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `(): AsRef<()>` is not satisfied
-   --> $DIR/cross-crate-bounds.rs:15:16
3    |
4 LL |     type Bar = ();
4 LL |     type Bar = ();
-    |                ^^ the trait `AsRef<()>` is not implemented for `()`
+    |     ^^^^^^^^^^^^^^ the trait `AsRef<()>` is not implemented for `()`
6    |
7 note: required by a bound in `foo_defn::Foo::Bar`
8   --> $DIR/auxiliary/foo_defn.rs:6:15

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/cross-crate-bounds/cross-crate-bounds.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/cross-crate-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/cross-crate-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/cross-crate-bounds" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/cross-crate-bounds/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `(): AsRef<()>` is not satisfied
   |
LL |     type Bar = ();
LL |     type Bar = ();
   |     ^^^^^^^^^^^^^^ the trait `AsRef<()>` is not implemented for `()`
   |
note: required by a bound in `foo_defn::Foo::Bar`
  --> /checkout/src/test/ui/generic-associated-types/auxiliary/foo_defn.rs:6:15
   |
LL |     type Bar: AsRef<()>;
   |               ^^^^^^^^^ required by this bound in `foo_defn::Foo::Bar`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.

---
1 error[E0277]: `T` doesn't implement `std::fmt::Display`
-   --> $DIR/generic-associated-types-where.rs:20:22
+   --> $DIR/generic-associated-types-where.rs:20:5
3    |
4 LL |     type Assoc2<T> = Vec<T>;
-    |                      ^^^^^^ `T` cannot be formatted with the default formatter
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^ `T` cannot be formatted with the default formatter
6    |
7    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
8 help: consider restricting type parameter `T`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/generic-associated-types-where/generic-associated-types-where.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/generic-associated-types-where.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/generic-associated-types-where.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/generic-associated-types-where" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/generic-associated-types-where/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `T` doesn't implement `std::fmt::Display`
  --> /checkout/src/test/ui/generic-associated-types/generic-associated-types-where.rs:20:5
   |
LL |     type Assoc2<T> = Vec<T>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ `T` cannot be formatted with the default formatter
   |
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
help: consider restricting type parameter `T`
   |
LL |     type Assoc2<T: std::fmt::Display> = Vec<T>;

error[E0276]: impl has stricter requirements than trait
  --> /checkout/src/test/ui/generic-associated-types/generic-associated-types-where.rs:22:5
   |
   |
LL |     type Assoc3<T>;
   |     --------------- definition of `Assoc3` from trait
...
LL |     type Assoc3<T> where T: Iterator = Vec<T>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl has extra requirement `T: Iterator`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0276, E0277.
For more information about an error, try `rustc --explain E0276`.
---
diff of stderr:

17    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ found
18 
19 error[E0478]: lifetime bound not satisfied
-   --> $DIR/impl_bounds.rs:17:35
21    |
21    |
22 LL |     type B<'a, 'b> where 'b: 'a = (&'a(), &'b ());
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
24    |
24    |
25 note: lifetime parameter instantiated with the lifetime `'a` as defined here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/impl_bounds/impl_bounds.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/impl_bounds/impl_bounds.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/impl_bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/impl_bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/impl_bounds" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/impl_bounds/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `impl` associated type signature for `A` doesn't match `trait` associated type signature
   |
   |
LL |     type A<'a> where Self: 'a;
...
...
LL |     type A<'a> where Self: 'static = (&'a ());
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ found

error: `impl` associated type signature for `B` doesn't match `trait` associated type signature
   |
   |
LL |     type B<'a, 'b> where 'a: 'b;
...
...
LL |     type B<'a, 'b> where 'b: 'a = (&'a(), &'b ());
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ found

error[E0478]: lifetime bound not satisfied
   |
   |
LL |     type B<'a, 'b> where 'b: 'a = (&'a(), &'b ());
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined here
   |
   |
LL |     type B<'a, 'b> where 'b: 'a = (&'a(), &'b ());
   |            ^^
note: but lifetime parameter must outlive the lifetime `'b` as defined here
   |
   |
LL |     type B<'a, 'b> where 'b: 'a = (&'a(), &'b ());


error[E0277]: the trait bound `T: Copy` is not satisfied
   |
   |
LL |     type C where Self: Copy = String;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `T`
   |
note: required because of the requirements on the impl of `Copy` for `Fooy<T>`
   |
LL | #[derive(Copy, Clone)]
   |          ^^^^
   |          ^^^^
note: the requirement `Fooy<T>: Copy` appears on the associated impl type `C` but not on the corresponding associated trait type
   |
LL | trait Foo {
   |       --- in this trait
...
...
LL |     type C where Self: Clone;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^ this trait associated type doesn't have the requirement `Fooy<T>: Copy`
   = note: this error originates in the derive macro `Copy` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider restricting type parameter `T`
   |
LL | impl<T: std::marker::Copy> Foo for Fooy<T> {


error[E0277]: the trait bound `T: Copy` is not satisfied
   |
   |
LL |     fn d() where Self: Copy {}
   |                        ^^^^ the trait `Copy` is not implemented for `T`
   |
note: required because of the requirements on the impl of `Copy` for `Fooy<T>`
   |
LL | #[derive(Copy, Clone)]
   |          ^^^^
   |          ^^^^
note: the requirement `Fooy<T>: Copy` appears on the impl method `d` but not on the corresponding trait method
   |
LL | trait Foo {
   |       --- in this trait
...
...
LL |     fn d() where Self: Clone;
   |        ^ this trait method doesn't have the requirement `Fooy<T>: Copy`
   = note: this error originates in the derive macro `Copy` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider restricting type parameter `T`
   |
LL | impl<T: std::marker::Copy> Foo for Fooy<T> {

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0277, E0478.
---

---- [ui] ui/generic-associated-types/issue-68641-check-gat-bounds.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `T: Copy` is not satisfied
-   --> $DIR/issue-68641-check-gat-bounds.rs:14:21
3    |
3    |
4 LL |     type Item<'a> = T;
-    |                     ^ the trait `Copy` is not implemented for `T`
+    |     ^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `T`
6    |
7 note: required by a bound in `UnsafeCopy::Item`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68641-check-gat-bounds/issue-68641-check-gat-bounds.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68641-check-gat-bounds/issue-68641-check-gat-bounds.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-68641-check-gat-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-68641-check-gat-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68641-check-gat-bounds" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68641-check-gat-bounds/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `T: Copy` is not satisfied
   |
   |
LL |     type Item<'a> = T;
   |     ^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `T`
   |
note: required by a bound in `UnsafeCopy::Item`
   |
   |
LL |     type Item<'a>: Copy;
   |                    ^^^^ required by this bound in `UnsafeCopy::Item`
help: consider restricting type parameter `T`
   |
LL | impl<T: std::marker::Copy> UnsafeCopy for T {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/generic-associated-types/issue-68642-broken-llvm-ir.rs stdout ----
diff of stderr:

1 error[E0277]: expected a `Fn<()>` closure, found `T`
-   --> $DIR/issue-68642-broken-llvm-ir.rs:14:18
3    |
3    |
4 LL |     type F<'a> = Self;
-    |                  ^^^^ expected an `Fn<()>` closure, found `T`
+    |     ^^^^^^^^^^^^^^^^^^ expected an `Fn<()>` closure, found `T`
6    |
7    = note: wrap the `T` in a closure with no arguments: `|| { /* code */ }`
8 note: required by a bound in `Fun::F`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68642-broken-llvm-ir/issue-68642-broken-llvm-ir.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-68642-broken-llvm-ir.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-68642-broken-llvm-ir.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68642-broken-llvm-ir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68642-broken-llvm-ir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: expected a `Fn<()>` closure, found `T`
   |
   |
LL |     type F<'a> = Self;
   |     ^^^^^^^^^^^^^^^^^^ expected an `Fn<()>` closure, found `T`
   |
   = note: wrap the `T` in a closure with no arguments: `|| { /* code */ }`
note: required by a bound in `Fun::F`
   |
   |
LL |     type F<'a>: Fn() -> u32;
   |                 ^^^^^^^^^^^ required by this bound in `Fun::F`
help: consider restricting type parameter `T`
   |
LL | impl<T: std::ops::Fn<()>> Fun for T {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/generic-associated-types/issue-68643-broken-mir.rs stdout ----
diff of stderr:

1 error[E0277]: expected a `Fn<()>` closure, found `T`
-   --> $DIR/issue-68643-broken-mir.rs:14:18
3    |
3    |
4 LL |     type F<'a> = Self;
-    |                  ^^^^ expected an `Fn<()>` closure, found `T`
+    |     ^^^^^^^^^^^^^^^^^^ expected an `Fn<()>` closure, found `T`
6    |
7    = note: wrap the `T` in a closure with no arguments: `|| { /* code */ }`
8 note: required by a bound in `Fun::F`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68643-broken-mir/issue-68643-broken-mir.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-68643-broken-mir.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-68643-broken-mir.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68643-broken-mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68643-broken-mir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: expected a `Fn<()>` closure, found `T`
   |
   |
LL |     type F<'a> = Self;
   |     ^^^^^^^^^^^^^^^^^^ expected an `Fn<()>` closure, found `T`
   |
   = note: wrap the `T` in a closure with no arguments: `|| { /* code */ }`
note: required by a bound in `Fun::F`
   |
   |
LL |     type F<'a>: Fn() -> u32;
   |                 ^^^^^^^^^^^ required by this bound in `Fun::F`
help: consider restricting type parameter `T`
   |
LL | impl<T: std::ops::Fn<()>> Fun for T {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/generic-associated-types/issue-68644-codegen-selection.rs stdout ----
diff of stderr:

1 error[E0277]: expected a `Fn<()>` closure, found `T`
-   --> $DIR/issue-68644-codegen-selection.rs:14:18
3    |
3    |
4 LL |     type F<'a> = Self;
-    |                  ^^^^ expected an `Fn<()>` closure, found `T`
+    |     ^^^^^^^^^^^^^^^^^^ expected an `Fn<()>` closure, found `T`
6    |
7    = note: wrap the `T` in a closure with no arguments: `|| { /* code */ }`
8 note: required by a bound in `Fun::F`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68644-codegen-selection/issue-68644-codegen-selection.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-68644-codegen-selection.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-68644-codegen-selection.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68644-codegen-selection" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68644-codegen-selection/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: expected a `Fn<()>` closure, found `T`
   |
   |
LL |     type F<'a> = Self;
   |     ^^^^^^^^^^^^^^^^^^ expected an `Fn<()>` closure, found `T`
   |
   = note: wrap the `T` in a closure with no arguments: `|| { /* code */ }`
note: required by a bound in `Fun::F`
   |
   |
LL |     type F<'a>: Fn() -> u32;
   |                 ^^^^^^^^^^^ required by this bound in `Fun::F`
help: consider restricting type parameter `T`
   |
LL | impl<T: std::ops::Fn<()>> Fun for T {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/generic-associated-types/issue-68645-codegen-fulfillment.rs stdout ----
diff of stderr:

1 error[E0277]: expected a `Fn<()>` closure, found `T`
-   --> $DIR/issue-68645-codegen-fulfillment.rs:14:18
3    |
3    |
4 LL |     type F<'a> = Self;
-    |                  ^^^^ expected an `Fn<()>` closure, found `T`
+    |     ^^^^^^^^^^^^^^^^^^ expected an `Fn<()>` closure, found `T`
6    |
7    = note: wrap the `T` in a closure with no arguments: `|| { /* code */ }`
8 note: required by a bound in `Fun::F`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68645-codegen-fulfillment/issue-68645-codegen-fulfillment.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-68645-codegen-fulfillment.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-68645-codegen-fulfillment.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68645-codegen-fulfillment" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68645-codegen-fulfillment/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: expected a `Fn<()>` closure, found `T`
   |
   |
LL |     type F<'a> = Self;
   |     ^^^^^^^^^^^^^^^^^^ expected an `Fn<()>` closure, found `T`
   |
   = note: wrap the `T` in a closure with no arguments: `|| { /* code */ }`
note: required by a bound in `Fun::F`
   |
   |
LL |     type F<'a>: Fn() -> u32;
   |                 ^^^^^^^^^^^ required by this bound in `Fun::F`
help: consider restricting type parameter `T`
   |
LL | impl<T: std::ops::Fn<()>> Fun for T {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/generic-associated-types/issue-68656-unsized-values.rs stdout ----
diff of stderr:

1 error[E0271]: type mismatch resolving `<T as Deref>::Target == T`
-   --> $DIR/issue-68656-unsized-values.rs:15:21
3    |
3    |
4 LL | impl<T: Copy + std::ops::Deref> UnsafeCopy<T> for T {
5    |      - this type parameter

6 LL |     type Item<'a> = T;
-    |                     ^ expected type parameter `T`, found associated type
+    |     ^^^^^^^^^^^^^^^^^^ expected type parameter `T`, found associated type
9    = note: expected type parameter `T`
10              found associated type `<T as Deref>::Target`



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68656-unsized-values/issue-68656-unsized-values.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-68656-unsized-values.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-68656-unsized-values.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68656-unsized-values" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68656-unsized-values/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<T as Deref>::Target == T`
   |
   |
LL | impl<T: Copy + std::ops::Deref> UnsafeCopy<T> for T {
   |      - this type parameter
LL |     type Item<'a> = T;
   |     ^^^^^^^^^^^^^^^^^^ expected type parameter `T`, found associated type
   = note: expected type parameter `T`
             found associated type `<T as Deref>::Target`
             found associated type `<T as Deref>::Target`
note: required by a bound in `UnsafeCopy::Item`
   |
   |
LL |     type Item<'a>: std::ops::Deref<Target = T>;
   |                                    ^^^^^^^^^^ required by this bound in `UnsafeCopy::Item`
   |
   |
LL | impl<T: Copy + std::ops::Deref + Deref<Target = T>> UnsafeCopy<T> for T {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.
---
1 error: incompatible lifetime on type
-   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:17:18
+   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:17:5
3    |
4 LL |     type T<'a> = Box<dyn A + 'a>;
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
6    |
7 note: because this has an unmet lifetime requirement

26    |                      ++++
27 
28 error: incompatible lifetime on type
28 error: incompatible lifetime on type
-   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:27:18
+   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:27:5
30    |
31 LL |     type T<'a> = Box<dyn A + 'a>;
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
33    |
33    |
34 note: because this has an unmet lifetime requirement

48    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
49 
50 error: incompatible lifetime on type
50 error: incompatible lifetime on type
-   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:37:18
+   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:37:5
52    |
53 LL |     type T<'a> = (Box<dyn A + 'a>, Box<dyn A + 'a>);
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
55    |
55    |
56 note: because this has an unmet lifetime requirement


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box/issue-78113-lifetime-mismatch-dyn-trait-box.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box/issue-78113-lifetime-mismatch-dyn-trait-box.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: incompatible lifetime on type
  --> /checkout/src/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box.rs:17:5
   |
LL |     type T<'a> = Box<dyn A + 'a>; //~ incompatible lifetime on type
   |
   |
note: because this has an unmet lifetime requirement
   |
   |
LL |     type T<'a>: A;
   |                 ^ introduces a `'static` lifetime requirement
note: the lifetime `'a` as defined here...
   |
   |
LL |     type T<'a> = Box<dyn A + 'a>; //~ incompatible lifetime on type
   |            ^^
   = note: ...does not necessarily outlive the static lifetime introduced by the compatible `impl`
note: this has an implicit `'static` lifetime requirement
   |
   |
LL | impl A for Box<dyn A> {}
   |                    ^
help: consider relaxing the implicit `'static` requirement
   |
LL | impl A for Box<dyn A + '_> {}

error: incompatible lifetime on type
  --> /checkout/src/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box.rs:27:5
   |
   |
LL |     type T<'a> = Box<dyn A + 'a>; //~ incompatible lifetime on type
   |
   |
note: because this has an unmet lifetime requirement
   |
   |
LL |     type T<'a>: C;
   |                 ^ introduces a `'static` lifetime requirement
note: the lifetime `'a` as defined here...
   |
   |
LL |     type T<'a> = Box<dyn A + 'a>; //~ incompatible lifetime on type
   |            ^^
note: ...does not necessarily outlive the static lifetime introduced by the compatible `impl`
   |
   |
LL | impl C for Box<dyn A + 'static> {}

error: incompatible lifetime on type
  --> /checkout/src/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box.rs:37:5
   |
   |
LL |     type T<'a> = (Box<dyn A + 'a>, Box<dyn A + 'a>); //~ incompatible lifetime on type
   |
   |
note: because this has an unmet lifetime requirement
   |
   |
LL |     type T<'a>: E;
   |                 ^ introduces a `'static` lifetime requirement
note: the lifetime `'a` as defined here...
   |
   |
LL |     type T<'a> = (Box<dyn A + 'a>, Box<dyn A + 'a>); //~ incompatible lifetime on type
   |            ^^
   = note: ...does not necessarily outlive the static lifetime introduced by the compatible `impl`
note: this has an implicit `'static` lifetime requirement
   |
   |
LL | impl E for (Box<dyn A>, Box<dyn A>) {}
   |                     ^
note: this has an implicit `'static` lifetime requirement
   |
   |
LL | impl E for (Box<dyn A>, Box<dyn A>) {}
   |                                 ^
help: consider relaxing the implicit `'static` requirement
   |
LL | impl E for (Box<dyn A + '_>, Box<dyn A>) {}
   |                       ++++
help: consider relaxing the implicit `'static` requirement
   |
LL | impl E for (Box<dyn A>, Box<dyn A + '_>) {}

error: aborting due to 3 previous errors


---
11 error[E0277]: can't compare `Foo` with `Foo`
-   --> $DIR/issue-87429-specialization.rs:21:31
+   --> $DIR/issue-87429-specialization.rs:21:5
13    |
14 LL |     default type Member<'a> = Foo;
-    |                               ^^^ no implementation for `Foo == Foo`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `Foo == Foo`
16    |
17    = help: the trait `PartialEq` is not implemented for `Foo`
18 note: required by a bound in `Family::Member`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87429-specialization/issue-87429-specialization.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-87429-specialization.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-87429-specialization.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87429-specialization" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87429-specialization/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/generic-associated-types/issue-87429-specialization.rs:3:12
   |
LL | #![feature(specialization)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete
   = help: consider using `min_specialization` instead, which is more stable and complete

error[E0277]: can't compare `Foo` with `Foo`
  --> /checkout/src/test/ui/generic-associated-types/issue-87429-specialization.rs:21:5
   |
LL |     default type Member<'a> = Foo;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `Foo == Foo`
   |
   = help: the trait `PartialEq` is not implemented for `Foo`
note: required by a bound in `Family::Member`
   |
   |
LL |     type Member<'a>: for<'b> PartialEq<Self::Member<'b>>;
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Family::Member`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/generic-associated-types/issue-88595.rs stdout ----
diff of stderr:

1 error[E0478]: lifetime bound not satisfied
-   --> $DIR/issue-88595.rs:19:18
3    |
3    |
4 LL |     type B<'b> = impl Clone;
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^
6    |
6    |
7 note: lifetime parameter instantiated with the lifetime `'a` as defined here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-88595/issue-88595.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-88595/issue-88595.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-88595.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-88595.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-88595" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-88595/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0478]: lifetime bound not satisfied
   |
   |
LL |     type B<'b> = impl Clone;
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined here
   |
   |
LL | impl<'a> A<'a> for C {
   |      ^^
note: but lifetime parameter must outlive the lifetime `'b` as defined here
   |
   |
LL |     type B<'b> = impl Clone;

error: non-defining opaque type use in defining scope
  --> /checkout/src/test/ui/generic-associated-types/issue-88595.rs:23:23
   |
   |
LL |     fn a(&'a self) -> Self::B<'a> {} //~ ERROR: non-defining opaque type use in defining scope
   |
note: lifetime used multiple times
  --> /checkout/src/test/ui/generic-associated-types/issue-88595.rs:18:6
   |
   |
LL | impl<'a> A<'a> for C {
   |      ^^
LL |     type B<'b> = impl Clone;

error: could not find defining uses
  --> /checkout/src/test/ui/generic-associated-types/issue-88595.rs:19:18
   |
   |
LL |     type B<'b> = impl Clone;

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0478`.
For more information about this error, try `rustc --explain E0478`.

------------------------------------------


---- [ui] ui/generic-associated-types/issue-90014.rs stdout ----
diff of stderr:

1 error[E0477]: the type `&mut ()` does not fulfill the required lifetime
-   --> $DIR/issue-90014.rs:14:20
3    |
3    |
4 LL |     type Fut<'a> = impl Future<Output = ()>;
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
6    |
7 note: type must outlive the lifetime `'a` as defined here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-90014/issue-90014.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-90014/issue-90014.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-90014.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-90014.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-90014" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-90014/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0477]: the type `&mut ()` does not fulfill the required lifetime
   |
   |
LL |     type Fut<'a> = impl Future<Output = ()>;
   |
   |
note: type must outlive the lifetime `'a` as defined here
   |
   |
LL |     type Fut<'a> = impl Future<Output = ()>;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0477`.
For more information about this error, try `rustc --explain E0477`.

------------------------------------------


---- [ui] ui/generic-associated-types/unsatisfied-outlives-bound.rs stdout ----
diff of stderr:

1 error[E0477]: the type `&'b ()` does not fulfill the required lifetime
-   --> $DIR/unsatisfied-outlives-bound.rs:8:21
3    |
3    |
4 LL |     type Item<'a> = &'b ();
+    |     ^^^^^^^^^^^^^^^^^^^^^^^
6    |
6    |
7 note: type must outlive the lifetime `'a` as defined here as required by this binding

11    |               ^^
12 
12 
13 error[E0477]: the type `&'a ()` does not fulfill the required lifetime
-   --> $DIR/unsatisfied-outlives-bound.rs:17:21
15    |
15    |
16 LL |     type Item<'a> = &'a ();
+    |     ^^^^^^^^^^^^^^^^^^^^^^^
18    |
19 note: type must satisfy the static lifetime as required by this binding
20   --> $DIR/unsatisfied-outlives-bound.rs:13:20
---
To only update this specific test, also pass `--test-args generic-associated-types/unsatisfied-outlives-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/unsatisfied-outlives-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/unsatisfied-outlives-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/unsatisfied-outlives-bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0477]: the type `&'b ()` does not fulfill the required lifetime
   |
   |
LL |     type Item<'a> = &'b ();
   |
   |
note: type must outlive the lifetime `'a` as defined here as required by this binding
   |
   |
LL |     type Item<'a> = &'b ();


error[E0477]: the type `&'a ()` does not fulfill the required lifetime
   |
   |
LL |     type Item<'a> = &'a ();
   |
note: type must satisfy the static lifetime as required by this binding
  --> /checkout/src/test/ui/generic-associated-types/unsatisfied-outlives-bound.rs:13:20
   |
   |
LL |     type Item<'a>: 'static;

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0477`.
For more information about this error, try `rustc --explain E0477`.

------------------------------------------


---- [ui] ui/never_type/issue-51506.rs stdout ----
diff of stderr:

1 error[E0277]: `!` is not an iterator
-   --> $DIR/issue-51506.rs:13:24
3    |
4 LL |     default type Out = !;
4 LL |     default type Out = !;
-    |                        ^ `!` is not an iterator
+    |     ^^^^^^^^^^^^^^^^^^^^^ `!` is not an iterator
6    |
7    = help: the trait `Iterator` is not implemented for `!`
8 note: required by a bound in `Trait::Out`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/issue-51506/issue-51506.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args never_type/issue-51506.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/never_type/issue-51506.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/issue-51506" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/issue-51506/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `!` is not an iterator
   |
   |
LL |     default type Out = !; //~ ERROR: `!` is not an iterator
   |     ^^^^^^^^^^^^^^^^^^^^^ `!` is not an iterator
   |
   = help: the trait `Iterator` is not implemented for `!`
note: required by a bound in `Trait::Out`
   |
   |
LL |     type Out: Iterator<Item = u32>;
   |               ^^^^^^^^^^^^^^^^^^^^ required by this bound in `Trait::Out`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/regions/regions-assoc-type-region-bound-in-trait-not-met.rs stdout ----
diff of stderr:

1 error[E0477]: the type `&'a i32` does not fulfill the required lifetime
-   --> $DIR/regions-assoc-type-region-bound-in-trait-not-met.rs:15:18
3    |
4 LL |     type Value = &'a i32;
-    |                  ^^^^^^^
+    |     ^^^^^^^^^^^^^^^^^^^^^
+    |     ^^^^^^^^^^^^^^^^^^^^^
6    |
7 note: type must satisfy the static lifetime as required by this binding
8   --> $DIR/regions-assoc-type-region-bound-in-trait-not-met.rs:5:17

11    |                 ^^
12 
13 error[E0477]: the type `&'a i32` does not fulfill the required lifetime
-   --> $DIR/regions-assoc-type-region-bound-in-trait-not-met.rs:20:18
15    |
16 LL |     type Value = &'a i32;
-    |                  ^^^^^^^
+    |     ^^^^^^^^^^^^^^^^^^^^^
+    |     ^^^^^^^^^^^^^^^^^^^^^
18    |
19 note: type must outlive the lifetime `'b` as defined here as required by this binding


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-region-bound-in-trait-not-met/regions-assoc-type-region-bound-in-trait-not-met.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-region-bound-in-trait-not-met/regions-assoc-type-region-bound-in-trait-not-met.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-assoc-type-region-bound-in-trait-not-met.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-assoc-type-region-bound-in-trait-not-met.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-region-bound-in-trait-not-met" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-region-bound-in-trait-not-met/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0477]: the type `&'a i32` does not fulfill the required lifetime
   |
LL |     type Value = &'a i32;
   |     ^^^^^^^^^^^^^^^^^^^^^
   |
   |
note: type must satisfy the static lifetime as required by this binding
  --> /checkout/src/test/ui/regions/regions-assoc-type-region-bound-in-trait-not-met.rs:5:17
   |
LL |     type Value: 'a;
   |                 ^^

error[E0477]: the type `&'a i32` does not fulfill the required lifetime
   |
LL |     type Value = &'a i32;
   |     ^^^^^^^^^^^^^^^^^^^^^
   |
   |
note: type must outlive the lifetime `'b` as defined here as required by this binding
   |
   |
LL | impl<'a, 'b> Foo<'b> for &'a i64 {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0477`.
For more information about this error, try `rustc --explain E0477`.

------------------------------------------


---- [ui] ui/regions/regions-assoc-type-static-bound-in-trait-not-met.rs stdout ----
diff of stderr:

1 error[E0477]: the type `&'a i32` does not fulfill the required lifetime
-   --> $DIR/regions-assoc-type-static-bound-in-trait-not-met.rs:10:18
3    |
4 LL |     type Value = &'a i32;
-    |                  ^^^^^^^
+    |     ^^^^^^^^^^^^^^^^^^^^^
---
To only update this specific test, also pass `--test-args regions/regions-assoc-type-static-bound-in-trait-not-met.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-assoc-type-static-bound-in-trait-not-met.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-static-bound-in-trait-not-met" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-static-bound-in-trait-not-met/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0477]: the type `&'a i32` does not fulfill the required lifetime
   |
LL |     type Value = &'a i32;
   |     ^^^^^^^^^^^^^^^^^^^^^
   |
---

---- [ui] ui/rfc-2632-const-trait-impl/assoc-type.rs stdout ----
diff of stderr:

1 error[E0277]: cannot add `NonConstAdd` to `NonConstAdd`
-   --> $DIR/assoc-type.rs:18:16
3    |
3    |
4 LL |     type Bar = NonConstAdd;
-    |                ^^^^^^^^^^^ no implementation for `NonConstAdd + NonConstAdd`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `NonConstAdd + NonConstAdd`
6    |
7    = help: the trait `Add` is not implemented for `NonConstAdd`
8 note: required by a bound in `Foo::Bar`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/assoc-type/assoc-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/assoc-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/assoc-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/assoc-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/assoc-type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: cannot add `NonConstAdd` to `NonConstAdd`
   |
   |
LL |     type Bar = NonConstAdd;
   |     ^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `NonConstAdd + NonConstAdd`
   |
   = help: the trait `Add` is not implemented for `NonConstAdd`
note: required by a bound in `Foo::Bar`
   |
   |
LL |     type Bar: ~const std::ops::Add;
   |               ^^^^^^^^^^^^^^^^^^^^ required by this bound in `Foo::Bar`
help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
   |
LL | impl const Foo for NonConstAdd where NonConstAdd: Add {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
---
diff of stderr:

9    = help: consider using `min_specialization` instead, which is more stable and complete
10 
11 error[E0277]: can't compare `&'static B` with `B`
-   --> $DIR/deafult-associated-type-bound-2.rs:16:22
13    |
14 LL |     default type U = &'static B;
14 LL |     default type U = &'static B;
-    |                      ^^^^^^^^^^ no implementation for `&'static B == B`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `&'static B == B`
16    |
17    = help: the trait `PartialEq<B>` is not implemented for `&'static B`
18 note: required by a bound in `X::U`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/deafult-associated-type-bound-2/deafult-associated-type-bound-2.stderr
To only update this specific test, also pass `--test-args specialization/deafult-associated-type-bound-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/deafult-associated-type-bound-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/deafult-associated-type-bound-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/deafult-associated-type-bound-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/specialization/deafult-associated-type-bound-2.rs:2:12
   |
LL | #![feature(specialization)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete
   = help: consider using `min_specialization` instead, which is more stable and complete

error[E0277]: can't compare `&'static B` with `B`
   |
LL |     default type U = &'static B;
LL |     default type U = &'static B;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `&'static B == B`
   |
   = help: the trait `PartialEq<B>` is not implemented for `&'static B`
note: required by a bound in `X::U`
   |
   |
LL |     type U: PartialEq<T>;
   |             ^^^^^^^^^^^^ required by this bound in `X::U`
help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
   |
LL | impl<B: 'static, T> X<B> for T where &'static B: PartialEq<B> {

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.
---
11 error[E0277]: the trait bound `str: Clone` is not satisfied
-   --> $DIR/deafult-associated-type-bound-1.rs:19:22
+   --> $DIR/deafult-associated-type-bound-1.rs:19:5
13    |
14 LL |     default type U = str;
-    |                      ^^^ the trait `Clone` is not implemented for `str`
+    |     ^^^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `str`
16    |
17 note: required by a bound in `X::U`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/deafult-associated-type-bound-1/deafult-associated-type-bound-1.stderr
To only update this specific test, also pass `--test-args specialization/deafult-associated-type-bound-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/deafult-associated-type-bound-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/deafult-associated-type-bound-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/deafult-associated-type-bound-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/specialization/deafult-associated-type-bound-1.rs:5:12
   |
LL | #![feature(specialization)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete
   = help: consider using `min_specialization` instead, which is more stable and complete

error[E0277]: the trait bound `str: Clone` is not satisfied
  --> /checkout/src/test/ui/specialization/deafult-associated-type-bound-1.rs:19:5
   |
LL |     default type U = str;
   |     ^^^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `str`
   |
note: required by a bound in `X::U`
   |
   |
LL |     type U: Clone;
   |             ^^^^^ required by this bound in `X::U`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.

---
diff of stderr:

9    = help: consider using `min_specialization` instead, which is more stable and complete
10 
11 error[E0277]: can't compare `T` with `T`
-   --> $DIR/deafult-generic-associated-type-bound.rs:18:26
13    |
13    |
14 LL |     default type U<'a> = &'a T;
-    |                          ^^^^^ no implementation for `T == T`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `T == T`
16    |
17    = note: required because of the requirements on the impl of `PartialEq` for `&'a T`
18 note: required by a bound in `X::U`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/deafult-generic-associated-type-bound/deafult-generic-associated-type-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args specialization/deafult-generic-associated-type-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/deafult-generic-associated-type-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/deafult-generic-associated-type-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/deafult-generic-associated-type-bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/specialization/deafult-generic-associated-type-bound.rs:3:12
   |
LL | #![feature(specialization)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete
   = help: consider using `min_specialization` instead, which is more stable and complete

error[E0277]: can't compare `T` with `T`
   |
   |
LL |     default type U<'a> = &'a T;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `T == T`
   |
   = note: required because of the requirements on the impl of `PartialEq` for `&'a T`
note: required by a bound in `X::U`
   |
   |
LL |     type U<'a>: PartialEq<&'a Self> where Self: 'a;
   |                 ^^^^^^^^^^^^^^^^^^^ required by this bound in `X::U`
   |
   |
LL | impl<T: 'static + std::cmp::PartialEq> X for T {

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/specialization/issue-33017.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `T: Copy` is not satisfied
-   --> $DIR/issue-33017.rs:12:27
3    |
4 LL |     default type Output = Self;
-    |                           ^^^^ the trait `Copy` is not implemented for `T`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `T`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `T`
6    |
7 note: required by a bound in `UncheckedCopy::Output`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-33017/issue-33017.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-33017/issue-33017.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args specialization/issue-33017.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/issue-33017.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-33017" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-33017/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `T: Copy` is not satisfied
   |
LL |     default type Output = Self;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `T`
   |
   |
note: required by a bound in `UncheckedCopy::Output`
   |
   |
LL |     type Output: From<Self> + Copy + Into<Self>;
   |                               ^^^^ required by this bound in `UncheckedCopy::Output`
help: consider restricting type parameter `T`
   |
LL | impl<T: std::marker::Copy> UncheckedCopy for T {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
---
diff of stderr:

9    = help: consider using `min_specialization` instead, which is more stable and complete
10 
11 error[E0277]: the trait bound `(): Valid` is not satisfied
-   --> $DIR/issue-38091.rs:12:23
13    |
14 LL |     default type Ty = ();
14 LL |     default type Ty = ();
-    |                       ^^ the trait `Valid` is not implemented for `()`
+    |     ^^^^^^^^^^^^^^^^^^^^^ the trait `Valid` is not implemented for `()`
17 note: required by a bound in `Iterate::Ty`
18   --> $DIR/issue-38091.rs:5:14



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-38091/issue-38091.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args specialization/issue-38091.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/issue-38091.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-38091" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-38091/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/specialization/issue-38091.rs:1:12
   |
LL | #![feature(specialization)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete
   = help: consider using `min_specialization` instead, which is more stable and complete

error[E0277]: the trait bound `(): Valid` is not satisfied
   |
LL |     default type Ty = ();
LL |     default type Ty = ();
   |     ^^^^^^^^^^^^^^^^^^^^^ the trait `Valid` is not implemented for `()`
note: required by a bound in `Iterate::Ty`
  --> /checkout/src/test/ui/specialization/issue-38091.rs:5:14
   |
   |
LL |     type Ty: Valid;
   |              ^^^^^ required by this bound in `Iterate::Ty`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/specialization/issue-44861.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `(): CoerceUnsized<*const [u8]>` is not satisfied
-   --> $DIR/issue-44861.rs:21:26
3    |
4 LL |     default type Data2 = ();
4 LL |     default type Data2 = ();
-    |                          ^^ the trait `CoerceUnsized<*const [u8]>` is not implemented for `()`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `CoerceUnsized<*const [u8]>` is not implemented for `()`
6    |
7 note: required by a bound in `Smartass::Data2`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-44861/issue-44861.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-44861/issue-44861.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args specialization/issue-44861.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/issue-44861.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-44861" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-44861/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `(): CoerceUnsized<*const [u8]>` is not satisfied
   |
LL |     default type Data2 = ();
LL |     default type Data2 = ();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `CoerceUnsized<*const [u8]>` is not implemented for `()`
   |
note: required by a bound in `Smartass::Data2`
   |
   |
LL |     type Data2: CoerceUnsized<*const [u8]>;
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Smartass::Data2`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/specialization/issue-59435.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `MyStruct: Default` is not satisfied
-   --> $DIR/issue-59435.rs:11:27
3    |
3    |
4 LL |     default type MyType = MyStruct;
-    |                           ^^^^^^^^ the trait `Default` is not implemented for `MyStruct`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Default` is not implemented for `MyStruct`
6    |
7 note: required by a bound in `MyTrait::MyType`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-59435/issue-59435.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-59435/issue-59435.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args specialization/issue-59435.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/issue-59435.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-59435" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-59435/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `MyStruct: Default` is not satisfied
   |
   |
LL |     default type MyType = MyStruct;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Default` is not implemented for `MyStruct`
   |
note: required by a bound in `MyTrait::MyType`
   |
   |
LL |     type MyType: Default;
   |                  ^^^^^^^ required by this bound in `MyTrait::MyType`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/traits/inductive-overflow/two-traits.rs stdout ----
diff of stderr:

1 error[E0277]: `T` cannot be shared between threads safely
-   --> $DIR/two-traits.rs:11:14
3    |
4 LL |     type X = Self;
4 LL |     type X = Self;
-    |              ^^^^ `T` cannot be shared between threads safely
+    |     ^^^^^^^^^^^^^^ `T` cannot be shared between threads safely
6    |
7 note: required by a bound in `Magic::X`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/two-traits/two-traits.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/two-traits/two-traits.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/inductive-overflow/two-traits.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/inductive-overflow/two-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/two-traits" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/two-traits/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `T` cannot be shared between threads safely
   |
LL |     type X = Self;
LL |     type X = Self;
   |     ^^^^^^^^^^^^^^ `T` cannot be shared between threads safely
   |
note: required by a bound in `Magic::X`
   |
   |
LL |     type X: Trait;
   |             ^^^^^ required by this bound in `Magic::X`
   |
   |
LL | impl<T: Magic + std::marker::Sync> Magic for T {


error[E0275]: overflow evaluating the requirement `*mut (): Magic`
   |
   |
LL |     wizard::<*mut ()>(); //~ ERROR E0275
   |
   |
note: required by a bound in `wizard`
   |
   |
LL | fn wizard<T: Magic>() { check::<<T as Magic>::X>(); }
   |              ^^^^^ required by this bound in `wizard`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0275, E0277.
For more information about an error, try `rustc --explain E0275`.
For more information about an error, try `rustc --explain E0275`.

------------------------------------------


---- [ui] ui/traits/issue-65673.rs stdout ----
diff of stderr:

1 error[E0277]: the size for values of type `(dyn Trait + 'static)` cannot be known at compilation time
-   --> $DIR/issue-65673.rs:9:16
3    |
3    |
4 LL |     type Ctx = dyn Alias<T>;
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
6    |
6    |
7    = help: the trait `Sized` is not implemented for `(dyn Trait + 'static)`
8 note: required by a bound in `WithType::Ctx`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-65673/issue-65673.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/issue-65673.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-65673.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-65673" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-65673/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `(dyn Trait + 'static)` cannot be known at compilation time
   |
   |
LL |     type Ctx = dyn Alias<T>;
   |
   |
   = help: the trait `Sized` is not implemented for `(dyn Trait + 'static)`
note: required by a bound in `WithType::Ctx`
   |
LL |     type Ctx;
LL |     type Ctx;
   |     ^^^^^^^^^ required by this bound in `WithType::Ctx`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.

---
test result: FAILED. 12387 passed; 35 failed; 119 ignored; 0 measured; 0 filtered out; finished in 147.71s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:24
