plain
.......................................................i................................ 3960/13046
.......................................................................F................ 4048/13046
..............i......................................................................... 4136/13046
........................................................................................ 4224/13046
...................................F.................................................... 4312/13046
.................................................F....F................................. 4400/13046
.F...................................................................................... 4488/13046
........................................................................................ 4664/13046
........................................................................................ 4752/13046
........................................................................................ 4840/13046
........................................................................................ 4928/13046
---
diff of stderr:

5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
7    = help: consider adding an explicit lifetime bound `T: 'static`...
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
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
+ note: lifetime parameter instantiated with the anonymous lifetime as defined here
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
note: lifetime parameter instantiated with the anonymous lifetime as defined here
  --> /checkout/src/test/ui/generic-associated-types/issue-90014.rs:13:19
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
