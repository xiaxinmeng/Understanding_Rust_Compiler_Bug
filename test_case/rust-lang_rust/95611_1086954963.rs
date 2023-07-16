plain
.................................................................................................... 5600/12853
...................................................................i................................ 5700/12853
.................................................................................................... 5800/12853
.................................................................................................... 5900/12853
....................................................F............................................... 6000/12853
..........................F.....................................................................F... 6100/12853
.............i...................................................................................... 6300/12853
.............................................................................................i...... 6400/12853
.................................................................................................... 6500/12853
..........................................................i......................................... 6600/12853
---
................................................iii................................................. 12800/12853
.....................................................
failures:

---- [ui] ui/associated-types/bound-lifetime-in-binding-only.rs#ok stdout ----

- warning: where-clause bound is impossible to satisfy
- warning: where-clause bound is impossible to satisfy
+ error: where-clause bound is impossible to satisfy
3    |
3    |
4 LL | fn ok3<T>() where for<'a> Parameterized<'a>: Foo<Item=&'a i32> {
7    = note: this bound was previously accepted, but it may become a hard error in a future release
7    = note: this bound was previously accepted, but it may become a hard error in a future release
8    = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
- warning: where-clause bound is impossible to satisfy
- warning: where-clause bound is impossible to satisfy
+ error: where-clause bound is impossible to satisfy
12    |
12    |
13 LL | fn ok3<T>() where for<'a> Parameterized<'a>: Foo<Item=&'a i32> {
16    = note: this bound was previously accepted, but it may become a hard error in a future release
16    = note: this bound was previously accepted, but it may become a hard error in a future release
17    = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
18 
- error: fatal error triggered by #[rustc_error]
-   --> $DIR/bound-lifetime-in-binding-only.rs:71:1
- LL | fn main() { }
-    | ^^^^^^^^^
- 
- error: aborting due to previous error; 2 warnings emitted
- error: aborting due to previous error; 2 warnings emitted
+ error: aborting due to 2 previous errors
26 
27 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/bound-lifetime-in-binding-only.ok/bound-lifetime-in-binding-only.ok.stderr
To only update this specific test, also pass `--test-args associated-types/bound-lifetime-in-binding-only.rs`


error in revision `ok`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/bound-lifetime-in-binding-only.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "ok" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/bound-lifetime-in-binding-only.ok" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/bound-lifetime-in-binding-only.ok/auxiliary"
stdout: none
--- stderr -------------------------------
error: where-clause bound is impossible to satisfy
   |
   |
LL | fn ok3<T>() where for<'a> Parameterized<'a>: Foo<Item=&'a i32> {
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
error: where-clause bound is impossible to satisfy
  --> /checkout/src/test/ui/associated-types/bound-lifetime-in-binding-only.rs:67:19
   |
   |
LL | fn ok3<T>() where for<'a> Parameterized<'a>: Foo<Item=&'a i32> {
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] ui/associated-types/issue-69398.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-69398.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-69398" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-69398/auxiliary"
stdout: none
--- stderr -------------------------------
error: where-clause bound is impossible to satisfy
   |
   |
LL |     fn broken(&self) where Self::Assoc: Foo {
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
error: aborting due to previous error
------------------------------------------



---- [ui] ui/consts/issue-67696-const-prop-ice.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-67696-const-prop-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-67696-const-prop-ice" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--emit=mir,link" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-67696-const-prop-ice/auxiliary"
stdout: none
--- stderr -------------------------------
error: where-clause bound is impossible to satisfy
   |
   |
LL |     fn foo(&self) -> Self where Self: Copy { *(&[] as &[_]) }
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
error: where-clause bound is impossible to satisfy
  --> /checkout/src/test/ui/consts/issue-67696-const-prop-ice.rs:13:33
   |
   |
LL |     fn foo(&self) -> Self where Self: Copy { *(&[] as &[_]) }
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
error: where-clause bound is impossible to satisfy
  --> /checkout/src/test/ui/consts/issue-67696-const-prop-ice.rs:13:33
   |
   |
LL |     fn foo(&self) -> Self where Self: Copy { *(&[] as &[_]) }
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
error: aborting due to 3 previous errors
------------------------------------------



---- [ui] ui/feature-gates/feature-gate-trivial_bounds.rs stdout ----
diff of stderr:

107    = help: see issue #48214
108    = help: add `#![feature(trivial_bounds)]` to the crate attributes to enable
- warning: where-clause bound is impossible to satisfy
- warning: where-clause bound is impossible to satisfy
+ error: where-clause bound is impossible to satisfy
112    |
112    |
113 LL | fn global_hr(x: fn(&())) where fn(&()): Foo { // OK
116    = note: this bound was previously accepted, but it may become a hard error in a future release
116    = note: this bound was previously accepted, but it may become a hard error in a future release
117    = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
- error: aborting due to 11 previous errors; 1 warning emitted
+ error: aborting due to 12 previous errors
120 
121 For more information about this error, try `rustc --explain E0277`.
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-trivial_bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-trivial_bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-trivial_bounds" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-trivial_bounds/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `i32: Foo` is not satisfied
   |
   |
LL | enum E where i32: Foo { V } //~ ERROR
   |
   = help: see issue #48214
   = help: see issue #48214
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to enable
error[E0277]: the trait bound `i32: Foo` is not satisfied
  --> /checkout/src/test/ui/feature-gates/feature-gate-trivial_bounds.rs:12:16
   |
   |
LL | struct S where i32: Foo; //~ ERROR
   |
   = help: see issue #48214
   = help: see issue #48214
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to enable
error[E0277]: the trait bound `i32: Foo` is not satisfied
  --> /checkout/src/test/ui/feature-gates/feature-gate-trivial_bounds.rs:14:15
   |
   |
LL | trait T where i32: Foo {} //~ ERROR
   |
   = help: see issue #48214
   = help: see issue #48214
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to enable
error[E0277]: the trait bound `i32: Foo` is not satisfied
  --> /checkout/src/test/ui/feature-gates/feature-gate-trivial_bounds.rs:16:15
   |
   |
LL | union U where i32: Foo { f: i32 } //~ ERROR
   |
   = help: see issue #48214
   = help: see issue #48214
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to enable
error[E0277]: the trait bound `i32: Foo` is not satisfied
  --> /checkout/src/test/ui/feature-gates/feature-gate-trivial_bounds.rs:20:23
   |
   |
LL | impl Foo for () where i32: Foo { //~ ERROR
   |
   = help: see issue #48214
   = help: see issue #48214
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to enable
error[E0277]: the trait bound `i32: Foo` is not satisfied
  --> /checkout/src/test/ui/feature-gates/feature-gate-trivial_bounds.rs:28:14
   |
   |
LL | fn f() where i32: Foo //~ ERROR
   |
   = help: see issue #48214
   = help: see issue #48214
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to enable
error[E0277]: the trait bound `String: Neg` is not satisfied
  --> /checkout/src/test/ui/feature-gates/feature-gate-trivial_bounds.rs:36:38
   |
   |
LL | fn use_op(s: String) -> String where String: ::std::ops::Neg<Output=String> { //~ ERROR
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Neg` is not implemented for `String`
   = help: see issue #48214
   = help: see issue #48214
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to enable
error[E0277]: `i32` is not an iterator
  --> /checkout/src/test/ui/feature-gates/feature-gate-trivial_bounds.rs:40:20
   |
   |
LL | fn use_for() where i32: Iterator { //~ ERROR
   |                    ^^^^^^^^^^^^^ `i32` is not an iterator
   = help: the trait `Iterator` is not implemented for `i32`
   = help: the trait `Iterator` is not implemented for `i32`
   = note: if you want to iterate between `start` until a value `end`, use the exclusive range syntax `start..end` or the inclusive range syntax `start..=end`
   = help: see issue #48214
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to enable
error[E0277]: the size for values of type `str` cannot be known at compilation time
  --> /checkout/src/test/ui/feature-gates/feature-gate-trivial_bounds.rs:52:32
   |
   |
LL | struct TwoStrs(str, str) where str: Sized; //~ ERROR
   |
   = help: the trait `Sized` is not implemented for `str`
   = help: see issue #48214
   = help: see issue #48214
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to enable

error[E0277]: the size for values of type `(dyn A + 'static)` cannot be known at compilation time
   |
   |
LL | fn unsized_local() where Dst<dyn A>: Sized { //~ ERROR
   |
   |
   = help: within `Dst<(dyn A + 'static)>`, the trait `Sized` is not implemented for `(dyn A + 'static)`
note: required because it appears within the type `Dst<(dyn A + 'static)>`
   |
   |
LL | struct Dst<X: ?Sized> {
   = help: see issue #48214
   = help: see issue #48214
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to enable
error[E0277]: the size for values of type `str` cannot be known at compilation time
  --> /checkout/src/test/ui/feature-gates/feature-gate-trivial_bounds.rs:59:30
   |
   |
LL | fn return_str() -> str where str: Sized { //~ ERROR
   |
   = help: the trait `Sized` is not implemented for `str`
   = help: see issue #48214
   = help: see issue #48214
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to enable
error: where-clause bound is impossible to satisfy
  --> /checkout/src/test/ui/feature-gates/feature-gate-trivial_bounds.rs:65:32
   |
   |
LL | fn global_hr(x: fn(&())) where fn(&()): Foo { // OK
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
error: aborting due to 12 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] ui/higher-rank-trait-bounds/normalize-under-binder/issue-89118.rs stdout ----
diff of stderr:

- warning: where-clause bound is impossible to satisfy
+ error: where-clause bound is impossible to satisfy
3    |
3    |
4 LL |     Ctx<()>: for<'a> BufferUdpStateContext<&'a ()>,

47 LL |     Ctx<()>: for<'a> BufferUdpStateContext<&'a ()>,
48    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `StackContext`
- warning: where-clause bound is impossible to satisfy
- warning: where-clause bound is impossible to satisfy
+ error: where-clause bound is impossible to satisfy
52    |
52    |
53 LL |     Ctx<()>: for<'a> BufferUdpStateContext<&'a ()>;

76 LL |     Ctx<()>: for<'a> BufferUdpStateContext<&'a ()>;
77    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `EthernetWorker`
- error: aborting due to 3 previous errors; 2 warnings emitted
+ error: aborting due to 5 previous errors
80 
81 For more information about this error, try `rustc --explain E0277`.
---
To only update this specific test, also pass `--test-args higher-rank-trait-bounds/normalize-under-binder/issue-89118.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-89118.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-89118" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-89118/auxiliary"
stdout: none
--- stderr -------------------------------
error: where-clause bound is impossible to satisfy
   |
   |
LL |     Ctx<()>: for<'a> BufferUdpStateContext<&'a ()>,
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it

error[E0277]: the trait bound `for<'a> &'a (): BufferMut` is not satisfied
   |
LL |     C: StackContext,
LL |     C: StackContext,
   |        ^^^^^^^^^^^^ the trait `for<'a> BufferMut` is not implemented for `&'a ()`
   |
note: required because of the requirements on the impl of `for<'a> BufferUdpStateContext<&'a ()>` for `Ctx<()>`
   |
   |
LL | impl<B: BufferMut, C> BufferUdpStateContext<B> for C {}
note: required by a bound in `StackContext`
  --> /checkout/src/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-89118.rs:9:14
   |
LL | trait StackContext
LL | trait StackContext
   |       ------------ required by a bound in this
LL | where
LL |     Ctx<()>: for<'a> BufferUdpStateContext<&'a ()>,
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `StackContext`

error[E0277]: the trait bound `for<'a> &'a (): BufferMut` is not satisfied
   |
   |
LL |     type Handler = Ctx<C::Dispatcher>;
   |                    ^^^^^^^^^^^^^^^^^^ the trait `for<'a> BufferMut` is not implemented for `&'a ()`
   |
note: required because of the requirements on the impl of `for<'a> BufferUdpStateContext<&'a ()>` for `Ctx<()>`
   |
   |
LL | impl<B: BufferMut, C> BufferUdpStateContext<B> for C {}
note: required by a bound in `StackContext`
  --> /checkout/src/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-89118.rs:9:14
   |
LL | trait StackContext
LL | trait StackContext
   |       ------------ required by a bound in this
LL | where
LL |     Ctx<()>: for<'a> BufferUdpStateContext<&'a ()>,
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `StackContext`
error: where-clause bound is impossible to satisfy
  --> /checkout/src/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-89118.rs:28:5
   |
   |
LL |     Ctx<()>: for<'a> BufferUdpStateContext<&'a ()>;
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it

error[E0277]: the trait bound `for<'a> &'a (): BufferMut` is not satisfied
   |
   |
LL | impl<C> EthernetWorker<C> {}
   |         ^^^^^^^^^^^^^^^^^ the trait `for<'a> BufferMut` is not implemented for `&'a ()`
   |
note: required because of the requirements on the impl of `for<'a> BufferUdpStateContext<&'a ()>` for `Ctx<()>`
   |
   |
LL | impl<B: BufferMut, C> BufferUdpStateContext<B> for C {}
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^     ^
note: required by a bound in `EthernetWorker`
   |
   |
LL | struct EthernetWorker<C>(C)
   |        -------------- required by a bound in this
LL | where
LL |     Ctx<()>: for<'a> BufferUdpStateContext<&'a ()>;
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `EthernetWorker`
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-36839.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-36839.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36839" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36839/auxiliary"
stdout: none
--- stderr -------------------------------
error: where-clause bound is impossible to satisfy
   |
   |
LL |     fn broken(&self) where Self::Assoc: Foo {
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
error: aborting due to previous error
------------------------------------------



---- [ui] ui/issues/issue-39970.rs stdout ----
diff of stderr:

- warning: where-clause bound is impossible to satisfy
+ error: where-clause bound is impossible to satisfy
3    |
3    |
4 LL |     (): for<'a> Array<'a, Element=()>, // ICE
7    = note: this bound was previously accepted, but it may become a hard error in a future release
7    = note: this bound was previously accepted, but it may become a hard error in a future release
8    = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
9 
- error[E0271]: type mismatch resolving `for<'a> <() as Array<'a>>::Element == ()`
-   --> $DIR/issue-39970.rs:19:5
-    |
- LL |     <() as Visit>::visit();
-    |     ^^^^^^^^^^^^^^^^^^^^ type mismatch resolving `for<'a> <() as Array<'a>>::Element == ()`
- note: expected this to be `()`
-   --> $DIR/issue-39970.rs:10:20
-    |
-    |
- LL |     type Element = &'a ();
- note: required because of the requirements on the impl of `Visit` for `()`
-   --> $DIR/issue-39970.rs:13:6
-    |
-    |
- LL | impl Visit for () where
+ error: aborting due to previous error
26 
- error: aborting due to previous error; 1 warning emitted
- 
---
To only update this specific test, also pass `--test-args issues/issue-39970.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-39970.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39970" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39970/auxiliary"
stdout: none
--- stderr -------------------------------
error: where-clause bound is impossible to satisfy
   |
   |
LL |     (): for<'a> Array<'a, Element=()>, // ICE
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
error: aborting due to previous error
------------------------------------------

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [ui] ui/issues/issue-42796.rs stdout ----
diff of stderr:

- warning: where-clause bound is impossible to satisfy
+ error: where-clause bound is impossible to satisfy
3    |
3    |
4 LL | pub fn poison<S>(victim: String) where <String as Mirror<S>>::Image: Copy {
7    = note: this bound was previously accepted, but it may become a hard error in a future release
7    = note: this bound was previously accepted, but it may become a hard error in a future release
8    = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
9 
- error[E0382]: borrow of moved value: `s`
-   --> $DIR/issue-42796.rs:18:20
-    |
- LL |     let s = "Hello!".to_owned();
-    |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
- LL |     let mut s_copy = s;
-    |                      - value moved here
- LL |     println!("{}", s);
- LL |     println!("{}", s);
-    |                    ^ value borrowed here after move
-    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
+ error: aborting due to previous error
22 
- error: aborting due to previous error; 1 warning emitted
---
To only update this specific test, also pass `--test-args issues/issue-42796.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-42796.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42796" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42796/auxiliary"
stdout: none
--- stderr -------------------------------
error: where-clause bound is impossible to satisfy
   |
   |
LL | pub fn poison<S>(victim: String) where <String as Mirror<S>>::Image: Copy {
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
error: aborting due to previous error
------------------------------------------



---- [ui] ui/mir/issue-91745.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/issue-91745.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-91745" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-91745/auxiliary"
stdout: none
--- stderr -------------------------------
error: where-clause bound is impossible to satisfy
   |
   |
LL |     fn broken(&self) where Self::Assoc: Foo {
   |
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = note: this bound was previously accepted, but it may become a hard error in a future release
   = help: add `#![feature(trivial_bounds)]` to the crate attributes to allow it
error: aborting due to previous error
------------------------------------------



---- [ui] ui/regions/regions-normalize-in-where-clause-list.rs stdout ----

error: /checkout/src/test/ui/regions/regions-normalize-in-where-clause-list.rs:28: unexpected error: '28:5: 28:38: cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements [E0495]'
error: /checkout/src/test/ui/regions/regions-normalize-in-where-clause-list.rs:24: expected error not found: cannot infer

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-normalize-in-where-clause-list.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-normalize-in-where-clause-list" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-normalize-in-where-clause-list/auxiliary"
    Error {
        line_num: 28,
        kind: Some(
            Error,
            Error,
        ),
        msg: "28:5: 28:38: cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements [E0495]",
]

not found errors (from test file): [
    Error {
