plain
.................F..............................i....................................... 4136/13299
........................................................................................ 4224/13299
....................................................................F................... 4312/13299
........................................................................................ 4400/13299
.F.F......................................F............................................. 4488/13299
......................................................F................................. 4576/13299
........................................................................................ 4752/13299
........................................................................................ 4840/13299
........................................................................................ 4928/13299
........................................................................................ 5016/13299
---
........................................................................................ 6160/13299
........................................................................................ 6248/13299
........................................................................................ 6336/13299
...............i........................................................................ 6424/13299
.................................................F..............F....................... 6512/13299
........................................................................................ 6688/13299
....i.....................................................ii.ii........i...i............ 6776/13299
....i.....................................................ii.ii........i...i............ 6776/13299
..............F............F............................................................ 6864/13299
........................................................................................ 6952/13299
..........................................i..F.i........................................ 7040/13299
.i...................................................................................... 7216/13299
..................i..................................................................... 7304/13299
........................................................................................ 7392/13299
........................................................................................ 7480/13299
---
........................................................................................ 9768/13299
........................................................................................ 9856/13299
.........ii...............i...........................................................ii 9944/13299
........................................................................................ 10032/13299
................................................................FF..F.........F.......F. 10120/13299
........F...F........................................................................... 10208/13299
........................................................................................ 10384/13299
........................................................................................ 10472/13299
........................................................................................ 10560/13299
.......................................................................................i 10648/13299
---
........................................................................................ 11352/13299
........................................................................................ 11440/13299
........................................................................................ 11528/13299
........................................................................................ 11616/13299
.........................F.F............................................................ 11704/13299
.i......i...........................i................................................... 11880/13299
........................................................................................ 11968/13299
........................................................................................ 12056/13299
........................................................................................ 12144/13299
---
---- [ui] src/test/ui/async-await/issues/issue-62097.rs#base stdout ----
diff of stderr:

6 LL |
7 LL |         foo(|| self.bar()).await;
8    |         --- ...is used and required to live as long as `'static` here
-    |
- note: `'static` lifetime requirement introduced by this bound
-   --> $DIR/issue-62097.rs:8:19
- LL |     F: FnOnce() + 'static
-    |                   ^^^^^^^
15 
16 error: aborting due to previous error
16 error: aborting due to previous error
17 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62097.base/issue-62097.base.stderr
To only update this specific test, also pass `--test-args async-await/issues/issue-62097.rs`


error in revision `base`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-62097.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "base" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62097.base" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62097.base/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0759]: `self` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement
   |
   |
LL |     pub async fn run_dummy_fn(&self) {
   |                               ^^^^^ this data with an anonymous lifetime `'_`...
LL |         //[base]~^ ERROR E0759
LL |         foo(|| self.bar()).await;
   |         --- ...is used and required to live as long as `'static` here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0759`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/issues/issue-72312.rs#base stdout ----
diff of stderr:

12    |
13 LL |         require_static(async move {
14    |         ^^^^^^^^^^^^^^
- note: `'static` lifetime requirement introduced by this bound
-   --> $DIR/issue-72312.rs:6:22
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- LL | fn require_static<T: 'static>(val: T) -> T {
20 
21 error: aborting due to previous error
22 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-72312.base/issue-72312.base.stderr
To only update this specific test, also pass `--test-args async-await/issues/issue-72312.rs`


error in revision `base`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-72312.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "base" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-72312.base" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-72312.base/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0759]: `self` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement
   |
LL |     pub async fn start(&self) {
LL |     pub async fn start(&self) {
   |                        ^^^^^ this data with an anonymous lifetime `'_`...
...
LL |             &self; //[base]~ NOTE ...is used here...
   |             ----- ...is used here...
   |
note: ...and is required to live as long as `'static` here
   |
   |
LL |         require_static(async move {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0759`.
---

17    |
18 LL |     bar(|| {
19    |     ^^^
- note: `'static` lifetime requirement introduced by this bound
-   --> $DIR/closure-bounds-static-cant-capture-borrowed.rs:5:39
-    |
- LL | fn bar<F>(blk: F) where F: FnOnce() + 'static {
25 
26 error: aborting due to previous error
27 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-bounds-static-cant-capture-borrowed.base/closure-bounds-static-cant-capture-borrowed.base.stderr
To only update this specific test, also pass `--test-args closures/closure-bounds-static-cant-capture-borrowed.rs`


error in revision `base`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/closure-bounds-static-cant-capture-borrowed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "base" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-bounds-static-cant-capture-borrowed.base" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-bounds-static-cant-capture-borrowed.base/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0759]: `x` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement
   |
   |
LL |   fn foo(x: &()) {
   |             --- this data with an anonymous lifetime `'_`...
LL |       bar(|| {
   |  _________^
LL | |         //[base]~^ ERROR `x` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement [E0759]
LL | |         //[nll]~^^ ERROR borrowed data escapes
LL | |         //[nll]~| ERROR closure may outlive
LL | |         let _ = x;
LL | |     })
   | |_____^ ...is used here...
   |
note: ...and is required to live as long as `'static` here
   |
LL |     bar(|| {
   |     ^^^

---
diff of stderr:

5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
7    = help: consider adding an explicit lifetime bound `T: 'static`...
-    = note: ...so that the type `[T]` will meet its required lifetime bounds
+    = note: ...so that the type `T` will meet its required lifetime bounds
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/implied-bounds-unnorm-associated-type-3/implied-bounds-unnorm-associated-type-3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fn/implied-bounds-unnorm-associated-type-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fn/implied-bounds-unnorm-associated-type-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/implied-bounds-unnorm-associated-type-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/implied-bounds-unnorm-associated-type-3/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0310]: the parameter type `T` may not live long enough
   |
   |
LL |     fn zero_copy_from<'b>(cart: &'b [T]) -> &'b [T] {
   |
   |
   = help: consider adding an explicit lifetime bound `T: 'static`...
   = note: ...so that the type `T` will meet its required lifetime bounds
error: aborting due to previous error

For more information about this error, try `rustc --explain E0310`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/generic-associated-types/bugs/issue-86218.rs stdout ----
diff of stderr:

- error[E0477]: the type `<() as Yay<&'a ()>>::InnerStream<'s>` does not fulfill the required lifetime
+ error[E0478]: lifetime bound not satisfied
3    |
3    |
4 LL |     type InnerStream<'s> = impl Stream<Item = i32> + 's;
5    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
6    |
- note: type must outlive the lifetime `'s` as defined here as required by this binding
+ note: lifetime parameter instantiated with the lifetime `'a` as defined here
+    |
+    |
+ LL | impl<'a> Yay<&'a ()> for () {
+    |      ^^
+ note: but lifetime parameter must outlive the lifetime `'s` as defined here
9    |
9    |
10 LL |     type InnerStream<'s> = impl Stream<Item = i32> + 's;
20 
21 error: aborting due to 2 previous errors
22 
- For more information about this error, try `rustc --explain E0477`.
---
To only update this specific test, also pass `--test-args generic-associated-types/bugs/issue-86218.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/bugs/issue-86218.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-86218" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-86218/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0478]: lifetime bound not satisfied
   |
   |
LL |     type InnerStream<'s> = impl Stream<Item = i32> + 's;
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined here
   |
   |
LL | impl<'a> Yay<&'a ()> for () {
   |      ^^
note: but lifetime parameter must outlive the lifetime `'s` as defined here
   |
   |
LL |     type InnerStream<'s> = impl Stream<Item = i32> + 's;

error: unconstrained opaque type
  --> /checkout/src/test/ui/generic-associated-types/bugs/issue-86218.rs:23:28
   |
   |
LL |     type InnerStream<'s> = impl Stream<Item = i32> + 's;
   |
   |
   = note: `InnerStream` must be used in combination with a concrete type within the same module
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0478`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/generic-associated-types/issue-90014.rs stdout ----
diff of stderr:

- error[E0477]: the type `&mut ()` does not fulfill the required lifetime
+ error[E0478]: lifetime bound not satisfied
3    |
3    |
4 LL |     type Fut<'a> where Self: 'a;

7 LL |     type Fut<'a> = impl Future<Output = ()>;
8    |                    ^^^^^^^^^^^^^^^^^^^^^^^^- help: try copying this clause from the trait: `where Self: 'a`
9    |
- note: type must outlive the lifetime `'a` as defined here
+ note: lifetime parameter instantiated with the lifetime `'_` as defined here
+    |
+    |
+ LL | impl MakeFut for &'_ mut () {
+    |                   ^^
+ note: but lifetime parameter must outlive the lifetime `'a` as defined here
12    |
12    |
13 LL |     type Fut<'a> = impl Future<Output = ()>;
15 
16 error: aborting due to previous error
17 
- For more information about this error, try `rustc --explain E0477`.
---
To only update this specific test, also pass `--test-args generic-associated-types/issue-90014.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-90014.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-90014" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-90014/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0478]: lifetime bound not satisfied
   |
   |
LL |     type Fut<'a> where Self: 'a;
   |     ---------------------------- definition of `Fut` from trait
...
LL |     type Fut<'a> = impl Future<Output = ()>;
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^- help: try copying this clause from the trait: `where Self: 'a`
   |
note: lifetime parameter instantiated with the lifetime `'_` as defined here
   |
   |
LL | impl MakeFut for &'_ mut () {
   |                   ^^
note: but lifetime parameter must outlive the lifetime `'a` as defined here
   |
   |
LL |     type Fut<'a> = impl Future<Output = ()>;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0478`.
For more information about this error, try `rustc --explain E0478`.
------------------------------------------


---- [ui] src/test/ui/generic-associated-types/issue-92033.rs stdout ----
diff of stderr:

- error[E0477]: the type `&'s Texture` does not fulfill the required lifetime
+ error[E0478]: lifetime bound not satisfied
3    |
3    |
4 LL | /     type TextureIter<'a>: Iterator<Item = &'a Texture>

9 LL |       type TextureIter<'a> = std::option::IntoIter<&'a Texture>;
10    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- help: try copying this clause from the trait: `where Self: 'a`
11    |
- note: type must outlive the lifetime `'a` as defined here
+ note: lifetime parameter instantiated with the lifetime `'s` as defined here
+    |
+    |
+ LL | impl<'s> Surface for &'s Texture {
+    |      ^^
+ note: but lifetime parameter must outlive the lifetime `'a` as defined here
14    |
14    |
15 LL |     type TextureIter<'a> = std::option::IntoIter<&'a Texture>;
17 
18 error: aborting due to previous error
19 
- For more information about this error, try `rustc --explain E0477`.
---
To only update this specific test, also pass `--test-args generic-associated-types/issue-92033.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-92033.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-92033" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-92033/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0478]: lifetime bound not satisfied
   |
   |
LL | /     type TextureIter<'a>: Iterator<Item = &'a Texture>
LL | |     where
LL | |         Self: 'a;
   | |_________________- definition of `TextureIter` from trait
...
LL |       type TextureIter<'a> = std::option::IntoIter<&'a Texture>;
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- help: try copying this clause from the trait: `where Self: 'a`
   |
note: lifetime parameter instantiated with the lifetime `'s` as defined here
   |
   |
LL | impl<'s> Surface for &'s Texture {
   |      ^^
note: but lifetime parameter must outlive the lifetime `'a` as defined here
   |
   |
LL |     type TextureIter<'a> = std::option::IntoIter<&'a Texture>;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0478`.
For more information about this error, try `rustc --explain E0478`.
------------------------------------------


---- [ui] src/test/ui/generic-associated-types/unsatisfied-outlives-bound.rs stdout ----
diff of stderr:

- error[E0477]: the type `&'b ()` does not fulfill the required lifetime
+ error[E0478]: lifetime bound not satisfied
3    |
3    |
4 LL |     type Item<'a> = &'b ();
5    |                     ^^^^^^
6    |
6    |
- note: type must outlive the lifetime `'a` as defined here as required by this binding
+ note: lifetime parameter instantiated with the lifetime `'b` as defined here
+    |
+    |
+ LL | impl<'b> ATy for &'b () {
+    |      ^^
+ note: but lifetime parameter must outlive the lifetime `'a` as defined here
9    |
9    |
10 LL |     type Item<'a> = &'b ();
11    |               ^^
12 
12 
- error[E0477]: the type `&'a ()` does not fulfill the required lifetime
+ error[E0478]: lifetime bound not satisfied
15    |
15    |
16 LL |     type Item<'a> = &'a ();
17    |                     ^^^^^^
18    |
- note: type must satisfy the static lifetime as required by this binding
-   --> $DIR/unsatisfied-outlives-bound.rs:13:20
-   --> $DIR/unsatisfied-outlives-bound.rs:13:20
+ note: lifetime parameter instantiated with the lifetime `'a` as defined here
21    |
21    |
- LL |     type Item<'a>: 'static;
-    |                    ^^^^^^^
+ LL |     type Item<'a> = &'a ();
+    = note: but lifetime parameter must outlive the static lifetime
24 
25 error: aborting due to 2 previous errors
26 
---
To only update this specific test, also pass `--test-args generic-associated-types/unsatisfied-outlives-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/unsatisfied-outlives-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/unsatisfied-outlives-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/unsatisfied-outlives-bound/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0478]: lifetime bound not satisfied
   |
   |
LL |     type Item<'a> = &'b ();
   |
   |
note: lifetime parameter instantiated with the lifetime `'b` as defined here
   |
   |
LL | impl<'b> ATy for &'b () {
   |      ^^
note: but lifetime parameter must outlive the lifetime `'a` as defined here
   |
   |
LL |     type Item<'a> = &'b ();


error[E0478]: lifetime bound not satisfied
   |
   |
LL |     type Item<'a> = &'a ();
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined here
   |
   |
LL |     type Item<'a> = &'a ();
   = note: but lifetime parameter must outlive the static lifetime

error: aborting due to 2 previous errors


For more information about this error, try `rustc --explain E0478`.
------------------------------------------


---- [ui] src/test/ui/higher-rank-trait-bounds/issue-59311.rs#base stdout ----
diff of stderr:

- error[E0477]: the type `&'a V` does not fulfill the required lifetime
+ error[E0478]: lifetime bound not satisfied
3    |
3    |
4 LL |     v.t(|| {});
5    |     ^^^^^^^^^^
6    |
- note: type must satisfy the static lifetime as required by this binding
-   --> $DIR/issue-59311.rs:19:24
-   --> $DIR/issue-59311.rs:19:24
-    |
- LL |     for<'a> &'a V: T + 'static,
+    = note: but lifetime parameter must outlive the static lifetime
12 
13 error: aborting due to previous error
14 
14 

- For more information about this error, try `rustc --explain E0477`.
+ For more information about this error, try `rustc --explain E0478`.
16 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-59311.base/issue-59311.base.stderr
To only update this specific test, also pass `--test-args higher-rank-trait-bounds/issue-59311.rs`


error in revision `base`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/higher-rank-trait-bounds/issue-59311.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "base" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-59311.base" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-59311.base/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0478]: lifetime bound not satisfied
   |
   |
LL |     v.t(|| {});
   |
   = note: but lifetime parameter must outlive the static lifetime

error: aborting due to previous error
---
- error[E0282]: type annotations needed
+ error[E0284]: type annotations needed: cannot satisfy `_: '_`
2   --> $DIR/issue-24013.rs:5:20
3    |
4 LL |     unsafe {swap::<&mut _>(transmute(&a), transmute(&b))};
-    |                    ^^^^^^ cannot infer type
+    |                    ^^^^^^ cannot satisfy `_: '_`
+    |
+    |
+    = note: required so that reference `&mut _` does not outlive its referent
7 error: aborting due to previous error
8 

- For more information about this error, try `rustc --explain E0282`.
---
To only update this specific test, also pass `--test-args issues/issue-24013.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-24013.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24013" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24013/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0284]: type annotations needed: cannot satisfy `_: '_`
   |
   |
LL |     unsafe {swap::<&mut _>(transmute(&a), transmute(&b))};
   |                    ^^^^^^ cannot satisfy `_: '_`
   |
   = note: required so that reference `&mut _` does not outlive its referent
error: aborting due to previous error

For more information about this error, try `rustc --explain E0284`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/issues/issue-26217.rs#base stdout ----
diff of stderr:

- error[E0477]: the type `&'a i32` does not fulfill the required lifetime
+ error[E0478]: lifetime bound not satisfied
3    |
3    |
4 LL |     foo::<&'a i32>();
5    |     ^^^^^^^^^^^^^^
+    |
+    |
+ note: lifetime parameter instantiated with the lifetime `'a` as defined here
+    |
+    |
+ LL | fn bar<'a>() {
6 
7 error: aborting due to previous error
8 


- For more information about this error, try `rustc --explain E0477`.
+ For more information about this error, try `rustc --explain E0478`.
10 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26217.base/issue-26217.base.stderr
To only update this specific test, also pass `--test-args issues/issue-26217.rs`


error in revision `base`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-26217.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "base" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26217.base" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26217.base/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0478]: lifetime bound not satisfied
   |
   |
LL |     foo::<&'a i32>();
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined here
   |
   |
LL | fn bar<'a>() {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0478`.
For more information about this error, try `rustc --explain E0478`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-54943.rs#base stdout ----
diff of stderr:

- error[E0477]: the type `&'a u32` does not fulfill the required lifetime
+ error[E0478]: lifetime bound not satisfied
3    |
3    |
4 LL |     let x = foo::<&'a u32>();
5    |             ^^^^^^^^^^^^^^
6    |
- note: type must satisfy the static lifetime as required by this binding
-   --> $DIR/issue-54943.rs:5:11
-   --> $DIR/issue-54943.rs:5:11
+ note: lifetime parameter instantiated with the lifetime `'a` as defined here
9    |
9    |
- LL | fn foo<T: 'static>() { }
-    |           ^^^^^^^
+ LL | fn boo<'a>() {
+    = note: but lifetime parameter must outlive the static lifetime
12 
13 error: aborting due to previous error
14 
14 

- For more information about this error, try `rustc --explain E0477`.
+ For more information about this error, try `rustc --explain E0478`.
16 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54943.base/issue-54943.base.stderr
To only update this specific test, also pass `--test-args issues/issue-54943.rs`


error in revision `base`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-54943.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "base" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54943.base" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54943.base/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0478]: lifetime bound not satisfied
   |
   |
LL |     let x = foo::<&'a u32>();
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined here
   |
   |
LL | fn boo<'a>() {
   = note: but lifetime parameter must outlive the static lifetime

error: aborting due to previous error

---
---- [ui] src/test/ui/issues/issue-55796.rs#base stdout ----
diff of stderr:

9    |
10 LL | pub trait Graph<'a> {
11    |                 ^^
- note: ...so that the type `Map<<Self as Graph<'a>>::EdgesIter, [closure@$DIR/issue-55796.rs:20:40: 20:54]>` will meet its required lifetime bounds
+ note: ...so that the type `<Self as Graph<'a>>::EdgesIter` will meet its required lifetime bounds
14    |
14    |
15 LL |         Box::new(self.out_edges(u).map(|e| e.target()))
34    |
34    |
35 LL | pub trait Graph<'a> {
36    |                 ^^
- note: ...so that the type `Map<<Self as Graph<'a>>::EdgesIter, [closure@$DIR/issue-55796.rs:26:39: 26:53]>` will meet its required lifetime bounds
+ note: ...so that the type `<Self as Graph<'a>>::EdgesIter` will meet its required lifetime bounds
39    |
39    |
40 LL |         Box::new(self.in_edges(u).map(|e| e.target()))

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55796.base/issue-55796.base.stderr
To only update this specific test, also pass `--test-args issues/issue-55796.rs`


error in revision `base`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-55796.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "base" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55796.base" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55796.base/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
   |
   |
LL |         Box::new(self.out_edges(u).map(|e| e.target()))
   |
   |
note: first, the lifetime cannot outlive the lifetime `'a` as defined here...
   |
   |
LL | pub trait Graph<'a> {
   |                 ^^
note: ...so that the type `<Self as Graph<'a>>::EdgesIter` will meet its required lifetime bounds
   |
   |
LL |         Box::new(self.out_edges(u).map(|e| e.target()))
   = note: but, the lifetime must be valid for the static lifetime...
note: ...so that the types are compatible
  --> /checkout/src/test/ui/issues/issue-55796.rs:20:9
   |
   |
LL |         Box::new(self.out_edges(u).map(|e| e.target()))
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: expected `Box<(dyn Iterator<Item = <Self as Graph<'a>>::Node> + 'static)>`
              found `Box<dyn Iterator<Item = <Self as Graph<'a>>::Node>>`
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
  --> /checkout/src/test/ui/issues/issue-55796.rs:26:9
   |
   |
LL |         Box::new(self.in_edges(u).map(|e| e.target()))
   |
   |
note: first, the lifetime cannot outlive the lifetime `'a` as defined here...
   |
   |
LL | pub trait Graph<'a> {
   |                 ^^
note: ...so that the type `<Self as Graph<'a>>::EdgesIter` will meet its required lifetime bounds
   |
   |
LL |         Box::new(self.in_edges(u).map(|e| e.target()))
   = note: but, the lifetime must be valid for the static lifetime...
note: ...so that the types are compatible
  --> /checkout/src/test/ui/issues/issue-55796.rs:26:9
   |
   |
LL |         Box::new(self.in_edges(u).map(|e| e.target()))
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: expected `Box<(dyn Iterator<Item = <Self as Graph<'a>>::Node> + 'static)>`
              found `Box<dyn Iterator<Item = <Self as Graph<'a>>::Node>>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0495`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/kindck/kindck-impl-type-params.rs#base stdout ----
diff of stderr:

66 LL | fn g<T: std::marker::Copy>(val: T) {
68 
68 
- error[E0477]: the type `&'a isize` does not fulfill the required lifetime
+ error[E0478]: lifetime bound not satisfied
70   --> $DIR/kindck-impl-type-params.rs:34:13
71    |
72 LL |     let a = &t as &dyn Gettable<&'a isize>;
73    |             ^^
74    |
-    = note: type must satisfy the static lifetime
-    = note: type must satisfy the static lifetime
+ note: lifetime parameter instantiated with the lifetime `'a` as defined here
+   --> $DIR/kindck-impl-type-params.rs:32:8
+    |
+ LL | fn foo<'a>() {
+    = note: but lifetime parameter must outlive the static lifetime
76 
77 error[E0277]: the trait bound `String: Copy` is not satisfied
78   --> $DIR/kindck-impl-type-params.rs:40:13
---
113 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-impl-type-params.base/kindck-impl-type-params.base.stderr
To only update this specific test, also pass `--test-args kindck/kindck-impl-type-params.rs`


error in revision `base`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/kindck/kindck-impl-type-params.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "base" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-impl-type-params.base" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-impl-type-params.base/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `T` cannot be sent between threads safely
   |
   |
LL |     let a = &t as &dyn Gettable<T>;
   |             ^^ `T` cannot be sent between threads safely
   |
note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
   |
   |
LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
   = note: required for the cast to the object type `dyn Gettable<T>`
help: consider restricting type parameter `T`
   |
   |
LL | fn f<T: std::marker::Send>(val: T) {


error[E0277]: the trait bound `T: Copy` is not satisfied
   |
   |
LL |     let a = &t as &dyn Gettable<T>;
   |             ^^ the trait `Copy` is not implemented for `T`
   |
note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
   |
   |
LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
   = note: required for the cast to the object type `dyn Gettable<T>`
help: consider restricting type parameter `T`
   |
   |
LL | fn f<T: std::marker::Copy>(val: T) {


error[E0277]: `T` cannot be sent between threads safely
   |
   |
LL |     let a: &dyn Gettable<T> = &t;
   |                               ^^ `T` cannot be sent between threads safely
   |
note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
   |
   |
LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
   = note: required for the cast to the object type `dyn Gettable<T>`
help: consider restricting type parameter `T`
   |
   |
LL | fn g<T: std::marker::Send>(val: T) {


error[E0277]: the trait bound `T: Copy` is not satisfied
   |
   |
LL |     let a: &dyn Gettable<T> = &t;
   |                               ^^ the trait `Copy` is not implemented for `T`
   |
note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
   |
   |
LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
   = note: required for the cast to the object type `dyn Gettable<T>`
help: consider restricting type parameter `T`
   |
   |
LL | fn g<T: std::marker::Copy>(val: T) {


error[E0478]: lifetime bound not satisfied
   |
   |
LL |     let a = &t as &dyn Gettable<&'a isize>;
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined here
   |
   |
LL | fn foo<'a>() {
   = note: but lifetime parameter must outlive the static lifetime

error[E0277]: the trait bound `String: Copy` is not satisfied
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:40:13
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:40:13
   |
LL |     let a = t as Box<dyn Gettable<String>>;
   |             ^ the trait `Copy` is not implemented for `String`
   |
   = help: the trait `Gettable<T>` is implemented for `S<T>`
note: required because of the requirements on the impl of `Gettable<String>` for `S<String>`
   |
   |
LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
   |                                ^^^^^^^^^^^     ^^^^
   = note: required for the cast to the object type `dyn Gettable<String>`
error[E0277]: the trait bound `Foo: Copy` is not satisfied
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:48:37
   |
   |
LL |     let a: Box<dyn Gettable<Foo>> = t;
   |                                     ^ the trait `Copy` is not implemented for `Foo`
   |
   = help: the trait `Gettable<T>` is implemented for `S<T>`
note: required because of the requirements on the impl of `Gettable<Foo>` for `S<Foo>`
   |
   |
LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
   |                                ^^^^^^^^^^^     ^^^^
   = note: required for the cast to the object type `dyn Gettable<Foo>`
help: consider annotating `Foo` with `#[derive(Copy)]`
LL |     #[derive(Copy)]
   |

error: aborting due to 7 previous errors
---

---- [ui] src/test/ui/kindck/kindck-send-object1.rs#base stdout ----
diff of stderr:

12 LL | fn assert_send<T:Send+'static>() { }
13    |                  ^^^^ required by this bound in `assert_send`
14 
- error[E0477]: the type `&'a (dyn Dummy + Sync + 'a)` does not fulfill the required lifetime
+ error[E0478]: lifetime bound not satisfied
16   --> $DIR/kindck-send-object1.rs:18:5
17    |
18 LL |     assert_send::<&'a (dyn Dummy + Sync)>();
19    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
20    |
- note: type must satisfy the static lifetime as required by this binding
-   --> $DIR/kindck-send-object1.rs:9:23
-   --> $DIR/kindck-send-object1.rs:9:23
+ note: lifetime parameter instantiated with the lifetime `'a` as defined here
+   --> $DIR/kindck-send-object1.rs:17:11
23    |
- LL | fn assert_send<T:Send+'static>() { }
-    |                       ^^^^^^^
+ LL | fn test52<'a>() {
+    = note: but lifetime parameter must outlive the static lifetime
26 
26 
27 error[E0277]: `(dyn Dummy + 'a)` cannot be sent between threads safely
28   --> $DIR/kindck-send-object1.rs:33:5
41 
42 error: aborting due to 3 previous errors
43 
- Some errors have detailed explanations: E0277, E0477.
- Some errors have detailed explanations: E0277, E0477.
+ Some errors have detailed explanations: E0277, E0478.
45 For more information about an error, try `rustc --explain E0277`.
46 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-send-object1.base/kindck-send-object1.base.stderr
To only update this specific test, also pass `--test-args kindck/kindck-send-object1.rs`


error in revision `base`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/kindck/kindck-send-object1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "base" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-send-object1.base" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-send-object1.base/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `(dyn Dummy + 'a)` cannot be shared between threads safely
   |
   |
LL |     assert_send::<&'a dyn Dummy>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Dummy + 'a)` cannot be shared between threads safely
   |
   = help: the trait `Sync` is not implemented for `(dyn Dummy + 'a)`
   = note: required because of the requirements on the impl of `Send` for `&'a (dyn Dummy + 'a)`
note: required by a bound in `assert_send`
   |
   |
LL | fn assert_send<T:Send+'static>() { }
   |                  ^^^^ required by this bound in `assert_send`

error[E0478]: lifetime bound not satisfied
   |
   |
LL |     assert_send::<&'a (dyn Dummy + Sync)>();
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined here
   |
   |
LL | fn test52<'a>() {
   = note: but lifetime parameter must outlive the static lifetime


error[E0277]: `(dyn Dummy + 'a)` cannot be sent between threads safely
   |
   |
LL |     assert_send::<Box<dyn Dummy + 'a>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Dummy + 'a)` cannot be sent between threads safely
   |
   = help: the trait `Send` is not implemented for `(dyn Dummy + 'a)`
   = note: required because of the requirements on the impl of `Send` for `Unique<(dyn Dummy + 'a)>`
   = note: required because it appears within the type `Box<(dyn Dummy + 'a)>`
note: required by a bound in `assert_send`
   |
   |
LL | fn assert_send<T:Send+'static>() { }
   |                  ^^^^ required by this bound in `assert_send`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0478.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/lifetimes/re-empty-in-error.rs#base stdout ----
diff of stderr:

- error[E0477]: the type `&'b ()` does not fulfill the required lifetime
+ error[E0478]: lifetime bound not satisfied
2   --> $DIR/re-empty-in-error.rs:12:5
4 LL |     foo(&10);

5    |     ^^^
6    |
6    |
- note: type must outlive the empty lifetime as required by this binding
-   --> $DIR/re-empty-in-error.rs:7:47
-    |
- LL | fn foo<'a>(_a: &'a u32) where for<'b> &'b (): 'a {
-    |                                               ^^
+    = note: but lifetime parameter must outlive the empty lifetime
13 error: aborting due to previous error
14 

- For more information about this error, try `rustc --explain E0477`.
- For more information about this error, try `rustc --explain E0477`.
+ For more information about this error, try `rustc --explain E0478`.
16 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/re-empty-in-error.base/re-empty-in-error.base.stderr
To only update this specific test, also pass `--test-args lifetimes/re-empty-in-error.rs`


error in revision `base`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/re-empty-in-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "base" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/re-empty-in-error.base" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/re-empty-in-error.base/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0478]: lifetime bound not satisfied
   |
LL |     foo(&10);
   |     ^^^
   |
   |
   = note: but lifetime parameter must outlive the empty lifetime
error: aborting due to previous error

For more information about this error, try `rustc --explain E0478`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/regions/regions-assoc-type-region-bound-in-trait-not-met.rs stdout ----
diff of stderr:

- error[E0477]: the type `&'a i32` does not fulfill the required lifetime
+ error[E0478]: lifetime bound not satisfied
3    |
4 LL |     type Value = &'a i32;

5    |                  ^^^^^^^
5    |                  ^^^^^^^
6    |
- note: type must satisfy the static lifetime as required by this binding
-   --> $DIR/regions-assoc-type-region-bound-in-trait-not-met.rs:5:17
+ note: lifetime parameter instantiated with the lifetime `'a` as defined here
9    |
- LL |     type Value: 'a;
-    |                 ^^
-    |                 ^^
+ LL | impl<'a> Foo<'static> for &'a i32 {
+    = note: but lifetime parameter must outlive the static lifetime
12 
12 
- error[E0477]: the type `&'a i32` does not fulfill the required lifetime
+ error[E0478]: lifetime bound not satisfied
15    |
16 LL |     type Value = &'a i32;

17    |                  ^^^^^^^
17    |                  ^^^^^^^
18    |
- note: type must outlive the lifetime `'b` as defined here as required by this binding
+ note: lifetime parameter instantiated with the lifetime `'a` as defined here
+    |
+    |
+ LL | impl<'a, 'b> Foo<'b> for &'a i64 {
+    |      ^^
+ note: but lifetime parameter must outlive the lifetime `'b` as defined here
21    |
21    |
22 LL | impl<'a, 'b> Foo<'b> for &'a i64 {
24 
25 error: aborting due to 2 previous errors
26 
- For more information about this error, try `rustc --explain E0477`.
---
To only update this specific test, also pass `--test-args regions/regions-assoc-type-region-bound-in-trait-not-met.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-assoc-type-region-bound-in-trait-not-met.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-region-bound-in-trait-not-met" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-region-bound-in-trait-not-met/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0478]: lifetime bound not satisfied
   |
LL |     type Value = &'a i32;
   |                  ^^^^^^^
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined here
   |
   |
LL | impl<'a> Foo<'static> for &'a i32 {
   = note: but lifetime parameter must outlive the static lifetime


error[E0478]: lifetime bound not satisfied
   |
LL |     type Value = &'a i32;
   |                  ^^^^^^^
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined here
   |
   |
LL | impl<'a, 'b> Foo<'b> for &'a i64 {
   |      ^^
note: but lifetime parameter must outlive the lifetime `'b` as defined here
   |
   |
LL | impl<'a, 'b> Foo<'b> for &'a i64 {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0478`.
For more information about this error, try `rustc --explain E0478`.
------------------------------------------


---- [ui] src/test/ui/regions/regions-assoc-type-static-bound-in-trait-not-met.rs stdout ----
diff of stderr:

- error[E0477]: the type `&'a i32` does not fulfill the required lifetime
+ error[E0478]: lifetime bound not satisfied
3    |
4 LL |     type Value = &'a i32;

5    |                  ^^^^^^^
5    |                  ^^^^^^^
6    |
- note: type must satisfy the static lifetime as required by this binding
-   --> $DIR/regions-assoc-type-static-bound-in-trait-not-met.rs:5:17
+ note: lifetime parameter instantiated with the lifetime `'a` as defined here
9    |
- LL |     type Value: 'static;
-    |                 ^^^^^^^
-    |                 ^^^^^^^
+ LL | impl<'a> Foo for &'a i32 {
+    = note: but lifetime parameter must outlive the static lifetime
12 
13 error: aborting due to previous error
14 
---
To only update this specific test, also pass `--test-args regions/regions-assoc-type-static-bound-in-trait-not-met.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-assoc-type-static-bound-in-trait-not-met.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-static-bound-in-trait-not-met" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-static-bound-in-trait-not-met/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0478]: lifetime bound not satisfied
   |
LL |     type Value = &'a i32;
   |                  ^^^^^^^
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined here
   |
   |
LL | impl<'a> Foo for &'a i32 {
   = note: but lifetime parameter must outlive the static lifetime

error: aborting due to previous error


For more information about this error, try `rustc --explain E0478`.
------------------------------------------


---- [ui] src/test/ui/regions/regions-bounded-by-trait-requiring-static.rs#base stdout ----
diff of stderr:

- error[E0477]: the type `&'a isize` does not fulfill the required lifetime
+ error[E0478]: lifetime bound not satisfied
3    |
3    |
4 LL |     assert_send::<&'a isize>();
5    |     ^^^^^^^^^^^^^^^^^^^^^^^^
6    |
- note: type must satisfy the static lifetime as required by this binding
-   --> $DIR/regions-bounded-by-trait-requiring-static.rs:10:18
-   --> $DIR/regions-bounded-by-trait-requiring-static.rs:10:18
+ note: lifetime parameter instantiated with the lifetime `'a` as defined here
9    |
9    |
- LL | fn assert_send<T:'static>() { }
-    |                  ^^^^^^^
+ LL | fn param_not_ok<'a>(x: &'a isize) {
+    = note: but lifetime parameter must outlive the static lifetime
12 
12 
- error[E0477]: the type `&'a str` does not fulfill the required lifetime
+ error[E0478]: lifetime bound not satisfied
15    |
15    |
16 LL |     assert_send::<&'a str>();
17    |     ^^^^^^^^^^^^^^^^^^^^^^
18    |
- note: type must satisfy the static lifetime as required by this binding
-   --> $DIR/regions-bounded-by-trait-requiring-static.rs:10:18
-   --> $DIR/regions-bounded-by-trait-requiring-static.rs:10:18
+ note: lifetime parameter instantiated with the lifetime `'a` as defined here
21    |
21    |
- LL | fn assert_send<T:'static>() { }
-    |                  ^^^^^^^
+ LL | fn param_not_ok1<'a>(_: &'a isize) {
+    = note: but lifetime parameter must outlive the static lifetime
24 
24 
- error[E0477]: the type `&'a [isize]` does not fulfill the required lifetime
+ error[E0478]: lifetime bound not satisfied
27    |
27    |
28 LL |     assert_send::<&'a [isize]>();
29    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
30    |
- note: type must satisfy the static lifetime as required by this binding
-   --> $DIR/regions-bounded-by-trait-requiring-static.rs:10:18
-   --> $DIR/regions-bounded-by-trait-requiring-static.rs:10:18
+ note: lifetime parameter instantiated with the lifetime `'a` as defined here
33    |
33    |
- LL | fn assert_send<T:'static>() { }
-    |                  ^^^^^^^
+ LL | fn param_not_ok2<'a>(_: &'a isize) {
+    = note: but lifetime parameter must outlive the static lifetime
36 
36 
- error[E0477]: the type `Box<&'a isize>` does not fulfill the required lifetime
+ error[E0478]: lifetime bound not satisfied
39    |
39    |
40 LL |     assert_send::<Box<&'a isize>>();
41    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
42    |
- note: type must satisfy the static lifetime as required by this binding
-   --> $DIR/regions-bounded-by-trait-requiring-static.rs:10:18
-   --> $DIR/regions-bounded-by-trait-requiring-static.rs:10:18
+ note: lifetime parameter instantiated with the lifetime `'a` as defined here
45    |
45    |
- LL | fn assert_send<T:'static>() { }
-    |                  ^^^^^^^
+ LL | fn box_with_region_not_ok<'a>() {
+    = note: but lifetime parameter must outlive the static lifetime
48 
48 
- error[E0477]: the type `*const &'a isize` does not fulfill the required lifetime
+ error[E0478]: lifetime bound not satisfied
51    |
51    |
52 LL |     assert_send::<*const &'a isize>();
53    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
54    |
- note: type must satisfy the static lifetime as required by this binding
-   --> $DIR/regions-bounded-by-trait-requiring-static.rs:10:18
-   --> $DIR/regions-bounded-by-trait-requiring-static.rs:10:18
+ note: lifetime parameter instantiated with the lifetime `'a` as defined here
57    |
57    |
- LL | fn assert_send<T:'static>() { }
-    |                  ^^^^^^^
+ LL | fn unsafe_ok2<'a>(_: &'a isize) {
+    = note: but lifetime parameter must outlive the static lifetime
60 
60 
- error[E0477]: the type `*mut &'a isize` does not fulfill the required lifetime
+ error[E0478]: lifetime bound not satisfied
63    |
63    |
64 LL |     assert_send::<*mut &'a isize>();
65    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
66    |
- note: type must satisfy the static lifetime as required by this binding
-   --> $DIR/regions-bounded-by-trait-requiring-static.rs:10:18
-   --> $DIR/regions-bounded-by-trait-requiring-static.rs:10:18
+ note: lifetime parameter instantiated with the lifetime `'a` as defined here
69    |
69    |
- LL | fn assert_send<T:'static>() { }
-    |                  ^^^^^^^
+ LL | fn unsafe_ok3<'a>(_: &'a isize) {
+    = note: but lifetime parameter must outlive the static lifetime
72 
73 error: aborting due to 6 previous errors
74 
74 

- For more information about this error, try `rustc --explain E0477`.
+ For more information about this error, try `rustc --explain E0478`.
76 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-bounded-by-trait-requiring-static.base/regions-bounded-by-trait-requiring-static.base.stderr
To only update this specific test, also pass `--test-args regions/regions-bounded-by-trait-requiring-static.rs`


error in revision `base`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-bounded-by-trait-requiring-static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "base" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-bounded-by-trait-requiring-static.base" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-bounded-by-trait-requiring-static.base/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0478]: lifetime bound not satisfied
   |
   |
LL |     assert_send::<&'a isize>();
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined here
   |
   |
LL | fn param_not_ok<'a>(x: &'a isize) {
   = note: but lifetime parameter must outlive the static lifetime


error[E0478]: lifetime bound not satisfied
   |
   |
LL |     assert_send::<&'a str>();
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined here
   |
   |
LL | fn param_not_ok1<'a>(_: &'a isize) {
   = note: but lifetime parameter must outlive the static lifetime


error[E0478]: lifetime bound not satisfied
   |
   |
LL |     assert_send::<&'a [isize]>();
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined here
   |
   |
LL | fn param_not_ok2<'a>(_: &'a isize) {
   = note: but lifetime parameter must outlive the static lifetime


error[E0478]: lifetime bound not satisfied
   |
   |
LL |     assert_send::<Box<&'a isize>>();
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined here
   |
   |
LL | fn box_with_region_not_ok<'a>() {
   = note: but lifetime parameter must outlive the static lifetime


error[E0478]: lifetime bound not satisfied
   |
   |
LL |     assert_send::<*const &'a isize>();
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined here
   |
   |
LL | fn unsafe_ok2<'a>(_: &'a isize) {
   = note: but lifetime parameter must outlive the static lifetime


error[E0478]: lifetime bound not satisfied
   |
   |
LL |     assert_send::<*mut &'a isize>();
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined here
   |
   |
LL | fn unsafe_ok3<'a>(_: &'a isize) {
   = note: but lifetime parameter must outlive the static lifetime

error: aborting due to 6 previous errors


For more information about this error, try `rustc --explain E0478`.
------------------------------------------


---- [ui] src/test/ui/regions/regions-bounded-method-type-parameters.rs#base stdout ----
diff of stderr:

- error[E0477]: the type `&'a isize` does not fulfill the required lifetime
+ error[E0478]: lifetime bound not satisfied
3    |
3    |
4 LL |     Foo.some_method::<&'a isize>();
5    |         ^^^^^^^^^^^
6    |
- note: type must satisfy the static lifetime as required by this binding
-   --> $DIR/regions-bounded-method-type-parameters.rs:12:22
-   --> $DIR/regions-bounded-method-type-parameters.rs:12:22
+ note: lifetime parameter instantiated with the lifetime `'a` as defined here
9    |
9    |
- LL |     fn some_method<A:'static>(self) { }
-    |                      ^^^^^^^
+ LL | fn caller<'a>(x: &isize) {
+    = note: but lifetime parameter must outlive the static lifetime
12 
13 error: aborting due to previous error
14 
---
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-bounded-method-type-parameters.base/regions-bounded-method-type-parameters.base.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-bounded-method-type-parameters.rs`

error in revision `base`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-bounded-method-type-parameters.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "base" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-bounded-method-type-parameters.base" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-bounded-method-type-parameters.base/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0478]: lifetime bound not satisfied
   |
   |
LL |     Foo.some_method::<&'a isize>();
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined here
   |
   |
LL | fn caller<'a>(x: &isize) {
   = note: but lifetime parameter must outlive the static lifetime

error: aborting due to previous error

---
diff of stderr:

14    |     ^^^^^^^^^^^^^^
15    |
16    = help: consider adding an explicit lifetime bound `<T as Iter>::Item: 'static`...
-    = note: ...so that the type `Box<<T as Iter>::Item>` will meet its required lifetime bounds
+    = note: ...so that the type `<T as Iter>::Item` will meet its required lifetime bounds
18 
19 error[E0309]: the associated type `<T as Iter>::Item` may not live long enough

32    |     ^^^^^^^^^^^^^^
33    |
33    |
34    = help: consider adding an explicit lifetime bound `<T as Iter>::Item: 'a`...
-    = note: ...so that the type `Box<<T as Iter>::Item>` will meet its required lifetime bounds
+    = note: ...so that the type `<T as Iter>::Item` will meet its required lifetime bounds
37 error: aborting due to 4 previous errors
38 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-associated-type-into-object.base/regions-close-associated-type-into-object.base.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-close-associated-type-into-object.rs`

error in revision `base`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-close-associated-type-into-object.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "base" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-associated-type-into-object.base" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-associated-type-into-object.base/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0310]: the associated type `<T as Iter>::Item` may not live long enough
   |
   |
LL |     Box::new(item) //~ ERROR associated type `<T as Iter>::Item` may not live long enough
   |
   |
   = help: consider adding an explicit lifetime bound `<T as Iter>::Item: 'static`...
   = note: ...so that the type `<T as Iter>::Item` will meet its required lifetime bounds

error[E0310]: the associated type `<T as Iter>::Item` may not live long enough
   |
   |
LL |     Box::new(item) //~ ERROR associated type `<T as Iter>::Item` may not live long enough
   |
   |
   = help: consider adding an explicit lifetime bound `<T as Iter>::Item: 'static`...
   = note: ...so that the type `<T as Iter>::Item` will meet its required lifetime bounds

error[E0309]: the associated type `<T as Iter>::Item` may not live long enough
   |
   |
LL |     Box::new(item) //~ ERROR associated type `<T as Iter>::Item` may not live long enough
   |
   |
   = help: consider adding an explicit lifetime bound `<T as Iter>::Item: 'a`...
   = note: ...so that the type `<T as Iter>::Item` will meet its required lifetime bounds

error[E0309]: the associated type `<T as Iter>::Item` may not live long enough
   |
   |
LL |     Box::new(item) //~ ERROR associated type `<T as Iter>::Item` may not live long enough
   |
   |
   = help: consider adding an explicit lifetime bound `<T as Iter>::Item: 'a`...
   = note: ...so that the type `<T as Iter>::Item` will meet its required lifetime bounds
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0309, E0310.
For more information about an error, try `rustc --explain E0309`.
---
diff of stderr:

18   --> $DIR/regions-close-object-into-object-5.rs:21:5
19    |
20 LL |     Box::new(B(&*v)) as Box<dyn X>
-    |     ^^^^^^^^^^^^^^^^ ...so that the type `B<'_, T>` will meet its required lifetime bounds
+    |     ^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
23 help: consider adding an explicit lifetime bound...
24    |

42    |           +++++++++
42    |           +++++++++
43 
44 error[E0310]: the parameter type `T` may not live long enough
-   --> $DIR/regions-close-object-into-object-5.rs:21:14
-    |
- LL |     Box::new(B(&*v)) as Box<dyn X>
-    |              ^^^^^^ ...so that the type `T` will meet its required lifetime bounds...
- note: ...that is required by this bound
-   --> $DIR/regions-close-object-into-object-5.rs:13:17
-    |
-    |
- LL | struct B<'a, T: 'a>(&'a (A<T> + 'a));
- help: consider adding an explicit lifetime bound...
-    |
-    |
- LL | fn f<'a, T: 'static, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {
- 
- error[E0310]: the parameter type `T` may not live long enough
61   --> $DIR/regions-close-object-into-object-5.rs:21:16
62    |
62    |
63 LL |     Box::new(B(&*v)) as Box<dyn X>

-    |                ^^^ ...so that the reference type `&dyn A<T>` does not outlive the data it points at
- help: consider adding an explicit lifetime bound...
-    |
-    |
- LL | fn f<'a, T: 'static, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {
- 
- error[E0310]: the parameter type `T` may not live long enough
-   --> $DIR/regions-close-object-into-object-5.rs:21:16
-    |
-    |
- LL |     Box::new(B(&*v)) as Box<dyn X>
75    |                ^^^ ...so that the type `(dyn A<T> + 'static)` is not borrowed for too long
77 help: consider adding an explicit lifetime bound...


90 LL | fn f<'a, T: 'static, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {
92 
- error: aborting due to 7 previous errors
+ error: aborting due to 5 previous errors
94 
94 
95 For more information about this error, try `rustc --explain E0310`.
96 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-5.base/regions-close-object-into-object-5.base.stderr
To only update this specific test, also pass `--test-args regions/regions-close-object-into-object-5.rs`


error in revision `base`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-close-object-into-object-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "base" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-5.base" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-5.base/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0310]: the parameter type `T` may not live long enough
   |
   |
LL |     Box::new(B(&*v)) as Box<dyn X>
   |     ^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds...
note: ...that is required by this bound
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-5.rs:13:17
   |
   |
LL | struct B<'a, T: 'a>(&'a (A<T> + 'a));
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn f<'a, T: 'static, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {

error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-5.rs:21:5
   |
   |
LL |     Box::new(B(&*v)) as Box<dyn X>
   |     ^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn f<'a, T: 'static, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {

error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-5.rs:21:14
   |
   |
LL |     Box::new(B(&*v)) as Box<dyn X>
   |              ^ ...so that the type `T` will meet its required lifetime bounds...
note: ...that is required by this bound
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-5.rs:13:17
   |
   |
LL | struct B<'a, T: 'a>(&'a (A<T> + 'a));
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn f<'a, T: 'static, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {

error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-5.rs:21:16
   |
   |
LL |     Box::new(B(&*v)) as Box<dyn X>
   |                ^^^ ...so that the type `(dyn A<T> + 'static)` is not borrowed for too long
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn f<'a, T: 'static, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {

error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-5.rs:21:16
   |
   |
LL |     Box::new(B(&*v)) as Box<dyn X>
   |                ^^^ ...so that the type `(dyn A<T> + 'static)` is not borrowed for too long
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn f<'a, T: 'static, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0310`.
---
diff of stderr:

13   --> $DIR/regions-close-param-into-object.rs:16:5
14    |
15 LL |     Box::new(v)
-    |     ^^^^^^^^^^^ ...so that the type `Box<T>` will meet its required lifetime bounds
+    |     ^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
18 help: consider adding an explicit lifetime bound...
19    |

35   --> $DIR/regions-close-param-into-object.rs:28:5
35   --> $DIR/regions-close-param-into-object.rs:28:5
36    |
37 LL |     Box::new(v)
-    |     ^^^^^^^^^^^ ...so that the type `Box<T>` will meet its required lifetime bounds
+    |     ^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
40 help: consider adding an explicit lifetime bound...
41    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-param-into-object.base/regions-close-param-into-object.base.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-close-param-into-object.rs`

error in revision `base`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-close-param-into-object.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "base" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-param-into-object.base" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-param-into-object.base/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0310]: the parameter type `T` may not live long enough
   |
   |
LL |     Box::new(v) //~ ERROR parameter type `T` may not live long enough
   |     ^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL |     where T : X + 'static

error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-param-into-object.rs:16:5
   |
   |
LL |     Box::new(v) //~ ERROR parameter type `T` may not live long enough
   |     ^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn p2<T: 'static>(v: Box<T>) -> Box<dyn X + 'static>

error[E0309]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-param-into-object.rs:22:5
   |
   |
LL |     Box::new(v) //~ ERROR parameter type `T` may not live long enough
   |     ^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL |     where T : X + 'a

error[E0309]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-param-into-object.rs:28:5
   |
   |
LL |     Box::new(v) //~ ERROR parameter type `T` may not live long enough
   |     ^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn p4<'a,T: 'a>(v: Box<T>) -> Box<dyn X + 'a>

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0309, E0310.
---
---- [ui] src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature-2.rs#base stdout ----
diff of stderr:

9    |
10 LL | fn func<T: Test>(foo: &Foo, t: T) {
11    |                        ^^^
- note: ...so that the type `[closure@$DIR/missing-lifetimes-in-signature-2.rs:24:13: 27:6]` will meet its required lifetime bounds...
+ note: ...so that the type `T` will meet its required lifetime bounds...
14    |
14    |
15 LL |     foo.bar(move |_| {

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature-2.base/missing-lifetimes-in-signature-2.base.stderr
To only update this specific test, also pass `--test-args suggestions/lifetimes/missing-lifetimes-in-signature-2.rs`


error in revision `base`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "base" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature-2.base" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature-2.base/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0311]: the parameter type `T` may not live long enough
   |
   |
LL |     foo.bar(move |_| {
   |
   |
note: the parameter type `T` must be valid for the anonymous lifetime defined here...
   |
   |
LL | fn func<T: Test>(foo: &Foo, t: T) {
   |                        ^^^
note: ...so that the type `T` will meet its required lifetime bounds...
   |
   |
LL |     foo.bar(move |_| {
note: ...that is required by this bound
  --> /checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature-2.rs:15:12
   |
LL |         F: 'a,
LL |         F: 'a,
   |            ^^
help: consider adding an explicit lifetime bound...
   |
LL | fn func<T: Test + 'a>(foo: &Foo, t: T) {

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs#base stdout ----
diff of stderr:

34    |
35 LL | fn bar<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
36    |                          ^^^^^^
- note: ...so that the type `[closure@$DIR/missing-lifetimes-in-signature.rs:35:5: 38:6]` will meet its required lifetime bounds
+ note: ...so that the type `G` will meet its required lifetime bounds
39    |
39    |
40 LL | fn bar<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
41    |                                     ^^^^^^^^^^^^^^^^^^
- help: consider introducing an explicit lifetime bound
- help: consider introducing an explicit lifetime bound
+ help: consider adding an explicit lifetime bound...
43    |
- LL ~ fn bar<'a, G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'a
- LL |
- LL | where
- LL ~     G: Get<T> + 'a,
-    |
+ LL |     G: Get<T> + 'a,
49 
49 
50 error[E0311]: the parameter type `G` may not live long enough

58    |
58    |
59 LL | fn qux<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
60    |                                  ^^^^^^
- note: ...so that the type `[closure@$DIR/missing-lifetimes-in-signature.rs:58:5: 61:6]` will meet its required lifetime bounds
+ note: ...so that the type `G` will meet its required lifetime bounds
63    |
63    |
64 LL | fn qux<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
65    |                                             ^^^^^^^^^^^^^^^^^^
- help: consider introducing an explicit lifetime bound
- help: consider introducing an explicit lifetime bound
+ help: consider adding an explicit lifetime bound...
67    |
- LL | fn qux<'b, 'a, G: 'a + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'b
-    |        +++           ++++                                               ++++
+ LL | fn qux<'a, G: 'a + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
70 
70 
71 error[E0311]: the parameter type `G` may not live long enough

79    |
79    |
80 LL |     fn qux<'b, G: Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {
81    |                                               ^^^^^^
- note: ...so that the type `[closure@$DIR/missing-lifetimes-in-signature.rs:68:9: 71:10]` will meet its required lifetime bounds
+ note: ...so that the type `G` will meet its required lifetime bounds
84    |
84    |
85 LL |     fn qux<'b, G: Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {
86    |                                                          ^^^^^^^^^^^^^^^^^^
- help: consider introducing an explicit lifetime bound
- help: consider introducing an explicit lifetime bound
+ help: consider adding an explicit lifetime bound...
88    |
- LL |     fn qux<'c, 'b, G: Get<T> + 'b + 'c, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'c {
-    |            +++                    ++++                                               ++++
+ LL |     fn qux<'b, G: Get<T> + 'b + 'c, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {
91 
91 
92 error[E0621]: explicit lifetime required in the type of `dest`

101   --> $DIR/missing-lifetimes-in-signature.rs:89:44
102    |
102    |
103 LL | fn bak<'a, G, T>(g: G, dest: &'a mut T) -> impl FnOnce() + 'a
-    |                                            ^^^^^^^^^^^^^^^^^^ ...so that the type `[closure@$DIR/missing-lifetimes-in-signature.rs:94:5: 97:6]` will meet its required lifetime bounds
+    |                                            ^^^^^^^^^^^^^^^^^^ ...so that the type `G` will meet its required lifetime bounds
106 help: consider adding an explicit lifetime bound...
107    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.base/missing-lifetimes-in-signature.base.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/lifetimes/missing-lifetimes-in-signature.rs`

error in revision `base`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "base" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.base" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.base/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0261]: use of undeclared lifetime name `'a`
   |
   |
LL | fn baz<G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
   |        -  ^^ undeclared lifetime
   |        |
   |        help: consider introducing lifetime `'a` here: `'a,`

error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
   |
   |
LL |   fn foo<G, T>(g: G, dest: &mut T) -> impl FnOnce()
   |                            ------ hidden type `[closure@/checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs:23:5: 26:6]` captures the anonymous lifetime defined here
LL | /     move || {
LL | /     move || {
LL | |         //~^ ERROR hidden type for `impl Trait` captures lifetime
LL | |         *dest = g.get();
LL | |     }
   |
   |
help: to declare that the `impl Trait` captures `'_`, you can add an explicit `'_` lifetime bound
   |
LL | fn foo<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_


error[E0311]: the parameter type `G` may not live long enough
   |
   |
LL | fn bar<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
   |
   |
note: the parameter type `G` must be valid for the anonymous lifetime defined here...
   |
   |
LL | fn bar<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
   |                          ^^^^^^
note: ...so that the type `G` will meet its required lifetime bounds
   |
   |
LL | fn bar<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
help: consider adding an explicit lifetime bound...
   |
   |
LL |     G: Get<T> + 'a,


error[E0311]: the parameter type `G` may not live long enough
   |
   |
LL | fn qux<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
   |
   |
note: the parameter type `G` must be valid for the anonymous lifetime defined here...
   |
   |
LL | fn qux<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
   |                                  ^^^^^^
note: ...so that the type `G` will meet its required lifetime bounds
   |
   |
LL | fn qux<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn qux<'a, G: 'a + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_


error[E0311]: the parameter type `G` may not live long enough
   |
   |
LL |     fn qux<'b, G: Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {
   |
   |
note: the parameter type `G` must be valid for the anonymous lifetime defined here...
   |
   |
LL |     fn qux<'b, G: Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {
   |                                               ^^^^^^
note: ...so that the type `G` will meet its required lifetime bounds
   |
   |
LL |     fn qux<'b, G: Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {
help: consider adding an explicit lifetime bound...
   |
   |
LL |     fn qux<'b, G: Get<T> + 'b + 'c, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {


error[E0621]: explicit lifetime required in the type of `dest`
   |
   |
LL | fn bat<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'a
   |                                  ------     ^^^^^^^^^^^^^^^^^^^^^^^ lifetime `'a` required
   |                                  |
   |                                  help: add explicit lifetime `'a` to the type of `dest`: `&'a mut T`

error[E0309]: the parameter type `G` may not live long enough
   |
   |
LL | fn bak<'a, G, T>(g: G, dest: &'a mut T) -> impl FnOnce() + 'a
   |                                            ^^^^^^^^^^^^^^^^^^ ...so that the type `G` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL |     G: Get<T> + 'a,

error: aborting due to 7 previous errors

Some errors have detailed explanations: E0261, E0309, E0621, E0700.
Some errors have detailed explanations: E0261, E0309, E0621, E0700.
For more information about an error, try `rustc --explain E0261`.
------------------------------------------


---- [ui] src/test/ui/traits/object/supertrait-lifetime-bound.rs#base stdout ----
diff of stderr:

- error[E0477]: the type `(dyn Bar<&'a u32> + 'static)` does not fulfill the required lifetime
+ error[E0478]: lifetime bound not satisfied
3    |
3    |
4 LL |     test1::<dyn Bar<&'a u32>, _>();
5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
- note: type must satisfy the static lifetime as required by this binding
-   --> $DIR/supertrait-lifetime-bound.rs:9:22
-   --> $DIR/supertrait-lifetime-bound.rs:9:22
+ note: lifetime parameter instantiated with the lifetime `'a` as defined here
9    |
9    |
- LL | fn test1<T: ?Sized + Bar<S>, S>() { }
-    |                      ^^^^^^
+ LL | fn test2<'a>() {
+    = note: but lifetime parameter must outlive the static lifetime
12 
13 error: aborting due to previous error
14 
14 

- For more information about this error, try `rustc --explain E0477`.
+ For more information about this error, try `rustc --explain E0478`.
16 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/supertrait-lifetime-bound.base/supertrait-lifetime-bound.base.stderr
To only update this specific test, also pass `--test-args traits/object/supertrait-lifetime-bound.rs`


error in revision `base`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/object/supertrait-lifetime-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "base" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/supertrait-lifetime-bound.base" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/supertrait-lifetime-bound.base/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0478]: lifetime bound not satisfied
   |
   |
LL |     test1::<dyn Bar<&'a u32>, _>();
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined here
   |
   |
LL | fn test2<'a>() {
   = note: but lifetime parameter must outlive the static lifetime

error: aborting due to previous error

