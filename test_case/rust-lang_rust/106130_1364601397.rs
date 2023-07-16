plain
iiiiiii.....................i..................i........................................ 264/14054
........................................................................................ 352/14054
........................................................................................ 440/14054
........................................................................................ 528/14054
.......................F.......F.................F.................F.F.................. 616/14054
........................................................................................ 792/14054
.....................................................i.................................. 880/14054
...............................................................i........................ 968/14054
........................................................................................ 1056/14054
---
.........................iii............................................................ 4312/14054
....................................................................................i... 4400/14054
........................................................................................ 4488/14054
........................................................................................ 4576/14054
.........................................F...........................F..F............... 4664/14054
..F.FF...FF..F..F...................................FF........FF........................ 4752/14054
..........F........................................i.................................... 4840/14054
........................................................................................ 5016/14054
........................................................................................ 5104/14054
........................................................................................ 5192/14054
.....F.................................................................................. 5280/14054
---
........................................................................................ 11352/14054
......iiiiii.i..iiiii.iiiiiii........................................................... 11440/14054
........................................................................................ 11528/14054
........................................................................................ 11616/14054
.......................................................................F.FF............F 11704/14054
...F......F....F........................................................................ 11792/14054
........................................................................................ 11968/14054
........................................................................................ 12056/14054
........................................................................................ 12144/14054
........................................................................................ 12232/14054
---
........................................................................................ 12848/14054
........................................................................................ 12936/14054
........................................................................................ 13024/14054
........................................................................................ 13112/14054
........................................i.F............................................. 13200/14054
........................................................................................ 13376/14054
........................................................................................ 13464/14054
........................................................................................ 13552/14054
........................................................................................ 13640/14054
---

---- [ui] src/test/ui/associated-types/issue-43784-associated-type.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `T: Copy` is not satisfied
-   --> $DIR/issue-43784-associated-type.rs:14:18
3    |
4 LL |     type Assoc = T;
-    |                  ^ the trait `Copy` is not implemented for `T`
+    |     ^^^^^^^^^^ the trait `Copy` is not implemented for `T`
---
To only update this specific test, also pass `--test-args associated-types/issue-43784-associated-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-43784-associated-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-43784-associated-type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-43784-associated-type/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `T: Copy` is not satisfied
   |
   |
LL |     type Assoc = T; //~ ERROR the trait bound `T: Copy` is not satisfied
   |     ^^^^^^^^^^ the trait `Copy` is not implemented for `T`
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


---- [ui] src/test/ui/associated-types/issue-54108.rs stdout ----
diff of stderr:

1 error[E0277]: cannot add `<T as SubEncoder>::ActualSize` to `<T as SubEncoder>::ActualSize`
-   --> $DIR/issue-54108.rs:19:17
3    |
3    |
4 LL |     type Size = <Self as SubEncoder>::ActualSize;
-    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `<T as SubEncoder>::ActualSize + <T as SubEncoder>::ActualSize`
+    |     ^^^^^^^^^ no implementation for `<T as SubEncoder>::ActualSize + <T as SubEncoder>::ActualSize`
6    |
7    = help: the trait `Add` is not implemented for `<T as SubEncoder>::ActualSize`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-54108/issue-54108.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-54108/issue-54108.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/issue-54108.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-54108.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-54108" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-54108/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: cannot add `<T as SubEncoder>::ActualSize` to `<T as SubEncoder>::ActualSize`
   |
   |
LL |     type Size = <Self as SubEncoder>::ActualSize;
   |     ^^^^^^^^^ no implementation for `<T as SubEncoder>::ActualSize + <T as SubEncoder>::ActualSize`
   |
   = help: the trait `Add` is not implemented for `<T as SubEncoder>::ActualSize`
  --> /checkout/src/test/ui/associated-types/issue-54108.rs:4:16
   |
   |
LL |     type Size: Add<Output = Self::Size>;
help: consider further restricting the associated type
   |
   |
LL |     T: SubEncoder, <T as SubEncoder>::ActualSize: Add

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/associated-types/issue-72806.rs stdout ----
diff of stderr:

1 error[E0271]: type mismatch resolving `<Foo2 as Bar2>::Ok == char`
-   --> $DIR/issue-72806.rs:14:20
3    |
4 LL |     type Sibling = Foo2;
4 LL |     type Sibling = Foo2;
-    |                    ^^^^ type mismatch resolving `<Foo2 as Bar2>::Ok == char`
+    |     ^^^^^^^^^^^^ type mismatch resolving `<Foo2 as Bar2>::Ok == char`
7 note: expected this to be `char`
8   --> $DIR/issue-72806.rs:18:15



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-72806/issue-72806.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/issue-72806.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-72806.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-72806" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-72806/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0271]: type mismatch resolving `<Foo2 as Bar2>::Ok == char`
   |
LL |     type Sibling = Foo2;
LL |     type Sibling = Foo2;
   |     ^^^^^^^^^^^^ type mismatch resolving `<Foo2 as Bar2>::Ok == char`
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
------------------------------------------


---- [ui] src/test/ui/associated-types/point-at-type-on-obligation-failure-2.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `bool: Bar` is not satisfied
-   --> $DIR/point-at-type-on-obligation-failure-2.rs:8:18
3    |
4 LL |     type Assoc = bool;
-    |                  ^^^^ the trait `Bar` is not implemented for `bool`
+    |     ^^^^^^^^^^ the trait `Bar` is not implemented for `bool`
+    |     ^^^^^^^^^^ the trait `Bar` is not implemented for `bool`
6    |
7 note: required by a bound in `Foo::Assoc`
8   --> $DIR/point-at-type-on-obligation-failure-2.rs:4:17

11    |                 ^^^ required by this bound in `Foo::Assoc`
12 
13 error[E0277]: the trait bound `bool: Bar` is not satisfied
-   --> $DIR/point-at-type-on-obligation-failure-2.rs:19:18
15    |
16 LL |     type Assoc = bool;
-    |                  ^^^^ the trait `Bar` is not implemented for `bool`
+    |     ^^^^^^^^^^ the trait `Bar` is not implemented for `bool`
+    |     ^^^^^^^^^^ the trait `Bar` is not implemented for `bool`
18    |
19 note: required by a bound in `Baz::Assoc`

26    |          ----- required by a bound in this
27 
27 
28 error[E0277]: the trait bound `bool: Bar` is not satisfied
-   --> $DIR/point-at-type-on-obligation-failure-2.rs:30:18
30    |
31 LL |     type Assoc = bool;
-    |                  ^^^^ the trait `Bar` is not implemented for `bool`
+    |     ^^^^^^^^^^ the trait `Bar` is not implemented for `bool`
+    |     ^^^^^^^^^^ the trait `Bar` is not implemented for `bool`
33    |
34 note: required by a bound in `Bat::Assoc`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/point-at-type-on-obligation-failure-2/point-at-type-on-obligation-failure-2.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/point-at-type-on-obligation-failure-2/point-at-type-on-obligation-failure-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/point-at-type-on-obligation-failure-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/point-at-type-on-obligation-failure-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/point-at-type-on-obligation-failure-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/point-at-type-on-obligation-failure-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `bool: Bar` is not satisfied
   |
   |
LL |     type Assoc = bool; //~ ERROR the trait bound `bool: Bar` is not satisfied
   |     ^^^^^^^^^^ the trait `Bar` is not implemented for `bool`
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
   |     ^^^^^^^^^^ the trait `Bar` is not implemented for `bool`
   |
note: required by a bound in `Baz::Assoc`
   |
   |
LL |     Self::Assoc: Bar,
   |                  ^^^ required by this bound in `Baz::Assoc`
LL | {
LL |     type Assoc;


error[E0277]: the trait bound `bool: Bar` is not satisfied
   |
   |
LL |     type Assoc = bool; //~ ERROR the trait bound `bool: Bar` is not satisfied
   |     ^^^^^^^^^^ the trait `Bar` is not implemented for `bool`
   |
note: required by a bound in `Bat::Assoc`
   |
   |
LL |     <Self as Bat>::Assoc: Bar,
   |                           ^^^ required by this bound in `Bat::Assoc`
LL | {
LL |     type Assoc;

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/associated-types/point-at-type-on-obligation-failure.rs stdout ----
diff of stderr:

1 error[E0271]: type mismatch resolving `<Foo2 as Bar2>::Ok == ()`
-   --> $DIR/point-at-type-on-obligation-failure.rs:14:20
3    |
4 LL |     type Sibling = Foo2;
4 LL |     type Sibling = Foo2;
-    |                    ^^^^ type mismatch resolving `<Foo2 as Bar2>::Ok == ()`
+    |     ^^^^^^^^^^^^ type mismatch resolving `<Foo2 as Bar2>::Ok == ()`
7 note: expected this to be `()`
8   --> $DIR/point-at-type-on-obligation-failure.rs:18:15



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/point-at-type-on-obligation-failure/point-at-type-on-obligation-failure.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/point-at-type-on-obligation-failure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/point-at-type-on-obligation-failure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/point-at-type-on-obligation-failure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/point-at-type-on-obligation-failure/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0271]: type mismatch resolving `<Foo2 as Bar2>::Ok == ()`
   |
LL |     type Sibling = Foo2;
LL |     type Sibling = Foo2;
   |     ^^^^^^^^^^^^ type mismatch resolving `<Foo2 as Bar2>::Ok == ()`
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
---
8 note: required by a bound in `Bar::Item`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/impl_wf_2/impl_wf_2.stderr
To only update this specific test, also pass `--test-args chalkify/impl_wf_2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/chalkify/impl_wf_2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/impl_wf_2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/impl_wf_2/auxiliary" "-Z" "chalk"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `f32: Foo` is not satisfied
   |
LL |     type Item = f32;
   |     ^^^^^^^^^ the trait `Foo` is not implemented for `f32`
   |
---

---- [ui] src/test/ui/const-generics/associated-type-bound-fail.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `u16: Bar<N>` is not satisfied
-   --> $DIR/associated-type-bound-fail.rs:9:18
3    |
4 LL |     type Assoc = u16;
4 LL |     type Assoc = u16;
-    |                  ^^^ the trait `Bar<N>` is not implemented for `u16`
+    |     ^^^^^^^^^^ the trait `Bar<N>` is not implemented for `u16`
6    |
7    = help: the trait `Bar<3>` is implemented for `u16`
8 note: required by a bound in `Foo::Assoc`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/associated-type-bound-fail/associated-type-bound-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/associated-type-bound-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/associated-type-bound-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/associated-type-bound-fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/associated-type-bound-fail/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `u16: Bar<N>` is not satisfied
   |
   |
LL |     type Assoc = u16; //~ ERROR the trait bound `u16: Bar<N>`
   |     ^^^^^^^^^^ the trait `Bar<N>` is not implemented for `u16`
   |
   = help: the trait `Bar<3>` is implemented for `u16`
note: required by a bound in `Foo::Assoc`
   |
   |
LL |     type Assoc: Bar<N>;
   |                 ^^^^^^ required by this bound in `Foo::Assoc`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/generic-associated-types/bugs/issue-87755.rs stdout ----
diff of stderr:

1 error[E0275]: overflow evaluating the requirement `<Bar as Foo>::Ass == _`
-   --> $DIR/issue-87755.rs:16:16
3    |
4 LL |     type Ass = Bar;
-    |                ^^^
+    |     ^^^^^^^^
---
To only update this specific test, also pass `--test-args generic-associated-types/bugs/issue-87755.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/bugs/issue-87755.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-87755" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-87755/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0275]: overflow evaluating the requirement `<Bar as Foo>::Ass == _`
   |
LL |     type Ass = Bar;
   |     ^^^^^^^^

---

---- [ui] src/test/ui/generic-associated-types/cross-crate-bounds.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `(): AsRef<()>` is not satisfied
-   --> $DIR/cross-crate-bounds.rs:15:16
3    |
4 LL |     type Bar = ();
4 LL |     type Bar = ();
-    |                ^^ the trait `AsRef<()>` is not implemented for `()`
+    |     ^^^^^^^^ the trait `AsRef<()>` is not implemented for `()`
6    |
7 note: required by a bound in `foo_defn::Foo::Bar`
8   --> $DIR/auxiliary/foo_defn.rs:4:15

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/cross-crate-bounds/cross-crate-bounds.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/cross-crate-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/cross-crate-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/cross-crate-bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/cross-crate-bounds/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `(): AsRef<()>` is not satisfied
   |
LL |     type Bar = ();
LL |     type Bar = ();
   |     ^^^^^^^^ the trait `AsRef<()>` is not implemented for `()`
   |
note: required by a bound in `foo_defn::Foo::Bar`
  --> /checkout/src/test/ui/generic-associated-types/auxiliary/foo_defn.rs:4:15
   |
LL |     type Bar: AsRef<()>;
   |               ^^^^^^^^^ required by this bound in `Foo::Bar`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
---
1 error[E0277]: `T` doesn't implement `std::fmt::Display`
-   --> $DIR/generic-associated-types-where.rs:18:22
+   --> $DIR/generic-associated-types-where.rs:18:5
3    |
4 LL |     type Assoc2<T> = Vec<T>;
-    |                      ^^^^^^ `T` cannot be formatted with the default formatter
+    |     ^^^^^^^^^^^^^^ `T` cannot be formatted with the default formatter
7    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
8 help: consider restricting type parameter `T`



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/generic-associated-types-where/generic-associated-types-where.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/generic-associated-types-where.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/generic-associated-types-where.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/generic-associated-types-where" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/generic-associated-types-where/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `T` doesn't implement `std::fmt::Display`
   |
   |
LL |     type Assoc2<T> = Vec<T>;
   |     ^^^^^^^^^^^^^^ `T` cannot be formatted with the default formatter
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
help: consider restricting type parameter `T`
   |
   |
LL |     type Assoc2<T: std::fmt::Display> = Vec<T>;

error[E0276]: impl has stricter requirements than trait
  --> /checkout/src/test/ui/generic-associated-types/generic-associated-types-where.rs:20:38
   |
   |
LL |     type Assoc3<T>;
   |     -------------- definition of `Assoc3` from trait
...
LL |     type Assoc3<T> = Vec<T> where T: Iterator;
   |                                      ^^^^^^^^ impl has extra requirement `T: Iterator`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0276, E0277.
For more information about an error, try `rustc --explain E0276`.
For more information about an error, try `rustc --explain E0276`.
------------------------------------------


---- [ui] src/test/ui/generic-associated-types/issue-68643-broken-mir.rs stdout ----
diff of stderr:

1 error[E0277]: expected a `Fn<()>` closure, found `T`
-   --> $DIR/issue-68643-broken-mir.rs:12:18
3    |
3    |
4 LL |     type F<'a> = Self;
-    |                  ^^^^ expected an `Fn<()>` closure, found `T`
+    |     ^^^^^^^^^^ expected an `Fn<()>` closure, found `T`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-68643-broken-mir.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68643-broken-mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68643-broken-mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: expected a `Fn<()>` closure, found `T`
   |
   |
LL |     type F<'a> = Self;
   |     ^^^^^^^^^^ expected an `Fn<()>` closure, found `T`
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


---- [ui] src/test/ui/generic-associated-types/issue-68641-check-gat-bounds.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `T: Copy` is not satisfied
-   --> $DIR/issue-68641-check-gat-bounds.rs:12:21
3    |
3    |
4 LL |     type Item<'a> = T;
-    |                     ^ the trait `Copy` is not implemented for `T`
+    |     ^^^^^^^^^^^^^ the trait `Copy` is not implemented for `T`
6    |
7 note: required by a bound in `UnsafeCopy::Item`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68641-check-gat-bounds/issue-68641-check-gat-bounds.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68641-check-gat-bounds/issue-68641-check-gat-bounds.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-68641-check-gat-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-68641-check-gat-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68641-check-gat-bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68641-check-gat-bounds/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `T: Copy` is not satisfied
   |
   |
LL |     type Item<'a> = T;
   |     ^^^^^^^^^^^^^ the trait `Copy` is not implemented for `T`
   |
note: required by a bound in `UnsafeCopy::Item`
   |
   |
LL |     type Item<'a>: Copy;
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |                    ^^^^ required by this bound in `UnsafeCopy::Item`
help: consider restricting type parameter `T`
   |
LL | impl<T: std::marker::Copy> UnsafeCopy for T {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/generic-associated-types/issue-68642-broken-llvm-ir.rs stdout ----
diff of stderr:

1 error[E0277]: expected a `Fn<()>` closure, found `T`
-   --> $DIR/issue-68642-broken-llvm-ir.rs:12:18
3    |
3    |
4 LL |     type F<'a> = Self;
-    |                  ^^^^ expected an `Fn<()>` closure, found `T`
+    |     ^^^^^^^^^^ expected an `Fn<()>` closure, found `T`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-68642-broken-llvm-ir.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68642-broken-llvm-ir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68642-broken-llvm-ir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: expected a `Fn<()>` closure, found `T`
   |
   |
LL |     type F<'a> = Self;
   |     ^^^^^^^^^^ expected an `Fn<()>` closure, found `T`
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


---- [ui] src/test/ui/generic-associated-types/issue-68645-codegen-fulfillment.rs stdout ----
diff of stderr:

1 error[E0277]: expected a `Fn<()>` closure, found `T`
-   --> $DIR/issue-68645-codegen-fulfillment.rs:12:18
3    |
3    |
4 LL |     type F<'a> = Self;
-    |                  ^^^^ expected an `Fn<()>` closure, found `T`
+    |     ^^^^^^^^^^ expected an `Fn<()>` closure, found `T`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-68645-codegen-fulfillment.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68645-codegen-fulfillment" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68645-codegen-fulfillment/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: expected a `Fn<()>` closure, found `T`
   |
   |
LL |     type F<'a> = Self;
   |     ^^^^^^^^^^ expected an `Fn<()>` closure, found `T`
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


---- [ui] src/test/ui/generic-associated-types/issue-68644-codegen-selection.rs stdout ----
diff of stderr:

1 error[E0277]: expected a `Fn<()>` closure, found `T`
-   --> $DIR/issue-68644-codegen-selection.rs:12:18
3    |
3    |
4 LL |     type F<'a> = Self;
-    |                  ^^^^ expected an `Fn<()>` closure, found `T`
+    |     ^^^^^^^^^^ expected an `Fn<()>` closure, found `T`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-68644-codegen-selection.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68644-codegen-selection" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68644-codegen-selection/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: expected a `Fn<()>` closure, found `T`
   |
   |
LL |     type F<'a> = Self;
   |     ^^^^^^^^^^ expected an `Fn<()>` closure, found `T`
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


---- [ui] src/test/ui/generic-associated-types/issue-68656-unsized-values.rs stdout ----
diff of stderr:

1 error[E0271]: type mismatch resolving `<T as Deref>::Target == T`
-   --> $DIR/issue-68656-unsized-values.rs:13:21
3    |
3    |
4 LL | impl<T: Copy + std::ops::Deref> UnsafeCopy<T> for T {


6 LL |     type Item<'a> = T;
-    |                     ^ expected type parameter `T`, found associated type
+    |     ^^^^^^^^^^^^^ expected type parameter `T`, found associated type
9    = note: expected type parameter `T`
10              found associated type `<T as Deref>::Target`



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68656-unsized-values/issue-68656-unsized-values.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-68656-unsized-values.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-68656-unsized-values.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68656-unsized-values" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68656-unsized-values/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0271]: type mismatch resolving `<T as Deref>::Target == T`
   |
   |
LL | impl<T: Copy + std::ops::Deref> UnsafeCopy<T> for T {
   |      - this type parameter
LL |     type Item<'a> = T;
   |     ^^^^^^^^^^^^^ expected type parameter `T`, found associated type
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
-   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:15:18
+   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:15:5
3    |
4 LL |     type T<'a> = Box<dyn A + 'a>;
+    |     ^^^^^^^^^^
6    |
6    |
7 note: because this has an unmet lifetime requirement

26    |                      ++++
27 
28 error: incompatible lifetime on type
28 error: incompatible lifetime on type
-   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:25:18
+   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:25:5
30    |
31 LL |     type T<'a> = Box<dyn A + 'a>;
+    |     ^^^^^^^^^^
33    |
33    |
34 note: because this has an unmet lifetime requirement

48    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
49 
50 error: incompatible lifetime on type
50 error: incompatible lifetime on type
-   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:35:18
+   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:35:5
52    |
53 LL |     type T<'a> = (Box<dyn A + 'a>, Box<dyn A + 'a>);
+    |     ^^^^^^^^^^
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box.rs:15:5
   |
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
   = note: ...does not necessarily outlive the static lifetime introduced by the compatible `impl`
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
  --> /checkout/src/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box.rs:25:5
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
note: ...does not necessarily outlive the static lifetime introduced by the compatible `impl`
  --> /checkout/src/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box.rs:19:1
   |
   |
LL | impl C for Box<dyn A + 'static> {}

error: incompatible lifetime on type
  --> /checkout/src/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box.rs:35:5
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
   = note: ...does not necessarily outlive the static lifetime introduced by the compatible `impl`
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
------------------------------------------

---
14 LL |     default type Member<'a> = Foo;
-    |                               ^^^ no implementation for `Foo == Foo`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `Foo == Foo`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-87429-specialization.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87429-specialization" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87429-specialization/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(specialization)]
   |            ^^^^^^^^^^^^^^
   |
---
   |
LL |     default type Member<'a> = Foo;
   |     ^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `Foo == Foo`
   |
   = help: the trait `PartialEq` is not implemented for `Foo`
note: required by a bound in `Family::Member`
   |
   |
LL |     type Member<'a>: for<'b> PartialEq<Self::Member<'b>>;
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Family::Member`
help: consider annotating `Foo` with `#[derive(PartialEq)]`
   |
LL | #[derive(PartialEq)]

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/generic-associated-types/issue-90014.rs stdout ----
diff of stderr:

1 error[E0477]: the type `&mut ()` does not fulfill the required lifetime
-   --> $DIR/issue-90014.rs:13:20
3    |
3    |
4 LL |     type Fut<'a> where Self: 'a;
5    |     ------------ definition of `Fut` from trait
6 ...
6 ...
7 LL |     type Fut<'a> = impl Future<Output = ()>;
-    |                    ^^^^^^^^^^^^^^^^^^^^^^^^- help: try copying this clause from the trait: `where Self: 'a`
+    |     ^^^^^^^^^^^^                           - help: try copying this clause from the trait: `where Self: 'a`
9    |
10 note: type must outlive the lifetime `'a` as defined here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-90014/issue-90014.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-90014/issue-90014.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-90014.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-90014.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-90014" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-90014/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0477]: the type `&mut ()` does not fulfill the required lifetime
   |
   |
LL |     type Fut<'a> where Self: 'a;
   |     ------------ definition of `Fut` from trait
...
LL |     type Fut<'a> = impl Future<Output = ()>;
   |     ^^^^^^^^^^^^                           - help: try copying this clause from the trait: `where Self: 'a`
   |
note: type must outlive the lifetime `'a` as defined here
   |
   |
LL |     type Fut<'a> = impl Future<Output = ()>;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0477`.
For more information about this error, try `rustc --explain E0477`.
------------------------------------------


---- [ui] src/test/ui/generic-associated-types/issue-92033.rs stdout ----
diff of stderr:

1 error[E0477]: the type `&'s Texture` does not fulfill the required lifetime
-   --> $DIR/issue-92033.rs:20:28
3    |
3    |
4 LL |     type TextureIter<'a>: Iterator<Item = &'a Texture>
5    |     -------------------------------------------------- definition of `TextureIter` from trait
6 ...
6 ...
7 LL |     type TextureIter<'a> = std::option::IntoIter<&'a Texture>;
-    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- help: try copying this clause from the trait: `where Self: 'a`
+    |     ^^^^^^^^^^^^^^^^^^^^                                     - help: try copying this clause from the trait: `where Self: 'a`
9    |
10 note: type must outlive the lifetime `'a` as defined here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-92033/issue-92033.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-92033/issue-92033.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-92033.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-92033.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-92033" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-92033/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0477]: the type `&'s Texture` does not fulfill the required lifetime
   |
   |
LL |     type TextureIter<'a>: Iterator<Item = &'a Texture>
   |     -------------------------------------------------- definition of `TextureIter` from trait
...
LL |     type TextureIter<'a> = std::option::IntoIter<&'a Texture>;
   |     ^^^^^^^^^^^^^^^^^^^^                                     - help: try copying this clause from the trait: `where Self: 'a`
   |
note: type must outlive the lifetime `'a` as defined here
   |
   |
LL |     type TextureIter<'a> = std::option::IntoIter<&'a Texture>;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0477`.
For more information about this error, try `rustc --explain E0477`.
------------------------------------------


---- [ui] src/test/ui/generic-associated-types/issue-91883.rs stdout ----
diff of stderr:

1 error[E0478]: lifetime bound not satisfied
-   --> $DIR/issue-91883.rs:30:24
3    |
3    |
4 LL |     type Cursor<'tx>: Cursor<'tx>
5    |     ----------------------------- definition of `Cursor` from trait
6 ...
6 ...
7 LL |     type Cursor<'tx> = CursorImpl<'tx>;
-    |                        ^^^^^^^^^^^^^^^- help: try copying these clauses from the trait: `where 'db: 'tx, Self: 'tx`
+    |     ^^^^^^^^^^^^^^^^                  - help: try copying these clauses from the trait: `where 'db: 'tx, Self: 'tx`
9    |
10 note: lifetime parameter instantiated with the lifetime `'db` as defined here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-91883/issue-91883.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-91883/issue-91883.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-91883.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-91883.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-91883" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-91883/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0478]: lifetime bound not satisfied
   |
   |
LL |     type Cursor<'tx>: Cursor<'tx>
   |     ----------------------------- definition of `Cursor` from trait
...
LL |     type Cursor<'tx> = CursorImpl<'tx>; //~ ERROR lifetime bound not satisfied
   |     ^^^^^^^^^^^^^^^^                  - help: try copying these clauses from the trait: `where 'db: 'tx, Self: 'tx`
   |
note: lifetime parameter instantiated with the lifetime `'db` as defined here
   |
   |
LL | impl<'db> Transaction<'db> for TransactionImpl<'db> {
   |      ^^^
note: but lifetime parameter must outlive the lifetime `'tx` as defined here
   |
   |
LL |     type Cursor<'tx> = CursorImpl<'tx>; //~ ERROR lifetime bound not satisfied

error: aborting due to previous error

For more information about this error, try `rustc --explain E0478`.
For more information about this error, try `rustc --explain E0478`.
------------------------------------------


---- [ui] src/test/ui/generic-associated-types/unsatisfied-outlives-bound.rs stdout ----
diff of stderr:

1 error[E0477]: the type `&'b ()` does not fulfill the required lifetime
-   --> $DIR/unsatisfied-outlives-bound.rs:6:21
3    |
3    |
4 LL |     type Item<'a> = &'b ();
+    |     ^^^^^^^^^^^^^
6    |
6    |
7 note: type must outlive the lifetime `'a` as defined here as required by this binding

11    |               ^^
12 
12 
13 error[E0477]: the type `&'a ()` does not fulfill the required lifetime
-   --> $DIR/unsatisfied-outlives-bound.rs:15:21
15    |
15    |
16 LL |     type Item<'a> = &'a ();
+    |     ^^^^^^^^^^^^^
18    |
19 note: type must satisfy the static lifetime as required by this binding
20   --> $DIR/unsatisfied-outlives-bound.rs:11:20
---
To only update this specific test, also pass `--test-args generic-associated-types/unsatisfied-outlives-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/unsatisfied-outlives-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/unsatisfied-outlives-bound" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/unsatisfied-outlives-bound/auxiliary"
stdout: none
--- stderr -------------------------------
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
  --> /checkout/src/test/ui/generic-associated-types/unsatisfied-outlives-bound.rs:11:20
   |
   |
LL |     type Item<'a>: 'static;

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0477`.
For more information about this error, try `rustc --explain E0477`.
------------------------------------------


---- [ui] src/test/ui/impl-trait/issues/issue-99348-impl-compatibility.rs stdout ----
diff of stderr:

1 error[E0271]: type mismatch resolving `<Concrete as Bar>::Other == Concrete`
-   --> $DIR/issue-99348-impl-compatibility.rs:8:17
3    |
4 LL | type Tait = impl Sized;
5    |             ---------- the found opaque type


6 ...
7 LL |     type Item = Concrete;
-    |                 ^^^^^^^^ type mismatch resolving `<Concrete as Bar>::Other == Concrete`
+    |     ^^^^^^^^^ type mismatch resolving `<Concrete as Bar>::Other == Concrete`
10 note: expected this to be `Concrete`
11   --> $DIR/issue-99348-impl-compatibility.rs:13:18



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-99348-impl-compatibility/issue-99348-impl-compatibility.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/issues/issue-99348-impl-compatibility.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/issue-99348-impl-compatibility.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-99348-impl-compatibility" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-99348-impl-compatibility/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0271]: type mismatch resolving `<Concrete as Bar>::Other == Concrete`
   |
LL | type Tait = impl Sized;
   |             ---------- the found opaque type
...
...
LL |     type Item = Concrete;
   |     ^^^^^^^^^ type mismatch resolving `<Concrete as Bar>::Other == Concrete`
note: expected this to be `Concrete`
  --> /checkout/src/test/ui/impl-trait/issues/issue-99348-impl-compatibility.rs:13:18
   |
   |
LL |     type Other = Tait;
   = note:   expected struct `Concrete`
           found opaque type `Tait`
note: required by a bound in `Foo::Item`
  --> /checkout/src/test/ui/impl-trait/issues/issue-99348-impl-compatibility.rs:17:20
  --> /checkout/src/test/ui/impl-trait/issues/issue-99348-impl-compatibility.rs:17:20
   |
LL |     type Item: Bar<Other = Self>;
   |                    ^^^^^^^^^^^^ required by this bound in `Foo::Item`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.
------------------------------------------
---
1 error[E0277]: `!` is not an iterator
-   --> $DIR/issue-51506.rs:13:24
+   --> $DIR/issue-51506.rs:13:5
3    |
4 LL |     default type Out = !;
-    |                        ^ `!` is not an iterator
+    |     ^^^^^^^^^^^^^^^^ `!` is not an iterator
7    = help: the trait `Iterator` is not implemented for `!`
8 note: required by a bound in `Trait::Out`



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/issue-51506/issue-51506.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args never_type/issue-51506.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/never_type/issue-51506.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/issue-51506" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/issue-51506/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `!` is not an iterator
   |
   |
LL |     default type Out = !; //~ ERROR: `!` is not an iterator
   |     ^^^^^^^^^^^^^^^^ `!` is not an iterator
   = help: the trait `Iterator` is not implemented for `!`
note: required by a bound in `Trait::Out`
  --> /checkout/src/test/ui/never_type/issue-51506.rs:7:15
   |
---

---- [ui] src/test/ui/regions/regions-assoc-type-region-bound-in-trait-not-met.rs stdout ----
diff of stderr:

1 error[E0477]: the type `&'a i32` does not fulfill the required lifetime
-   --> $DIR/regions-assoc-type-region-bound-in-trait-not-met.rs:15:18
3    |
4 LL |     type Value = &'a i32;
-    |                  ^^^^^^^
+    |     ^^^^^^^^^^
+    |     ^^^^^^^^^^
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
+    |     ^^^^^^^^^^
+    |     ^^^^^^^^^^
18    |
19 note: type must outlive the lifetime `'b` as defined here as required by this binding


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-region-bound-in-trait-not-met/regions-assoc-type-region-bound-in-trait-not-met.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-region-bound-in-trait-not-met/regions-assoc-type-region-bound-in-trait-not-met.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-assoc-type-region-bound-in-trait-not-met.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-assoc-type-region-bound-in-trait-not-met.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-region-bound-in-trait-not-met" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-region-bound-in-trait-not-met/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0477]: the type `&'a i32` does not fulfill the required lifetime
   |
LL |     type Value = &'a i32;
   |     ^^^^^^^^^^
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
   |     ^^^^^^^^^^
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


---- [ui] src/test/ui/regions/regions-assoc-type-static-bound-in-trait-not-met.rs stdout ----
diff of stderr:

1 error[E0477]: the type `&'a i32` does not fulfill the required lifetime
-   --> $DIR/regions-assoc-type-static-bound-in-trait-not-met.rs:10:18
3    |
4 LL |     type Value = &'a i32;
-    |                  ^^^^^^^
+    |     ^^^^^^^^^^
---
To only update this specific test, also pass `--test-args regions/regions-assoc-type-static-bound-in-trait-not-met.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-assoc-type-static-bound-in-trait-not-met.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-static-bound-in-trait-not-met" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-static-bound-in-trait-not-met/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0477]: the type `&'a i32` does not fulfill the required lifetime
   |
LL |     type Value = &'a i32;
   |     ^^^^^^^^^^
   |
---

---- [ui] src/test/ui/rfc-2632-const-trait-impl/assoc-type.rs stdout ----
diff of stderr:

1 error[E0277]: cannot add `NonConstAdd` to `NonConstAdd` in const contexts
-   --> $DIR/assoc-type.rs:19:16
3    |
4 LL |     type Bar = NonConstAdd;
4 LL |     type Bar = NonConstAdd;
-    |                ^^^^^^^^^^^ no implementation for `NonConstAdd + NonConstAdd`
+    |     ^^^^^^^^ no implementation for `NonConstAdd + NonConstAdd`
6    |
7    = help: the trait `~const Add` is not implemented for `NonConstAdd`
8 note: the trait `Add` is implemented for `NonConstAdd`, but that implementation is not `const`
-   --> $DIR/assoc-type.rs:19:16
+   --> $DIR/assoc-type.rs:19:5
10    |
11 LL |     type Bar = NonConstAdd;
---
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/assoc-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/assoc-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/assoc-type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/assoc-type/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: cannot add `NonConstAdd` to `NonConstAdd` in const contexts
   |
LL |     type Bar = NonConstAdd;
LL |     type Bar = NonConstAdd;
   |     ^^^^^^^^ no implementation for `NonConstAdd + NonConstAdd`
   |
   = help: the trait `~const Add` is not implemented for `NonConstAdd`
note: the trait `Add` is implemented for `NonConstAdd`, but that implementation is not `const`
   |
LL |     type Bar = NonConstAdd;
   |     ^^^^^^^^
note: required by a bound in `Foo::Bar`
note: required by a bound in `Foo::Bar`
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/assoc-type.rs:15:15
   |
LL |     type Bar: ~const std::ops::Add;
   |               ^^^^^^^^^^^^^^^^^^^^ required by this bound in `Foo::Bar`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/specialization/default-associated-type-bound-2.rs stdout ----
diff of stderr:

9    = note: `#[warn(incomplete_features)]` on by default
10 
11 error[E0277]: can't compare `&'static B` with `B`
-   --> $DIR/default-associated-type-bound-2.rs:16:22
13    |
14 LL |     default type U = &'static B;
14 LL |     default type U = &'static B;
-    |                      ^^^^^^^^^^ no implementation for `&'static B == B`
+    |     ^^^^^^^^^^^^^^ no implementation for `&'static B == B`
16    |
17    = help: the trait `PartialEq<B>` is not implemented for `&'static B`
18 note: required by a bound in `X::U`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/default-associated-type-bound-2/default-associated-type-bound-2.stderr
To only update this specific test, also pass `--test-args specialization/default-associated-type-bound-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/default-associated-type-bound-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/default-associated-type-bound-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/default-associated-type-bound-2/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(specialization)]
   |            ^^^^^^^^^^^^^^
   |
   |
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete
   = note: `#[warn(incomplete_features)]` on by default

error[E0277]: can't compare `&'static B` with `B`
   |
LL |     default type U = &'static B;
LL |     default type U = &'static B;
   |     ^^^^^^^^^^^^^^ no implementation for `&'static B == B`
   |
   = help: the trait `PartialEq<B>` is not implemented for `&'static B`
note: required by a bound in `X::U`
   |
   |
LL |     type U: PartialEq<T>;
   |             ^^^^^^^^^^^^ required by this bound in `X::U`
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL | impl<B: 'static, T> X<B> for T where &'static B: PartialEq<B> {

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.
---
14 LL |     default type U = str;
-    |                      ^^^ the trait `Clone` is not implemented for `str`
+    |     ^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `str`
16    |
17    = help: the trait `Clone` is implemented for `String`
18 note: required by a bound in `X::U`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/default-associated-type-bound-1/default-associated-type-bound-1.stderr
To only update this specific test, also pass `--test-args specialization/default-associated-type-bound-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/default-associated-type-bound-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/default-associated-type-bound-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/default-associated-type-bound-1/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(specialization)]
   |            ^^^^^^^^^^^^^^
   |
---
   |
LL |     default type U = str;
   |     ^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `str`
   |
   = help: the trait `Clone` is implemented for `String`
note: required by a bound in `X::U`
   |
   |
LL |     type U: Clone;
   |             ^^^^^ required by this bound in `X::U`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/specialization/default-generic-associated-type-bound.rs stdout ----
diff of stderr:

9    = note: `#[warn(incomplete_features)]` on by default
10 
11 error[E0277]: can't compare `T` with `T`
-   --> $DIR/default-generic-associated-type-bound.rs:17:26
13    |
13    |
14 LL |     default type U<'a> = &'a T;
-    |                          ^^^^^ no implementation for `T == T`
+    |     ^^^^^^^^^^^^^^^^^^ no implementation for `T == T`
16    |
17    = note: required for `&'a T` to implement `PartialEq`
18 note: required by a bound in `X::U`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/default-generic-associated-type-bound/default-generic-associated-type-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args specialization/default-generic-associated-type-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/default-generic-associated-type-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/default-generic-associated-type-bound" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/default-generic-associated-type-bound/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(specialization)]
   |            ^^^^^^^^^^^^^^
   |
   |
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete
   = note: `#[warn(incomplete_features)]` on by default

error[E0277]: can't compare `T` with `T`
   |
   |
LL |     default type U<'a> = &'a T;
   |     ^^^^^^^^^^^^^^^^^^ no implementation for `T == T`
   |
   = note: required for `&'a T` to implement `PartialEq`
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
---
diff of stderr:

9    = note: `#[warn(incomplete_features)]` on by default
10 
11 error[E0277]: the trait bound `(): Valid` is not satisfied
-   --> $DIR/issue-38091.rs:12:23
13    |
14 LL |     default type Ty = ();
14 LL |     default type Ty = ();
-    |                       ^^ the trait `Valid` is not implemented for `()`
+    |     ^^^^^^^^^^^^^^^ the trait `Valid` is not implemented for `()`
17 note: required by a bound in `Iterate::Ty`
18   --> $DIR/issue-38091.rs:5:14



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-38091/issue-38091.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args specialization/issue-38091.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/issue-38091.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-38091" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-38091/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(specialization)]
   |            ^^^^^^^^^^^^^^
   |
   |
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete
   = note: `#[warn(incomplete_features)]` on by default

error[E0277]: the trait bound `(): Valid` is not satisfied
   |
LL |     default type Ty = ();
LL |     default type Ty = ();
   |     ^^^^^^^^^^^^^^^ the trait `Valid` is not implemented for `()`
note: required by a bound in `Iterate::Ty`
  --> /checkout/src/test/ui/specialization/issue-38091.rs:5:14
   |
LL |     type Ty: Valid;
---

---- [ui] src/test/ui/specialization/issue-33017.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `T: Copy` is not satisfied
-   --> $DIR/issue-33017.rs:12:27
3    |
4 LL |     default type Output = Self;
-    |                           ^^^^ the trait `Copy` is not implemented for `T`
+    |     ^^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `T`
+    |     ^^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `T`
6    |
7 note: required by a bound in `UncheckedCopy::Output`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-33017/issue-33017.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-33017/issue-33017.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args specialization/issue-33017.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/issue-33017.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-33017" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-33017/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `T: Copy` is not satisfied
   |
LL |     default type Output = Self;
   |     ^^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `T`
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
For more information about this error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/specialization/issue-44861.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `(): CoerceUnsized<*const [u8]>` is not satisfied
-   --> $DIR/issue-44861.rs:21:26
3    |
4 LL |     default type Data2 = ();
4 LL |     default type Data2 = ();
-    |                          ^^ the trait `CoerceUnsized<*const [u8]>` is not implemented for `()`
+    |     ^^^^^^^^^^^^^^^^^^ the trait `CoerceUnsized<*const [u8]>` is not implemented for `()`
6    |
7 note: required by a bound in `Smartass::Data2`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-44861/issue-44861.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-44861/issue-44861.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args specialization/issue-44861.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/issue-44861.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-44861" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-44861/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `(): CoerceUnsized<*const [u8]>` is not satisfied
   |
LL |     default type Data2 = ();
LL |     default type Data2 = ();
   |     ^^^^^^^^^^^^^^^^^^ the trait `CoerceUnsized<*const [u8]>` is not implemented for `()`
   |
note: required by a bound in `Smartass::Data2`
   |
   |
LL |     type Data2: CoerceUnsized<*const [u8]>;
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Smartass::Data2`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/specialization/issue-59435.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `MyStruct: Default` is not satisfied
-   --> $DIR/issue-59435.rs:11:27
3    |
3    |
4 LL |     default type MyType = MyStruct;
-    |                           ^^^^^^^^ the trait `Default` is not implemented for `MyStruct`
+    |     ^^^^^^^^^^^^^^^^^^^ the trait `Default` is not implemented for `MyStruct`
6    |
7 note: required by a bound in `MyTrait::MyType`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-59435/issue-59435.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-59435/issue-59435.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args specialization/issue-59435.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/issue-59435.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-59435" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-59435/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `MyStruct: Default` is not satisfied
   |
   |
LL |     default type MyType = MyStruct;
   |     ^^^^^^^^^^^^^^^^^^^ the trait `Default` is not implemented for `MyStruct`
   |
note: required by a bound in `MyTrait::MyType`
   |
   |
LL |     type MyType: Default;
   |                  ^^^^^^^ required by this bound in `MyTrait::MyType`
help: consider annotating `MyStruct` with `#[derive(Default)]`
LL | #[derive(Default)]
   |

error: aborting due to previous error
---

---- [ui] src/test/ui/traits/inductive-overflow/two-traits.rs stdout ----
diff of stderr:

1 error[E0277]: `T` cannot be shared between threads safely
-   --> $DIR/two-traits.rs:11:14
3    |
4 LL |     type X = Self;
4 LL |     type X = Self;
-    |              ^^^^ `T` cannot be shared between threads safely
+    |     ^^^^^^ `T` cannot be shared between threads safely
6    |
7 note: required by a bound in `Magic::X`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/two-traits/two-traits.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/two-traits/two-traits.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/inductive-overflow/two-traits.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/inductive-overflow/two-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/two-traits" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/two-traits/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `T` cannot be shared between threads safely
   |
LL |     type X = Self;
LL |     type X = Self;
   |     ^^^^^^ `T` cannot be shared between threads safely
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


---- [ui] src/test/ui/type-alias-impl-trait/issue-57961.rs stdout ----
diff of stderr:

1 error[E0271]: expected `IntoIter<u32>` to be an iterator that yields `X`, but it yields `u32`
-   --> $DIR/issue-57961.rs:10:16
3    |
3    |
4 LL | type X = impl Sized;

6 ...
7 LL |     type Bar = std::vec::IntoIter<u32>;
-    |                ^^^^^^^^^^^^^^^^^^^^^^^ expected opaque type, found `u32`
---
To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-57961.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-57961.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57961" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57961/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0271]: expected `IntoIter<u32>` to be an iterator that yields `X`, but it yields `u32`
   |
   |
LL | type X = impl Sized;
...
LL |     type Bar = std::vec::IntoIter<u32>;
   |     ^^^^^^^^ expected opaque type, found `u32`
   |
   |
   = note: expected opaque type `X`
                     found type `u32`
note: required by a bound in `Foo::Bar`
  --> /checkout/src/test/ui/type-alias-impl-trait/issue-57961.rs:6:24
   |
LL |     type Bar: Iterator<Item = X>;
   |                        ^^^^^^^^ required by this bound in `Foo::Bar`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.
------------------------------------------
