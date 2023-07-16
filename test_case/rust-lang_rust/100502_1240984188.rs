plain
    Finished release [optimized] target(s) in 21.06s
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 13494 tests
.................................................................................F...F.. 88/13494
ii.....................i..................i............................................. 264/13494
........................................................................................ 352/13494
........................................................................................ 440/13494
........................................................................................ 528/13494
---
diff of stderr:

28 help: did you mean
29    |
30 LL |     f(A, A, B, B, C, C);
+    |      ~~~~~~~~~~~~~~~~~~
32 
33 error[E0308]: arguments to this function are incorrect
34   --> $DIR/issue-101097.rs:17:5
34   --> $DIR/issue-101097.rs:17:5

56 help: did you mean
57    |
58 LL |     f(A, A, B, B, C, C);
+    |      ~~~~~~~~~~~~~~~~~~
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
60 
61 error[E0308]: arguments to this function are incorrect
61 error[E0308]: arguments to this function are incorrect
62   --> $DIR/issue-101097.rs:18:5

87 help: did you mean
88    |
89 LL |     f(A, A, B, B, /* C */, /* C */);
+    |      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
91 
92 error[E0308]: arguments to this function are incorrect
93   --> $DIR/issue-101097.rs:19:5
93   --> $DIR/issue-101097.rs:19:5

119 help: did you mean
120    |
121 LL |     f(A, A, B, B, C, C);
+    |      ~~~~~~~~~~~~~~~~~~
123 
124 error[E0308]: arguments to this function are incorrect
125   --> $DIR/issue-101097.rs:20:5
125   --> $DIR/issue-101097.rs:20:5

152 help: did you mean
153    |
154 LL |     f(A, A, /* B */, B, C, C);
+    |      ~~~~~~~~~~~~~~~~~~~~~~~~
156 
157 error: aborting due to 5 previous errors
158 
---
To only update this specific test, also pass `--test-args argument-suggestions/issue-101097.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/argument-suggestions/issue-101097.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/argument-suggestions/issue-101097" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/argument-suggestions/issue-101097/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/argument-suggestions/issue-101097.rs:16:5
   |
   |
LL |     f(C, A, A, A, B, B, C); //~ ERROR this function takes 6 arguments but 7 arguments were supplied [E0061]
   |     ^ -     -  -  - expected `C`, found `B`
   |       |     |  argument of type `A` unexpected
   |       |     |  argument of type `A` unexpected
   |       |     expected `B`, found `A`
   |       expected `A`, found `C`
note: function defined here
  --> /checkout/src/test/ui/argument-suggestions/issue-101097.rs:6:4
   |
LL | fn f(
---
LL |     c2: C,
   |     -----
help: did you mean
   |
LL |     f(A, A, B, B, C, C); //~ ERROR this function takes 6 arguments but 7 arguments were supplied [E0061]

error[E0308]: arguments to this function are incorrect
  --> /checkout/src/test/ui/argument-suggestions/issue-101097.rs:17:5
   |
   |
LL |     f(C, C, A, A, B, B);  //~ ERROR arguments to this function are incorrect [E0308]
   |
note: function defined here
  --> /checkout/src/test/ui/argument-suggestions/issue-101097.rs:6:4
   |
---
LL |     c2: C,
   |     -----
help: did you mean
   |
LL |     f(A, A, B, B, C, C);  //~ ERROR arguments to this function are incorrect [E0308]

error[E0308]: arguments to this function are incorrect
  --> /checkout/src/test/ui/argument-suggestions/issue-101097.rs:18:5
   |
   |
LL |     f(A, A, D, D, B, B);  //~ arguments to this function are incorrect [E0308]
   |     ^       -  -  ---- two arguments of type `C` and `C` are missing
   |             |  |
   |             |  argument of type `D` unexpected
   |             argument of type `D` unexpected
note: function defined here
  --> /checkout/src/test/ui/argument-suggestions/issue-101097.rs:6:4
   |
LL | fn f(
---
LL |     c2: C,
   |     -----
help: did you mean
   |
LL |     f(A, A, B, B, /* C */, /* C */);  //~ arguments to this function are incorrect [E0308]

error[E0308]: arguments to this function are incorrect
  --> /checkout/src/test/ui/argument-suggestions/issue-101097.rs:19:5
   |
   |
LL |     f(C, C, B, B, A, A);  //~ arguments to this function are incorrect [E0308]
   |     ^ -  -        -  - expected `C`, found `A`
   |       |  |        |
   |       |  |        expected `C`, found `A`
   |       |  expected `A`, found `C`
   |       expected `A`, found `C`
note: function defined here
  --> /checkout/src/test/ui/argument-suggestions/issue-101097.rs:6:4
   |
LL | fn f(
---
LL |     c2: C,
   |     -----
help: did you mean
   |
LL |     f(A, A, B, B, C, C);  //~ arguments to this function are incorrect [E0308]

error[E0308]: arguments to this function are incorrect
  --> /checkout/src/test/ui/argument-suggestions/issue-101097.rs:20:5
   |
   |
LL |     f(C, C, A, B, A, A);  //~ arguments to this function are incorrect [E0308]
   |     ^ -  -  -     -  - expected `C`, found `A`
   |       |  |  |     |
   |       |  |  |     expected `C`, found `A`
   |       |  |  expected struct `B`, found struct `A`
   |       |  expected `A`, found `C`
   |       expected `A`, found `C`
note: function defined here
  --> /checkout/src/test/ui/argument-suggestions/issue-101097.rs:6:4
   |
LL | fn f(
---
LL |     c2: C,
   |     -----
help: did you mean
   |
LL |     f(A, A, /* B */, B, C, C);  //~ arguments to this function are incorrect [E0308]

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0061, E0308.
---
diff of stderr:

15 help: provide the arguments
16    |
17 LL |     three_diff(/* T1 */, T2::new(0), /* T3 */);
+    |               ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
19 
20 error[E0308]: arguments to this function are incorrect
21   --> $DIR/issue-100478.rs:35:5
21   --> $DIR/issue-100478.rs:35:5

35 help: did you mean
36    |
37 LL |     four_shuffle(T1::default(), T2::default(), T3::default(), T4::default());
+    |                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
39 
40 error[E0308]: arguments to this function are incorrect
41   --> $DIR/issue-100478.rs:36:5
41   --> $DIR/issue-100478.rs:36:5

54 help: swap these arguments
55    |
56 LL |     four_shuffle(T1::default(), T2::default(), T3::default(), /* T4 */);
+    |                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
58 
59 error[E0061]: this function takes 8 arguments but 7 arguments were supplied
60   --> $DIR/issue-100478.rs:47:5
60   --> $DIR/issue-100478.rs:47:5

73 help: provide the argument
74    |
75 LL |     foo(p1, /* Arc<T2> */, p3, p4, p5, p6, p7, p8);
+    |        ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
77 
78 error: aborting due to 4 previous errors
79 
---
To only update this specific test, also pass `--test-args argument-suggestions/issue-100478.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/argument-suggestions/issue-100478.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/argument-suggestions/issue-100478" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/argument-suggestions/issue-100478/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/argument-suggestions/issue-100478.rs:34:5
   |
   |
LL |     three_diff(T2::new(0)); //~ ERROR this function takes
   |               ||
   |               ||
   |               |an argument of type `T1` is missing
   |               an argument of type `T3` is missing
note: function defined here
  --> /checkout/src/test/ui/argument-suggestions/issue-100478.rs:30:4
   |
   |
LL | fn three_diff(_a: T1, _b: T2, _c: T3) {}
help: provide the arguments
   |
   |
LL |     three_diff(/* T1 */, T2::new(0), /* T3 */); //~ ERROR this function takes

error[E0308]: arguments to this function are incorrect
  --> /checkout/src/test/ui/argument-suggestions/issue-100478.rs:35:5
   |
   |
LL |     four_shuffle(T3::default(), T4::default(), T1::default(), T2::default()); //~ ERROR 35:5: 35:17: arguments to this function are incor...
   |     ^^^^^^^^^^^^ -------------  -------------  -------------  ------------- expected `T4`, found `T2`
   |                  |              |              |
   |                  |              |              expected `T3`, found `T1`
   |                  |              expected `T2`, found `T4`
   |                  expected `T1`, found `T3`
note: function defined here
  --> /checkout/src/test/ui/argument-suggestions/issue-100478.rs:31:4
   |
   |
LL | fn four_shuffle(_a: T1, _b: T2, _c: T3, _d: T4) {}
help: did you mean
   |
   |
LL |     four_shuffle(T1::default(), T2::default(), T3::default(), T4::default()); //~ ERROR 35:5: 35:17: arguments to this function are incorrect [E0308]

error[E0308]: arguments to this function are incorrect
  --> /checkout/src/test/ui/argument-suggestions/issue-100478.rs:36:5
   |
   |
LL |     four_shuffle(T3::default(), T2::default(), T1::default(), T3::default()); //~ ERROR 36:5: 36:17: arguments to this function are incor...
   |     ^^^^^^^^^^^^ -------------                 -------------  ------------- expected struct `T4`, found struct `T3`
   |                  |                             |
   |                  |                             expected `T3`, found `T1`
   |                  expected `T1`, found `T3`
note: function defined here
  --> /checkout/src/test/ui/argument-suggestions/issue-100478.rs:31:4
   |
   |
LL | fn four_shuffle(_a: T1, _b: T2, _c: T3, _d: T4) {}
help: swap these arguments
   |
   |
LL |     four_shuffle(T1::default(), T2::default(), T3::default(), /* T4 */); //~ ERROR 36:5: 36:17: arguments to this function are incorrect [E0308]

error[E0061]: this function takes 8 arguments but 7 arguments were supplied
  --> /checkout/src/test/ui/argument-suggestions/issue-100478.rs:47:5
   |
   |
LL |     foo(
   |     ^^^
...
LL |         p3, p4, p5, p6, p7, p8,
   |         -- an argument of type `Arc<T2>` is missing
note: function defined here
  --> /checkout/src/test/ui/argument-suggestions/issue-100478.rs:29:4
   |
   |
LL | fn foo(p1: T1, p2: Arc<T2>, p3: T3, p4: Arc<T4>, p5: T5, p6: T6, p7: T7, p8: Arc<T8>) {}
help: provide the argument
   |
   |
LL |     foo(p1, /* Arc<T2> */, p3, p4, p5, p6, p7, p8);

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0061, E0308.
