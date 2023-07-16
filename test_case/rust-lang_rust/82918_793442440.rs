plain
.................................................................................................... 9200/11536
.................................................................................................... 9300/11536
.................................................................................................... 9400/11536
......................................................................i......i...................... 9500/11536
...............................F.................................................................... 9600/11536
................iiiiiii..iiiiii.i................................................................... 9700/11536
.................................................................................................... 9900/11536
.................................................................................................... 10000/11536
.................................................................................................... 10100/11536
.................................................................................................... 10200/11536
---
---- [ui] ui/feature-gates/feature-gate-object_safe_for_dispatch.rs stdout ----
diff of stderr:

57    |
58    = help: consider moving `foo` to another trait
59 note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
-   --> $DIR/feature-gate-object_safe_for_dispatch.rs:15:19
+   --> $DIR/feature-gate-object_safe_for_dispatch.rs:15:22
61    |
62 LL | trait NonObjectSafe4 {
63    |       -------------- this trait cannot be made into an object...

- LL |     fn foo(&self, &Self);
-    |                   ^^^^^ ...because method `foo` references the `Self` type in this parameter
+ LL |     fn foo(&self, s: &Self);
+    |                      ^^^^^ ...because method `foo` references the `Self` type in this parameter
66 
67 error[E0038]: the trait `NonObjectSafe1` cannot be made into an object
68   --> $DIR/feature-gate-object_safe_for_dispatch.rs:38:16

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-object_safe_for_dispatch/feature-gate-object_safe_for_dispatch.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-object_safe_for_dispatch/feature-gate-object_safe_for_dispatch.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/feature-gate-object_safe_for_dispatch.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-object_safe_for_dispatch" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-object_safe_for_dispatch/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0038]: the trait `NonObjectSafe1` cannot be made into an object
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:18:38
   |
LL | fn takes_non_object_safe_ref<T>(obj: &dyn NonObjectSafe1) {
   |                                      ^^^^^^^^^^^^^^^^^^^ `NonObjectSafe1` cannot be made into an object
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:4:23
   |
LL | trait NonObjectSafe1: Sized {}
   |       --------------  ^^^^^ ...because it requires `Self: Sized`
   |       |
   |       this trait cannot be made into an object...

error[E0038]: the trait `NonObjectSafe2` cannot be made into an object
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:22:36
   |
LL | fn return_non_object_safe_ref() -> &'static dyn NonObjectSafe2 {
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `NonObjectSafe2` cannot be made into an object
   |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:7:8
   |
LL | trait NonObjectSafe2 {
   |       -------------- this trait cannot be made into an object...
LL |     fn static_fn() {}
   |        ^^^^^^^^^ ...because associated function `static_fn` has no `self` parameter
help: consider turning `static_fn` into a method by giving it a `&self` argument
   |
LL |     fn static_fn(&self) {}
   |                  ^^^^^
help: alternatively, consider constraining `static_fn` so it does not apply to trait objects
   |
LL |     fn static_fn() where Self: Sized {}


error[E0038]: the trait `NonObjectSafe3` cannot be made into an object
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:27:35
   |
LL | fn takes_non_object_safe_box(obj: Box<dyn NonObjectSafe3>) {
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^ `NonObjectSafe3` cannot be made into an object
   |
   = help: consider moving `foo` to another trait
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:11:8
   |
LL | trait NonObjectSafe3 {
   |       -------------- this trait cannot be made into an object...
LL |     fn foo<T>(&self);
   |        ^^^ ...because method `foo` has generic type parameters

error[E0038]: the trait `NonObjectSafe4` cannot be made into an object
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:31:35
   |
LL | fn return_non_object_safe_rc() -> std::rc::Rc<dyn NonObjectSafe4> {
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `NonObjectSafe4` cannot be made into an object
   |
   = help: consider moving `foo` to another trait
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:15:22
   |
LL | trait NonObjectSafe4 {
   |       -------------- this trait cannot be made into an object...
LL |     fn foo(&self, s: &Self);
   |                      ^^^^^ ...because method `foo` references the `Self` type in this parameter

error[E0038]: the trait `NonObjectSafe1` cannot be made into an object
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:38:16
   |
LL | impl Trait for dyn NonObjectSafe1 {}
   |                ^^^^^^^^^^^^^^^^^^ `NonObjectSafe1` cannot be made into an object
   |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/feature-gates/feature-gate-object_safe_for_dispatch.rs:4:23
   |
LL | trait NonObjectSafe1: Sized {}
   |       --------------  ^^^^^ ...because it requires `Self: Sized`
   |       |
   |       this trait cannot be made into an object...
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0038`.

---
27 error[E0308]: mismatched types
-   --> $DIR/issue-78720.rs:7:36
+   --> $DIR/issue-78720.rs:7:39
29    |
- LL |     fn map2<F>(self, F) -> Map2<F> {}
-    |                                    ^^ expected struct `Map2`, found `()`
+ LL |     fn map2<F>(self, f: F) -> Map2<F> {}
+    |                                       ^^ expected struct `Map2`, found `()`
32    |
33    = note: expected struct `Map2<F>`


36 error[E0277]: the size for values of type `Self` cannot be known at compilation time
38    |
38    |
- LL |     fn map2<F>(self, F) -> Map2<F> {}
+ LL |     fn map2<F>(self, f: F) -> Map2<F> {}
40    |                ^^^^ doesn't have a size known at compile-time
41    |
42    = help: unsized fn params are gated as an unstable feature
43 help: consider further restricting `Self`
44    |
44    |
- LL |     fn map2<F>(self, F) -> Map2<F> where Self: Sized {}
-    |                                    ^^^^^^^^^^^^^^^^^
+ LL |     fn map2<F>(self, f: F) -> Map2<F> where Self: Sized {}
+    |                                       ^^^^^^^^^^^^^^^^^
47 help: function arguments must have a statically known size, borrowed types always have a known size
48    |
- LL |     fn map2<F>(&self, F) -> Map2<F> {}
+ LL |     fn map2<F>(&self, f: F) -> Map2<F> {}
51 
52 error: aborting due to 4 previous errors



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-78720/issue-78720.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-78720.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-78720.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-78720" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-78720/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: at least one trait must be specified
  --> /checkout/src/test/ui/issues/issue-78720.rs:1:16
   |
LL | fn server() -> impl {

error[E0412]: cannot find type `F` in this scope
  --> /checkout/src/test/ui/issues/issue-78720.rs:13:12
   |
   |
LL |     _func: F,
   | 
  ::: /checkout/library/core/src/ops/function.rs:67:1
   |
   |
LL | pub trait Fn<Args>: FnMut<Args> {
   | ------------------------------- similarly named trait `Fn` defined here
help: a trait with a similar name exists
   |
   |
LL |     _func: Fn,
help: you might be missing a type parameter
   |
   |
LL | struct Map2<Segment2, F> {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-78720.rs:7:39
   |
   |
LL |     fn map2<F>(self, f: F) -> Map2<F> {}
   |                                       ^^ expected struct `Map2`, found `()`
   |
   = note: expected struct `Map2<F>`


error[E0277]: the size for values of type `Self` cannot be known at compilation time
   |
   |
LL |     fn map2<F>(self, f: F) -> Map2<F> {}
   |                ^^^^ doesn't have a size known at compile-time
   |
   = help: unsized fn params are gated as an unstable feature
help: consider further restricting `Self`
   |
LL |     fn map2<F>(self, f: F) -> Map2<F> where Self: Sized {}
   |                                       ^^^^^^^^^^^^^^^^^
help: function arguments must have a statically known size, borrowed types always have a known size
   |
LL |     fn map2<F>(&self, f: F) -> Map2<F> {}

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0277, E0308, E0412.
---

---- [ui] ui/parser/variadic-ffi-semantic-restrictions.rs stdout ----
diff of stderr:

1 error: only foreign or `unsafe extern "C" functions may be C-variadic
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:5:19
3    |
3    |
4 LL | fn f1_1(x: isize, ...) {}

6 
7 error: C-variadic function must be declared with at least one named argument
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:8:9
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:8:9
+   --> $DIR/variadic-ffi-semantic-restrictions.rs:9:9
9    |
10 LL | fn f1_2(...) {}

12 
12 
13 error: only foreign or `unsafe extern "C" functions may be C-variadic
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:8:9
15    |
15    |
16 LL | fn f1_2(...) {}

18 
18 
19 error: only foreign or `unsafe extern "C" functions may be C-variadic
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:12:30
21    |
21    |
22 LL | extern "C" fn f2_1(x: isize, ...) {}

24 
25 error: C-variadic function must be declared with at least one named argument
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:15:20
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:15:20
+   --> $DIR/variadic-ffi-semantic-restrictions.rs:16:20
27    |
28 LL | extern "C" fn f2_2(...) {}

30 
30 
31 error: only foreign or `unsafe extern "C" functions may be C-variadic
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:15:20
33    |
33    |
34 LL | extern "C" fn f2_2(...) {}

36 
36 
37 error: `...` must be the last argument of a C-variadic function
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:19:20
39    |
39    |
40 LL | extern "C" fn f2_3(..., x: isize) {}

42 
42 
43 error: only foreign or `unsafe extern "C" functions may be C-variadic
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:19:20
45    |
45    |
46 LL | extern "C" fn f2_3(..., x: isize) {}

48 
48 
49 error: only foreign or `unsafe extern "C" functions may be C-variadic
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:23:30
51    |
51    |
52 LL | extern "C" fn f3_1(x: isize, ...) {}

54 
55 error: C-variadic function must be declared with at least one named argument
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:26:20
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:26:20
+   --> $DIR/variadic-ffi-semantic-restrictions.rs:27:20
57    |
58 LL | extern "C" fn f3_2(...) {}

60 
60 
61 error: only foreign or `unsafe extern "C" functions may be C-variadic
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:26:20
63    |
63    |
64 LL | extern "C" fn f3_2(...) {}

66 
66 
67 error: `...` must be the last argument of a C-variadic function
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:30:20
69    |
69    |
70 LL | extern "C" fn f3_3(..., x: isize) {}

72 
72 
73 error: only foreign or `unsafe extern "C" functions may be C-variadic
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:30:20
75    |
75    |
76 LL | extern "C" fn f3_3(..., x: isize) {}

78 
79 error: C-variadic function must be declared with at least one named argument
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:35:13
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:35:13
+   --> $DIR/variadic-ffi-semantic-restrictions.rs:36:13
81    |
82 LL |     fn e_f1(...);

84 
84 
85 error: `...` must be the last argument of a C-variadic function
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:37:13
87    |
87    |
88 LL |     fn e_f2(..., x: isize);

90 
90 
91 error: only foreign or `unsafe extern "C" functions may be C-variadic
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:44:23
93    |
93    |
94 LL |     fn i_f1(x: isize, ...) {}

96 
97 error: C-variadic function must be declared with at least one named argument
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:46:13
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:46:13
+   --> $DIR/variadic-ffi-semantic-restrictions.rs:47:13
99    |
100 LL |     fn i_f2(...) {}

102 
102 
103 error: only foreign or `unsafe extern "C" functions may be C-variadic
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:46:13
105    |
105    |
106 LL |     fn i_f2(...) {}

108 
108 
109 error: `...` must be the last argument of a C-variadic function
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:49:13
111    |
111    |
112 LL |     fn i_f3(..., x: isize, ...) {}

114 
114 
115 error: only foreign or `unsafe extern "C" functions may be C-variadic
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:49:13
117    |
117    |
118 LL |     fn i_f3(..., x: isize, ...) {}

120 
120 
121 error: only foreign or `unsafe extern "C" functions may be C-variadic
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:49:28
123    |
123    |
124 LL |     fn i_f3(..., x: isize, ...) {}

126 
126 
127 error: `...` must be the last argument of a C-variadic function
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:53:13
129    |
129    |
130 LL |     fn i_f4(..., x: isize, ...) {}

132 
132 
133 error: only foreign or `unsafe extern "C" functions may be C-variadic
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:53:13
135    |
135    |
136 LL |     fn i_f4(..., x: isize, ...) {}

138 
138 
139 error: only foreign or `unsafe extern "C" functions may be C-variadic
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:53:28
141    |
141    |
142 LL |     fn i_f4(..., x: isize, ...) {}

144 
144 
145 error: only foreign or `unsafe extern "C" functions may be C-variadic
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:60:23
147    |
147    |
148 LL |     fn t_f1(x: isize, ...) {}

150 
150 
151 error: only foreign or `unsafe extern "C" functions may be C-variadic
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:62:23
153    |
153    |
154 LL |     fn t_f2(x: isize, ...);

156 
157 error: C-variadic function must be declared with at least one named argument
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:64:13
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:64:13
+   --> $DIR/variadic-ffi-semantic-restrictions.rs:65:13
159    |
160 LL |     fn t_f3(...) {}

162 
162 
163 error: only foreign or `unsafe extern "C" functions may be C-variadic
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:64:13
165    |
165    |
166 LL |     fn t_f3(...) {}

168 
169 error: C-variadic function must be declared with at least one named argument
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:67:13
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:67:13
+   --> $DIR/variadic-ffi-semantic-restrictions.rs:68:13
171    |
172 LL |     fn t_f4(...);

174 
174 
175 error: only foreign or `unsafe extern "C" functions may be C-variadic
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:67:13
177    |
177    |
178 LL |     fn t_f4(...);

180 
180 
181 error: `...` must be the last argument of a C-variadic function
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:70:13
183    |
183    |
184 LL |     fn t_f5(..., x: isize) {}

186 
186 
187 error: only foreign or `unsafe extern "C" functions may be C-variadic
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:70:13
189    |
189    |
190 LL |     fn t_f5(..., x: isize) {}

192 
192 
193 error: `...` must be the last argument of a C-variadic function
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:73:13
195    |
195    |
196 LL |     fn t_f6(..., x: isize);

198 
198 
199 error: only foreign or `unsafe extern "C" functions may be C-variadic
-   --> $DIR/variadic-ffi-semantic-restrictions.rs:73:13
201    |
201    |
202 LL |     fn t_f6(..., x: isize);


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/variadic-ffi-semantic-restrictions/variadic-ffi-semantic-restrictions.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/variadic-ffi-semantic-restrictions/variadic-ffi-semantic-restrictions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/variadic-ffi-semantic-restrictions.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/variadic-ffi-semantic-restrictions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/variadic-ffi-semantic-restrictions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/variadic-ffi-semantic-restrictions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL | fn f1_1(x: isize, ...) {}

error: C-variadic function must be declared with at least one named argument
  --> /checkout/src/test/ui/parser/variadic-ffi-semantic-restrictions.rs:9:9
   |
   |
LL | fn f1_2(...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL | fn f1_2(...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL | extern "C" fn f2_1(x: isize, ...) {}

error: C-variadic function must be declared with at least one named argument
  --> /checkout/src/test/ui/parser/variadic-ffi-semantic-restrictions.rs:16:20
   |
   |
LL | extern "C" fn f2_2(...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL | extern "C" fn f2_2(...) {}


error: `...` must be the last argument of a C-variadic function
   |
   |
LL | extern "C" fn f2_3(..., x: isize) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL | extern "C" fn f2_3(..., x: isize) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL | extern "C" fn f3_1(x: isize, ...) {}

error: C-variadic function must be declared with at least one named argument
  --> /checkout/src/test/ui/parser/variadic-ffi-semantic-restrictions.rs:27:20
   |
   |
LL | extern "C" fn f3_2(...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL | extern "C" fn f3_2(...) {}


error: `...` must be the last argument of a C-variadic function
   |
   |
LL | extern "C" fn f3_3(..., x: isize) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL | extern "C" fn f3_3(..., x: isize) {}

error: C-variadic function must be declared with at least one named argument
  --> /checkout/src/test/ui/parser/variadic-ffi-semantic-restrictions.rs:36:13
   |
   |
LL |     fn e_f1(...);


error: `...` must be the last argument of a C-variadic function
   |
   |
LL |     fn e_f2(..., x: isize);


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn i_f1(x: isize, ...) {}

error: C-variadic function must be declared with at least one named argument
  --> /checkout/src/test/ui/parser/variadic-ffi-semantic-restrictions.rs:47:13
   |
   |
LL |     fn i_f2(...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn i_f2(...) {}


error: `...` must be the last argument of a C-variadic function
   |
   |
LL |     fn i_f3(..., x: isize, ...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn i_f3(..., x: isize, ...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn i_f3(..., x: isize, ...) {}


error: `...` must be the last argument of a C-variadic function
   |
   |
LL |     fn i_f4(..., x: isize, ...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn i_f4(..., x: isize, ...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn i_f4(..., x: isize, ...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn t_f1(x: isize, ...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn t_f2(x: isize, ...);

error: C-variadic function must be declared with at least one named argument
  --> /checkout/src/test/ui/parser/variadic-ffi-semantic-restrictions.rs:65:13
   |
   |
LL |     fn t_f3(...) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn t_f3(...) {}

error: C-variadic function must be declared with at least one named argument
  --> /checkout/src/test/ui/parser/variadic-ffi-semantic-restrictions.rs:68:13
   |
   |
LL |     fn t_f4(...);


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn t_f4(...);


error: `...` must be the last argument of a C-variadic function
   |
   |
LL |     fn t_f5(..., x: isize) {}


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn t_f5(..., x: isize) {}


error: `...` must be the last argument of a C-variadic function
   |
   |
LL |     fn t_f6(..., x: isize);


error: only foreign or `unsafe extern "C" functions may be C-variadic
   |
   |
LL |     fn t_f6(..., x: isize);

error: aborting due to 34 previous errors



------------------------------------------


---

---- [ui] ui/rfc-2565-param-attrs/proc-macro-cannot-be-used.rs stdout ----
diff of stderr:

1 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:10:23
3    |
3    |
4 LL | extern "C" { fn ffi(#[id] arg1: i32, #[id] ...); }
5    |                       ^^ not a non-macro attribute
6 
6 
7 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:10:40
9    |
9    |
10 LL | extern "C" { fn ffi(#[id] arg1: i32, #[id] ...); }
11    |                                        ^^ not a non-macro attribute
12 
12 
13 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:14:40
15    |
15    |
16 LL | unsafe extern "C" fn cvar(arg1: i32, #[id] mut args: ...) {}
17    |                                        ^^ not a non-macro attribute
18 
18 
19 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:17:30
21    |
21    |
22 LL | type Alias = extern "C" fn(#[id] u8, #[id] ...);
23    |                              ^^ not a non-macro attribute
24 
24 
25 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:17:40
27    |
27    |
28 LL | type Alias = extern "C" fn(#[id] u8, #[id] ...);
29    |                                        ^^ not a non-macro attribute
30 
30 
31 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:21:11
33    |
33    |
34 LL | fn free(#[id] arg1: u8) {
35    |           ^^ not a non-macro attribute
36 
36 
37 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:23:18
39    |
39    |
40 LL |     let lam = |#[id] W(x), #[id] y: usize| ();
41    |                  ^^ not a non-macro attribute
42 
42 
43 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:23:30
45    |
45    |
46 LL |     let lam = |#[id] W(x), #[id] y: usize| ();
47    |                              ^^ not a non-macro attribute
48 
48 
49 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:29:20
51    |
51    |
52 LL |     fn inherent1(#[id] self, #[id] arg1: u8) {}
53    |                    ^^ not a non-macro attribute
54 
54 
55 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:29:32
57    |
57    |
58 LL |     fn inherent1(#[id] self, #[id] arg1: u8) {}
59    |                                ^^ not a non-macro attribute
60 
60 
61 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:32:20
63    |
63    |
64 LL |     fn inherent2(#[id] &self, #[id] arg1: u8) {}
65    |                    ^^ not a non-macro attribute
66 
66 
67 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:32:33
69    |
69    |
70 LL |     fn inherent2(#[id] &self, #[id] arg1: u8) {}
71    |                                 ^^ not a non-macro attribute
72 
72 
73 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:35:24
75    |
75    |
76 LL |     fn inherent3<'a>(#[id] &'a mut self, #[id] arg1: u8) {}
77    |                        ^^ not a non-macro attribute
78 
78 
79 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:35:44
81    |
81    |
82 LL |     fn inherent3<'a>(#[id] &'a mut self, #[id] arg1: u8) {}
83    |                                            ^^ not a non-macro attribute
84 
84 
85 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:38:24
87    |
87    |
88 LL |     fn inherent4<'a>(#[id] self: Box<Self>, #[id] arg1: u8) {}
89    |                        ^^ not a non-macro attribute
90 
90 
91 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:38:47
93    |
93    |
94 LL |     fn inherent4<'a>(#[id] self: Box<Self>, #[id] arg1: u8) {}
95    |                                               ^^ not a non-macro attribute
96 
96 
97 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:41:40
99    |
99    |
100 LL |     fn issue_64682_associated_fn<'a>(#[id] arg1: u8, #[id] arg2: u8) {}
101    |                                        ^^ not a non-macro attribute
102 
102 
103 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:41:56
105    |
105    |
106 LL |     fn issue_64682_associated_fn<'a>(#[id] arg1: u8, #[id] arg2: u8) {}
107    |                                                        ^^ not a non-macro attribute
108 
108 
109 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:47:17
111    |
111    |
112 LL |     fn trait1(#[id] self, #[id] arg1: u8);
113    |                 ^^ not a non-macro attribute
114 
114 
115 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:47:29
117    |
117    |
118 LL |     fn trait1(#[id] self, #[id] arg1: u8);
119    |                             ^^ not a non-macro attribute
120 
120 
121 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:50:17
123    |
123    |
124 LL |     fn trait2(#[id] &self, #[id] arg1: u8);
125    |                 ^^ not a non-macro attribute
126 
126 
127 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:50:30
129    |
129    |
130 LL |     fn trait2(#[id] &self, #[id] arg1: u8);
131    |                              ^^ not a non-macro attribute
132 
132 
133 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:53:21
135    |
135    |
136 LL |     fn trait3<'a>(#[id] &'a mut self, #[id] arg1: u8);
137    |                     ^^ not a non-macro attribute
138 
138 
139 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:53:41
141    |
141    |
142 LL |     fn trait3<'a>(#[id] &'a mut self, #[id] arg1: u8);
143    |                                         ^^ not a non-macro attribute
144 
144 
145 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:56:21
147    |
147    |
148 LL |     fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
149    |                     ^^ not a non-macro attribute
150 
150 
151 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:56:44
153    |
153    |
154 LL |     fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
155    |                                            ^^ not a non-macro attribute
156 
156 
157 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:56:60
159    |
159    |
160 LL |     fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
161    |                                                            ^^ not a non-macro attribute
162 
162 
163 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:60:40
165    |
165    |
166 LL |     fn issue_64682_associated_fn<'a>(#[id] arg1: u8, #[id] arg2: u8);
167    |                                        ^^ not a non-macro attribute
168 
168 
169 error: expected non-macro attribute, found attribute macro `id`
-   --> $DIR/proc-macro-cannot-be-used.rs:60:56
171    |
171    |
172 LL |     fn issue_64682_associated_fn<'a>(#[id] arg1: u8, #[id] arg2: u8);
173    |                                                        ^^ not a non-macro attribute

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/proc-macro-cannot-be-used/proc-macro-cannot-be-used.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/proc-macro-cannot-be-used/proc-macro-cannot-be-used.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2565-param-attrs/proc-macro-cannot-be-used.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2565-param-attrs/proc-macro-cannot-be-used.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/proc-macro-cannot-be-used" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/proc-macro-cannot-be-used/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL | extern "C" { fn ffi(#[id] arg1: i32, #[id] ...); }
   |                       ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL | extern "C" { fn ffi(#[id] arg1: i32, #[id] ...); }
   |                                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL | unsafe extern "C" fn cvar(arg1: i32, #[id] mut args: ...) {}
   |                                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL | type Alias = extern "C" fn(#[id] u8, #[id] ...);
   |                              ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL | type Alias = extern "C" fn(#[id] u8, #[id] ...);
   |                                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL | fn free(#[id] arg1: u8) {
   |           ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     let lam = |#[id] W(x), #[id] y: usize| ();
   |                  ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     let lam = |#[id] W(x), #[id] y: usize| ();
   |                              ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent1(#[id] self, #[id] arg1: u8) {}
   |                    ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent1(#[id] self, #[id] arg1: u8) {}
   |                                ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent2(#[id] &self, #[id] arg1: u8) {}
   |                    ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent2(#[id] &self, #[id] arg1: u8) {}
   |                                 ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent3<'a>(#[id] &'a mut self, #[id] arg1: u8) {}
   |                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent3<'a>(#[id] &'a mut self, #[id] arg1: u8) {}
   |                                            ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent4<'a>(#[id] self: Box<Self>, #[id] arg1: u8) {}
   |                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn inherent4<'a>(#[id] self: Box<Self>, #[id] arg1: u8) {}
   |                                               ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn issue_64682_associated_fn<'a>(#[id] arg1: u8, #[id] arg2: u8) {}
   |                                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn issue_64682_associated_fn<'a>(#[id] arg1: u8, #[id] arg2: u8) {}
   |                                                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait1(#[id] self, #[id] arg1: u8);
   |                 ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait1(#[id] self, #[id] arg1: u8);
   |                             ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait2(#[id] &self, #[id] arg1: u8);
   |                 ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait2(#[id] &self, #[id] arg1: u8);
   |                              ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait3<'a>(#[id] &'a mut self, #[id] arg1: u8);
   |                     ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait3<'a>(#[id] &'a mut self, #[id] arg1: u8);
   |                                         ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
   |                     ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
   |                                            ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
   |                                                            ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn issue_64682_associated_fn<'a>(#[id] arg1: u8, #[id] arg2: u8);
   |                                        ^^ not a non-macro attribute

error: expected non-macro attribute, found attribute macro `id`
   |
   |
LL |     fn issue_64682_associated_fn<'a>(#[id] arg1: u8, #[id] arg2: u8);
   |                                                        ^^ not a non-macro attribute
error: aborting due to 29 previous errors


------------------------------------------
---
test result: FAILED. 11438 passed; 5 failed; 93 ignored; 0 measured; 0 filtered out; finished in 130.24s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:48
