plain

---- [ui] src/test/ui/associated-types/cache/project-fn-ret-invariant-nll.rs#oneuse stdout ----
diff of stderr:

15    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
17 error: lifetime may not live long enough
-   --> $DIR/project-fn-ret-invariant-nll.rs:47:13
+   --> $DIR/project-fn-ret-invariant-nll.rs:46:13
19    |
19    |
20 LL | fn baz<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
21    |        --  -- lifetime `'b` defined here
22    |        |
22    |        |
23    |        lifetime `'a` defined here
- ...
- LL |     let b = bar(f, y);
+ LL |     let f = foo; // <-- No consistent type can be inferred for `f` here.
+ LL |     let a = bar(f, x);
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
26    |             ^^^^^^^^^ argument requires that `'b` must outlive `'a`
27    |
28    = help: consider adding the following bound: `'b: 'a`

-    = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant
-    = note: the struct `Type<'a>` is invariant over the parameter `'a`
+    = note: requirement occurs because of a function pointer to `foo`
+    = note: the function `foo` is invariant over the parameter `'a`
31    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
32 
33 help: `'a` and `'b` must be the same: replace one with the other

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant-nll.oneuse/project-fn-ret-invariant-nll.oneuse.stderr
To only update this specific test, also pass `--test-args associated-types/cache/project-fn-ret-invariant-nll.rs`


error in revision `oneuse`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant-nll.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "oneuse" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant-nll.oneuse" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant-nll.oneuse/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn baz<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
   |        --  -- lifetime `'b` defined here
   |        |
   |        lifetime `'a` defined here
LL |     let f = foo; // <-- No consistent type can be inferred for `f` here.
LL |     let a = bar(f, x); //[oneuse]~ ERROR lifetime may not live long enough
   |             ^^^^^^^^^ argument requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `Type<'a>` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant-nll.rs:46:13
   |
   |
LL | fn baz<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
   |        --  -- lifetime `'b` defined here
   |        |
   |        lifetime `'a` defined here
LL |     let f = foo; // <-- No consistent type can be inferred for `f` here.
LL |     let a = bar(f, x); //[oneuse]~ ERROR lifetime may not live long enough
   |             ^^^^^^^^^ argument requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of a function pointer to `foo`
   = note: the function `foo` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

help: `'a` and `'b` must be the same: replace one with the other
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/async-await/issues/issue-72312.rs#nll stdout ----
diff of stderr:

1 error[E0521]: borrowed data escapes outside of associated function
-   --> $DIR/issue-72312.rs:20:24
3    |
4 LL |       pub async fn start(&self) {
5    |                          -----


7    |                          `self` is a reference that is only valid in the associated function body
8    |                          let's call the lifetime of this reference `'1`
9 ...
- LL |           require_static(async move {
-    |  ________________________^
+ LL | /         require_static(async move {
12 LL | |
13 LL | |
14 LL | |
15 LL | |
16 LL | |             &self;
17 LL | |         });
-    | |         ^
-    | |         ^
-    | |         |
-    | |_________`self` escapes the associated function body here
-    |           argument requires that `'1` must outlive `'static`
+    | |          |
+    | |          |
+    | |__________`self` escapes the associated function body here
+    |            argument requires that `'1` must outlive `'static`
23 error: aborting due to previous error
24 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-72312.nll/issue-72312.nll.stderr
To only update this specific test, also pass `--test-args async-await/issues/issue-72312.rs`


error in revision `nll`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-72312.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-72312.nll" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-72312.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0521]: borrowed data escapes outside of associated function
   |
LL |       pub async fn start(&self) {
   |                          -----
   |                          |
   |                          |
   |                          `self` is a reference that is only valid in the associated function body
   |                          let's call the lifetime of this reference `'1`
...
LL | /         require_static(async move {
LL | |             //[base]~^ NOTE ...and is required to live as long as `'static` here
LL | |             //[nll]~^^ ERROR borrowed data escapes
LL | |             //[nll]~| NOTE `self` escapes
LL | |             //[nll]~| NOTE argument requires
LL | |             &self; //[base]~ NOTE ...is used here...
LL | |         });
   | |          |
   | |          |
   | |__________`self` escapes the associated function body here
   |            argument requires that `'1` must outlive `'static`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0521`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/suggestions/impl-on-dyn-trait-with-implicit-static-bound-nll.rs stdout ----
diff of stderr:

12    |         argument requires that `'a` must outlive `'static`
13    |
14 note: the used `impl` has a `'static` requirement
-   --> $DIR/impl-on-dyn-trait-with-implicit-static-bound.rs:14:32
16    |
16    |
17 LL |     impl<T> MyTrait<T> for dyn ObjectTrait<T> {
18    |                                ^^^^^^^^^^^^^^ this has an implicit `'static` lifetime requirement

37    |         argument requires that `'a` must outlive `'static`
38    |
39 note: the used `impl` has a `'static` requirement
-   --> $DIR/impl-on-dyn-trait-with-implicit-static-bound.rs:64:14
41    |
41    |
42 LL |     impl dyn ObjectTrait {
43    |              ^^^^^^^^^^^ this has an implicit `'static` lifetime requirement

62    |         argument requires that `'a` must outlive `'static`
63    |
64 note: the used `impl` has a `'static` requirement
-   --> $DIR/impl-on-dyn-trait-with-implicit-static-bound.rs:85:26
66    |
66    |
67 LL |         fn use_self(&self) -> &() { panic!() }
68    |            -------- calling this method introduces the `impl`'s 'static` requirement

88    |         argument requires that `'a` must outlive `'static`
89    |
90 note: the used `impl` has a `'static` requirement
-   --> $DIR/impl-on-dyn-trait-with-implicit-static-bound.rs:104:26
92    |
92    |
93 LL |         fn use_self(&self) -> &() { panic!() }
94    |            -------- calling this method introduces the `impl`'s 'static` requirement

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/impl-on-dyn-trait-with-implicit-static-bound-nll/impl-on-dyn-trait-with-implicit-static-bound-nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/impl-on-dyn-trait-with-implicit-static-bound-nll.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/impl-on-dyn-trait-with-implicit-static-bound-nll.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/impl-on-dyn-trait-with-implicit-static-bound-nll" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/impl-on-dyn-trait-with-implicit-static-bound-nll/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0521]: borrowed data escapes outside of function
   |
   |
LL |     fn use_it<'a, T>(val: &'a dyn ObjectTrait<T>) -> impl OtherTrait<'a> + 'a {
   |               --     --- `val` is a reference that is only valid in the function body
   |               |
   |               lifetime `'a` defined here
LL |         val.use_self::<T>() //~ ERROR borrowed data escapes
   |         |
   |         |
   |         `val` escapes the function body here
   |         argument requires that `'a` must outlive `'static`
   |
note: the used `impl` has a `'static` requirement
   |
   |
LL |     impl<T> MyTrait<T> for dyn ObjectTrait<T> {
   |                                ^^^^^^^^^^^^^^ this has an implicit `'static` lifetime requirement
LL |         fn use_self<K>(&self) -> &() { panic!() }
   |            -------- calling this method introduces the `impl`'s 'static` requirement
help: consider relaxing the implicit `'static` requirement
   |
LL |     impl<T> MyTrait<T> for dyn ObjectTrait<T> + '_ {


error[E0521]: borrowed data escapes outside of function
   |
   |
LL |     fn use_it<'a>(val: &'a dyn ObjectTrait) -> impl OtherTrait<'a> + 'a {
   |               --  --- `val` is a reference that is only valid in the function body
   |               |
   |               lifetime `'a` defined here
LL |         val.use_self()
   |         |
   |         |
   |         `val` escapes the function body here
   |         argument requires that `'a` must outlive `'static`
   |
note: the used `impl` has a `'static` requirement
   |
   |
LL |     impl dyn ObjectTrait {
   |              ^^^^^^^^^^^ this has an implicit `'static` lifetime requirement
LL |         fn use_self(&self) -> &() { panic!() }
   |            -------- calling this method introduces the `impl`'s 'static` requirement
help: consider relaxing the implicit `'static` requirement
   |
LL |     impl dyn ObjectTrait + '_ {


error[E0521]: borrowed data escapes outside of function
   |
   |
LL |     fn use_it<'a>(val: &'a dyn ObjectTrait) -> impl OtherTrait<'a> {
   |               --  --- `val` is a reference that is only valid in the function body
   |               |
   |               lifetime `'a` defined here
LL |         val.use_self() //~ ERROR borrowed data escapes
   |         |
   |         |
   |         `val` escapes the function body here
   |         argument requires that `'a` must outlive `'static`
   |
note: the used `impl` has a `'static` requirement
   |
   |
LL |         fn use_self(&self) -> &() { panic!() }
   |            -------- calling this method introduces the `impl`'s 'static` requirement
...
LL |     impl MyTrait for dyn ObjectTrait {}
   |                          ^^^^^^^^^^^ this has an implicit `'static` lifetime requirement
help: consider relaxing the implicit `'static` requirement
   |
LL |     impl MyTrait for dyn ObjectTrait + '_ {}


error[E0521]: borrowed data escapes outside of function
   |
   |
LL |     fn use_it<'a>(val: &'a dyn ObjectTrait) -> impl OtherTrait<'a> + 'a {
   |               --  --- `val` is a reference that is only valid in the function body
   |               |
   |               lifetime `'a` defined here
LL |         MyTrait::use_self(val) //~ ERROR borrowed data escapes
   |         |
   |         |
   |         `val` escapes the function body here
   |         argument requires that `'a` must outlive `'static`
   |
note: the used `impl` has a `'static` requirement
   |
   |
LL |         fn use_self(&self) -> &() { panic!() }
   |            -------- calling this method introduces the `impl`'s 'static` requirement
...
LL |     impl MyTrait for dyn ObjectTrait {}
   |                          ^^^^^^^^^^^ this has an implicit `'static` lifetime requirement
help: consider relaxing the implicit `'static` requirement
   |
LL |     impl MyTrait for dyn ObjectTrait + '_ {}

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0521`.
