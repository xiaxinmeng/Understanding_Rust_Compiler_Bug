plain
...............................................................................F........ 4752/13197
........................................................................................ 4840/13197
........................................................................................ 4928/13197
........................................................................................ 5016/13197
..........................................................F..F.......................... 5104/13197
........................................................................................ 5280/13197
........................................................................................ 5368/13197
................................F....................................................... 5456/13197
........................................................................................ 5544/13197
---
........................................................................................ 11616/13197
..................................................i........i........i.....i............. 11704/13197
........i............................................................................... 11792/13197
........................................................................................ 11880/13197
..........................................F............................................. 11968/13197
..........................................................F............................F 12056/13197
..........................F..........................................F...........F.F.... 12144/13197
....F................................................................................... 12232/13197
..........................i............................................................. 12408/13197
........................................................................................ 12496/13197
........................................................................................ 12584/13197
........................................................................................ 12672/13197
---
3    |
4 LL |     let x: isize = Foo::bar();
-    |                    ^^^^^^^^ cannot infer type
-    |
-    = note: cannot satisfy `_: Foo`
+    |                    ^^^^^^^^ cannot call trait method as a free function
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-unconstrained/associated-types-unconstrained.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/associated-types-unconstrained.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-unconstrained.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-unconstrained" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-unconstrained/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/associated-types/associated-types-unconstrained.rs:14:20
   |
LL |     let x: isize = Foo::bar();
LL |     let x: isize = Foo::bar();
   |                    ^^^^^^^^ cannot call trait method as a free function
error: aborting due to previous error

For more information about this error, try `rustc --explain E0283`.
------------------------------------------
---
11 
12 error[E0283]: type annotations needed
+   --> $DIR/const_eval_resolve_canonical.rs:26:18
+    |
+ LL |     let mut _q = Default::default();
+    |                  ^^^^^^^^^^^^^^^^ cannot call trait method as a free function
+ error[E0283]: type annotations needed
13   --> $DIR/const_eval_resolve_canonical.rs:29:10
14    |
14    |
15 LL |     _q = foo::<_, 2>(_q);

32 LL |     (): Foo<{ N + 1 }>,
33    |         ^^^^^^^^^^^^^^ required by this bound in `foo`
- error: aborting due to 2 previous errors
+ error: aborting due to 3 previous errors
36 
37 Some errors have detailed explanations: E0282, E0283.
---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/const_eval_resolve_canonical.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical.rs:26:9
   |
   |
LL |     let mut _q = Default::default();
   |
   |
help: consider giving `_q` an explicit type
   |
LL |     let mut _q: _ = Default::default();

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical.rs:26:18
   |
   |
LL |     let mut _q = Default::default();
   |                  ^^^^^^^^^^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical.rs:29:10
   |
   |
LL |     _q = foo::<_, 2>(_q);
   |          ^^^^^^^^^^^ cannot infer the value of the constant `{ N + 1 }`
   |
note: multiple `impl`s satisfying `(): Foo<{ N + 1 }>` found
   |
   |
LL | impl Foo<0> for () {
...
...
LL | impl Foo<3> for () {
note: required by a bound in `foo`
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical.rs:18:9
   |
   |
LL | fn foo<T, const N: usize>(_: T) -> <() as Foo<{ N + 1 }>>::Assoc
   |    --- required by a bound in this
LL | where
LL |     (): Foo<{ N + 1 }>,
   |         ^^^^^^^^^^^^^^ required by this bound in `foo`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0282, E0283.
For more information about an error, try `rustc --explain E0282`.
For more information about an error, try `rustc --explain E0282`.
------------------------------------------


---- [ui] src/test/ui/const-generics/occurs-check/unused-substs-4.rs stdout ----
diff of stderr:

4 LL |     arr = bind(arr);
6 
- error: aborting due to previous error
+ error[E0283]: type annotations needed
+   --> $DIR/unused-substs-4.rs:9:19
+   --> $DIR/unused-substs-4.rs:9:19
+    |
+ LL |     let mut arr = Default::default();
+    |                   ^^^^^^^^^^^^^^^^ cannot call trait method as a free function
- For more information about this error, try `rustc --explain E0308`.
+ error: aborting due to 2 previous errors
+ 
+ Some errors have detailed explanations: E0283, E0308.
+ Some errors have detailed explanations: E0283, E0308.
+ For more information about an error, try `rustc --explain E0283`.
10 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-4/unused-substs-4.stderr
To only update this specific test, also pass `--test-args const-generics/occurs-check/unused-substs-4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/occurs-check/unused-substs-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-4" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-4/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/occurs-check/unused-substs-4.rs:10:11
   |
   |
LL |     arr = bind(arr); //~ ERROR mismatched type

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/const-generics/occurs-check/unused-substs-4.rs:9:19
   |
   |
LL |     let mut arr = Default::default();
   |                   ^^^^^^^^^^^^^^^^ cannot call trait method as a free function
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0283, E0308.
For more information about an error, try `rustc --explain E0283`.
---
diff of stderr:

2   --> $DIR/E0283.rs:30:21
3    |
4 LL |     let cont: u32 = Generator::create();
-    |
-    = note: cannot satisfy `_: Generator`
-    = note: cannot satisfy `_: Generator`
+    |                     ^^^^^^^^^^^^^^^^^ cannot call trait method as a free function
9 error[E0283]: type annotations needed
10   --> $DIR/E0283.rs:35:24

11    |
11    |
12 LL |     let bar = foo_impl.into() * 1u32;
-    |
-    |
- note: multiple `impl`s satisfying `Impl: Into<_>` found
-   --> $DIR/E0283.rs:17:1
-    |
- LL | impl Into<u32> for Impl {
-    = note: and another `impl` found in the `core` crate:
-    = note: and another `impl` found in the `core` crate:
-            - impl<T, U> Into<U> for T
-              where U: From<T>;
- help: try using a fully qualified path to specify the expected types
-    |
- LL |     let bar = <Impl as Into<T>>::into(foo_impl) * 1u32;
-    |               ++++++++++++++++++++++++        ~
+    |                        ^^^^ cannot call trait method as a free function
28 error: aborting due to 2 previous errors
29 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0283/E0283.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0283.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0283.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0283" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0283/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/error-codes/E0283.rs:30:21
   |
   |
LL |     let cont: u32 = Generator::create(); //~ ERROR E0283
   |                     ^^^^^^^^^^^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/error-codes/E0283.rs:35:24
   |
   |
LL |     let bar = foo_impl.into() * 1u32; //~ ERROR E0283
   |                        ^^^^ cannot call trait method as a free function
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0283`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/impl-trait/cross-return-site-inference.rs stdout ----
diff of stderr:

20 LL |     return Err::<(), E>(From::from("foo"));
22 
+ error[E0283]: type annotations needed
+   --> $DIR/cross-return-site-inference.rs:37:16
+    |
+    |
+ LL |     return Err(From::from("foo"));
+    |                ^^^^^^^^^^ cannot call trait method as a free function
23 error[E0282]: type annotations needed
24   --> $DIR/cross-return-site-inference.rs:42:5
25    |


31 LL |     Err::<(), E>(From::from("foo"))
33 
- error: aborting due to 3 previous errors
+ error[E0283]: type annotations needed
+   --> $DIR/cross-return-site-inference.rs:42:9
+   --> $DIR/cross-return-site-inference.rs:42:9
+    |
+ LL |     Err(From::from("foo"))
+    |         ^^^^^^^^^^ cannot call trait method as a free function
- For more information about this error, try `rustc --explain E0282`.
+ error: aborting due to 5 previous errors
+ 
+ Some errors have detailed explanations: E0282, E0283.
---
To only update this specific test, also pass `--test-args impl-trait/cross-return-site-inference.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/cross-return-site-inference.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/cross-return-site-inference" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/cross-return-site-inference/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/impl-trait/cross-return-site-inference.rs:33:5
   |
   |
LL |     Ok(()) //~ ERROR type annotations needed
   |     ^^ cannot infer type of the type parameter `E` declared on the enum `Result`
help: consider specifying the generic arguments
   |
   |
LL |     Ok::<(), E>(()) //~ ERROR type annotations needed

error[E0282]: type annotations needed
  --> /checkout/src/test/ui/impl-trait/cross-return-site-inference.rs:37:12
   |
   |
LL |     return Err(From::from("foo")); //~ ERROR type annotations needed
   |            ^^^ cannot infer type of the type parameter `E` declared on the enum `Result`
help: consider specifying the generic arguments
   |
   |
LL |     return Err::<(), E>(From::from("foo")); //~ ERROR type annotations needed

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/impl-trait/cross-return-site-inference.rs:37:16
   |
   |
LL |     return Err(From::from("foo")); //~ ERROR type annotations needed
   |                ^^^^^^^^^^ cannot call trait method as a free function
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/impl-trait/cross-return-site-inference.rs:42:5
   |
   |
LL |     Err(From::from("foo")) //~ ERROR type annotations needed
   |     ^^^ cannot infer type of the type parameter `E` declared on the enum `Result`
help: consider specifying the generic arguments
   |
   |
LL |     Err::<(), E>(From::from("foo")) //~ ERROR type annotations needed

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/impl-trait/cross-return-site-inference.rs:42:9
   |
   |
LL |     Err(From::from("foo")) //~ ERROR type annotations needed
   |         ^^^^^^^^^^ cannot call trait method as a free function
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0282, E0283.
For more information about an error, try `rustc --explain E0282`.
---
diff of stderr:

2   --> $DIR/issue-72690.rs:7:5
3    |
4 LL |     String::from("x".as_ref());
-    |     ^^^^^^^^^^^^ cannot infer type for reference `&_`
-    |
-    = note: multiple `impl`s satisfying `String: From<&_>` found in the `alloc` crate:
-            - impl<> From<&String> for String;
-            - impl<> From<&str> for String;
+    |     ^^^^^^^^^^^^ cannot call trait method as a free function
11 error[E0283]: type annotations needed
12   --> $DIR/issue-72690.rs:7:22

13    |
13    |
14 LL |     String::from("x".as_ref());
-    |
-    |
-    = note: multiple `impl`s satisfying `str: AsRef<_>` found in the following crates: `core`, `std`:
-            - impl AsRef<OsStr> for str;
-            - impl AsRef<Path> for str;
-            - impl AsRef<[u8]> for str;
-            - impl AsRef<str> for str;
- help: try using a fully qualified path to specify the expected types
-    |
- LL |     String::from(<str as AsRef<T>>::as_ref("x"));
-    |                  ++++++++++++++++++++++++++   ~
+    |                      ^^^^^^ cannot call trait method as a free function
27 error[E0282]: type annotations needed
28   --> $DIR/issue-72690.rs:12:6

36    |       +++
36    |       +++
37 
38 error[E0283]: type annotations needed
-   --> $DIR/issue-72690.rs:12:26
+   --> $DIR/issue-72690.rs:12:9
40    |
41 LL |     |x| String::from("x".as_ref());
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |         ^^^^^^^^^^^^ cannot call trait method as a free function
+ error[E0283]: type annotations needed
+   --> $DIR/issue-72690.rs:12:26
43    |
43    |
-    = note: multiple `impl`s satisfying `str: AsRef<_>` found in the following crates: `core`, `std`:
-            - impl AsRef<OsStr> for str;
-            - impl AsRef<Path> for str;
-            - impl AsRef<[u8]> for str;
-            - impl AsRef<str> for str;
- help: try using a fully qualified path to specify the expected types
-    |
- LL |     |x| String::from(<str as AsRef<T>>::as_ref("x"));
-    |                      ++++++++++++++++++++++++++   ~
+ LL |     |x| String::from("x".as_ref());
+    |                          ^^^^^^ cannot call trait method as a free function
- error[E0283]: type annotations needed for `&T`
-   --> $DIR/issue-72690.rs:17:9
+ error[E0283]: type annotations needed
+   --> $DIR/issue-72690.rs:17:17
+   --> $DIR/issue-72690.rs:17:17
56    |
57 LL |     let _ = "x".as_ref();
-    |         ^       ------ type must be known at this point
-    |
-    = note: multiple `impl`s satisfying `str: AsRef<_>` found in the following crates: `core`, `std`:
-            - impl AsRef<OsStr> for str;
-            - impl AsRef<Path> for str;
-            - impl AsRef<[u8]> for str;
-            - impl AsRef<str> for str;
- help: consider giving this pattern a type, where the type for type parameter `T` is specified
-    |
- LL |     let _: &T = "x".as_ref();
-    |          ++++
+    |                 ^^^^^^ cannot call trait method as a free function
70 error[E0283]: type annotations needed
71   --> $DIR/issue-72690.rs:21:5

72    |
72    |
73 LL |     String::from("x".as_ref());
-    |     ^^^^^^^^^^^^ cannot infer type for reference `&_`
-    |
-    = note: multiple `impl`s satisfying `String: From<&_>` found in the `alloc` crate:
-            - impl<> From<&String> for String;
-            - impl<> From<&str> for String;
+    |     ^^^^^^^^^^^^ cannot call trait method as a free function
80 error[E0283]: type annotations needed
81   --> $DIR/issue-72690.rs:21:22

82    |
82    |
83 LL |     String::from("x".as_ref());
-    |
-    |
-    = note: multiple `impl`s satisfying `str: AsRef<_>` found in the following crates: `core`, `std`:
-            - impl AsRef<OsStr> for str;
-            - impl AsRef<Path> for str;
-            - impl AsRef<[u8]> for str;
-            - impl AsRef<str> for str;
- help: try using a fully qualified path to specify the expected types
-    |
- LL |     String::from(<str as AsRef<T>>::as_ref("x"));
-    |                  ++++++++++++++++++++++++++   ~
+    |                      ^^^^^^ cannot call trait method as a free function
96 error[E0283]: type annotations needed
97   --> $DIR/issue-72690.rs:28:5

98    |
98    |
99 LL |     String::from("x".as_ref());
-    |     ^^^^^^^^^^^^ cannot infer type for reference `&_`
-    |
-    = note: multiple `impl`s satisfying `String: From<&_>` found in the `alloc` crate:
-            - impl<> From<&String> for String;
-            - impl<> From<&str> for String;
+    |     ^^^^^^^^^^^^ cannot call trait method as a free function
106 error[E0283]: type annotations needed
107   --> $DIR/issue-72690.rs:28:22

108    |
108    |
109 LL |     String::from("x".as_ref());
-    |
-    |
-    = note: multiple `impl`s satisfying `str: AsRef<_>` found in the following crates: `core`, `std`:
-            - impl AsRef<OsStr> for str;
-            - impl AsRef<Path> for str;
-            - impl AsRef<[u8]> for str;
-            - impl AsRef<str> for str;
- help: try using a fully qualified path to specify the expected types
-    |
- LL |     String::from(<str as AsRef<T>>::as_ref("x"));
-    |                  ++++++++++++++++++++++++++   ~
+    |                      ^^^^^^ cannot call trait method as a free function
122 error[E0283]: type annotations needed
123   --> $DIR/issue-72690.rs:37:5

124    |
124    |
125 LL |     String::from("x".as_ref());
-    |     ^^^^^^^^^^^^ cannot infer type for reference `&_`
-    |
-    = note: multiple `impl`s satisfying `String: From<&_>` found in the `alloc` crate:
-            - impl<> From<&String> for String;
-            - impl<> From<&str> for String;
+    |     ^^^^^^^^^^^^ cannot call trait method as a free function
132 error[E0283]: type annotations needed
133   --> $DIR/issue-72690.rs:37:22

134    |
134    |
135 LL |     String::from("x".as_ref());
-    |
-    |
-    = note: multiple `impl`s satisfying `str: AsRef<_>` found in the following crates: `core`, `std`:
-            - impl AsRef<OsStr> for str;
-            - impl AsRef<Path> for str;
-            - impl AsRef<[u8]> for str;
-            - impl AsRef<str> for str;
- help: try using a fully qualified path to specify the expected types
-    |
- LL |     String::from(<str as AsRef<T>>::as_ref("x"));
-    |                  ++++++++++++++++++++++++++   ~
+    |                      ^^^^^^ cannot call trait method as a free function
148 error[E0283]: type annotations needed
149   --> $DIR/issue-72690.rs:46:5

150    |
150    |
151 LL |     String::from("x".as_ref());
-    |     ^^^^^^^^^^^^ cannot infer type for reference `&_`
-    |
-    = note: multiple `impl`s satisfying `String: From<&_>` found in the `alloc` crate:
-            - impl<> From<&String> for String;
-            - impl<> From<&str> for String;
+    |     ^^^^^^^^^^^^ cannot call trait method as a free function
158 error[E0283]: type annotations needed
159   --> $DIR/issue-72690.rs:46:22

160    |
160    |
161 LL |     String::from("x".as_ref());
-    |
-    |
-    = note: multiple `impl`s satisfying `str: AsRef<_>` found in the following crates: `core`, `std`:
-            - impl AsRef<OsStr> for str;
-            - impl AsRef<Path> for str;
-            - impl AsRef<[u8]> for str;
-            - impl AsRef<str> for str;
- help: try using a fully qualified path to specify the expected types
-    |
- LL |     String::from(<str as AsRef<T>>::as_ref("x"));
-    |                  ++++++++++++++++++++++++++   ~
+    |                      ^^^^^^ cannot call trait method as a free function
174 error[E0283]: type annotations needed
175   --> $DIR/issue-72690.rs:53:5

176    |
176    |
177 LL |     String::from("x".as_ref());
-    |     ^^^^^^^^^^^^ cannot infer type for reference `&_`
-    |
-    = note: multiple `impl`s satisfying `String: From<&_>` found in the `alloc` crate:
-            - impl<> From<&String> for String;
-            - impl<> From<&str> for String;
+    |     ^^^^^^^^^^^^ cannot call trait method as a free function
184 error[E0283]: type annotations needed
185   --> $DIR/issue-72690.rs:53:22

186    |
186    |
187 LL |     String::from("x".as_ref());
-    |
-    |
-    = note: multiple `impl`s satisfying `str: AsRef<_>` found in the following crates: `core`, `std`:
-            - impl AsRef<OsStr> for str;
-            - impl AsRef<Path> for str;
-            - impl AsRef<[u8]> for str;
-            - impl AsRef<str> for str;
- help: try using a fully qualified path to specify the expected types
-    |
- LL |     String::from(<str as AsRef<T>>::as_ref("x"));
-    |                  ++++++++++++++++++++++++++   ~
+    |                      ^^^^^^ cannot call trait method as a free function
200 error[E0283]: type annotations needed
201   --> $DIR/issue-72690.rs:62:5

202    |
202    |
203 LL |     String::from("x".as_ref());
-    |     ^^^^^^^^^^^^ cannot infer type for reference `&_`
-    |
-    = note: multiple `impl`s satisfying `String: From<&_>` found in the `alloc` crate:
-            - impl<> From<&String> for String;
-            - impl<> From<&str> for String;
+    |     ^^^^^^^^^^^^ cannot call trait method as a free function
210 error[E0283]: type annotations needed
211   --> $DIR/issue-72690.rs:62:22

212    |
212    |
213 LL |     String::from("x".as_ref());
-    |
-    |
-    = note: multiple `impl`s satisfying `str: AsRef<_>` found in the following crates: `core`, `std`:
-            - impl AsRef<OsStr> for str;
-            - impl AsRef<Path> for str;
-            - impl AsRef<[u8]> for str;
-            - impl AsRef<str> for str;
- help: try using a fully qualified path to specify the expected types
-    |
- LL |     String::from(<str as AsRef<T>>::as_ref("x"));
-    |                  ++++++++++++++++++++++++++   ~
+    |                      ^^^^^^ cannot call trait method as a free function
- error: aborting due to 17 previous errors
+ error: aborting due to 18 previous errors
227 
228 Some errors have detailed explanations: E0282, E0283.
---
To only update this specific test, also pass `--test-args inference/issue-72690.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/issue-72690.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-72690" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-72690/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/inference/issue-72690.rs:7:5
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |     ^^^^^^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:7:22
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |                      ^^^^^^ cannot call trait method as a free function
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:12:6
   |
   |
LL |     |x| String::from("x".as_ref()); //~ ERROR type annotations needed
   |
   |
help: consider giving this closure parameter an explicit type
   |
LL |     |x: _| String::from("x".as_ref()); //~ ERROR type annotations needed

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:12:9
   |
   |
LL |     |x| String::from("x".as_ref()); //~ ERROR type annotations needed
   |         ^^^^^^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:12:26
   |
   |
LL |     |x| String::from("x".as_ref()); //~ ERROR type annotations needed
   |                          ^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:17:17
   |
   |
LL |     let _ = "x".as_ref(); //~ ERROR type annotations needed
   |                 ^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:21:5
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |     ^^^^^^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:21:22
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |                      ^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:28:5
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |     ^^^^^^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:28:22
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |                      ^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:37:5
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |     ^^^^^^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:37:22
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |                      ^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:46:5
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |     ^^^^^^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:46:22
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |                      ^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:53:5
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |     ^^^^^^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:53:22
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |                      ^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:62:5
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |     ^^^^^^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:62:22
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |                      ^^^^^^ cannot call trait method as a free function
error: aborting due to 18 previous errors

Some errors have detailed explanations: E0282, E0283.
For more information about an error, try `rustc --explain E0282`.
---
diff of stderr:

13   --> $DIR/concrete-impl.rs:13:5
14    |
15 LL |     <Struct as Ambiguous<_>>::method();
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type of the type parameter `Self` declared on the trait `Ambiguous`
-    |
- note: multiple `impl`s satisfying `Struct: Ambiguous<_>` found
-   --> $DIR/concrete-impl.rs:9:1
-    |
- LL | impl Ambiguous<One> for Struct {}
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- LL | impl Ambiguous<Two> for Struct {}
- help: consider specifying the generic argument
-    |
-    |
- LL |     <Struct as Ambiguous::<_>>::method();
-    |                         ~~~~~
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot call trait method as a free function
30 error: aborting due to 2 previous errors
31 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/need_type_info/concrete-impl/concrete-impl.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inference/need_type_info/concrete-impl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/need_type_info/concrete-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/need_type_info/concrete-impl" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/need_type_info/concrete-impl/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/inference/need_type_info/concrete-impl.rs:13:5
   |
   |
LL |     <Struct as Ambiguous<_>>::method();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type of the type parameter `Self` declared on the trait `Ambiguous`
help: consider specifying the generic argument
   |
   |
LL |     <Struct as Ambiguous::<_>>::method();

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/need_type_info/concrete-impl.rs:13:5
   |
   |
LL |     <Struct as Ambiguous<_>>::method();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot call trait method as a free function
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0282, E0283.
For more information about an error, try `rustc --explain E0282`.
For more information about an error, try `rustc --explain E0282`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-16966.rs stdout ----
diff of stderr:

11 LL |         $crate::rt::begin_panic::<M>($msg)
13 
- error: aborting due to previous error
+ error[E0283]: type annotations needed
+   --> $DIR/issue-16966.rs:2:12
+   --> $DIR/issue-16966.rs:2:12
+    |
+ LL |     panic!(std::default::Default::default());
+    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot call trait method as a free function
- For more information about this error, try `rustc --explain E0282`.
+ error: aborting due to 2 previous errors
+ 
+ Some errors have detailed explanations: E0282, E0283.
---
To only update this specific test, also pass `--test-args issues/issue-16966.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-16966.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16966" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16966/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-16966.rs:2:5
   |
LL |     panic!(std::default::Default::default());
LL |     panic!(std::default::Default::default());
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type of the type parameter `M` declared on the function `begin_panic`
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider specifying the generic argument
  --> /checkout/library/std/src/panic.rs:22:32
   |
   |
LL |         $crate::rt::begin_panic::<M>($msg)

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-16966.rs:2:12
   |
   |
LL |     panic!(std::default::Default::default());
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot call trait method as a free function
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0282, E0283.
For more information about an error, try `rustc --explain E0282`.
---
diff of stderr:

2   --> $DIR/issue-29147.rs:21:13
3    |
4 LL |     let _ = <S5<_>>::xxx;
-    |             ^^^^^^^^^^^^ cannot infer type for struct `S5<_>`
-    |
- note: multiple `impl`s satisfying `S5<_>: Foo` found
-   --> $DIR/issue-29147.rs:17:1
-    |
- LL | impl Foo for S5<u32> { fn xxx(&self) {} }
-    | ^^^^^^^^^^^^^^^^^^^^
- LL | impl Foo for S5<u64> { fn xxx(&self) {} }
-    | ^^^^^^^^^^^^^^^^^^^^
+    |             ^^^^^^^^^^^^ cannot call trait method as a free function
15 error: aborting due to previous error
16 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-29147/issue-29147.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-29147.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-29147.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-29147" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-29147/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-29147.rs:21:13
   |
   |
LL |     let _ = <S5<_>>::xxx; //~ ERROR type annotations needed
   |             ^^^^^^^^^^^^ cannot call trait method as a free function
error: aborting due to previous error

For more information about this error, try `rustc --explain E0283`.
------------------------------------------
---
13 error[E0283]: type annotations needed
-   --> $DIR/issue-69455.rs:29:41
+   --> $DIR/issue-69455.rs:29:26
15    |
16 LL |     println!("{}", 23u64.test(xs.iter().sum()));
-    |                          ----           ^^^ cannot infer type of the type parameter `S` declared on the associated function `sum`
-    |                          type must be known at this point
-    |
-    |
- note: multiple `impl`s satisfying `u64: Test<_>` found
-   --> $DIR/issue-69455.rs:11:1
-    |
- LL | impl Test<u32> for u64 {
- ...
- ...
- LL | impl Test<u64> for u64 {
- help: consider specifying the generic argument
-    |
-    |
- LL |     println!("{}", 23u64.test(xs.iter().sum::<S>()));
-    |                                            +++++
+    |                          ^^^^ cannot call trait method as a free function
34 error: aborting due to 2 previous errors
35 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69455/issue-69455.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-69455.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-69455.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69455" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69455/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-69455.rs:29:20
   |
   |
LL |     println!("{}", 23u64.test(xs.iter().sum())); //~ ERROR: type annotations needed
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type of the type parameter `T` declared on the associated function `new_display`
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider specifying the generic argument
   |
   |
LL |     println!("{}", 23u64.test(xs.iter().sum())::<T>); //~ ERROR: type annotations needed

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-69455.rs:29:26
   |
   |
LL |     println!("{}", 23u64.test(xs.iter().sum())); //~ ERROR: type annotations needed
   |                          ^^^^ cannot call trait method as a free function
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0282, E0283.
For more information about an error, try `rustc --explain E0282`.
---
diff of stderr:

13   --> $DIR/method-ambig-one-trait-unknown-int-type.rs:26:7
14    |
15 LL |     x.foo();
-    |
-    |
- note: multiple `impl`s satisfying `Vec<_>: Foo` found
-   --> $DIR/method-ambig-one-trait-unknown-int-type.rs:9:1
- LL | impl Foo for Vec<usize> {
-    | ^^^^^^^^^^^^^^^^^^^^^^^
- ...
- LL | impl Foo for Vec<isize> {
- LL | impl Foo for Vec<isize> {
-    | ^^^^^^^^^^^^^^^^^^^^^^^
- help: try using a fully qualified path to specify the expected types
-    |
- LL |     <Vec<T> as Foo>::foo(&x);
-    |     ++++++++++++++++++++++ ~
+    |       ^^^ cannot call trait method as a free function
31 error[E0308]: mismatched types
32   --> $DIR/method-ambig-one-trait-unknown-int-type.rs:33:20



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-one-trait-unknown-int-type/method-ambig-one-trait-unknown-int-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args methods/method-ambig-one-trait-unknown-int-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-ambig-one-trait-unknown-int-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-one-trait-unknown-int-type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-one-trait-unknown-int-type/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0282]: type annotations needed for `Vec<T>`
   |
LL |     let mut x = Vec::new();
   |         ^^^^^
   |
   |
help: consider giving `x` an explicit type, where the type for type parameter `T` is specified
   |
LL |     let mut x: Vec<T> = Vec::new();

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/methods/method-ambig-one-trait-unknown-int-type.rs:26:7
   |
   |
LL |     x.foo(); //~ ERROR type annotations needed
   |       ^^^ cannot call trait method as a free function
error[E0308]: mismatched types
  --> /checkout/src/test/ui/methods/method-ambig-one-trait-unknown-int-type.rs:33:20
   |
   |
LL |     let y: usize = x.foo(); //~ ERROR mismatched types
   |            -----   ^^^^^^^ expected `usize`, found `isize`
   |            expected due to this
   |
   |
help: you can convert an `isize` to a `usize` and panic if the converted value doesn't fit
   |
LL |     let y: usize = x.foo().try_into().unwrap(); //~ ERROR mismatched types

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0282, E0283, E0308.
Some errors have detailed explanations: E0282, E0283, E0308.
For more information about an error, try `rustc --explain E0282`.
------------------------------------------


---- [ui] src/test/ui/structs/struct-record-suggestion.rs stdout ----
diff of stderr:

18 LL |     let q = A { c: 5, .. Default::default() };
20 
- error: aborting due to 2 previous errors
+ error[E0283]: type annotations needed
+   --> $DIR/struct-record-suggestion.rs:10:25
+   --> $DIR/struct-record-suggestion.rs:10:25
+    |
+ LL |     let q = A { c: 5 .. Default::default() };
+    |                         ^^^^^^^^^^^^^^^^ cannot call trait method as a free function
- Some errors have detailed explanations: E0063, E0308.
+ error: aborting due to 3 previous errors
+ 
+ Some errors have detailed explanations: E0063, E0283, E0308.
---
To only update this specific test, also pass `--test-args structs/struct-record-suggestion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/structs/struct-record-suggestion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/struct-record-suggestion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/struct-record-suggestion/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/structs/struct-record-suggestion.rs:10:20
   |
   |
LL |     let q = A { c: 5 .. Default::default() };
   |                    ^^^^^^^^^^^^^^^^^^^^^^^ expected `u64`, found struct `std::ops::Range`
   = note: expected type `u64`
            found struct `std::ops::Range<{integer}>`


error[E0063]: missing fields `b` and `d` in initializer of `A`
   |
   |
LL |     let q = A { c: 5 .. Default::default() };
   |             ^ missing `b` and `d`
   |
help: to set the remaining fields from `Default::default()`, separate the last named field with a comma
   |
LL |     let q = A { c: 5, .. Default::default() };

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/structs/struct-record-suggestion.rs:10:25
   |
   |
LL |     let q = A { c: 5 .. Default::default() };
   |                         ^^^^^^^^^^^^^^^^ cannot call trait method as a free function
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0063, E0283, E0308.
For more information about an error, try `rustc --explain E0063`.
---
diff of stderr:

13   --> $DIR/do-not-mention-type-params-by-name-in-suggestion-issue-96292.rs:17:11
14    |
15 LL |     thing.method(42);
-    |
-    |
- note: multiple `impl`s satisfying `Thing<bool>: Method<_>` found
-   --> $DIR/do-not-mention-type-params-by-name-in-suggestion-issue-96292.rs:7:1
-    |
- LL | impl<X> Method<i32> for Thing<X> {
- ...
- ...
- LL | impl<X> Method<u32> for Thing<X> {
- help: try using a fully qualified path to specify the expected types
-    |
-    |
- LL |     <Thing<bool> as Method<T>>::method(thing, 42);
-    |     +++++++++++++++++++++++++++++++++++     ~
+    |           ^^^^^^ cannot call trait method as a free function
31 error: aborting due to 2 previous errors
32 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/do-not-mention-type-params-by-name-in-suggestion-issue-96292/do-not-mention-type-params-by-name-in-suggestion-issue-96292.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/do-not-mention-type-params-by-name-in-suggestion-issue-96292.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/do-not-mention-type-params-by-name-in-suggestion-issue-96292.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/do-not-mention-type-params-by-name-in-suggestion-issue-96292" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/do-not-mention-type-params-by-name-in-suggestion-issue-96292/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/traits/do-not-mention-type-params-by-name-in-suggestion-issue-96292.rs:17:11
   |
   |
LL |     thing.method(42);
   |
help: try using a fully qualified path to specify the expected types
   |
   |
LL |     <Thing<bool> as Method<T>>::method(thing, 42);

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/do-not-mention-type-params-by-name-in-suggestion-issue-96292.rs:17:11
   |
   |
LL |     thing.method(42);
   |           ^^^^^^ cannot call trait method as a free function
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0282, E0283.
For more information about an error, try `rustc --explain E0282`.
---
21 error[E0283]: type annotations needed
-   --> $DIR/issue-77982.rs:8:10
+   --> $DIR/issue-77982.rs:8:18
23    |
24 LL |     opts.get(opt.as_ref());
-    |          ^^^     ------ type must be known at this point
-    |          |
-    |          cannot infer type of the type parameter `Q` declared on the associated function `get`
-    |
-    = note: multiple `impl`s satisfying `String: AsRef<_>` found in the following crates: `alloc`, `std`:
-            - impl AsRef<OsStr> for String;
-            - impl AsRef<Path> for String;
-            - impl AsRef<[u8]> for String;
-            - impl AsRef<str> for String;
- help: consider specifying the generic argument
-    |
- LL |     opts.get::<Q>(opt.as_ref());
-    |             +++++
+    |                  ^^^^^^ cannot call trait method as a free function
39 error[E0283]: type annotations needed
-   --> $DIR/issue-77982.rs:13:59
+   --> $DIR/issue-77982.rs:13:44
41    |
41    |
42 LL |     let ips: Vec<_> = (0..100_000).map(|_| u32::from(0u32.into())).collect();
-    |                                            |
-    |                                            type must be known at this point
-    |
-    |
-    = note: multiple `impl`s satisfying `u32: From<_>` found in the following crates: `core`, `std`:
-            - impl From<Ipv4Addr> for u32;
-            - impl From<NonZeroU32> for u32;
-            - impl From<bool> for u32;
-            - impl From<char> for u32;
-            and 3 more
- help: try using a fully qualified path to specify the expected types
-    |
- LL |     let ips: Vec<_> = (0..100_000).map(|_| u32::from(<u32 as Into<T>>::into(0u32))).collect();
-    |                                                      +++++++++++++++++++++++    ~
+    |                                            ^^^^^^^^^ cannot call trait method as a free function
- error[E0283]: type annotations needed for `Box<T>`
-   --> $DIR/issue-77982.rs:36:9
+ error[E0283]: type annotations needed
+   --> $DIR/issue-77982.rs:36:16
+   --> $DIR/issue-77982.rs:36:16
60    |
61 LL |     let _ = ().foo();
-    |         ^      --- type must be known at this point
-    |
- note: multiple `impl`s satisfying `(): Foo<'_, _>` found
-   --> $DIR/issue-77982.rs:29:1
-    |
- LL | impl Foo<'static, u32> for () {}
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- LL | impl<'a> Foo<'a, i16> for () {}
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- help: consider giving this pattern a type, where the type for type parameter `T` is specified
-    |
- LL |     let _: Box<T> = ().foo();
-    |          ++++++++
+    |                ^^^ cannot call trait method as a free function
- error[E0283]: type annotations needed for `Box<T>`
-   --> $DIR/issue-77982.rs:40:9
+ error[E0283]: type annotations needed
+   --> $DIR/issue-77982.rs:40:19
+   --> $DIR/issue-77982.rs:40:19
78    |
79 LL |     let _ = (&()).bar();
-    |         ^         --- type must be known at this point
-    |
- note: multiple `impl`s satisfying `&(): Bar<'_, _>` found
-   --> $DIR/issue-77982.rs:32:1
-    |
- LL | impl<'a> Bar<'static, u32> for &'a () {}
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- LL | impl<'a> Bar<'a, i16> for &'a () {}
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- help: consider giving this pattern a type, where the type for type parameter `T` is specified
-    |
- LL |     let _: Box<T> = (&()).bar();
-    |          ++++++++
+    |                   ^^^ cannot call trait method as a free function
94 error: aborting due to 5 previous errors
95 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982/issue-77982.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/issue-77982.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-77982.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/traits/issue-77982.rs:8:10
   |
   |
LL |     opts.get(opt.as_ref()); //~ ERROR type annotations needed
   |          ^^^ cannot infer type of the type parameter `Q` declared on the associated function `get`
   |
   = note: multiple `impl`s satisfying `String: Borrow<_>` found in the following crates: `alloc`, `core`:
           - impl Borrow<str> for String;
           - impl<T> Borrow<T> for T
             where T: ?Sized;
note: required by a bound in `HashMap::<K, V, S>::get`
   |
   |
LL |         K: Borrow<Q>,
   |            ^^^^^^^^^ required by this bound in `HashMap::<K, V, S>::get`
help: consider specifying the type argument in the function call
   |
LL |     opts.get::<Q>(opt.as_ref()); //~ ERROR type annotations needed

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:8:18
   |
   |
LL |     opts.get(opt.as_ref()); //~ ERROR type annotations needed
   |                  ^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:13:44
   |
   |
LL |     let ips: Vec<_> = (0..100_000).map(|_| u32::from(0u32.into())).collect();
   |                                            ^^^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:36:16
   |
   |
LL |     let _ = ().foo(); //~ ERROR type annotations needed
   |                ^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:40:19
   |
   |
LL |     let _ = (&()).bar(); //~ ERROR type annotations needed
   |                   ^^^ cannot call trait method as a free function
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0283`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/traits/multidispatch-convert-ambig-dest.rs stdout ----
diff of stderr:

35 LL |     test::<T, U>(22, std::default::Default::default());
37 
- error: aborting due to 2 previous errors
+ error[E0283]: type annotations needed
+   --> $DIR/multidispatch-convert-ambig-dest.rs:26:14
+   --> $DIR/multidispatch-convert-ambig-dest.rs:26:14
+    |
+ LL |     test(22, std::default::Default::default());
+    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot call trait method as a free function
+ error: aborting due to 3 previous errors
39 
40 Some errors have detailed explanations: E0282, E0283.
41 For more information about an error, try `rustc --explain E0282`.
---
To only update this specific test, also pass `--test-args traits/multidispatch-convert-ambig-dest.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/multidispatch-convert-ambig-dest.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/multidispatch-convert-ambig-dest" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/multidispatch-convert-ambig-dest/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/traits/multidispatch-convert-ambig-dest.rs:26:5
   |
LL |     test(22, std::default::Default::default());
LL |     test(22, std::default::Default::default());
   |     ^^^^ cannot infer type of the type parameter `U` declared on the function `test`
help: consider specifying the generic arguments
   |
   |
LL |     test::<i32, U>(22, std::default::Default::default());

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/multidispatch-convert-ambig-dest.rs:26:5
   |
   |
LL |     test(22, std::default::Default::default());
   |     ^^^^ cannot infer type of the type parameter `U` declared on the function `test`
   |
note: multiple `impl`s satisfying `i32: Convert<_>` found
   |
   |
LL | impl Convert<i8> for i32 {
...
...
LL | impl Convert<i16> for i32 {
note: required by a bound in `test`
  --> /checkout/src/test/ui/traits/multidispatch-convert-ambig-dest.rs:21:11
   |
   |
LL | fn test<T,U>(_: T, _: U)
   |    ---- required by a bound in this
LL | where T : Convert<U>
   |           ^^^^^^^^^^ required by this bound in `test`
help: consider specifying the type arguments in the function call
   |
LL |     test::<T, U>(22, std::default::Default::default());

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/multidispatch-convert-ambig-dest.rs:26:14
   |
   |
LL |     test(22, std::default::Default::default());
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot call trait method as a free function
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0282, E0283.
For more information about an error, try `rustc --explain E0282`.
---
- error: aborting due to previous error
+ error[E0283]: type annotations needed
+   --> $DIR/no-fallback-multiple-impls.rs:14:7
+    |
+ LL |     0.foo();
+    |       ^^^ cannot call trait method as a free function
- For more information about this error, try `rustc --explain E0425`.
+ error: aborting due to 2 previous errors
+ 
+ Some errors have detailed explanations: E0283, E0425.
---
To only update this specific test, also pass `--test-args traits/no-fallback-multiple-impls.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/no-fallback-multiple-impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/no-fallback-multiple-impls" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/no-fallback-multiple-impls/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find function `missing` in this scope
   |
LL |     missing();
   |     ^^^^^^^ not found in this scope


error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/no-fallback-multiple-impls.rs:14:7
   |
LL |     0.foo();
   |       ^^^ cannot call trait method as a free function
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0283, E0425.
For more information about an error, try `rustc --explain E0283`.
---
diff of stderr:

2   --> $DIR/static-method-generic-inference.rs:24:25
3    |
4 LL |     let _f: base::Foo = base::HasNew::new();
-    |
-    |
-    = note: cannot satisfy `_: HasNew<Foo>`
+    |                         ^^^^^^^^^^^^^^^^^ cannot call trait method as a free function
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/static-method-generic-inference/static-method-generic-inference.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/static-method-generic-inference.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/static-method-generic-inference.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/static-method-generic-inference" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/static-method-generic-inference/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/traits/static-method-generic-inference.rs:24:25
   |
   |
LL |     let _f: base::Foo = base::HasNew::new();
   |                         ^^^^^^^^^^^^^^^^^ cannot call trait method as a free function
error: aborting due to previous error

For more information about this error, try `rustc --explain E0283`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/traits/suggest-fully-qualified-path-without-adjustment.rs stdout ----
diff of stderr:

13   --> $DIR/suggest-fully-qualified-path-without-adjustment.rs:45:15
14    |
15 LL |     ref_thing.method();
-    |
-    |
- note: multiple `impl`s satisfying `Thing: Method<_>` found
-   --> $DIR/suggest-fully-qualified-path-without-adjustment.rs:10:1
-    |
- LL | impl Method<i32> for Thing {
- ...
- ...
- LL | impl Method<u32> for Thing {
- help: try using a fully qualified path to specify the expected types
-    |
-    |
- LL |     <Thing as Method<T>>::method(ref_thing);
-    |     +++++++++++++++++++++++++++++         ~
+    |               ^^^^^^ cannot call trait method as a free function
31 error[E0283]: type annotations needed
32   --> $DIR/suggest-fully-qualified-path-without-adjustment.rs:48:15

33    |
33    |
34 LL |     ref_thing.by_self();
-    |
-    |
- note: multiple `impl`s satisfying `&Thing: MethodRef<_>` found
-   --> $DIR/suggest-fully-qualified-path-without-adjustment.rs:23:1
-    |
- LL | impl MethodRef<i32> for &Thing {
- ...
- ...
- LL | impl MethodRef<u32> for &Thing {
- help: try using a fully qualified path to specify the expected types
-    |
-    |
- LL |     <&Thing as MethodRef<T>>::by_self(ref_thing);
-    |     ++++++++++++++++++++++++++++++++++         ~
+    |               ^^^^^^^ cannot call trait method as a free function
50 error[E0283]: type annotations needed
51   --> $DIR/suggest-fully-qualified-path-without-adjustment.rs:51:15

52    |
52    |
53 LL |     mut_thing.method();
-    |
-    |
- note: multiple `impl`s satisfying `Thing: Method<_>` found
-   --> $DIR/suggest-fully-qualified-path-without-adjustment.rs:10:1
-    |
- LL | impl Method<i32> for Thing {
- ...
- ...
- LL | impl Method<u32> for Thing {
- help: try using a fully qualified path to specify the expected types
-    |
-    |
- LL |     <Thing as Method<T>>::method(mut_thing);
-    |     +++++++++++++++++++++++++++++         ~
+    |               ^^^^^^ cannot call trait method as a free function
69 error[E0283]: type annotations needed
70   --> $DIR/suggest-fully-qualified-path-without-adjustment.rs:52:15

71    |
71    |
72 LL |     mut_thing.mut_method();
-    |
-    |
- note: multiple `impl`s satisfying `Thing: Method<_>` found
-   --> $DIR/suggest-fully-qualified-path-without-adjustment.rs:10:1
-    |
- LL | impl Method<i32> for Thing {
- ...
- ...
- LL | impl Method<u32> for Thing {
- help: try using a fully qualified path to specify the expected types
-    |
-    |
- LL |     <Thing as Method<T>>::mut_method(mut_thing);
-    |     +++++++++++++++++++++++++++++++++         ~
+    |               ^^^^^^^^^^ cannot call trait method as a free function
88 error[E0283]: type annotations needed
89   --> $DIR/suggest-fully-qualified-path-without-adjustment.rs:53:15

90    |
90    |
91 LL |     mut_thing.by_self();
-    |
-    |
- note: multiple `impl`s satisfying `&Thing: MethodRef<_>` found
-   --> $DIR/suggest-fully-qualified-path-without-adjustment.rs:23:1
-    |
- LL | impl MethodRef<i32> for &Thing {
- ...
- ...
- LL | impl MethodRef<u32> for &Thing {
- help: try using a fully qualified path to specify the expected types
-    |
-    |
- LL |     <&Thing as MethodRef<T>>::by_self(mut_thing);
-    |     ++++++++++++++++++++++++++++++++++         ~
+    |               ^^^^^^^ cannot call trait method as a free function
107 error[E0283]: type annotations needed
108   --> $DIR/suggest-fully-qualified-path-without-adjustment.rs:56:14

109    |
109    |
110 LL |     deref_to.method();
-    |
-    |
- note: multiple `impl`s satisfying `Thing: Method<_>` found
-   --> $DIR/suggest-fully-qualified-path-without-adjustment.rs:10:1
-    |
- LL | impl Method<i32> for Thing {
- ...
- ...
- LL | impl Method<u32> for Thing {
- help: try using a fully qualified path to specify the expected types
-    |
-    |
- LL |     <Thing as Method<T>>::method(deref_to);
-    |     +++++++++++++++++++++++++++++        ~
+    |              ^^^^^^ cannot call trait method as a free function
126 error[E0283]: type annotations needed
127   --> $DIR/suggest-fully-qualified-path-without-adjustment.rs:57:14

128    |
128    |
129 LL |     deref_to.mut_method();
-    |
-    |
- note: multiple `impl`s satisfying `Thing: Method<_>` found
-   --> $DIR/suggest-fully-qualified-path-without-adjustment.rs:10:1
-    |
- LL | impl Method<i32> for Thing {
- ...
- ...
- LL | impl Method<u32> for Thing {
- help: try using a fully qualified path to specify the expected types
-    |
-    |
- LL |     <Thing as Method<T>>::mut_method(deref_to);
-    |     +++++++++++++++++++++++++++++++++        ~
+    |              ^^^^^^^^^^ cannot call trait method as a free function
145 error[E0283]: type annotations needed
146   --> $DIR/suggest-fully-qualified-path-without-adjustment.rs:58:14

147    |
147    |
148 LL |     deref_to.by_self();
-    |
-    |
- note: multiple `impl`s satisfying `&Thing: MethodRef<_>` found
-   --> $DIR/suggest-fully-qualified-path-without-adjustment.rs:23:1
-    |
- LL | impl MethodRef<i32> for &Thing {
- ...
- ...
- LL | impl MethodRef<u32> for &Thing {
- help: try using a fully qualified path to specify the expected types
-    |
-    |
- LL |     <&Thing as MethodRef<T>>::by_self(deref_to);
-    |     ++++++++++++++++++++++++++++++++++        ~
+    |              ^^^^^^^ cannot call trait method as a free function
164 error[E0283]: type annotations needed
165   --> $DIR/suggest-fully-qualified-path-without-adjustment.rs:61:20

166    |
166    |
167 LL |     deref_deref_to.method();
-    |
-    |
- note: multiple `impl`s satisfying `Thing: Method<_>` found
-   --> $DIR/suggest-fully-qualified-path-without-adjustment.rs:10:1
-    |
- LL | impl Method<i32> for Thing {
- ...
- ...
- LL | impl Method<u32> for Thing {
- help: try using a fully qualified path to specify the expected types
-    |
-    |
- LL |     <Thing as Method<T>>::method(deref_deref_to);
-    |     +++++++++++++++++++++++++++++              ~
+    |                    ^^^^^^ cannot call trait method as a free function
183 error[E0283]: type annotations needed
184   --> $DIR/suggest-fully-qualified-path-without-adjustment.rs:62:20

185    |
185    |
186 LL |     deref_deref_to.mut_method();
-    |
-    |
- note: multiple `impl`s satisfying `Thing: Method<_>` found
-   --> $DIR/suggest-fully-qualified-path-without-adjustment.rs:10:1
-    |
- LL | impl Method<i32> for Thing {
- ...
- ...
- LL | impl Method<u32> for Thing {
- help: try using a fully qualified path to specify the expected types
-    |
-    |
- LL |     <Thing as Method<T>>::mut_method(deref_deref_to);
-    |     +++++++++++++++++++++++++++++++++              ~
+    |                    ^^^^^^^^^^ cannot call trait method as a free function
202 error[E0283]: type annotations needed
203   --> $DIR/suggest-fully-qualified-path-without-adjustment.rs:63:20

204    |
204    |
205 LL |     deref_deref_to.by_self();
-    |
-    |
- note: multiple `impl`s satisfying `&Thing: MethodRef<_>` found
-   --> $DIR/suggest-fully-qualified-path-without-adjustment.rs:23:1
-    |
- LL | impl MethodRef<i32> for &Thing {
- ...
- ...
- LL | impl MethodRef<u32> for &Thing {
- help: try using a fully qualified path to specify the expected types
-    |
-    |
- LL |     <&Thing as MethodRef<T>>::by_self(deref_deref_to);
-    |     ++++++++++++++++++++++++++++++++++              ~
+    |                    ^^^^^^^ cannot call trait method as a free function
221 error: aborting due to 12 previous errors
222 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-fully-qualified-path-without-adjustment/suggest-fully-qualified-path-without-adjustment.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/suggest-fully-qualified-path-without-adjustment.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/suggest-fully-qualified-path-without-adjustment.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-fully-qualified-path-without-adjustment" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-fully-qualified-path-without-adjustment/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-path-without-adjustment.rs:45:15
   |
   |
LL |     ref_thing.method();
   |
help: try using a fully qualified path to specify the expected types
   |
   |
LL |     <Thing as Method<T>>::method(ref_thing);

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-path-without-adjustment.rs:45:15
   |
   |
LL |     ref_thing.method();
   |               ^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-path-without-adjustment.rs:48:15
   |
   |
LL |     ref_thing.by_self(); //~ ERROR type annotations needed
   |               ^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-path-without-adjustment.rs:51:15
   |
   |
LL |     mut_thing.method(); //~ ERROR type annotations needed
   |               ^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-path-without-adjustment.rs:52:15
   |
   |
LL |     mut_thing.mut_method(); //~ ERROR type annotations needed
   |               ^^^^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-path-without-adjustment.rs:53:15
   |
   |
LL |     mut_thing.by_self(); //~ ERROR type annotations needed
   |               ^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-path-without-adjustment.rs:56:14
   |
   |
LL |     deref_to.method(); //~ ERROR type annotations needed
   |              ^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-path-without-adjustment.rs:57:14
   |
   |
LL |     deref_to.mut_method(); //~ ERROR type annotations needed
   |              ^^^^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-path-without-adjustment.rs:58:14
   |
   |
LL |     deref_to.by_self(); //~ ERROR type annotations needed
   |              ^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-path-without-adjustment.rs:61:20
   |
   |
LL |     deref_deref_to.method(); //~ ERROR type annotations needed
   |                    ^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-path-without-adjustment.rs:62:20
   |
   |
LL |     deref_deref_to.mut_method(); //~ ERROR type annotations needed
   |                    ^^^^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-path-without-adjustment.rs:63:20
   |
   |
LL |     deref_deref_to.by_self(); //~ ERROR type annotations needed
   |                    ^^^^^^^ cannot call trait method as a free function
error: aborting due to 12 previous errors

Some errors have detailed explanations: E0282, E0283.
For more information about an error, try `rustc --explain E0282`.
---
diff of stderr:

13   --> $DIR/suggest-fully-qualified-path-with-adjustment.rs:45:11
14    |
15 LL |     thing.method();
-    |
-    |
- note: multiple `impl`s satisfying `Thing: Method<_>` found
-   --> $DIR/suggest-fully-qualified-path-with-adjustment.rs:10:1
-    |
- LL | impl Method<i32> for Thing {
- ...
- ...
- LL | impl Method<u32> for Thing {
- help: try using a fully qualified path to specify the expected types
-    |
-    |
- LL |     <Thing as Method<T>>::method(&thing);
-    |     ++++++++++++++++++++++++++++++     ~
+    |           ^^^^^^ cannot call trait method as a free function
31 error[E0283]: type annotations needed
32   --> $DIR/suggest-fully-qualified-path-with-adjustment.rs:48:11

33    |
33    |
34 LL |     thing.mut_method();
-    |
-    |
- note: multiple `impl`s satisfying `Thing: Method<_>` found
-   --> $DIR/suggest-fully-qualified-path-with-adjustment.rs:10:1
-    |
- LL | impl Method<i32> for Thing {
- ...
- ...
- LL | impl Method<u32> for Thing {
- help: try using a fully qualified path to specify the expected types
-    |
-    |
- LL |     <Thing as Method<T>>::mut_method(&mut thing);
-    |     +++++++++++++++++++++++++++++++++++++      ~
+    |           ^^^^^^^^^^ cannot call trait method as a free function
50 error[E0283]: type annotations needed
51   --> $DIR/suggest-fully-qualified-path-with-adjustment.rs:49:11

52    |
52    |
53 LL |     thing.by_self();
-    |
-    |
- note: multiple `impl`s satisfying `&Thing: MethodRef<_>` found
-   --> $DIR/suggest-fully-qualified-path-with-adjustment.rs:22:1
-    |
- LL | impl MethodRef<i32> for &Thing {
- ...
- ...
- LL | impl MethodRef<u32> for &Thing {
- help: try using a fully qualified path to specify the expected types
-    |
-    |
- LL |     <&Thing as MethodRef<T>>::by_self(&thing);
-    |     +++++++++++++++++++++++++++++++++++     ~
+    |           ^^^^^^^ cannot call trait method as a free function
69 error[E0283]: type annotations needed
70   --> $DIR/suggest-fully-qualified-path-with-adjustment.rs:52:14

71    |
71    |
72 LL |     deref_to.method();
-    |
-    |
- note: multiple `impl`s satisfying `Thing: Method<_>` found
-   --> $DIR/suggest-fully-qualified-path-with-adjustment.rs:10:1
-    |
- LL | impl Method<i32> for Thing {
- ...
- ...
- LL | impl Method<u32> for Thing {
- help: try using a fully qualified path to specify the expected types
-    |
-    |
- LL |     <Thing as Method<T>>::method(&deref_to);
-    |     ++++++++++++++++++++++++++++++        ~
+    |              ^^^^^^ cannot call trait method as a free function
88 error[E0283]: type annotations needed
89   --> $DIR/suggest-fully-qualified-path-with-adjustment.rs:53:14

90    |
90    |
91 LL |     deref_to.mut_method();
-    |
-    |
- note: multiple `impl`s satisfying `Thing: Method<_>` found
-   --> $DIR/suggest-fully-qualified-path-with-adjustment.rs:10:1
-    |
- LL | impl Method<i32> for Thing {
- ...
- ...
- LL | impl Method<u32> for Thing {
- help: try using a fully qualified path to specify the expected types
-    |
-    |
- LL |     <Thing as Method<T>>::mut_method(&mut deref_to);
-    |     +++++++++++++++++++++++++++++++++++++         ~
+    |              ^^^^^^^^^^ cannot call trait method as a free function
107 error[E0283]: type annotations needed
108   --> $DIR/suggest-fully-qualified-path-with-adjustment.rs:54:14

109    |
109    |
110 LL |     deref_to.by_self();
-    |
-    |
- note: multiple `impl`s satisfying `&Thing: MethodRef<_>` found
-   --> $DIR/suggest-fully-qualified-path-with-adjustment.rs:22:1
-    |
- LL | impl MethodRef<i32> for &Thing {
- ...
- ...
- LL | impl MethodRef<u32> for &Thing {
- help: try using a fully qualified path to specify the expected types
-    |
-    |
- LL |     <&Thing as MethodRef<T>>::by_self(&deref_to);
-    |     +++++++++++++++++++++++++++++++++++        ~
+    |              ^^^^^^^ cannot call trait method as a free function
126 error[E0283]: type annotations needed
127   --> $DIR/suggest-fully-qualified-path-with-adjustment.rs:57:20

128    |
128    |
129 LL |     deref_deref_to.method();
-    |
-    |
- note: multiple `impl`s satisfying `Thing: Method<_>` found
-   --> $DIR/suggest-fully-qualified-path-with-adjustment.rs:10:1
-    |
- LL | impl Method<i32> for Thing {
- ...
- ...
- LL | impl Method<u32> for Thing {
- help: try using a fully qualified path to specify the expected types
-    |
-    |
- LL |     <Thing as Method<T>>::method(&deref_deref_to);
-    |     ++++++++++++++++++++++++++++++              ~
+    |                    ^^^^^^ cannot call trait method as a free function
145 error[E0283]: type annotations needed
146   --> $DIR/suggest-fully-qualified-path-with-adjustment.rs:58:20

147    |
147    |
148 LL |     deref_deref_to.mut_method();
-    |
-    |
- note: multiple `impl`s satisfying `Thing: Method<_>` found
-   --> $DIR/suggest-fully-qualified-path-with-adjustment.rs:10:1
-    |
- LL | impl Method<i32> for Thing {
- ...
- ...
- LL | impl Method<u32> for Thing {
- help: try using a fully qualified path to specify the expected types
-    |
-    |
- LL |     <Thing as Method<T>>::mut_method(&mut deref_deref_to);
-    |     +++++++++++++++++++++++++++++++++++++               ~
+    |                    ^^^^^^^^^^ cannot call trait method as a free function
164 error[E0283]: type annotations needed
165   --> $DIR/suggest-fully-qualified-path-with-adjustment.rs:59:20

166    |
166    |
167 LL |     deref_deref_to.by_self();
-    |
-    |
- note: multiple `impl`s satisfying `&Thing: MethodRef<_>` found
-   --> $DIR/suggest-fully-qualified-path-with-adjustment.rs:22:1
-    |
- LL | impl MethodRef<i32> for &Thing {
- ...
- ...
- LL | impl MethodRef<u32> for &Thing {
- help: try using a fully qualified path to specify the expected types
-    |
-    |
- LL |     <&Thing as MethodRef<T>>::by_self(&deref_deref_to);
-    |     +++++++++++++++++++++++++++++++++++              ~
+    |                    ^^^^^^^ cannot call trait method as a free function
183 error: aborting due to 10 previous errors
184 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-fully-qualified-path-with-adjustment/suggest-fully-qualified-path-with-adjustment.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/suggest-fully-qualified-path-with-adjustment.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/suggest-fully-qualified-path-with-adjustment.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-fully-qualified-path-with-adjustment" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-fully-qualified-path-with-adjustment/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-path-with-adjustment.rs:45:11
   |
   |
LL |     thing.method();
   |
help: try using a fully qualified path to specify the expected types
   |
   |
LL |     <Thing as Method<T>>::method(&thing);

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-path-with-adjustment.rs:45:11
   |
   |
LL |     thing.method();
   |           ^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-path-with-adjustment.rs:48:11
   |
   |
LL |     thing.mut_method(); //~ ERROR type annotations needed
   |           ^^^^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-path-with-adjustment.rs:49:11
   |
   |
LL |     thing.by_self(); //~ ERROR type annotations needed
   |           ^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-path-with-adjustment.rs:52:14
   |
   |
LL |     deref_to.method(); //~ ERROR type annotations needed
   |              ^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-path-with-adjustment.rs:53:14
   |
   |
LL |     deref_to.mut_method(); //~ ERROR type annotations needed
   |              ^^^^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-path-with-adjustment.rs:54:14
   |
   |
LL |     deref_to.by_self(); //~ ERROR type annotations needed
   |              ^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-path-with-adjustment.rs:57:20
   |
   |
LL |     deref_deref_to.method(); //~ ERROR type annotations needed
   |                    ^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-path-with-adjustment.rs:58:20
   |
   |
LL |     deref_deref_to.mut_method(); //~ ERROR type annotations needed
   |                    ^^^^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-path-with-adjustment.rs:59:20
   |
   |
LL |     deref_deref_to.by_self(); //~ ERROR type annotations needed
   |                    ^^^^^^^ cannot call trait method as a free function
error: aborting due to 10 previous errors

Some errors have detailed explanations: E0282, E0283.
For more information about an error, try `rustc --explain E0282`.
For more information about an error, try `rustc --explain E0282`.
------------------------------------------


---- [ui] src/test/ui/traits/test-2.rs stdout ----
diff of stderr:

79    = note: required because of the requirements on the impl of `CoerceUnsized<Box<dyn bar>>` for `Box<{integer}>`
80    = note: required by cast to type `Box<dyn bar>`
- error: aborting due to 5 previous errors
+ error[E0283]: type annotations needed
+   --> $DIR/test-2.rs:9:8
+    |
+    |
+ LL |     10.dup::<i32>();
+    |        ^^^ cannot call trait method as a free function
- Some errors have detailed explanations: E0038, E0107.
+ error[E0283]: type annotations needed
+   --> $DIR/test-2.rs:11:8
+    |
+    |
+ LL |     10.blah::<i32, i32>();
+    |        ^^^^ cannot call trait method as a free function
+ error: aborting due to 7 previous errors
+ 
+ Some errors have detailed explanations: E0038, E0107, E0283.
85 For more information about an error, try `rustc --explain E0038`.
85 For more information about an error, try `rustc --explain E0038`.
86 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/test-2/test-2.stderr
To only update this specific test, also pass `--test-args traits/test-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/test-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/test-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/test-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0107]: this associated function takes 0 generic arguments but 1 generic argument was supplied
   |
   |
LL |     10.dup::<i32>();
   |        ^^^------- help: remove these generics
   |        expected 0 generic arguments
   |
note: associated function defined here, with 0 generic parameters
  --> /checkout/src/test/ui/traits/test-2.rs:4:16
  --> /checkout/src/test/ui/traits/test-2.rs:4:16
   |
LL | trait bar { fn dup(&self) -> Self; fn blah<X>(&self); }


error[E0107]: this associated function takes 1 generic argument but 2 generic arguments were supplied
   |
   |
LL |     10.blah::<i32, i32>();
   |        ^^^^        --- help: remove this generic argument
   |        expected 1 generic argument
   |
   |
note: associated function defined here, with 1 generic parameter: `X`
   |
   |
LL | trait bar { fn dup(&self) -> Self; fn blah<X>(&self); }


error[E0038]: the trait `bar` cannot be made into an object
   |
   |
LL |     (Box::new(10) as Box<dyn bar>).dup();
   |                      ^^^^^^^^^^^^ `bar` cannot be made into an object
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/traits/test-2.rs:4:30
   |
   |
LL | trait bar { fn dup(&self) -> Self; fn blah<X>(&self); }
   |       ---                    ^^^^     ^^^^ ...because method `blah` has generic type parameters
   |       |                      |
   |       |                      ...because method `dup` references the `Self` type in its return type
   |       this trait cannot be made into an object...
   = help: consider moving `dup` to another trait
   = help: consider moving `blah` to another trait

error[E0038]: the trait `bar` cannot be made into an object
   |
   |
LL |     (Box::new(10) as Box<dyn bar>).dup();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `bar` cannot be made into an object
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/traits/test-2.rs:4:30
   |
   |
LL | trait bar { fn dup(&self) -> Self; fn blah<X>(&self); }
   |       ---                    ^^^^     ^^^^ ...because method `blah` has generic type parameters
   |       |                      |
   |       |                      ...because method `dup` references the `Self` type in its return type
   |       this trait cannot be made into an object...
   = help: consider moving `dup` to another trait
   = help: consider moving `blah` to another trait

error[E0038]: the trait `bar` cannot be made into an object
   |
   |
LL |     (Box::new(10) as Box<dyn bar>).dup();
   |      ^^^^^^^^^^^^ `bar` cannot be made into an object
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/traits/test-2.rs:4:30
   |
   |
LL | trait bar { fn dup(&self) -> Self; fn blah<X>(&self); }
   |       ---                    ^^^^     ^^^^ ...because method `blah` has generic type parameters
   |       |                      |
   |       |                      ...because method `dup` references the `Self` type in its return type
   |       this trait cannot be made into an object...
   = help: consider moving `dup` to another trait
   = help: consider moving `blah` to another trait
   = note: required because of the requirements on the impl of `CoerceUnsized<Box<dyn bar>>` for `Box<{integer}>`
   = note: required by cast to type `Box<dyn bar>`
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/test-2.rs:9:8
   |
   |
LL |     10.dup::<i32>();
   |        ^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/test-2.rs:11:8
   |
   |
LL |     10.blah::<i32, i32>();
   |        ^^^^ cannot call trait method as a free function
error: aborting due to 7 previous errors

Some errors have detailed explanations: E0038, E0107, E0283.
For more information about an error, try `rustc --explain E0038`.
