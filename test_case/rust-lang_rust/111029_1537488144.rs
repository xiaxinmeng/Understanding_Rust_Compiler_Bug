plain
........................................................................................  8624/14946
................ii......................................................................  8712/14946
........................................................................................  8800/14946
........................................................................................  8888/14946
...........F.......FF..F......F......F.F.F...............F..........ii.................i  8976/14946
......i..i.............................................................................i  9064/14946
........................................................................................  9240/14946
........................................................................................  9328/14946
........................................................................................  9416/14946
.......................................................i..i.............................  9504/14946
---

6    |     |
7    |     expected due to this
8    |
-    = note: expected closure signature `for<'a, 'b> fn(&'a (), &'b ()) -> _`
-               found closure signature `fn((), ()) -> _`
+    = note: expected closure signature `fn((), ()) -> _`
+               found closure signature `for<'a, 'b> fn(&'a (), &'b ()) -> _`
11 note: required by a bound in `f1`
13    |


14 LL | fn f1<F>(_: F) where F: Fn(&(), &()) {}
15    |                         ^^^^^^^^^^^^ required by this bound in `f1`
- help: consider borrowing the argument
+ help: do not borrow the argument
17    |
- LL |     f1(|_: &(), _: &()| {});
-    |            +       +
+ LL |     f1(|_: (), _: ()| {});
20 
21 error[E0631]: type mismatch in closure arguments
22   --> $DIR/anonymous-higher-ranked-lifetime.rs:3:5


26    |     |
27    |     expected due to this
28    |
-    = note: expected closure signature `for<'a, 'b> fn(&'a (), &'b ()) -> _`
-               found closure signature `fn((), ()) -> _`
+    = note: expected closure signature `fn((), ()) -> _`
+               found closure signature `for<'a, 'b> fn(&'a (), &'b ()) -> _`
31 note: required by a bound in `f2`
33    |


34 LL | fn f2<F>(_: F) where F: for<'a> Fn(&'a (), &()) {}
35    |                         ^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `f2`
- help: consider borrowing the argument
+ help: do not borrow the argument
37    |
- LL |     f2(|_: &(), _: &()| {});
-    |            +       +
+ LL |     f2(|_: (), _: ()| {});
40 
41 error[E0631]: type mismatch in closure arguments
42   --> $DIR/anonymous-higher-ranked-lifetime.rs:4:5


46    |     |
47    |     expected due to this
48    |
-    = note: expected closure signature `for<'a> fn(&(), &'a ()) -> _`
-               found closure signature `fn((), ()) -> _`
+    = note: expected closure signature `fn((), ()) -> _`
+               found closure signature `for<'a> fn(&(), &'a ()) -> _`
51 note: required by a bound in `f3`
53    |


54 LL | fn f3<'a, F>(_: F) where F: Fn(&'a (), &()) {}
55    |                             ^^^^^^^^^^^^^^^ required by this bound in `f3`
- help: consider borrowing the argument
+ help: do not borrow the argument
57    |
- LL |     f3(|_: &(), _: &()| {});
-    |            +       +
+ LL |     f3(|_: (), _: ()| {});
60 
61 error[E0631]: type mismatch in closure arguments
62   --> $DIR/anonymous-higher-ranked-lifetime.rs:5:5


66    |     |
67    |     expected due to this
68    |
-    = note: expected closure signature `for<'r, 'a> fn(&'a (), &'r ()) -> _`
-               found closure signature `fn((), ()) -> _`
+    = note: expected closure signature `fn((), ()) -> _`
+               found closure signature `for<'r, 'a> fn(&'a (), &'r ()) -> _`
71 note: required by a bound in `f4`
73    |


74 LL | fn f4<F>(_: F) where F: for<'r> Fn(&(), &'r ()) {}
75    |                         ^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `f4`
- help: consider borrowing the argument
+ help: do not borrow the argument
77    |
- LL |     f4(|_: &(), _: &()| {});
-    |            +       +
+ LL |     f4(|_: (), _: ()| {});
80 
81 error[E0631]: type mismatch in closure arguments
82   --> $DIR/anonymous-higher-ranked-lifetime.rs:6:5


86    |     |
87    |     expected due to this
88    |
-    = note: expected closure signature `for<'r> fn(&'r (), &'r ()) -> _`
-               found closure signature `fn((), ()) -> _`
+    = note: expected closure signature `fn((), ()) -> _`
+               found closure signature `for<'r> fn(&'r (), &'r ()) -> _`
91 note: required by a bound in `f5`
93    |


94 LL | fn f5<F>(_: F) where F: for<'r> Fn(&'r (), &'r ()) {}
95    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `f5`
- help: consider borrowing the argument
+ help: do not borrow the argument
97    |
- LL |     f5(|_: &(), _: &()| {});
-    |            +       +
+ LL |     f5(|_: (), _: ()| {});
100 
101 error[E0631]: type mismatch in closure arguments
102   --> $DIR/anonymous-higher-ranked-lifetime.rs:7:5


106    |     |
107    |     expected due to this
108    |
-    = note: expected closure signature `for<'a> fn(&'a (), Box<(dyn for<'a> Fn(&'a ()) + 'static)>) -> _`
-               found closure signature `fn((), ()) -> _`
+    = note: expected closure signature `fn((), ()) -> _`
+               found closure signature `for<'a> fn(&'a (), Box<(dyn for<'a> Fn(&'a ()) + 'static)>) -> _`
111 note: required by a bound in `g1`
113    |


114 LL | fn g1<F>(_: F) where F: Fn(&(), Box<dyn Fn(&())>) {}
115    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `g1`
- help: consider borrowing the argument
+ help: do not borrow the argument
117    |
- LL |     g1(|_: &(), _: ()| {});
-    |            +
+ LL |     g1(|_: (), _: ()| {});
120 
121 error[E0631]: type mismatch in closure arguments
122   --> $DIR/anonymous-higher-ranked-lifetime.rs:8:5


126    |     |
127    |     expected due to this
128    |
-    = note: expected closure signature `for<'a> fn(&'a (), for<'a> fn(&'a ())) -> _`
-               found closure signature `fn((), ()) -> _`
+    = note: expected closure signature `fn((), ()) -> _`
+               found closure signature `for<'a> fn(&'a (), for<'a> fn(&'a ())) -> _`
131 note: required by a bound in `g2`
133    |


134 LL | fn g2<F>(_: F) where F: Fn(&(), fn(&())) {}
135    |                         ^^^^^^^^^^^^^^^^ required by this bound in `g2`
- help: consider borrowing the argument
+ help: do not borrow the argument
137    |
- LL |     g2(|_: &(), _: ()| {});
-    |            +
+ LL |     g2(|_: (), _: ()| {});
140 
141 error[E0631]: type mismatch in closure arguments
142   --> $DIR/anonymous-higher-ranked-lifetime.rs:9:5


146    |     |
147    |     expected due to this
148    |
-    = note: expected closure signature `for<'s> fn(&'s (), Box<(dyn for<'a> Fn(&'a ()) + 'static)>) -> _`
-               found closure signature `fn((), ()) -> _`
+    = note: expected closure signature `fn((), ()) -> _`
+               found closure signature `for<'s> fn(&'s (), Box<(dyn for<'a> Fn(&'a ()) + 'static)>) -> _`
151 note: required by a bound in `g3`
153    |


154 LL | fn g3<F>(_: F) where F: for<'s> Fn(&'s (), Box<dyn Fn(&())>) {}
155    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `g3`
- help: consider borrowing the argument
+ help: do not borrow the argument
157    |
- LL |     g3(|_: &(), _: ()| {});
-    |            +
+ LL |     g3(|_: (), _: ()| {});
160 
161 error[E0631]: type mismatch in closure arguments
162   --> $DIR/anonymous-higher-ranked-lifetime.rs:10:5


166    |     |
167    |     expected due to this
168    |
-    = note: expected closure signature `for<'a> fn(&'a (), for<'r> fn(&'r ())) -> _`
-               found closure signature `fn((), ()) -> _`
+    = note: expected closure signature `fn((), ()) -> _`
+               found closure signature `for<'a> fn(&'a (), for<'r> fn(&'r ())) -> _`
171 note: required by a bound in `g4`
173    |


174 LL | fn g4<F>(_: F) where F: Fn(&(), for<'r> fn(&'r ())) {}
175    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `g4`
- help: consider borrowing the argument
+ help: do not borrow the argument
177    |
- LL |     g4(|_: &(), _: ()| {});
-    |            +
+ LL |     g4(|_: (), _: ()| {});
180 
181 error[E0631]: type mismatch in closure arguments
182   --> $DIR/anonymous-higher-ranked-lifetime.rs:11:5


186    |     |
187    |     expected due to this
188    |
-    = note: expected closure signature `for<'a, 'b> fn(&'a (), Box<(dyn for<'a> Fn(&'a ()) + 'static)>, &'b (), for<'a, 'b> fn(&'a (), &'b ())) -> _`
-               found closure signature `fn((), (), (), ()) -> _`
+    = note: expected closure signature `fn((), (), (), ()) -> _`
+               found closure signature `for<'a, 'b> fn(&'a (), Box<(dyn for<'a> Fn(&'a ()) + 'static)>, &'b (), for<'a, 'b> fn(&'a (), &'b ())) -> _`
191 note: required by a bound in `h1`
193    |


194 LL | fn h1<F>(_: F) where F: Fn(&(), Box<dyn Fn(&())>, &(), fn(&(), &())) {}
195    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `h1`
- help: consider borrowing the argument
+ help: do not borrow the argument
197    |
- LL |     h1(|_: &(), _: (), _: &(), _: ()| {});
-    |            +              +
+ LL |     h1(|_: (), _: (), _: (), _: ()| {});
200 
201 error[E0631]: type mismatch in closure arguments
202   --> $DIR/anonymous-higher-ranked-lifetime.rs:12:5


206    |     |
207    |     expected due to this
208    |
-    = note: expected closure signature `for<'t0, 'a> fn(&'a (), Box<(dyn for<'a> Fn(&'a ()) + 'static)>, &'t0 (), for<'a, 'b> fn(&'a (), &'b ())) -> _`
-               found closure signature `fn((), (), (), ()) -> _`
+    = note: expected closure signature `fn((), (), (), ()) -> _`
+               found closure signature `for<'t0, 'a> fn(&'a (), Box<(dyn for<'a> Fn(&'a ()) + 'static)>, &'t0 (), for<'a, 'b> fn(&'a (), &'b ())) -> _`
211 note: required by a bound in `h2`
213    |


214 LL | fn h2<F>(_: F) where F: for<'t0> Fn(&(), Box<dyn Fn(&())>, &'t0 (), fn(&(), &())) {}
215    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `h2`
- help: consider borrowing the argument
+ help: do not borrow the argument
217    |
- LL |     h2(|_: &(), _: (), _: &(), _: ()| {});
-    |            +              +
+ LL |     h2(|_: (), _: (), _: (), _: ()| {});
220 
221 error: aborting due to 11 previous errors
222 

---
To only update this specific test, also pass `--test-args anonymous-higher-ranked-lifetime.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/anonymous-higher-ranked-lifetime.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anonymous-higher-ranked-lifetime" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anonymous-higher-ranked-lifetime/auxiliary"
stdout: none
error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/anonymous-higher-ranked-lifetime.rs:2:5
   |
   |
LL |     f1(|_: (), _: ()| {}); //~ ERROR type mismatch
   |     ^^ -------------- found signature defined here
   |     expected due to this
   |
   |
   = note: expected closure signature `fn((), ()) -> _`
              found closure signature `for<'a, 'b> fn(&'a (), &'b ()) -> _`
note: required by a bound in `f1`
  --> fake-test-src-base/anonymous-higher-ranked-lifetime.rs:16:25
   |
LL | fn f1<F>(_: F) where F: Fn(&(), &()) {}
   |                         ^^^^^^^^^^^^ required by this bound in `f1`
help: do not borrow the argument
   |
LL |     f1(|_: (), _: ()| {}); //~ ERROR type mismatch

error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/anonymous-higher-ranked-lifetime.rs:3:5
   |
   |
LL |     f2(|_: (), _: ()| {}); //~ ERROR type mismatch
   |     ^^ -------------- found signature defined here
   |     expected due to this
   |
   |
   = note: expected closure signature `fn((), ()) -> _`
              found closure signature `for<'a, 'b> fn(&'a (), &'b ()) -> _`
note: required by a bound in `f2`
  --> fake-test-src-base/anonymous-higher-ranked-lifetime.rs:17:25
   |
LL | fn f2<F>(_: F) where F: for<'a> Fn(&'a (), &()) {}
   |                         ^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `f2`
help: do not borrow the argument
   |
LL |     f2(|_: (), _: ()| {}); //~ ERROR type mismatch

error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/anonymous-higher-ranked-lifetime.rs:4:5
   |
   |
LL |     f3(|_: (), _: ()| {}); //~ ERROR type mismatch
   |     ^^ -------------- found signature defined here
   |     expected due to this
   |
   |
   = note: expected closure signature `fn((), ()) -> _`
              found closure signature `for<'a> fn(&(), &'a ()) -> _`
note: required by a bound in `f3`
  --> fake-test-src-base/anonymous-higher-ranked-lifetime.rs:18:29
   |
LL | fn f3<'a, F>(_: F) where F: Fn(&'a (), &()) {}
   |                             ^^^^^^^^^^^^^^^ required by this bound in `f3`
help: do not borrow the argument
   |
LL |     f3(|_: (), _: ()| {}); //~ ERROR type mismatch

error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/anonymous-higher-ranked-lifetime.rs:5:5
   |
   |
LL |     f4(|_: (), _: ()| {}); //~ ERROR type mismatch
   |     ^^ -------------- found signature defined here
   |     expected due to this
   |
   |
   = note: expected closure signature `fn((), ()) -> _`
              found closure signature `for<'r, 'a> fn(&'a (), &'r ()) -> _`
note: required by a bound in `f4`
  --> fake-test-src-base/anonymous-higher-ranked-lifetime.rs:19:25
   |
LL | fn f4<F>(_: F) where F: for<'r> Fn(&(), &'r ()) {}
   |                         ^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `f4`
help: do not borrow the argument
   |
LL |     f4(|_: (), _: ()| {}); //~ ERROR type mismatch

error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/anonymous-higher-ranked-lifetime.rs:6:5
   |
   |
LL |     f5(|_: (), _: ()| {}); //~ ERROR type mismatch
   |     ^^ -------------- found signature defined here
   |     expected due to this
   |
   |
   = note: expected closure signature `fn((), ()) -> _`
              found closure signature `for<'r> fn(&'r (), &'r ()) -> _`
note: required by a bound in `f5`
  --> fake-test-src-base/anonymous-higher-ranked-lifetime.rs:20:25
   |
LL | fn f5<F>(_: F) where F: for<'r> Fn(&'r (), &'r ()) {}
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `f5`
help: do not borrow the argument
   |
LL |     f5(|_: (), _: ()| {}); //~ ERROR type mismatch

error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/anonymous-higher-ranked-lifetime.rs:7:5
   |
   |
LL |     g1(|_: (), _: ()| {}); //~ ERROR type mismatch
   |     ^^ -------------- found signature defined here
   |     expected due to this
   |
   |
   = note: expected closure signature `fn((), ()) -> _`
              found closure signature `for<'a> fn(&'a (), Box<(dyn for<'a> Fn(&'a ()) + 'static)>) -> _`
note: required by a bound in `g1`
  --> fake-test-src-base/anonymous-higher-ranked-lifetime.rs:23:25
   |
LL | fn g1<F>(_: F) where F: Fn(&(), Box<dyn Fn(&())>) {}
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `g1`
help: do not borrow the argument
   |
LL |     g1(|_: (), _: ()| {}); //~ ERROR type mismatch

error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/anonymous-higher-ranked-lifetime.rs:8:5
   |
   |
LL |     g2(|_: (), _: ()| {}); //~ ERROR type mismatch
   |     ^^ -------------- found signature defined here
   |     expected due to this
   |
   |
   = note: expected closure signature `fn((), ()) -> _`
              found closure signature `for<'a> fn(&'a (), for<'a> fn(&'a ())) -> _`
note: required by a bound in `g2`
  --> fake-test-src-base/anonymous-higher-ranked-lifetime.rs:24:25
   |
LL | fn g2<F>(_: F) where F: Fn(&(), fn(&())) {}
   |                         ^^^^^^^^^^^^^^^^ required by this bound in `g2`
help: do not borrow the argument
   |
LL |     g2(|_: (), _: ()| {}); //~ ERROR type mismatch

error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/anonymous-higher-ranked-lifetime.rs:9:5
   |
   |
LL |     g3(|_: (), _: ()| {}); //~ ERROR type mismatch
   |     ^^ -------------- found signature defined here
   |     expected due to this
   |
   |
   = note: expected closure signature `fn((), ()) -> _`
              found closure signature `for<'s> fn(&'s (), Box<(dyn for<'a> Fn(&'a ()) + 'static)>) -> _`
note: required by a bound in `g3`
  --> fake-test-src-base/anonymous-higher-ranked-lifetime.rs:25:25
   |
LL | fn g3<F>(_: F) where F: for<'s> Fn(&'s (), Box<dyn Fn(&())>) {}
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `g3`
help: do not borrow the argument
   |
LL |     g3(|_: (), _: ()| {}); //~ ERROR type mismatch

error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/anonymous-higher-ranked-lifetime.rs:10:5
   |
   |
LL |     g4(|_: (), _: ()| {}); //~ ERROR type mismatch
   |     ^^ -------------- found signature defined here
   |     expected due to this
   |
   |
   = note: expected closure signature `fn((), ()) -> _`
              found closure signature `for<'a> fn(&'a (), for<'r> fn(&'r ())) -> _`
note: required by a bound in `g4`
  --> fake-test-src-base/anonymous-higher-ranked-lifetime.rs:26:25
   |
LL | fn g4<F>(_: F) where F: Fn(&(), for<'r> fn(&'r ())) {}
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `g4`
help: do not borrow the argument
   |
LL |     g4(|_: (), _: ()| {}); //~ ERROR type mismatch

error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/anonymous-higher-ranked-lifetime.rs:11:5
   |
   |
LL |     h1(|_: (), _: (), _: (), _: ()| {}); //~ ERROR type mismatch
   |     ^^ ---------------------------- found signature defined here
   |     expected due to this
   |
   |
   = note: expected closure signature `fn((), (), (), ()) -> _`
              found closure signature `for<'a, 'b> fn(&'a (), Box<(dyn for<'a> Fn(&'a ()) + 'static)>, &'b (), for<'a, 'b> fn(&'a (), &'b ())) -> _`
note: required by a bound in `h1`
  --> fake-test-src-base/anonymous-higher-ranked-lifetime.rs:29:25
   |
LL | fn h1<F>(_: F) where F: Fn(&(), Box<dyn Fn(&())>, &(), fn(&(), &())) {}
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `h1`
help: do not borrow the argument
   |
LL |     h1(|_: (), _: (), _: (), _: ()| {}); //~ ERROR type mismatch

error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/anonymous-higher-ranked-lifetime.rs:12:5
   |
   |
LL |     h2(|_: (), _: (), _: (), _: ()| {}); //~ ERROR type mismatch
   |     ^^ ---------------------------- found signature defined here
   |     expected due to this
   |
   |
   = note: expected closure signature `fn((), (), (), ()) -> _`
              found closure signature `for<'t0, 'a> fn(&'a (), Box<(dyn for<'a> Fn(&'a ()) + 'static)>, &'t0 (), for<'a, 'b> fn(&'a (), &'b ())) -> _`
note: required by a bound in `h2`
  --> fake-test-src-base/anonymous-higher-ranked-lifetime.rs:30:25
   |
LL | fn h2<F>(_: F) where F: for<'t0> Fn(&(), Box<dyn Fn(&())>, &'t0 (), fn(&(), &())) {}
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `h2`
help: do not borrow the argument
   |
LL |     h2(|_: (), _: (), _: (), _: ()| {}); //~ ERROR type mismatch

error: aborting due to 11 previous errors

For more information about this error, try `rustc --explain E0631`.
---

6    |     |
7    |     expected due to this
8    |
-    = note: expected closure signature `fn(_, _) -> _`
-               found closure signature `fn(u32, i32) -> _`
+    = note: expected closure signature `fn(u32, i32) -> _`
+               found closure signature `fn(_, _) -> _`
11 note: required by a bound in `with_closure`
13    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closure-expected-type/expect-infer-var-appearing-twice/expect-infer-var-appearing-twice.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closure-expected-type/expect-infer-var-appearing-twice.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/closure-expected-type/expect-infer-var-appearing-twice.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closure-expected-type/expect-infer-var-appearing-twice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closure-expected-type/expect-infer-var-appearing-twice/auxiliary"
stdout: none
error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/closure-expected-type/expect-infer-var-appearing-twice.rs:14:5
   |
   |
LL |     with_closure(|x: u32, y: i32| {
   |     |
   |     expected due to this
   |
   |
   = note: expected closure signature `fn(u32, i32) -> _`
              found closure signature `fn(_, _) -> _`
note: required by a bound in `with_closure`
  --> fake-test-src-base/closure-expected-type/expect-infer-var-appearing-twice.rs:2:14
   |
LL | fn with_closure<F, A>(_: F)
   |    ------------ required by a bound in this function
LL |     where F: FnOnce(A, A)
   |              ^^^^^^^^^^^^ required by this bound in `with_closure`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0631`.
------------------------------------------
---

6    |     |
7    |     expected due to this
8    |
-    = note: expected closure signature `fn(char) -> _`
-               found closure signature `for<'a> fn(&'a char) -> _`
+    = note: expected closure signature `for<'a> fn(&'a char) -> _`
+               found closure signature `fn(char) -> _`
11 note: closure inferred to have a different signature due to this bound
13    |

18    |
18    |
19 LL | fn foo<F: Fn(&char) -> bool + Fn(char) -> bool>(f: F) {
20    |                               ^^^^^^^^^^^^^^^^ required by this bound in `foo`
- help: do not borrow the argument
+ help: consider borrowing the argument
22    |
- LL |     foo(move |char| v);
-    |               ~~~~
+ LL |     foo(move |&x| v);
25 
26 error: aborting due to previous error
27 

---
To only update this specific test, also pass `--test-args closures/multiple-fn-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/closures/multiple-fn-bounds.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/multiple-fn-bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/multiple-fn-bounds/auxiliary"
stdout: none
error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/closures/multiple-fn-bounds.rs:10:5
   |
   |
LL |     foo(move |x| v);
   |     |
   |     expected due to this
   |
   |
   = note: expected closure signature `for<'a> fn(&'a char) -> _`
              found closure signature `fn(char) -> _`
note: closure inferred to have a different signature due to this bound
  --> fake-test-src-base/closures/multiple-fn-bounds.rs:1:11
   |
LL | fn foo<F: Fn(&char) -> bool + Fn(char) -> bool>(f: F) {
note: required by a bound in `foo`
  --> fake-test-src-base/closures/multiple-fn-bounds.rs:1:31
   |
   |
LL | fn foo<F: Fn(&char) -> bool + Fn(char) -> bool>(f: F) {
   |                               ^^^^^^^^^^^^^^^^ required by this bound in `foo`
   |
   |
LL |     foo(move |&x| v);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0631`.
For more information about this error, try `rustc --explain E0631`.
------------------------------------------


---- [ui] tests/ui/generator/issue-88653.rs stdout ----
diff of stderr:

7 LL |     |bar| {
9    |
9    |
-    = note: expected generator signature `fn((bool,)) -> _`
-               found generator signature `fn(bool) -> _`
+    = note: expected generator signature `fn(bool) -> _`
+               found generator signature `fn((bool,)) -> _`
13 error: aborting due to previous error
14 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-88653/issue-88653.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generator/issue-88653.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/generator/issue-88653.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-88653" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-88653/auxiliary"
stdout: none
error[E0631]: type mismatch in generator arguments
  --> fake-test-src-base/generator/issue-88653.rs:8:22
   |
   |
LL | fn foo(bar: bool) -> impl Generator<(bool,)> {
...
...
LL |     |bar| {
   |
   |
   = note: expected generator signature `fn(bool) -> _`
              found generator signature `fn((bool,)) -> _`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0631`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/generic-associated-types/bugs/issue-88382.rs stdout ----
diff of stderr:

9 LL | fn test<'a, I: Iterable>(_: &mut I::Iterator<'a>) {}
11    |
11    |
-    = note: expected function signature `for<'a> fn(&'a mut std::iter::Empty<usize>) -> _`
-               found function signature `for<'a, 'b> fn(&'b mut <_ as Iterable>::Iterator<'a>) -> _`
+    = note: expected function signature `for<'a, 'b> fn(&'b mut <_ as Iterable>::Iterator<'a>) -> _`
+               found function signature `for<'a> fn(&'a mut std::iter::Empty<usize>) -> _`
14 note: required by a bound in `do_something`
16    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-88382/issue-88382.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/bugs/issue-88382.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/generic-associated-types/bugs/issue-88382.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-88382" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-88382/auxiliary"
stdout: none
error[E0631]: type mismatch in function arguments
  --> fake-test-src-base/generic-associated-types/bugs/issue-88382.rs:26:40
   |
   |
LL |     do_something(SomeImplementation(), test);
   |     |
   |     required by a bound introduced by this call
...
...
LL | fn test<'a, I: Iterable>(_: &mut I::Iterator<'a>) {}
   |
   |
   = note: expected function signature `for<'a, 'b> fn(&'b mut <_ as Iterable>::Iterator<'a>) -> _`
              found function signature `for<'a> fn(&'a mut std::iter::Empty<usize>) -> _`
note: required by a bound in `do_something`
  --> fake-test-src-base/generic-associated-types/bugs/issue-88382.rs:20:48
   |
LL | fn do_something<I: Iterable>(i: I, mut f: impl for<'a> Fn(&mut I::Iterator<'a>)) {
   |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `do_something`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0631`.
------------------------------------------
---

82    |     |
83    |     required by a bound introduced by this call
84    |
-    = note: expected function signature `fn(bool) -> _`
-               found function signature `fn(i32) -> _`
+    = note: expected function signature `fn(i32) -> _`
+               found function signature `fn(bool) -> _`
88   --> $SRC_DIR/core/src/intrinsics.rs:LL:COL
89 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/const-eval-select-bad/const-eval-select-bad.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args intrinsics/const-eval-select-bad.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/intrinsics/const-eval-select-bad.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/const-eval-select-bad" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/const-eval-select-bad/auxiliary"
stdout: none
error: this argument must be a function item
  --> fake-test-src-base/intrinsics/const-eval-select-bad.rs:7:27
   |
   |
LL |     const_eval_select((), || {}, || {});
   |
   |
   = note: expected a function item, found [closure@fake-test-src-base/intrinsics/const-eval-select-bad.rs:7:27: 7:29]
   = help: consult the documentation on `const_eval_select` for more information
error: this argument must be a function item
  --> fake-test-src-base/intrinsics/const-eval-select-bad.rs:7:34
   |
   |
LL |     const_eval_select((), || {}, || {});
   |
   |
   = note: expected a function item, found [closure@fake-test-src-base/intrinsics/const-eval-select-bad.rs:7:34: 7:36]
   = help: consult the documentation on `const_eval_select` for more information
error: this argument must be a function item
  --> fake-test-src-base/intrinsics/const-eval-select-bad.rs:10:27
   |
   |
LL |     const_eval_select((), 42, 0xDEADBEEF);
   |
   = note: expected a function item, found {integer}
   = help: consult the documentation on `const_eval_select` for more information


error[E0277]: expected a `FnOnce<()>` closure, found `{integer}`
  --> fake-test-src-base/intrinsics/const-eval-select-bad.rs:10:27
   |
LL |     const_eval_select((), 42, 0xDEADBEEF);
   |     -----------------     ^^ expected an `FnOnce<()>` closure, found `{integer}`
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `FnOnce<()>` is not implemented for `{integer}`
   = note: wrap the `{integer}` in a closure with no arguments: `|| { /* code */ }`
  --> /rustc/FAKE_PREFIX/library/core/src/intrinsics.rs:2469:5

error: this argument must be a function item
  --> fake-test-src-base/intrinsics/const-eval-select-bad.rs:10:31
  --> fake-test-src-base/intrinsics/const-eval-select-bad.rs:10:31
   |
LL |     const_eval_select((), 42, 0xDEADBEEF);
   |
   = note: expected a function item, found {integer}
   = help: consult the documentation on `const_eval_select` for more information


error[E0277]: expected a `FnOnce<()>` closure, found `{integer}`
  --> fake-test-src-base/intrinsics/const-eval-select-bad.rs:10:31
   |
LL |     const_eval_select((), 42, 0xDEADBEEF);
   |     -----------------         ^^^^^^^^^^ expected an `FnOnce<()>` closure, found `{integer}`
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `FnOnce<()>` is not implemented for `{integer}`
   = note: wrap the `{integer}` in a closure with no arguments: `|| { /* code */ }`
  --> /rustc/FAKE_PREFIX/library/core/src/intrinsics.rs:2469:5


error[E0271]: expected `bar` to be a fn item that returns `i32`, but it returns `bool`
  --> fake-test-src-base/intrinsics/const-eval-select-bad.rs:32:34
   |
LL |     const_eval_select((1,), foo, bar);
   |     -----------------            ^^^ expected `i32`, found `bool`
   |     required by a bound introduced by this call
   |
note: required by a bound in `const_eval_select`
  --> /rustc/FAKE_PREFIX/library/core/src/intrinsics.rs:2469:5
  --> /rustc/FAKE_PREFIX/library/core/src/intrinsics.rs:2469:5

error[E0631]: type mismatch in function arguments
  --> fake-test-src-base/intrinsics/const-eval-select-bad.rs:37:32
   |
LL | const fn foo(n: i32) -> i32 {
...
...
LL |     const_eval_select((true,), foo, baz);
   |     |
   |     required by a bound introduced by this call
   |
   |
   = note: expected function signature `fn(i32) -> _`
              found function signature `fn(bool) -> _`
  --> /rustc/FAKE_PREFIX/library/core/src/intrinsics.rs:2469:5

error: this argument must be a `const fn`
  --> fake-test-src-base/intrinsics/const-eval-select-bad.rs:42:29
  --> fake-test-src-base/intrinsics/const-eval-select-bad.rs:42:29
   |
LL |     const_eval_select((1,), bar, bar);
   |
   = help: consult the documentation on `const_eval_select` for more information

error: aborting due to 9 previous errors
---

6    |     |
7    |     expected due to this
8    |
-    = note: expected closure signature `fn(usize) -> _`
-               found closure signature `fn(isize) -> _`
+    = note: expected closure signature `fn(isize) -> _`
+               found closure signature `fn(usize) -> _`
11 note: required by a bound in `foo`
12   --> $DIR/E0631.rs:3:11

22    |     |
23    |     expected due to this
24    |
24    |
-    = note: expected closure signature `fn(usize) -> _`
-               found closure signature `fn(isize) -> _`
+    = note: expected closure signature `fn(isize) -> _`
+               found closure signature `fn(usize) -> _`
27 note: required by a bound in `bar`
28   --> $DIR/E0631.rs:4:11

41    |     |
42    |     required by a bound introduced by this call
43    |
43    |
-    = note: expected function signature `fn(usize) -> _`
-               found function signature `fn(u64) -> _`
+    = note: expected function signature `fn(u64) -> _`
+               found function signature `fn(usize) -> _`
46 note: required by a bound in `foo`
47   --> $DIR/E0631.rs:3:11

60    |     |
61    |     required by a bound introduced by this call
62    |
62    |
-    = note: expected function signature `fn(usize) -> _`
-               found function signature `fn(u64) -> _`
+    = note: expected function signature `fn(u64) -> _`
+               found function signature `fn(usize) -> _`
65 note: required by a bound in `bar`
66   --> $DIR/E0631.rs:4:11


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/E0631/E0631.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/E0631/E0631.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mismatched_types/E0631.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/mismatched_types/E0631.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/E0631" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/E0631/auxiliary"
stdout: none
error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/mismatched_types/E0631.rs:7:5
   |
   |
LL |     foo(|_: isize| {}); //~ ERROR type mismatch
   |     |
   |     expected due to this
   |
   |
   = note: expected closure signature `fn(isize) -> _`
              found closure signature `fn(usize) -> _`
note: required by a bound in `foo`
  --> fake-test-src-base/mismatched_types/E0631.rs:3:11
   |
LL | fn foo<F: Fn(usize)>(_: F) {}
   |           ^^^^^^^^^ required by this bound in `foo`
error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/mismatched_types/E0631.rs:8:5
   |
   |
LL |     bar(|_: isize| {}); //~ ERROR type mismatch
   |     |
   |     expected due to this
   |
   |
   = note: expected closure signature `fn(isize) -> _`
              found closure signature `fn(usize) -> _`
note: required by a bound in `bar`
  --> fake-test-src-base/mismatched_types/E0631.rs:4:11
   |
LL | fn bar<F: Fn<(usize,)>>(_: F) {}
   |           ^^^^^^^^^^^^ required by this bound in `bar`
error[E0631]: type mismatch in function arguments
  --> fake-test-src-base/mismatched_types/E0631.rs:9:9
   |
   |
LL |     fn f(_: u64) {}
...
...
LL |     foo(f); //~ ERROR type mismatch
   |     --- ^ expected due to this
   |     required by a bound introduced by this call
   |
   |
   = note: expected function signature `fn(u64) -> _`
              found function signature `fn(usize) -> _`
note: required by a bound in `foo`
  --> fake-test-src-base/mismatched_types/E0631.rs:3:11
   |
LL | fn foo<F: Fn(usize)>(_: F) {}
   |           ^^^^^^^^^ required by this bound in `foo`
error[E0631]: type mismatch in function arguments
  --> fake-test-src-base/mismatched_types/E0631.rs:10:9
   |
   |
LL |     fn f(_: u64) {}
...
...
LL |     bar(f); //~ ERROR type mismatch
   |     --- ^ expected due to this
   |     required by a bound introduced by this call
   |
   |
   = note: expected function signature `fn(u64) -> _`
              found function signature `fn(usize) -> _`
note: required by a bound in `bar`
  --> fake-test-src-base/mismatched_types/E0631.rs:4:11
   |
LL | fn bar<F: Fn<(usize,)>>(_: F) {}
   |           ^^^^^^^^^^^^ required by this bound in `bar`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0631`.
------------------------------------------
---

6    |                        |
7    |                        expected due to this
8    |
-    = note: expected closure signature `for<'a> fn(&'a {integer}) -> _`
-               found closure signature `fn(i32) -> _`
+    = note: expected closure signature `fn(i32) -> _`
+               found closure signature `for<'a> fn(&'a {integer}) -> _`
11 note: required by a bound in `find`
12   --> $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL
- help: consider borrowing the argument
+ help: do not borrow the argument
14    |
- LL |     let _ = (-10..=10).find(|x: &i32| x.signum() == 0);
-    |                                 +
+ LL |     let _ = (-10..=10).find(|x: i32| x.signum() == 0);
17 
18 error[E0631]: type mismatch in closure arguments
19   --> $DIR/closure-arg-type-mismatch-issue-45727.rs:4:24


23    |                        |
24    |                        expected due to this
25    |
-    = note: expected closure signature `for<'a> fn(&'a {integer}) -> _`
-               found closure signature `for<'a, 'b, 'c> fn(&'a &'b &'c i32) -> _`
+    = note: expected closure signature `for<'a, 'b, 'c> fn(&'a &'b &'c i32) -> _`
+               found closure signature `for<'a> fn(&'a {integer}) -> _`
28 note: required by a bound in `find`
29   --> $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL
- help: do not borrow the argument
+ help: consider borrowing the argument
31    |
- LL -     let _ = (-10..=10).find(|x: &&&i32| x.signum() == 0);
- LL +     let _ = (-10..=10).find(|x: &i32| x.signum() == 0);
-    |
+ LL |     let _ = (-10..=10).find(|x: &&&&&i32| x.signum() == 0);
35 
36 error: aborting due to 2 previous errors
37 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-type-mismatch-issue-45727/closure-arg-type-mismatch-issue-45727.stderr
diff of fixed:

1 // run-rustfix
2 fn main() {
-     let _ = (-10..=10).find(|x: &i32| x.signum() == 0); //~ ERROR type mismatch in closure arguments
-     let _ = (-10..=10).find(|x: &i32| x.signum() == 0); //~ ERROR type mismatch in closure arguments
+     let _ = (-10..=10).find(|x: i32| x.signum() == 0); //~ ERROR type mismatch in closure arguments
+     let _ = (-10..=10).find(|x: &&&&&i32| x.signum() == 0); //~ ERROR type mismatch in closure arguments
6 


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-type-mismatch-issue-45727/closure-arg-type-mismatch-issue-45727.fixed
To only update this specific test, also pass `--test-args mismatched_types/closure-arg-type-mismatch-issue-45727.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/mismatched_types/closure-arg-type-mismatch-issue-45727.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-type-mismatch-issue-45727" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-type-mismatch-issue-45727/auxiliary"
stdout: none
error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/mismatched_types/closure-arg-type-mismatch-issue-45727.rs:3:24
   |
   |
LL |     let _ = (-10..=10).find(|x: i32| x.signum() == 0); //~ ERROR type mismatch in closure arguments
   |                        |
   |                        expected due to this
   |
   |
   = note: expected closure signature `fn(i32) -> _`
              found closure signature `for<'a> fn(&'a {integer}) -> _`
note: required by a bound in `find`
  --> /rustc/FAKE_PREFIX/library/core/src/iter/traits/iterator.rs:2760:5
help: do not borrow the argument
   |
LL |     let _ = (-10..=10).find(|x: i32| x.signum() == 0); //~ ERROR type mismatch in closure arguments

error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/mismatched_types/closure-arg-type-mismatch-issue-45727.rs:4:24
   |
   |
LL |     let _ = (-10..=10).find(|x: &&&i32| x.signum() == 0); //~ ERROR type mismatch in closure arguments
   |                        |
   |                        expected due to this
   |
   |
   = note: expected closure signature `for<'a, 'b, 'c> fn(&'a &'b &'c i32) -> _`
              found closure signature `for<'a> fn(&'a {integer}) -> _`
note: required by a bound in `find`
  --> /rustc/FAKE_PREFIX/library/core/src/iter/traits/iterator.rs:2760:5
   |
   |
LL |     let _ = (-10..=10).find(|x: &&&&&i32| x.signum() == 0); //~ ERROR type mismatch in closure arguments

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0631`.
For more information about this error, try `rustc --explain E0631`.
------------------------------------------


---- [ui] tests/ui/mismatched_types/closure-arg-count.rs stdout ----
diff of stderr:

45 LL |     [1, 2, 3].sort_by(|tuple, tuple2| panic!());
47 
- error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
+ error[E0593]: closure is expected to take 0 arguments, but it takes 0 arguments
49   --> $DIR/closure-arg-count.rs:13:5
49   --> $DIR/closure-arg-count.rs:13:5
50    |
51 LL |     f(|| panic!());
52    |     ^ -- takes 0 arguments
53    |     |
-    |     expected closure that takes 1 argument
+    |     expected closure that takes 0 arguments
+    |     expected closure that takes 0 arguments
55    |
56 note: required by a bound in `f`
57   --> $DIR/closure-arg-count.rs:3:9

58    |
59 LL | fn f<F: Fn<(usize,)>>(_: F) {}
60    |         ^^^^^^^^^^^^ required by this bound in `f`
- help: consider changing the closure to take and ignore the expected argument
+ help: consider changing the closure to take and ignore the expected arguments
62    |
- LL |     f(|_| panic!());
-    |       ~~~
+ LL |     f(|| panic!());
65 
- error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
+ error[E0593]: closure is expected to take 0 arguments, but it takes 0 arguments
67   --> $DIR/closure-arg-count.rs:15:5
67   --> $DIR/closure-arg-count.rs:15:5
68    |
69 LL |     f(  move    || panic!());
70    |     ^   ---------- takes 0 arguments
71    |     |
-    |     expected closure that takes 1 argument
+    |     expected closure that takes 0 arguments
+    |     expected closure that takes 0 arguments
73    |
74 note: required by a bound in `f`
75   --> $DIR/closure-arg-count.rs:3:9

76    |
77 LL | fn f<F: Fn<(usize,)>>(_: F) {}
78    |         ^^^^^^^^^^^^ required by this bound in `f`
- help: consider changing the closure to take and ignore the expected argument
+ help: consider changing the closure to take and ignore the expected arguments
80    |
- LL |     f(  move    |_| panic!());
-    |                 ~~~
+ LL |     f(  move    || panic!());
83 
83 
84 error[E0593]: closure is expected to take a single 2-tuple as argument, but it takes 2 distinct arguments

115    |                                                     |
115    |                                                     |
116    |                                                     expected closure that takes a single 2-tuple as argument
117 
- error[E0593]: function is expected to take a single 2-tuple as argument, but it takes 0 arguments
+ error[E0593]: function is expected to take 0 arguments, but it takes 0 arguments
120    |
120    |
121 LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(foo);

-    |                                                     --- ^^^ expected function that takes a single 2-tuple as argument
123    |                                                     |
124    |                                                     required by a bound introduced by this call
125 ...


129 note: required by a bound in `map`
130   --> $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL
131 
- error[E0593]: closure is expected to take a single 2-tuple as argument, but it takes 3 distinct arguments
+ error[E0593]: closure is expected to take 3 arguments, but it takes 3 arguments
134    |
134    |
135 LL |     let bar = |i, x, y| i;

-    |               --------- takes 3 distinct arguments
+    |               --------- takes 3 arguments
137 LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(bar);
-    |                                                     --- ^^^ expected closure that takes a single 2-tuple as argument
139    |                                                     |
140    |                                                     required by a bound introduced by this call
141    |


142 note: required by a bound in `map`
143   --> $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL
144 
- error[E0593]: function is expected to take a single 2-tuple as argument, but it takes 2 distinct arguments
+ error[E0593]: function is expected to take 2 arguments, but it takes 2 arguments
147    |
147    |
148 LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(qux);

-    |                                                     --- ^^^ expected function that takes a single 2-tuple as argument
150    |                                                     |
151    |                                                     required by a bound introduced by this call
152 ...


153 LL | fn qux(x: usize, y: usize) {}
-    | -------------------------- takes 2 distinct arguments
+    | -------------------------- takes 2 arguments
156 note: required by a bound in `map`
157   --> $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL

158 
158 
- error[E0593]: function is expected to take 1 argument, but it takes 2 arguments
+ error[E0593]: function is expected to take 2 arguments, but it takes 1 argument
160   --> $DIR/closure-arg-count.rs:32:45
161    |
162 LL |     let _it = vec![1, 2, 3].into_iter().map(usize::checked_add);
-    |                                         --- ^^^^^^^^^^^^^^^^^^ expected function that takes 1 argument
+    |                                         --- ^^^^^^^^^^^^^^^^^^ expected function that takes 2 arguments
164    |                                         |
165    |                                         required by a bound introduced by this call
---
To only update this specific test, also pass `--test-args mismatched_types/closure-arg-count.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/mismatched_types/closure-arg-count.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-count" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-count/auxiliary"
stdout: none
error[E0593]: closure is expected to take 2 arguments, but it takes 0 arguments
  --> fake-test-src-base/mismatched_types/closure-arg-count.rs:5:15
   |
   |
LL |     [1, 2, 3].sort_by(|| panic!());
   |               ^^^^^^^ -- takes 0 arguments
   |               expected closure that takes 2 arguments
   |
help: consider changing the closure to take and ignore the expected arguments
   |
   |
LL |     [1, 2, 3].sort_by(|_, _| panic!());

error[E0593]: closure is expected to take 2 arguments, but it takes 1 argument
  --> fake-test-src-base/mismatched_types/closure-arg-count.rs:7:15
   |
   |
LL |     [1, 2, 3].sort_by(|tuple| panic!());
   |               ^^^^^^^ ------- takes 1 argument
   |               expected closure that takes 2 arguments


error[E0593]: closure is expected to take 2 distinct arguments, but it takes a single 2-tuple as argument
  --> fake-test-src-base/mismatched_types/closure-arg-count.rs:9:15
   |
LL |     [1, 2, 3].sort_by(|(tuple, tuple2)| panic!());
   |               ^^^^^^^ ----------------- takes a single 2-tuple as argument
   |               expected closure that takes 2 distinct arguments
   |
   |
help: change the closure to take multiple arguments instead of a single tuple
   |
LL |     [1, 2, 3].sort_by(|tuple, tuple2| panic!());


error[E0593]: closure is expected to take 2 distinct arguments, but it takes a single 2-tuple as argument
  --> fake-test-src-base/mismatched_types/closure-arg-count.rs:11:15
   |
LL |     [1, 2, 3].sort_by(|(tuple, tuple2): (usize, _)| panic!());
   |               ^^^^^^^ ----------------------------- takes a single 2-tuple as argument
   |               expected closure that takes 2 distinct arguments
   |
   |
help: change the closure to take multiple arguments instead of a single tuple
   |
LL |     [1, 2, 3].sort_by(|tuple, tuple2| panic!());

error[E0593]: closure is expected to take 0 arguments, but it takes 0 arguments
  --> fake-test-src-base/mismatched_types/closure-arg-count.rs:13:5
   |
   |
LL |     f(|| panic!());
   |     ^ -- takes 0 arguments
   |     expected closure that takes 0 arguments
   |
note: required by a bound in `f`
  --> fake-test-src-base/mismatched_types/closure-arg-count.rs:3:9
  --> fake-test-src-base/mismatched_types/closure-arg-count.rs:3:9
   |
LL | fn f<F: Fn<(usize,)>>(_: F) {}
   |         ^^^^^^^^^^^^ required by this bound in `f`
help: consider changing the closure to take and ignore the expected arguments
   |
LL |     f(|| panic!());

error[E0593]: closure is expected to take 0 arguments, but it takes 0 arguments
  --> fake-test-src-base/mismatched_types/closure-arg-count.rs:15:5
   |
   |
LL |     f(  move    || panic!());
   |     ^   ---------- takes 0 arguments
   |     expected closure that takes 0 arguments
   |
note: required by a bound in `f`
  --> fake-test-src-base/mismatched_types/closure-arg-count.rs:3:9
  --> fake-test-src-base/mismatched_types/closure-arg-count.rs:3:9
   |
LL | fn f<F: Fn<(usize,)>>(_: F) {}
   |         ^^^^^^^^^^^^ required by this bound in `f`
help: consider changing the closure to take and ignore the expected arguments
   |
LL |     f(  move    || panic!());


error[E0593]: closure is expected to take a single 2-tuple as argument, but it takes 2 distinct arguments
  --> fake-test-src-base/mismatched_types/closure-arg-count.rs:18:53
   |
LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(|i, x| i);
   |                                                     ^^^ ------ takes 2 distinct arguments
   |                                                     |
   |                                                     expected closure that takes a single 2-tuple as argument
   |
help: change the closure to accept a tuple instead of individual arguments
   |
LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(|(i, x)| i);


error[E0593]: closure is expected to take a single 2-tuple as argument, but it takes 2 distinct arguments
  --> fake-test-src-base/mismatched_types/closure-arg-count.rs:20:53
   |
LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(|i: usize, x| i);
   |                                                     ^^^ ------------- takes 2 distinct arguments
   |                                                     |
   |                                                     expected closure that takes a single 2-tuple as argument
   |
help: change the closure to accept a tuple instead of individual arguments
   |
LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(|(i, x)| i);


error[E0593]: closure is expected to take a single 2-tuple as argument, but it takes 3 distinct arguments
  --> fake-test-src-base/mismatched_types/closure-arg-count.rs:22:53
   |
LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(|i, x, y| i);
   |                                                     ^^^ --------- takes 3 distinct arguments
   |                                                     |
   |                                                     expected closure that takes a single 2-tuple as argument
error[E0593]: function is expected to take 0 arguments, but it takes 0 arguments
  --> fake-test-src-base/mismatched_types/closure-arg-count.rs:24:57
   |
   |
LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(foo);
   |                                                     |
   |                                                     required by a bound introduced by this call
...
LL | fn foo() {}
---

error[E0593]: closure is expected to take 3 arguments, but it takes 3 arguments
  --> fake-test-src-base/mismatched_types/closure-arg-count.rs:27:57
   |
LL |     let bar = |i, x, y| i;
   |               --------- takes 3 arguments
LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(bar);
   |                                                     |
   |                                                     required by a bound introduced by this call
   |
note: required by a bound in `map`
note: required by a bound in `map`
  --> /rustc/FAKE_PREFIX/library/core/src/iter/traits/iterator.rs:800:5

error[E0593]: function is expected to take 2 arguments, but it takes 2 arguments
  --> fake-test-src-base/mismatched_types/closure-arg-count.rs:29:57
   |
LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(qux);
   |                                                     |
   |                                                     required by a bound introduced by this call
...
...
LL | fn qux(x: usize, y: usize) {}
   | -------------------------- takes 2 arguments
note: required by a bound in `map`
  --> /rustc/FAKE_PREFIX/library/core/src/iter/traits/iterator.rs:800:5

error[E0593]: function is expected to take 2 arguments, but it takes 1 argument
error[E0593]: function is expected to take 2 arguments, but it takes 1 argument
  --> fake-test-src-base/mismatched_types/closure-arg-count.rs:32:45
   |
LL |     let _it = vec![1, 2, 3].into_iter().map(usize::checked_add);
   |                                         |
   |                                         required by a bound introduced by this call
   |
note: required by a bound in `map`
---
   |
note: required by a bound in `call`
  --> fake-test-src-base/mismatched_types/closure-arg-count.rs:42:30
   |
LL | fn call<F, R>(_: F) where F: FnOnce() -> R {}

error: aborting due to 14 previous errors

For more information about this error, try `rustc --explain E0593`.
---

6    |              |
7    |              expected due to this
8    |
-    = note: expected closure signature `fn(&(u32, u32)) -> _`
-               found closure signature `fn((u32, u32)) -> _`
+    = note: expected closure signature `fn((u32, u32)) -> _`
+               found closure signature `fn(&(u32, u32)) -> _`
12   --> $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL
- help: consider borrowing the argument
- help: consider borrowing the argument
+ help: do not borrow the argument
14    |
- LL |     a.iter().map(|_: &(u32, u32)| 45);
-    |                      +
+ LL |     a.iter().map(|_: (u32, u32)| 45);
17 
18 error[E0631]: type mismatch in closure arguments
19   --> $DIR/closure-arg-type-mismatch.rs:4:14


23    |              |
24    |              expected due to this
25    |
-    = note: expected closure signature `fn(&(u32, u32)) -> _`
-               found closure signature `for<'a> fn(&'a (u16, u16)) -> _`
+    = note: expected closure signature `for<'a> fn(&'a (u16, u16)) -> _`
+               found closure signature `fn(&(u32, u32)) -> _`
29   --> $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL
30 

36    |              |
36    |              |
37    |              expected due to this
38    |
-    = note: expected closure signature `fn(&(u32, u32)) -> _`
-               found closure signature `fn((u16, u16)) -> _`
+    = note: expected closure signature `fn((u16, u16)) -> _`
+               found closure signature `fn(&(u32, u32)) -> _`
42   --> $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL
43 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-type-mismatch/closure-arg-type-mismatch.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mismatched_types/closure-arg-type-mismatch.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/mismatched_types/closure-arg-type-mismatch.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-type-mismatch" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-type-mismatch/auxiliary"
stdout: none
error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/mismatched_types/closure-arg-type-mismatch.rs:3:14
   |
   |
LL |     a.iter().map(|_: (u32, u32)| 45); //~ ERROR type mismatch
   |              |
   |              expected due to this
   |
   |
   = note: expected closure signature `fn((u32, u32)) -> _`
              found closure signature `fn(&(u32, u32)) -> _`
  --> /rustc/FAKE_PREFIX/library/core/src/iter/traits/iterator.rs:800:5
help: do not borrow the argument
   |
   |
LL |     a.iter().map(|_: (u32, u32)| 45); //~ ERROR type mismatch

error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/mismatched_types/closure-arg-type-mismatch.rs:4:14
   |
   |
LL |     a.iter().map(|_: &(u16, u16)| 45); //~ ERROR type mismatch
   |              |
   |              expected due to this
   |
   |
   = note: expected closure signature `for<'a> fn(&'a (u16, u16)) -> _`
              found closure signature `fn(&(u32, u32)) -> _`
  --> /rustc/FAKE_PREFIX/library/core/src/iter/traits/iterator.rs:800:5

error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/mismatched_types/closure-arg-type-mismatch.rs:5:14
  --> fake-test-src-base/mismatched_types/closure-arg-type-mismatch.rs:5:14
   |
LL |     a.iter().map(|_: (u16, u16)| 45); //~ ERROR type mismatch
   |              |
   |              expected due to this
   |
   |
   = note: expected closure signature `fn((u16, u16)) -> _`
              found closure signature `fn(&(u32, u32)) -> _`
  --> /rustc/FAKE_PREFIX/library/core/src/iter/traits/iterator.rs:800:5

error: aborting due to 3 previous errors

---

9    |     |
10    |     required by a bound introduced by this call
11    |
-    = note: expected function signature `fn(&{integer}) -> _`
-               found function signature `for<'a> fn(&'a mut isize) -> _`
+    = note: expected function signature `for<'a> fn(&'a mut isize) -> _`
+               found function signature `fn(&{integer}) -> _`
14 note: required by a bound in `apply`
15   --> $DIR/fn-variance-1.rs:5:37

28    |     |
29    |     required by a bound introduced by this call
30    |
30    |
-    = note: expected function signature `fn(&mut {integer}) -> _`
-               found function signature `for<'a> fn(&'a isize) -> _`
+    = note: expected function signature `for<'a> fn(&'a isize) -> _`
+               found function signature `fn(&mut {integer}) -> _`
33 note: required by a bound in `apply`
34   --> $DIR/fn-variance-1.rs:5:37


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/fn-variance-1/fn-variance-1.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/fn-variance-1/fn-variance-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mismatched_types/fn-variance-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/mismatched_types/fn-variance-1.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/fn-variance-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/fn-variance-1/auxiliary"
stdout: none
error[E0631]: type mismatch in function arguments
  --> fake-test-src-base/mismatched_types/fn-variance-1.rs:11:15
   |
   |
LL | fn takes_mut(x: &mut isize) { }
...
...
LL |     apply(&3, takes_mut);
   |     |
   |     required by a bound introduced by this call
   |
   |
   = note: expected function signature `for<'a> fn(&'a mut isize) -> _`
              found function signature `fn(&{integer}) -> _`
note: required by a bound in `apply`
  --> fake-test-src-base/mismatched_types/fn-variance-1.rs:5:37
   |
LL | fn apply<T, F>(t: T, f: F) where F: FnOnce(T) {
   |                                     ^^^^^^^^^ required by this bound in `apply`
error[E0631]: type mismatch in function arguments
  --> fake-test-src-base/mismatched_types/fn-variance-1.rs:15:19
   |
   |
LL | fn takes_imm(x: &isize) { }
...
...
LL |     apply(&mut 3, takes_imm);
   |     |
   |     required by a bound introduced by this call
   |
   |
   = note: expected function signature `for<'a> fn(&'a isize) -> _`
              found function signature `fn(&mut {integer}) -> _`
note: required by a bound in `apply`
  --> fake-test-src-base/mismatched_types/fn-variance-1.rs:5:37
   |
LL | fn apply<T, F>(t: T, f: F) where F: FnOnce(T) {
   |                                     ^^^^^^^^^ required by this bound in `apply`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0631`.
------------------------------------------
---

6    |                                |
7    |                                expected due to this
8    |
-    = note: expected closure signature `for<'a> fn(&'a &str) -> _`
-               found closure signature `for<'a> fn(&'a str) -> _`
+    = note: expected closure signature `for<'a> fn(&'a str) -> _`
+               found closure signature `for<'a> fn(&'a &str) -> _`
11 note: required by a bound in `filter`
12   --> $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL
- help: consider borrowing the argument
+ help: do not borrow the argument
14    |
- LL |     once::<&str>("str").fuse().filter(|a: &&str| true).count();
-    |                                           +
+ LL -     once::<&str>("str").fuse().filter(|a: &str| true).count();
+ LL +     once::<&str>("str").fuse().filter(|a: str| true).count();
17 
17 
18 error[E0599]: the method `count` exists for struct `Filter<Fuse<Once<&str>>, [closure@issue-36053-2.rs:7:39]>`, but its trait bounds were not satisfied


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2/issue-36053-2.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2/issue-36053-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mismatched_types/issue-36053-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/mismatched_types/issue-36053-2.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2/auxiliary"
stdout: none
error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/mismatched_types/issue-36053-2.rs:7:32
   |
   |
LL |     once::<&str>("str").fuse().filter(|a: &str| true).count();
   |                                |
   |                                expected due to this
   |
   |
   = note: expected closure signature `for<'a> fn(&'a str) -> _`
              found closure signature `for<'a> fn(&'a &str) -> _`
note: required by a bound in `filter`
  --> /rustc/FAKE_PREFIX/library/core/src/iter/traits/iterator.rs:922:5
help: do not borrow the argument
   |
LL -     once::<&str>("str").fuse().filter(|a: &str| true).count();
LL +     once::<&str>("str").fuse().filter(|a: str| true).count();


error[E0599]: the method `count` exists for struct `Filter<Fuse<Once<&str>>, [closure@issue-36053-2.rs:7:39]>`, but its trait bounds were not satisfied
  --> fake-test-src-base/mismatched_types/issue-36053-2.rs:7:55
   |
LL |     once::<&str>("str").fuse().filter(|a: &str| true).count();
   |                                       |
   |                                       |
   |                                       doesn't satisfy `<_ as FnOnce<(&&str,)>>::Output = bool`
   |                                       doesn't satisfy `_: FnMut<(&&str,)>`
  --> /rustc/FAKE_PREFIX/library/core/src/iter/adapters/filter.rs:15:1
   = note: doesn't satisfy `_: Iterator`
   |
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `<[closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48] as FnOnce<(&&str,)>>::Output = bool`
           which is required by `Filter<Fuse<std::iter::Once<&str>>, [closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48]>: Iterator`
           `[closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48]: FnMut<(&&str,)>`
           which is required by `Filter<Fuse<std::iter::Once<&str>>, [closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48]>: Iterator`
           `Filter<Fuse<std::iter::Once<&str>>, [closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48]>: Iterator`
           which is required by `&mut Filter<Fuse<std::iter::Once<&str>>, [closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48]>: Iterator`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0599, E0631.
For more information about an error, try `rustc --explain E0599`.
For more information about an error, try `rustc --explain E0599`.
------------------------------------------


---- [ui] tests/ui/mismatched_types/issue-47706-trait.rs stdout ----
diff of stderr:

- error[E0593]: function is expected to take a single 0-tuple as argument, but it takes 2 distinct arguments
+ error[E0593]: function is expected to take 2 arguments, but it takes 2 arguments
3    |
4 LL |     fn f(&self, _: ()) {


-    |     ------------------ takes 2 distinct arguments
+    |     ------------------ takes 2 arguments
6 LL |         None::<()>.map(Self::f);
-    |                    --- ^^^^^^^ expected function that takes a single 0-tuple as argument
8    |                    |
9    |                    required by a bound introduced by this call
10    |

---
To only update this specific test, also pass `--test-args mismatched_types/issue-47706-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/mismatched_types/issue-47706-trait.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-47706-trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-47706-trait/auxiliary"
stdout: none
error[E0593]: function is expected to take 2 arguments, but it takes 2 arguments
  --> fake-test-src-base/mismatched_types/issue-47706-trait.rs:3:24
   |
LL |     fn f(&self, _: ()) {
LL |     fn f(&self, _: ()) {
   |     ------------------ takes 2 arguments
LL |         None::<()>.map(Self::f);
   |                    |
   |                    required by a bound introduced by this call
   |
note: required by a bound in `Option::<T>::map`
---
- error[E0593]: function is expected to take 1 argument, but it takes 2 arguments
+ error[E0593]: function is expected to take 2 arguments, but it takes 2 arguments
2   --> $DIR/issue-47706.rs:11:22
3    |
4 LL |     pub fn new(foo: Option<i32>, _: ()) -> Foo {
5    |     ------------------------------------------ takes 2 arguments
6 ...
6 ...
7 LL |         self.foo.map(Foo::new)
+    |                  --- ^^^^^^^^ expected function that takes 2 arguments
9    |                  |
10    |                  required by a bound introduced by this call
11    |
---
18 LL |     Bar(i32),

19    |     --- takes 1 argument
20 ...
21 LL |     foo(Qux::Bar);
+    |     --- ^^^^^^^^ expected function that takes 1 argument
23    |     |
24    |     required by a bound introduced by this call
25    |
---
To only update this specific test, also pass `--test-args mismatched_types/issue-47706.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/mismatched_types/issue-47706.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-47706" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-47706/auxiliary"
stdout: none
error[E0593]: function is expected to take 2 arguments, but it takes 2 arguments
  --> fake-test-src-base/mismatched_types/issue-47706.rs:11:22
   |
   |
LL |     pub fn new(foo: Option<i32>, _: ()) -> Foo {
   |     ------------------------------------------ takes 2 arguments
...
LL |         self.foo.map(Foo::new)
   |                  |
   |                  required by a bound introduced by this call
   |
note: required by a bound in `Option::<T>::map`
---
   |
LL |     Bar(i32),
   |     --- takes 1 argument
...
LL |     foo(Qux::Bar);
   |     |
   |     required by a bound introduced by this call
   |
note: required by a bound in `foo`
note: required by a bound in `foo`
  --> fake-test-src-base/mismatched_types/issue-47706.rs:22:8
   |
LL | fn foo<F>(f: F)
LL | where
LL |     F: Fn(),
   |        ^^^^ required by this bound in `foo`

---

9    |             |
10    |             required by a bound introduced by this call
11    |
-    = note: expected closure signature `fn(isize, _) -> _`
-               found closure signature `fn(usize, _) -> _`
+    = note: expected closure signature `fn(usize, _) -> _`
+               found closure signature `fn(isize, _) -> _`
14 note: required by a bound in `call_it`
16    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/unboxed-closures-vtable-mismatch/unboxed-closures-vtable-mismatch.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mismatched_types/unboxed-closures-vtable-mismatch.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/mismatched_types/unboxed-closures-vtable-mismatch.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/unboxed-closures-vtable-mismatch" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/unboxed-closures-vtable-mismatch/auxiliary"
stdout: none
error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/mismatched_types/unboxed-closures-vtable-mismatch.rs:16:24
   |
   |
LL |     let f = to_fn_mut(|x: usize, y: isize| -> isize { (x as isize) + y });
   |                       ----------------------------- found signature defined here
LL |     //~^ NOTE found signature defined here
LL |     let z = call_it(3, f);
   |             |
   |             required by a bound introduced by this call
   |
   |
   = note: expected closure signature `fn(usize, _) -> _`
              found closure signature `fn(isize, _) -> _`
note: required by a bound in `call_it`
  --> fake-test-src-base/mismatched_types/unboxed-closures-vtable-mismatch.rs:7:15
   |
LL | fn call_it<F: FnMut(isize, isize) -> isize>(y: isize, mut f: F) -> isize {
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `call_it`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0631`.
------------------------------------------
---

34    |     |
35    |     expected due to this
36    |
-    = note: expected closure signature `fn(i32) -> _`
-               found closure signature `fn(()) -> _`
+    = note: expected closure signature `fn(()) -> _`
+               found closure signature `fn(i32) -> _`
39 note: required by a bound in `foo2`
41    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-fn-trait-from-fn-kw/recover-fn-trait-from-fn-kw.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/recover-fn-trait-from-fn-kw.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/parser/recover-fn-trait-from-fn-kw.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-fn-trait-from-fn-kw" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-fn-trait-from-fn-kw/auxiliary"
stdout: none
error: expected identifier, found keyword `fn`
  --> fake-test-src-base/parser/recover-fn-trait-from-fn-kw.rs:1:16
   |
   |
LL | fn foo(_: impl fn() -> i32) {}
   |
   |
help: use `Fn` to refer to the trait
   |
LL | fn foo(_: impl Fn() -> i32) {}

error: expected identifier, found keyword `fn`
  --> fake-test-src-base/parser/recover-fn-trait-from-fn-kw.rs:4:12
   |
   |
LL | fn foo2<T: fn(i32)>(_: T) {}
   |
   |
help: use `Fn` to refer to the trait
   |
LL | fn foo2<T: Fn(i32)>(_: T) {}

error[E0308]: mismatched types
  --> fake-test-src-base/parser/recover-fn-trait-from-fn-kw.rs:8:12
   |
   |
LL |     foo(|| ());
   |            ^^ expected `i32`, found `()`
error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/parser/recover-fn-trait-from-fn-kw.rs:10:5
   |
   |
LL |     foo2(|_: ()| {});
   |     |
   |     expected due to this
   |
   |
   = note: expected closure signature `fn(()) -> _`
              found closure signature `fn(i32) -> _`
note: required by a bound in `foo2`
  --> fake-test-src-base/parser/recover-fn-trait-from-fn-kw.rs:4:12
   |
LL | fn foo2<T: fn(i32)>(_: T) {}
   |            ^^^^^^^ required by this bound in `foo2`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0308, E0631.
For more information about an error, try `rustc --explain E0308`.
---

9    |     |
10    |     required by a bound introduced by this call
11    |
-    = note: expected function signature `fn(String) -> _`
-               found function signature `fn(&str) -> _`
+    = note: expected function signature `fn(&str) -> _`
+               found function signature `fn(String) -> _`
15   --> $DIR/enum-variant-arg-mismatch.rs:5:15
16    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/enum-variant-arg-mismatch/enum-variant-arg-mismatch.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/enum-variant-arg-mismatch.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/suggestions/enum-variant-arg-mismatch.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/enum-variant-arg-mismatch" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/enum-variant-arg-mismatch/auxiliary"
stdout: none
error[E0631]: type mismatch in function arguments
  --> fake-test-src-base/suggestions/enum-variant-arg-mismatch.rs:8:9
   |
   |
LL |     Ident(&'a str),
...
...
LL |     map(Sexpr::Ident);
   |     |
   |     required by a bound introduced by this call
   |
   |
   = note: expected function signature `fn(&str) -> _`
              found function signature `fn(String) -> _`
  --> fake-test-src-base/suggestions/enum-variant-arg-mismatch.rs:5:15
   |
   |
LL | fn map<'a, F: Fn(String) -> Sexpr<'a>>(f: F) {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0631`.
---

9    |            |
10    |            required by a bound introduced by this call
11    |
-    = note: expected closure signature `for<'a, 'b> fn(&'a mut Trader<'b>) -> _`
-               found closure signature `for<'a> fn(Trader<'a>) -> _`
+    = note: expected closure signature `for<'a> fn(Trader<'a>) -> _`
+               found closure signature `for<'a, 'b> fn(&'a mut Trader<'b>) -> _`
14 note: required by a bound in `Trader::<'a>::set_closure`
16    |


17 LL |     pub fn set_closure(&mut self, function: impl Fn(&mut Trader) + 'a) {
18    |                                                  ^^^^^^^^^^^^^^^ required by this bound in `Trader::<'a>::set_closure`
- help: consider borrowing the argument
+ help: do not borrow the argument
20    |
- LL |     let closure = |trader : &mut Trader| {
-    |                             ++++
+ LL |     let closure = |trader : Trader<'_>| {
23 
24 error: aborting due to previous error
25 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/late-bound-in-borrow-closure-sugg/late-bound-in-borrow-closure-sugg.stderr
To only update this specific test, also pass `--test-args suggestions/late-bound-in-borrow-closure-sugg.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/suggestions/late-bound-in-borrow-closure-sugg.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/late-bound-in-borrow-closure-sugg" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/late-bound-in-borrow-closure-sugg/auxiliary"
stdout: none
error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/suggestions/late-bound-in-borrow-closure-sugg.rs:26:24
   |
   |
LL |     let closure = |trader : Trader| {
...
...
LL |     trader.set_closure(closure);
   |            |
   |            required by a bound introduced by this call
   |
   |
   = note: expected closure signature `for<'a> fn(Trader<'a>) -> _`
              found closure signature `for<'a, 'b> fn(&'a mut Trader<'b>) -> _`
note: required by a bound in `Trader::<'a>::set_closure`
  --> fake-test-src-base/suggestions/late-bound-in-borrow-closure-sugg.rs:15:50
   |
LL |     pub fn set_closure(&mut self, function: impl Fn(&mut Trader) + 'a) {
   |                                                  ^^^^^^^^^^^^^^^ required by this bound in `Trader::<'a>::set_closure`
help: do not borrow the argument
   |
LL |     let closure = |trader : Trader<'_>| {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0631`.
