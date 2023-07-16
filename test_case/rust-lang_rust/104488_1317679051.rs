plain
...........................i............................................................ 968/13852
........................................................................................ 1056/13852
........................................................................................ 1144/13852
........................................................................................ 1232/13852
.................................................................F.....................F 1320/13852
....................................F................................................... 1496/13852
..i..................................................................................... 1584/13852
........................................................................................ 1672/13852
........................................................................................ 1760/13852
---
diff of stderr:

5    |         - help: consider making this binding mutable: `mut b`
6 ...
7 LL |     b = Box::new(1);
-    |     - first assignment to `b`
+    |     --------------- first assignment to `b`
9 LL |     b = Box::new(2);
10    |     ^ cannot assign twice to immutable variable
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-45199/issue-45199.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-45199/issue-45199.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/issue-45199.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-45199.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-45199" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-45199/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0384]: cannot assign twice to immutable variable `b`
   |
   |
LL |     let b: Box<isize>;
   |         - help: consider making this binding mutable: `mut b`
...
LL |     b = Box::new(1);    //~ NOTE first assignment
   |     --------------- first assignment to `b`
LL |     b = Box::new(2);    //~ ERROR cannot assign twice to immutable variable `b`
   |     ^ cannot assign twice to immutable variable
error[E0384]: cannot assign twice to immutable variable `b`
  --> /checkout/src/test/ui/borrowck/issue-45199.rs:14:5
   |
   |
LL |     let b = Box::new(1);    //~ NOTE first assignment
   |         |
   |         first assignment to `b`
   |         help: consider making this binding mutable: `mut b`
...
...
LL |     b = Box::new(2);        //~ ERROR cannot assign twice to immutable variable `b`
   |     ^ cannot assign twice to immutable variable
error[E0384]: cannot assign to immutable argument `b`
  --> /checkout/src/test/ui/borrowck/issue-45199.rs:20:5
   |
   |
LL | fn test_args(b: Box<i32>) {  //~ HELP consider making this binding mutable
   |              - help: consider making this binding mutable: `mut b`
LL |                                 //~| SUGGESTION mut b
LL |     b = Box::new(2);            //~ ERROR cannot assign to immutable argument `b`
   |     ^ cannot assign to immutable argument
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0384`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/borrowck/issue-58776-borrowck-scans-children.rs stdout ----
diff of stderr:

12 LL |     println!("thread result: {:?}", res);
14 
14 
- error[E0505]: cannot move out of `greeting` because it is borrowed
-   --> $DIR/issue-58776-borrowck-scans-children.rs:7:10
-    |
- LL |     let res = (|| (|| &greeting)())();
-    |                --      -------- borrow occurs due to use in closure
-    |                |
-    |                borrow of `greeting` occurs here
- ...
- LL |     drop(greeting);
-    |          ^^^^^^^^ move out of `greeting` occurs here
- ...
- LL |     println!("thread result: {:?}", res);
+ error: aborting due to previous error
28 
- error: aborting due to 2 previous errors
- 
---
To only update this specific test, also pass `--test-args borrowck/issue-58776-borrowck-scans-children.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-58776-borrowck-scans-children.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-58776-borrowck-scans-children" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-58776-borrowck-scans-children/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0506]: cannot assign to `greeting` because it is borrowed
   |
   |
LL |     let res = (|| (|| &greeting)())();
   |                --      -------- borrow occurs due to use in closure
   |                |
   |                borrow of `greeting` occurs here
LL |
LL |     greeting = "DEALLOCATED".to_string();
   |     ^^^^^^^^ assignment to borrowed `greeting` occurs here
...
LL |     println!("thread result: {:?}", res);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0506`.
For more information about this error, try `rustc --explain E0506`.
------------------------------------------


---- [ui] src/test/ui/c-variadic/variadic-ffi-4.rs stdout ----
diff of stderr:

55    |                                               |
56    |                                               has type `&mut VaListImpl<'1>`
57 LL |     *ap0 = ap1;
-    |     ^^^^ assignment requires that `'1` must outlive `'2`
+    |     ^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
59    |
60    = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
61    = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
69    |                                               |
70    |                                               has type `&mut VaListImpl<'1>`
71 LL |     *ap0 = ap1;
71 LL |     *ap0 = ap1;
-    |     ^^^^ assignment requires that `'2` must outlive `'1`
+    |     ^^^^^^^^^^ assignment requires that `'2` must outlive `'1`
73    |
74    = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
75    = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4/variadic-ffi-4.stderr
To only update this specific test, also pass `--test-args c-variadic/variadic-ffi-4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/variadic-ffi-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | pub unsafe extern "C" fn no_escape0<'f>(_: usize, ap: ...) -> VaListImpl<'f> {
   |                                     --            -- has type `VaListImpl<'1>`
   |                                     |
   |                                     lifetime `'f` defined here
LL |     ap
   |     ^^ function was supposed to return data with lifetime `'1` but it is returning data with lifetime `'f`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:8:5
   |
   |
LL | pub unsafe extern "C" fn no_escape0<'f>(_: usize, ap: ...) -> VaListImpl<'f> {
   |                                     --            -- has type `VaListImpl<'1>`
   |                                     |
   |                                     lifetime `'f` defined here
LL |     ap
   |     ^^ function was supposed to return data with lifetime `'f` but it is returning data with lifetime `'1`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:14:5
   |
   |
LL | pub unsafe extern "C" fn no_escape1(_: usize, ap: ...) -> VaListImpl<'static> {
   |                                               -- has type `VaListImpl<'1>`
LL |     ap //~ ERROR: lifetime may not live long enough
   |     ^^ returning this value requires that `'1` must outlive `'static`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:18:31
   |
   |
LL |     let _ = ap.with_copy(|ap| ap); //~ ERROR: lifetime may not live long enough
   |                           --- ^^ returning this value requires that `'1` must outlive `'2`
   |                           | |
   |                           | return type of closure is VaList<'2, '_>
   |                           has type `VaList<'1, '_>`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:22:5
   |
   |
LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     *ap0 = ap1;
LL |     *ap0 = ap1;
   |     ^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:22:5
   |
   |
LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     *ap0 = ap1;
LL |     *ap0 = ap1;
   |     ^^^^^^^^^^ assignment requires that `'2` must outlive `'1`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:28:5
   |
   |
LL | pub unsafe extern "C" fn no_escape4(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     ap0 = &mut ap1;
   |     ^^^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
   |
   = note: requirement occurs because of a mutable reference to `VaListImpl<'_>`
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:28:5
   |
   |
LL | pub unsafe extern "C" fn no_escape4(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     ap0 = &mut ap1;
   |     ^^^^^^^^^^^^^^ assignment requires that `'2` must outlive `'1`
   |
   = note: requirement occurs because of a mutable reference to `VaListImpl<'_>`
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

error[E0597]: `ap1` does not live long enough
   |
   |
LL | pub unsafe extern "C" fn no_escape4(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                                        - let's call the lifetime of this reference `'3`
LL |     ap0 = &mut ap1;
   |     |     |
   |     |     borrowed value does not live long enough
   |     |     borrowed value does not live long enough
   |     assignment requires that `ap1` is borrowed for `'3`
LL | }
LL | }
   | - `ap1` dropped here while still borrowed
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:35:12
   |
   |
LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     *ap0 = ap1.clone();
   |            ^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:35:12
   |
   |
LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     *ap0 = ap1.clone();
   |            ^^^^^^^^^^^ argument requires that `'2` must outlive `'1`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to 11 previous errors

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/object-lifetime/object-lifetime-default-from-box-error.rs stdout ----
diff of stderr:

25    |                   --------------- help: add explicit lifetime `'b` to the type of `ss`: `&mut SomeStruct<'b>`
26 ...
27 LL |     ss.r = b;
-    |     ^^^^ lifetime `'b` required
+    |     ^^^^^^^^ lifetime `'b` required
30 error: aborting due to 3 previous errors
31 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-from-box-error/object-lifetime-default-from-box-error.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args object-lifetime/object-lifetime-default-from-box-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/object-lifetime/object-lifetime-default-from-box-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-from-box-error" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-from-box-error/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn load(ss: &mut SomeStruct) -> Box<dyn SomeTrait> {
   |         -- has type `&mut SomeStruct<'1>`
LL |     ss.r
LL |     ss.r
   |     ^^^^ returning this value requires that `'1` must outlive `'static`
   |
help: to declare that the trait object captures data from argument `ss`, you can add an explicit `'_` lifetime bound
   |
LL | fn load(ss: &mut SomeStruct) -> Box<dyn SomeTrait + '_> {


error[E0507]: cannot move out of `ss.r` which is behind a mutable reference
   |
LL |     ss.r
LL |     ss.r
   |     ^^^^ move occurs because `ss.r` has type `Box<dyn SomeTrait>`, which does not implement the `Copy` trait

error[E0621]: explicit lifetime required in the type of `ss`
   |
   |
LL | fn store1<'b>(ss: &mut SomeStruct, b: Box<dyn SomeTrait+'b>) {
   |                   --------------- help: add explicit lifetime `'b` to the type of `ss`: `&mut SomeStruct<'b>`
...
LL |     ss.r = b; //~ ERROR explicit lifetime required in the type of `ss` [E0621]
   |     ^^^^^^^^ lifetime `'b` required
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0507, E0621.
For more information about an error, try `rustc --explain E0507`.
For more information about an error, try `rustc --explain E0507`.
------------------------------------------


---- [ui] src/test/ui/regions/regions-infer-paramd-indirect.rs stdout ----
diff of stderr:

7 LL |     fn set_f_bad(&mut self, b: Box<B>) {
8    |                             - has type `Box<Box<&'1 isize>>`
9 LL |         self.f = b;
-    |         ^^^^^^ assignment requires that `'1` must outlive `'a`
+    |         ^^^^^^^^^^ assignment requires that `'1` must outlive `'a`
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-paramd-indirect/regions-infer-paramd-indirect.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-infer-paramd-indirect.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-infer-paramd-indirect.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-paramd-indirect" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-paramd-indirect/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | impl<'a> SetF<'a> for C<'a> {
   |      -- lifetime `'a` defined here
...
LL |     fn set_f_bad(&mut self, b: Box<B>) {
   |                             - has type `Box<Box<&'1 isize>>`
LL |         self.f = b;
   |         ^^^^^^^^^^ assignment requires that `'1` must outlive `'a`
error: aborting due to previous error
------------------------------------------


---
23 LL |     factorial = Some(Box::new(f));
-    |     ^^^^^^^^^
+    |     ^^^^^^^^^--------------------
25    |     |
26    |     assignment to borrowed `factorial` occurs here
27    |     borrow later used here

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-1/unboxed-closures-failed-recursive-fn-1.stderr
To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closures-failed-recursive-fn-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `factorial` does not live long enough
   |
   |
LL |     let f = |x: u32| -> u32 {
   |             --------------- value captured here
LL |         let g = factorial.as_ref().unwrap();
   |                 ^^^^^^^^^ borrowed value does not live long enough
LL | }
   | -
   | |
   | |
   | `factorial` dropped here while still borrowed
   | borrow might be used here, when `factorial` is dropped and runs the destructor for type `Option<Box<dyn Fn(u32) -> u32>>`

error[E0506]: cannot assign to `factorial` because it is borrowed
   |
   |
LL |     let f = |x: u32| -> u32 {
   |             --------------- borrow of `factorial` occurs here
LL |         let g = factorial.as_ref().unwrap();
   |                 --------- borrow occurs due to use in closure
LL |     factorial = Some(Box::new(f));
   |     ^^^^^^^^^--------------------
   |     |
   |     |
   |     assignment to borrowed `factorial` occurs here
   |     borrow later used here

error[E0597]: `factorial` does not live long enough
   |
   |
LL |     let mut factorial: Option<Box<dyn Fn(u32) -> u32 + 'static>> = None;
   |                        ----------------------------------------- type annotation requires that `factorial` is borrowed for `'static`
LL |
LL |     let f = |x: u32| -> u32 {
   |             --------------- value captured here
LL |         let g = factorial.as_ref().unwrap();
   |                 ^^^^^^^^^ borrowed value does not live long enough
LL | }
LL | }
   | - `factorial` dropped here while still borrowed

error[E0506]: cannot assign to `factorial` because it is borrowed
   |
   |
LL |     let mut factorial: Option<Box<dyn Fn(u32) -> u32 + 'static>> = None;
   |                        ----------------------------------------- type annotation requires that `factorial` is borrowed for `'static`
LL |
LL |     let f = |x: u32| -> u32 {
   |             --------------- borrow of `factorial` occurs here
LL |         let g = factorial.as_ref().unwrap();
   |                 --------- borrow occurs due to use in closure
LL |     factorial = Some(Box::new(f));
LL |     factorial = Some(Box::new(f));
   |     ^^^^^^^^^ assignment to borrowed `factorial` occurs here
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0506, E0597.
For more information about an error, try `rustc --explain E0506`.
