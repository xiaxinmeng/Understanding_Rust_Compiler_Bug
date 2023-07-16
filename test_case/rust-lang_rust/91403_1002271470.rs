plain

---- [ui (nll)] ui/async-await/issues/issue-62097.rs stdout ----
diff of stderr:

16 LL |         foo(move || self.bar()).await;
18 
18 
- error[E0521]: borrowed data escapes outside of associated function
+ error: lifetime may not live long enough
21    |
21    |
22 LL |     pub async fn run_dummy_fn(&self) {
-    |                               -----
-    |                               |
-    |                               |
-    |                               `self` is a reference that is only valid in the associated function body
-    |                               let's call the lifetime of this reference `'1`
+    |                               - let's call the lifetime of this reference `'1`
27 LL |         foo(|| self.bar()).await;
-    |         |
-    |         |
-    |         `self` escapes the associated function body here
-    |         argument requires that `'1` must outlive `'static`
+    |         ^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'static`
+    |
+ help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
+    |
+ LL |     pub async fn run_dummy_fn(&self)  + '_{
32 
33 error: aborting due to 2 previous errors
34 


- Some errors have detailed explanations: E0373, E0521.
- For more information about an error, try `rustc --explain E0373`.
+ For more information about this error, try `rustc --explain E0373`.
37 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62097.nll/issue-62097.nll.stderr
To only update this specific test, also pass `--test-args async-await/issues/issue-62097.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-62097.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62097.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62097.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0373]: closure may outlive the current function, but it borrows `self`, which is owned by the current function
   |
   |
LL |         foo(|| self.bar()).await;
   |             ^^ ---- `self` is borrowed here
   |             may outlive borrowed value `self`
   |
   |
note: function requires argument type to outlive `'static`
   |
   |
LL |         foo(|| self.bar()).await;
   |         ^^^^^^^^^^^^^^^^^^
help: to force the closure to take ownership of `self` (and any other referenced variables), use the `move` keyword
   |
LL |         foo(move || self.bar()).await;

error: lifetime may not live long enough
  --> /checkout/src/test/ui/async-await/issues/issue-62097.rs:13:9
   |
   |
LL |     pub async fn run_dummy_fn(&self) { //~ ERROR E0759
   |                               - let's call the lifetime of this reference `'1`
LL |         foo(|| self.bar()).await;
   |         ^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'static`
   |
help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
   |
LL |     pub async fn run_dummy_fn(&self)  + '_{ //~ ERROR E0759

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0373`.
For more information about this error, try `rustc --explain E0373`.

------------------------------------------


---- [ui (nll)] ui/async-await/issues/issue-63388-1.rs stdout ----
diff of stderr:

9 LL | /     {
10 LL | |         foo
11 LL | |     }
-    | |_____^ associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'1`
+    | |_____^ returning this value requires that `'1` must outlive `'a`
14 error: aborting due to previous error
15 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1.nll/issue-63388-1.nll.stderr
To only update this specific test, also pass `--test-args async-await/issues/issue-63388-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-63388-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/async-await/issues/issue-63388-1.rs:13:5
   |
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |       async fn do_sth<'a>(
   |                       -- lifetime `'a` defined here
LL |           &'a self, foo: &dyn Foo
   |                          - let's call the lifetime of this reference `'1`
LL |       ) -> &dyn Foo
LL | /     {
LL | |         foo  //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'a`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui (nll)] ui/async-await/issues/issue-72312.rs stdout ----
diff of stderr:

- error[E0521]: borrowed data escapes outside of associated function
+ error: lifetime may not live long enough
3    |
3    |
4 LL |       pub async fn start(&self) {
-    |                          -----
-    |                          |
-    |                          |
-    |                          `self` is a reference that is only valid in the associated function body
-    |                          let's call the lifetime of this reference `'1`
+    |                          - let's call the lifetime of this reference `'1`
9 ...
10 LL |           require_static(async move {

12 LL | |             &self;
13 LL | |         });
-    | |         ^
-    | |         ^
-    | |         |
-    | |_________`self` escapes the associated function body here
-    |           argument requires that `'1` must outlive `'static`
+    | |_________^ argument requires that `'1` must outlive `'static`
+    |
+ help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
+    |
+ LL |     pub async fn start(&self)  + '_{
18 
19 error: aborting due to previous error
20 


- For more information about this error, try `rustc --explain E0521`.
22 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-72312.nll/issue-72312.nll.stderr
To only update this specific test, also pass `--test-args async-await/issues/issue-72312.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-72312.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-72312.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-72312.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/async-await/issues/issue-72312.rs:13:24
   |
LL |       pub async fn start(&self) { //~ ERROR E0759
   |                          - let's call the lifetime of this reference `'1`
...
LL |           require_static(async move { //~ NOTE ...and is required to live as long as `'static` here
   |  ________________________^
LL | |             &self; //~ NOTE ...is used here...
LL | |         });
   | |_________^ argument requires that `'1` must outlive `'static`
   |
help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
   |
LL |     pub async fn start(&self)  + '_{ //~ ERROR E0759

error: aborting due to previous error



------------------------------------------


---- [ui (nll)] ui/async-await/multiple-lifetimes/ret-impl-trait-one.rs stdout ----
diff of stderr:

9 LL | |
10 LL | |     (a, b)
11 LL | | }
-    | |_^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    | |_^ returning this value requires that `'a` must outlive `'b`
13    |
14    = help: consider adding the following bound: `'a: 'b`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.nll/ret-impl-trait-one.nll.stderr
To only update this specific test, also pass `--test-args async-await/multiple-lifetimes/ret-impl-trait-one.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.rs:10:85
   |
LL |   async fn async_ret_impl_trait3<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> + 'b {
   |  ________________________________--__--_______________________________________________^
   | |                                |   |
   | |                                |   lifetime `'b` defined here
   | |                                lifetime `'a` defined here
LL | |     //~^ ERROR lifetime mismatch
LL | |     (a, b)
LL | | }
   | |_^ returning this value requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`

error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
   |
   |
LL | async fn async_ret_impl_trait1<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> {
   |                                    |
   |                                    |
   |                                    hidden type `(&'a u8, &'b u8)` captures the lifetime `'b` as defined here
   |
help: to declare that the `impl Trait` captures `'b`, you can add an explicit `'b` lifetime bound
   |
LL | async fn async_ret_impl_trait1<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> + 'b {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
---
1 error: lifetime may not live long enough
-   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:8:52
+   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:8:50
3    |
4 LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
-    |                          -         -               ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+    |                          -         -             ^^^^^ returning this value requires that `'1` must outlive `'2`
6    |                          |         |
7    |                          |         let's call the lifetime of this reference `'1`
8    |                          let's call the lifetime of this reference `'2`
9 
10 error: lifetime may not live long enough
-   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:11:75
+   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:11:73
+   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:11:73
12    |
13 LL |     async fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) }
-    |                          -          -                                     ^^^^^^^^^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+    |                          -          -                                   ^^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'2`
15    |                          |          |
16    |                          |          let's call the lifetime of this reference `'1`
17    |                          let's call the lifetime of this reference `'2`
18 
19 error: lifetime may not live long enough
-   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:17:64
+   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:17:62
+   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:17:62
21    |
22 LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg }
-    |                  --              -                             ^^^ associated function was supposed to return data with lifetime `'1` but it is returning data with lifetime `'a`
-    |                  |               |
-    |                  |               let's call the lifetime of this reference `'1`
+    |                  --  ---- has type `Pin<&'1 Foo>`            ^^^^^^^ returning this value requires that `'a` must outlive `'1`
+    |                  |
26    |                  lifetime `'a` defined here
28 error: aborting due to 3 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.nll/arbitrary_self_types_pin_lifetime_mismatch-async.nll.stderr
To only update this specific test, also pass `--test-args self/arbitrary_self_types_pin_lifetime_mismatch-async.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:8:50
   |
LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
   |                          -         -             ^^^^^ returning this value requires that `'1` must outlive `'2`
   |                          |         |
   |                          |         let's call the lifetime of this reference `'1`
   |                          let's call the lifetime of this reference `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:11:73
   |
   |
LL |     async fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) }
   |                          -          -                                   ^^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'2`
   |                          |          |
   |                          |          let's call the lifetime of this reference `'1`
   |                          let's call the lifetime of this reference `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:17:62
   |
   |
LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg } //~ ERROR E0623
   |                  --  ---- has type `Pin<&'1 Foo>`            ^^^^^^^ returning this value requires that `'a` must outlive `'1`
   |                  |
   |                  lifetime `'a` defined here
error: aborting due to 3 previous errors


------------------------------------------
---
1 error: lifetime may not live long enough
-   --> $DIR/lt-ref-self-async.rs:13:9
+   --> $DIR/lt-ref-self-async.rs:12:47
3    |
- LL |     async fn ref_self(&self, f: &u32) -> &u32 {
-    |                       -         - let's call the lifetime of this reference `'1`
-    |                       |
-    |                       let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn ref_self(&self, f: &u32) -> &u32 {
+    |  _______________________-_________-_____________^
+    | |                       |         |
+    | |                       |         let's call the lifetime of this reference `'1`
+    | |                       let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
11 error: lifetime may not live long enough
-   --> $DIR/lt-ref-self-async.rs:19:9
+   --> $DIR/lt-ref-self-async.rs:18:53
13    |
13    |
- LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
-    |                             -         - let's call the lifetime of this reference `'1`
-    |                             |
-    |                             let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn ref_Self(self: &Self, f: &u32) -> &u32 {
+    |  _____________________________-_________-_____________^
+    | |                             |         |
+    | |                             |         let's call the lifetime of this reference `'1`
+    | |                             let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
21 error: lifetime may not live long enough
-   --> $DIR/lt-ref-self-async.rs:23:9
+   --> $DIR/lt-ref-self-async.rs:22:62
23    |
23    |
- LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
-    |                                     -          - let's call the lifetime of this reference `'1`
-    |                                     |
-    |                                     let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
+    |  _____________________________________-__________-_____________^
+    | |                                     |          |
+    | |                                     |          let's call the lifetime of this reference `'1`
+    | |                                     let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
31 error: lifetime may not live long enough
-   --> $DIR/lt-ref-self-async.rs:27:9
+   --> $DIR/lt-ref-self-async.rs:26:62
33    |
33    |
- LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
-    |                                     -          - let's call the lifetime of this reference `'1`
-    |                                     |
-    |                                     let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
+    |  _____________________________________-__________-_____________^
+    | |                                     |          |
+    | |                                     |          let's call the lifetime of this reference `'1`
+    | |                                     let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
41 error: lifetime may not live long enough
-   --> $DIR/lt-ref-self-async.rs:31:9
+   --> $DIR/lt-ref-self-async.rs:30:71
43    |
43    |
- LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
-    |                                             -           - let's call the lifetime of this reference `'1`
-    |                                             |
-    |                                             let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
+    |  _____________________________________________-___________-_____________^
+    | |                                             |           |
+    | |                                             |           let's call the lifetime of this reference `'1`
+    | |                                             let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
51 error: lifetime may not live long enough
-   --> $DIR/lt-ref-self-async.rs:35:9
+   --> $DIR/lt-ref-self-async.rs:34:67
53    |
53    |
- LL |     async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
-    |                                         -           - let's call the lifetime of this reference `'1`
-    |                                         |
-    |                                         let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
+    |  _________________________________________-___________-_____________^
+    | |                                         |           |
+    | |                                         |           let's call the lifetime of this reference `'1`
+    | |                                         let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
61 error: aborting due to 6 previous errors
62 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll/lt-ref-self-async.nll.stderr
To only update this specific test, also pass `--test-args self/elision/lt-ref-self-async.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/lt-ref-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:12:47
   |
LL |       async fn ref_self(&self, f: &u32) -> &u32 {
   |  _______________________-_________-_____________^
   | |                       |         |
   | |                       |         let's call the lifetime of this reference `'1`
   | |                       let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:18:53
   |
   |
LL |       async fn ref_Self(self: &Self, f: &u32) -> &u32 {
   |  _____________________________-_________-_____________^
   | |                             |         |
   | |                             |         let's call the lifetime of this reference `'1`
   | |                             let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:22:62
   |
   |
LL |       async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
   |  _____________________________________-__________-_____________^
   | |                                     |          |
   | |                                     |          let's call the lifetime of this reference `'1`
   | |                                     let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:26:62
   |
   |
LL |       async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
   |  _____________________________________-__________-_____________^
   | |                                     |          |
   | |                                     |          let's call the lifetime of this reference `'1`
   | |                                     let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:30:71
   |
   |
LL |       async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
   |  _____________________________________________-___________-_____________^
   | |                                             |           |
   | |                                             |           let's call the lifetime of this reference `'1`
   | |                                             let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:34:67
   |
   |
LL |       async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
   |  _________________________________________-___________-_____________^
   | |                                         |           |
   | |                                         |           let's call the lifetime of this reference `'1`
   | |                                         let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: aborting due to 6 previous errors


------------------------------------------
---
1 error: lifetime may not live long enough
-   --> $DIR/ref-mut-self-async.rs:13:9
+   --> $DIR/ref-mut-self-async.rs:12:51
3    |
- LL |     async fn ref_self(&mut self, f: &u32) -> &u32 {
-    |                       -             - let's call the lifetime of this reference `'1`
-    |                       |
-    |                       let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn ref_self(&mut self, f: &u32) -> &u32 {
+    |  _______________________-_____________-_____________^
+    | |                       |             |
+    | |                       |             let's call the lifetime of this reference `'1`
+    | |                       let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
11 error: lifetime may not live long enough
-   --> $DIR/ref-mut-self-async.rs:19:9
+   --> $DIR/ref-mut-self-async.rs:18:57
13    |
13    |
- LL |     async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
-    |                             -             - let's call the lifetime of this reference `'1`
-    |                             |
-    |                             let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
+    |  _____________________________-_____________-_____________^
+    | |                             |             |
+    | |                             |             let's call the lifetime of this reference `'1`
+    | |                             let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
21 error: lifetime may not live long enough
-   --> $DIR/ref-mut-self-async.rs:23:9
+   --> $DIR/ref-mut-self-async.rs:22:66
23    |
23    |
- LL |     async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
-    |                                     -              - let's call the lifetime of this reference `'1`
-    |                                     |
-    |                                     let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
+    |  _____________________________________-______________-_____________^
+    | |                                     |              |
+    | |                                     |              let's call the lifetime of this reference `'1`
+    | |                                     let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
31 error: lifetime may not live long enough
-   --> $DIR/ref-mut-self-async.rs:27:9
+   --> $DIR/ref-mut-self-async.rs:26:66
33    |
33    |
- LL |     async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
-    |                                     -              - let's call the lifetime of this reference `'1`
-    |                                     |
-    |                                     let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
+    |  _____________________________________-______________-_____________^
+    | |                                     |              |
+    | |                                     |              let's call the lifetime of this reference `'1`
+    | |                                     let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
41 error: lifetime may not live long enough
-   --> $DIR/ref-mut-self-async.rs:31:9
+   --> $DIR/ref-mut-self-async.rs:30:75
43    |
43    |
- LL |     async fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
-    |                                             -               - let's call the lifetime of this reference `'1`
-    |                                             |
-    |                                             let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
+    |  _____________________________________________-_______________-_____________^
+    | |                                             |               |
+    | |                                             |               let's call the lifetime of this reference `'1`
+    | |                                             let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
51 error: lifetime may not live long enough
-   --> $DIR/ref-mut-self-async.rs:35:9
+   --> $DIR/ref-mut-self-async.rs:34:75
53    |
53    |
- LL |     async fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
-    |                                             -               - let's call the lifetime of this reference `'1`
-    |                                             |
-    |                                             let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
+    |  _____________________________________________-_______________-_____________^
+    | |                                             |               |
+    | |                                             |               let's call the lifetime of this reference `'1`
+    | |                                             let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
61 error: aborting due to 6 previous errors
62 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async.nll/ref-mut-self-async.nll.stderr
To only update this specific test, also pass `--test-args self/elision/ref-mut-self-async.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-mut-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:12:51
   |
LL |       async fn ref_self(&mut self, f: &u32) -> &u32 {
   |  _______________________-_____________-_____________^
   | |                       |             |
   | |                       |             let's call the lifetime of this reference `'1`
   | |                       let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:18:57
   |
   |
LL |       async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
   |  _____________________________-_____________-_____________^
   | |                             |             |
   | |                             |             let's call the lifetime of this reference `'1`
   | |                             let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:22:66
   |
   |
LL |       async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
   |  _____________________________________-______________-_____________^
   | |                                     |              |
   | |                                     |              let's call the lifetime of this reference `'1`
   | |                                     let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:26:66
   |
   |
LL |       async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
   |  _____________________________________-______________-_____________^
   | |                                     |              |
   | |                                     |              let's call the lifetime of this reference `'1`
   | |                                     let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:30:75
   |
   |
LL |       async fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
   |  _____________________________________________-_______________-_____________^
   | |                                             |               |
   | |                                             |               let's call the lifetime of this reference `'1`
   | |                                             let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:34:75
   |
   |
LL |       async fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
   |  _____________________________________________-_______________-_____________^
   | |                                             |               |
   | |                                             |               let's call the lifetime of this reference `'1`
   | |                                             let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: aborting due to 6 previous errors


------------------------------------------
---
1 error: lifetime may not live long enough
-   --> $DIR/ref-mut-struct-async.rs:13:9
+   --> $DIR/ref-mut-struct-async.rs:12:61
3    |
- LL |     async fn ref_Struct(self: &mut Struct, f: &u32) -> &u32 {
-    |                               -               - let's call the lifetime of this reference `'1`
-    |                               |
-    |                               let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn ref_Struct(self: &mut Struct, f: &u32) -> &u32 {
+    |  _______________________________-_______________-_____________^
+    | |                               |               |
+    | |                               |               let's call the lifetime of this reference `'1`
+    | |                               let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
11 error: lifetime may not live long enough
-   --> $DIR/ref-mut-struct-async.rs:17:9
+   --> $DIR/ref-mut-struct-async.rs:16:70
13    |
13    |
- LL |     async fn box_ref_Struct(self: Box<&mut Struct>, f: &u32) -> &u32 {
-    |                                       -                - let's call the lifetime of this reference `'1`
-    |                                       |
-    |                                       let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn box_ref_Struct(self: Box<&mut Struct>, f: &u32) -> &u32 {
+    |  _______________________________________-________________-_____________^
+    | |                                       |                |
+    | |                                       |                let's call the lifetime of this reference `'1`
+    | |                                       let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
21 error: lifetime may not live long enough
-   --> $DIR/ref-mut-struct-async.rs:21:9
+   --> $DIR/ref-mut-struct-async.rs:20:70
23    |
23    |
- LL |     async fn pin_ref_Struct(self: Pin<&mut Struct>, f: &u32) -> &u32 {
-    |                                       -                - let's call the lifetime of this reference `'1`
-    |                                       |
-    |                                       let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn pin_ref_Struct(self: Pin<&mut Struct>, f: &u32) -> &u32 {
+    |  _______________________________________-________________-_____________^
+    | |                                       |                |
+    | |                                       |                let's call the lifetime of this reference `'1`
+    | |                                       let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
31 error: lifetime may not live long enough
-   --> $DIR/ref-mut-struct-async.rs:25:9
+   --> $DIR/ref-mut-struct-async.rs:24:79
33    |
33    |
- LL |     async fn box_box_ref_Struct(self: Box<Box<&mut Struct>>, f: &u32) -> &u32 {
-    |                                               -                 - let's call the lifetime of this reference `'1`
-    |                                               |
-    |                                               let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn box_box_ref_Struct(self: Box<Box<&mut Struct>>, f: &u32) -> &u32 {
+    |  _______________________________________________-_________________-_____________^
+    | |                                               |                 |
+    | |                                               |                 let's call the lifetime of this reference `'1`
+    | |                                               let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
41 error: lifetime may not live long enough
-   --> $DIR/ref-mut-struct-async.rs:29:9
+   --> $DIR/ref-mut-struct-async.rs:28:79
43    |
43    |
- LL |     async fn box_pin_ref_Struct(self: Box<Pin<&mut Struct>>, f: &u32) -> &u32 {
-    |                                               -                 - let's call the lifetime of this reference `'1`
-    |                                               |
-    |                                               let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn box_pin_ref_Struct(self: Box<Pin<&mut Struct>>, f: &u32) -> &u32 {
+    |  _______________________________________________-_________________-_____________^
+    | |                                               |                 |
+    | |                                               |                 let's call the lifetime of this reference `'1`
+    | |                                               let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
51 error: aborting due to 5 previous errors
52 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-struct-async.nll/ref-mut-struct-async.nll.stderr
To only update this specific test, also pass `--test-args self/elision/ref-mut-struct-async.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-mut-struct-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-struct-async.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-struct-async.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-mut-struct-async.rs:12:61
   |
LL |       async fn ref_Struct(self: &mut Struct, f: &u32) -> &u32 {
   |  _______________________________-_______________-_____________^
   | |                               |               |
   | |                               |               let's call the lifetime of this reference `'1`
   | |                               let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-mut-struct-async.rs:16:70
   |
   |
LL |       async fn box_ref_Struct(self: Box<&mut Struct>, f: &u32) -> &u32 {
   |  _______________________________________-________________-_____________^
   | |                                       |                |
   | |                                       |                let's call the lifetime of this reference `'1`
   | |                                       let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-mut-struct-async.rs:20:70
   |
   |
LL |       async fn pin_ref_Struct(self: Pin<&mut Struct>, f: &u32) -> &u32 {
   |  _______________________________________-________________-_____________^
   | |                                       |                |
   | |                                       |                let's call the lifetime of this reference `'1`
   | |                                       let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-mut-struct-async.rs:24:79
   |
   |
LL |       async fn box_box_ref_Struct(self: Box<Box<&mut Struct>>, f: &u32) -> &u32 {
   |  _______________________________________________-_________________-_____________^
   | |                                               |                 |
   | |                                               |                 let's call the lifetime of this reference `'1`
   | |                                               let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-mut-struct-async.rs:28:79
   |
   |
LL |       async fn box_pin_ref_Struct(self: Box<Pin<&mut Struct>>, f: &u32) -> &u32 {
   |  _______________________________________________-_________________-_____________^
   | |                                               |                 |
   | |                                               |                 let's call the lifetime of this reference `'1`
   | |                                               let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: aborting due to 5 previous errors


------------------------------------------
---
1 error: lifetime may not live long enough
-   --> $DIR/ref-self-async.rs:23:9
+   --> $DIR/ref-self-async.rs:22:47
3    |
- LL |     async fn ref_self(&self, f: &u32) -> &u32 {
-    |                       -         - let's call the lifetime of this reference `'1`
-    |                       |
-    |                       let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn ref_self(&self, f: &u32) -> &u32 {
+    |  _______________________-_________-_____________^
+    | |                       |         |
+    | |                       |         let's call the lifetime of this reference `'1`
+    | |                       let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
11 error: lifetime may not live long enough
-   --> $DIR/ref-self-async.rs:29:9
+   --> $DIR/ref-self-async.rs:28:53
13    |
13    |
- LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
-    |                             -         - let's call the lifetime of this reference `'1`
-    |                             |
-    |                             let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn ref_Self(self: &Self, f: &u32) -> &u32 {
+    |  _____________________________-_________-_____________^
+    | |                             |         |
+    | |                             |         let's call the lifetime of this reference `'1`
+    | |                             let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
21 error: lifetime may not live long enough
-   --> $DIR/ref-self-async.rs:33:9
+   --> $DIR/ref-self-async.rs:32:62
23    |
23    |
- LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
-    |                                     -          - let's call the lifetime of this reference `'1`
-    |                                     |
-    |                                     let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
+    |  _____________________________________-__________-_____________^
+    | |                                     |          |
+    | |                                     |          let's call the lifetime of this reference `'1`
+    | |                                     let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
31 error: lifetime may not live long enough
-   --> $DIR/ref-self-async.rs:37:9
+   --> $DIR/ref-self-async.rs:36:62
33    |
33    |
- LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
-    |                                     -          - let's call the lifetime of this reference `'1`
-    |                                     |
-    |                                     let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
+    |  _____________________________________-__________-_____________^
+    | |                                     |          |
+    | |                                     |          let's call the lifetime of this reference `'1`
+    | |                                     let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
41 error: lifetime may not live long enough
-   --> $DIR/ref-self-async.rs:41:9
+   --> $DIR/ref-self-async.rs:40:71
43    |
43    |
- LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
-    |                                             -           - let's call the lifetime of this reference `'1`
-    |                                             |
-    |                                             let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
+    |  _____________________________________________-___________-_____________^
+    | |                                             |           |
+    | |                                             |           let's call the lifetime of this reference `'1`
+    | |                                             let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
51 error: lifetime may not live long enough
-   --> $DIR/ref-self-async.rs:45:9
+   --> $DIR/ref-self-async.rs:44:71
53    |
53    |
- LL |     async fn box_pin_ref_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
-    |                                             -           - let's call the lifetime of this reference `'1`
-    |                                             |
-    |                                             let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn box_pin_ref_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
+    |  _____________________________________________-___________-_____________^
+    | |                                             |           |
+    | |                                             |           let's call the lifetime of this reference `'1`
+    | |                                             let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
61 error: lifetime may not live long enough
-   --> $DIR/ref-self-async.rs:49:9
+   --> $DIR/ref-self-async.rs:48:73
63    |
63    |
- LL |     async fn wrap_ref_Self_Self(self: Wrap<&Self, Self>, f: &u8) -> &u8 {
-    |                                            -                - let's call the lifetime of this reference `'1`
-    |                                            |
-    |                                            let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn wrap_ref_Self_Self(self: Wrap<&Self, Self>, f: &u8) -> &u8 {
+    |  ____________________________________________-________________-___________^
+    | |                                            |                |
+    | |                                            |                let's call the lifetime of this reference `'1`
+    | |                                            let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
71 error: aborting due to 7 previous errors
72 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-self-async.nll/ref-self-async.nll.stderr
To only update this specific test, also pass `--test-args self/elision/ref-self-async.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-self-async.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-self-async.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-self-async.rs:22:47
   |
LL |       async fn ref_self(&self, f: &u32) -> &u32 {
   |  _______________________-_________-_____________^
   | |                       |         |
   | |                       |         let's call the lifetime of this reference `'1`
   | |                       let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-self-async.rs:28:53
   |
   |
LL |       async fn ref_Self(self: &Self, f: &u32) -> &u32 {
   |  _____________________________-_________-_____________^
   | |                             |         |
   | |                             |         let's call the lifetime of this reference `'1`
   | |                             let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-self-async.rs:32:62
   |
   |
LL |       async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
   |  _____________________________________-__________-_____________^
   | |                                     |          |
   | |                                     |          let's call the lifetime of this reference `'1`
   | |                                     let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-self-async.rs:36:62
   |
   |
LL |       async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
   |  _____________________________________-__________-_____________^
   | |                                     |          |
   | |                                     |          let's call the lifetime of this reference `'1`
   | |                                     let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-self-async.rs:40:71
   |
   |
LL |       async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
   |  _____________________________________________-___________-_____________^
   | |                                             |           |
   | |                                             |           let's call the lifetime of this reference `'1`
   | |                                             let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-self-async.rs:44:71
   |
   |
LL |       async fn box_pin_ref_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
   |  _____________________________________________-___________-_____________^
   | |                                             |           |
   | |                                             |           let's call the lifetime of this reference `'1`
   | |                                             let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-self-async.rs:48:73
   |
   |
LL |       async fn wrap_ref_Self_Self(self: Wrap<&Self, Self>, f: &u8) -> &u8 {
   |  ____________________________________________-________________-___________^
   | |                                            |                |
   | |                                            |                let's call the lifetime of this reference `'1`
   | |                                            let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: aborting due to 7 previous errors


------------------------------------------
---
1 error: lifetime may not live long enough
-   --> $DIR/ref-struct-async.rs:13:9
+   --> $DIR/ref-struct-async.rs:12:57
3    |
- LL |     async fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
-    |                               -           - let's call the lifetime of this reference `'1`
-    |                               |
-    |                               let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
+    |  _______________________________-___________-_____________^
+    | |                               |           |
+    | |                               |           let's call the lifetime of this reference `'1`
+    | |                               let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
11 error: lifetime may not live long enough
-   --> $DIR/ref-struct-async.rs:17:9
+   --> $DIR/ref-struct-async.rs:16:66
13    |
13    |
- LL |     async fn box_ref_Struct(self: Box<&Struct>, f: &u32) -> &u32 {
-    |                                       -            - let's call the lifetime of this reference `'1`
-    |                                       |
-    |                                       let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn box_ref_Struct(self: Box<&Struct>, f: &u32) -> &u32 {
+    |  _______________________________________-____________-_____________^
+    | |                                       |            |
+    | |                                       |            let's call the lifetime of this reference `'1`
+    | |                                       let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
21 error: lifetime may not live long enough
-   --> $DIR/ref-struct-async.rs:21:9
+   --> $DIR/ref-struct-async.rs:20:66
23    |
23    |
- LL |     async fn pin_ref_Struct(self: Pin<&Struct>, f: &u32) -> &u32 {
-    |                                       -            - let's call the lifetime of this reference `'1`
-    |                                       |
-    |                                       let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn pin_ref_Struct(self: Pin<&Struct>, f: &u32) -> &u32 {
+    |  _______________________________________-____________-_____________^
+    | |                                       |            |
+    | |                                       |            let's call the lifetime of this reference `'1`
+    | |                                       let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
31 error: lifetime may not live long enough
-   --> $DIR/ref-struct-async.rs:25:9
+   --> $DIR/ref-struct-async.rs:24:75
33    |
33    |
- LL |     async fn box_box_ref_Struct(self: Box<Box<&Struct>>, f: &u32) -> &u32 {
-    |                                               -             - let's call the lifetime of this reference `'1`
-    |                                               |
-    |                                               let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn box_box_ref_Struct(self: Box<Box<&Struct>>, f: &u32) -> &u32 {
+    |  _______________________________________________-_____________-_____________^
+    | |                                               |             |
+    | |                                               |             let's call the lifetime of this reference `'1`
+    | |                                               let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
41 error: lifetime may not live long enough
-   --> $DIR/ref-struct-async.rs:29:9
+   --> $DIR/ref-struct-async.rs:28:71
43    |
43    |
- LL |     async fn box_pin_Struct(self: Box<Pin<&Struct>>, f: &u32) -> &u32 {
-    |                                           -             - let's call the lifetime of this reference `'1`
-    |                                           |
-    |                                           let's call the lifetime of this reference `'2`
- LL |         f
-    |         ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+ LL |       async fn box_pin_Struct(self: Box<Pin<&Struct>>, f: &u32) -> &u32 {
+    |  ___________________________________________-_____________-_____________^
+    | |                                           |             |
+    | |                                           |             let's call the lifetime of this reference `'1`
+    | |                                           let's call the lifetime of this reference `'2`
+ LL | |         f
+ LL | |     }
+    | |_____^ returning this value requires that `'1` must outlive `'2`
51 error: aborting due to 5 previous errors
52 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-struct-async.nll/ref-struct-async.nll.stderr
To only update this specific test, also pass `--test-args self/elision/ref-struct-async.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-struct-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-struct-async.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-struct-async.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-struct-async.rs:12:57
   |
LL |       async fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
   |  _______________________________-___________-_____________^
   | |                               |           |
   | |                               |           let's call the lifetime of this reference `'1`
   | |                               let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-struct-async.rs:16:66
   |
   |
LL |       async fn box_ref_Struct(self: Box<&Struct>, f: &u32) -> &u32 {
   |  _______________________________________-____________-_____________^
   | |                                       |            |
   | |                                       |            let's call the lifetime of this reference `'1`
   | |                                       let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-struct-async.rs:20:66
   |
   |
LL |       async fn pin_ref_Struct(self: Pin<&Struct>, f: &u32) -> &u32 {
   |  _______________________________________-____________-_____________^
   | |                                       |            |
   | |                                       |            let's call the lifetime of this reference `'1`
   | |                                       let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-struct-async.rs:24:75
   |
   |
LL |       async fn box_box_ref_Struct(self: Box<Box<&Struct>>, f: &u32) -> &u32 {
   |  _______________________________________________-_____________-_____________^
   | |                                               |             |
   | |                                               |             let's call the lifetime of this reference `'1`
   | |                                               let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/elision/ref-struct-async.rs:28:71
   |
   |
LL |       async fn box_pin_Struct(self: Box<Pin<&Struct>>, f: &u32) -> &u32 {
   |  ___________________________________________-_____________-_____________^
   | |                                           |             |
   | |                                           |             let's call the lifetime of this reference `'1`
   | |                                           let's call the lifetime of this reference `'2`
LL | |         f //~ ERROR lifetime mismatch
LL | |     }
   | |_____^ returning this value requires that `'1` must outlive `'2`
error: aborting due to 5 previous errors


------------------------------------------
---
test result: FAILED. 12356 passed; 10 failed; 156 ignored; 0 measured; 0 filtered out; finished in 134.39s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.59.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always" "--compare-mode" "nll"


Build completed unsuccessfully in 0:19:56
