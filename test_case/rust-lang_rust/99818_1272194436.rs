plain
........................................................................................ 13640/13644
....
failures:

---- [ui] src/test/ui/nll/closure-malformed-projection-input-issue-102800.rs stdout ----

1 error: lifetime may not live long enough
-   --> $DIR/closure-malformed-projection-input-issue-102800.rs:17:48
+   --> $DIR/closure-malformed-projection-input-issue-102800.rs:18:48
+   --> $DIR/closure-malformed-projection-input-issue-102800.rs:18:48
3    |
4 LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
5    |                                                ^-^

8    |                                                requires that `'1` must outlive `'static`
10 error: higher-ranked subtype error
-   --> $DIR/closure-malformed-projection-input-issue-102800.rs:17:48
+   --> $DIR/closure-malformed-projection-input-issue-102800.rs:18:48
12    |
12    |
13 LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};

15 
16 error: higher-ranked subtype error
-   --> $DIR/closure-malformed-projection-input-issue-102800.rs:17:48
-   --> $DIR/closure-malformed-projection-input-issue-102800.rs:17:48
+   --> $DIR/closure-malformed-projection-input-issue-102800.rs:18:48
18    |
19 LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};

21 
22 error: implementation of `Trait` is not general enough
-   --> $DIR/closure-malformed-projection-input-issue-102800.rs:17:48
-   --> $DIR/closure-malformed-projection-input-issue-102800.rs:17:48
+   --> $DIR/closure-malformed-projection-input-issue-102800.rs:18:48
24    |
25 LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};
26    |                                                ^^^^^^ implementation of `Trait` is not general enough
29    = note: ...but `Trait` is actually implemented for the type `&'static ()`
30 
31 error: implementation of `Trait` is not general enough
-   --> $DIR/closure-malformed-projection-input-issue-102800.rs:17:12
-   --> $DIR/closure-malformed-projection-input-issue-102800.rs:17:12
+   --> $DIR/closure-malformed-projection-input-issue-102800.rs:18:12
33    |
34 LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};
35    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `Trait` is not general enough
38    = note: ...but `Trait` is actually implemented for the type `&'static ()`
39 
40 error: implementation of `Trait` is not general enough
-   --> $DIR/closure-malformed-projection-input-issue-102800.rs:17:12
-   --> $DIR/closure-malformed-projection-input-issue-102800.rs:17:12
+   --> $DIR/closure-malformed-projection-input-issue-102800.rs:18:12
42    |
43 LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};
44    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `Trait` is not general enough
47    = note: ...but `Trait` is actually implemented for the type `&'static ()`
48 
49 error: implementation of `Trait` is not general enough
-   --> $DIR/closure-malformed-projection-input-issue-102800.rs:17:12
-   --> $DIR/closure-malformed-projection-input-issue-102800.rs:17:12
+   --> $DIR/closure-malformed-projection-input-issue-102800.rs:18:12
51    |
52 LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};
53    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `Trait` is not general enough
56    = note: ...but `Trait` is actually implemented for the type `&'static ()`
57 
58 error: implementation of `Trait` is not general enough
-   --> $DIR/closure-malformed-projection-input-issue-102800.rs:17:12
-   --> $DIR/closure-malformed-projection-input-issue-102800.rs:17:12
+   --> $DIR/closure-malformed-projection-input-issue-102800.rs:18:12
60    |
61 LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};
62    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `Trait` is not general enough
65    = note: ...but `Trait` is actually implemented for the type `&'static ()`
66 
67 error: implementation of `Trait` is not general enough
-   --> $DIR/closure-malformed-projection-input-issue-102800.rs:17:48
-   --> $DIR/closure-malformed-projection-input-issue-102800.rs:17:48
+   --> $DIR/closure-malformed-projection-input-issue-102800.rs:18:48
69    |
70 LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};
71    |                                                ^^^^^^ implementation of `Trait` is not general enough
74    = note: ...but `Trait` is actually implemented for the type `&'static ()`
75 
76 error: implementation of `Trait` is not general enough
-   --> $DIR/closure-malformed-projection-input-issue-102800.rs:17:48
-   --> $DIR/closure-malformed-projection-input-issue-102800.rs:17:48
+   --> $DIR/closure-malformed-projection-input-issue-102800.rs:18:48
78    |
79 LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};
80    |                                                ^^^^^^ implementation of `Trait` is not general enough
83    = note: ...but `Trait` is actually implemented for the type `&'static ()`
84 
85 error: implementation of `Trait` is not general enough
-   --> $DIR/closure-malformed-projection-input-issue-102800.rs:17:48
-   --> $DIR/closure-malformed-projection-input-issue-102800.rs:17:48
+   --> $DIR/closure-malformed-projection-input-issue-102800.rs:18:48
87    |
88 LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};
89    |                                                ^^^^^^ implementation of `Trait` is not general enough
92    = note: ...but `Trait` is actually implemented for the type `&'static ()`
93 
94 error: implementation of `Trait` is not general enough
-   --> $DIR/closure-malformed-projection-input-issue-102800.rs:17:48
-   --> $DIR/closure-malformed-projection-input-issue-102800.rs:17:48
+   --> $DIR/closure-malformed-projection-input-issue-102800.rs:18:48
96    |
97 LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};
98    |                                                ^^^^^^ implementation of `Trait` is not general enough

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-malformed-projection-input-issue-102800/closure-malformed-projection-input-issue-102800.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-malformed-projection-input-issue-102800.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-malformed-projection-input-issue-102800.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-malformed-projection-input-issue-102800" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-malformed-projection-input-issue-102800/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/closure-malformed-projection-input-issue-102800.rs:18:48
   |
LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};
   |                                                ^-^
   |                                                ||
   |                                                |has type `<&'1 () as Trait>::Ty`
   |                                                requires that `'1` must outlive `'static`
error: higher-ranked subtype error
  --> /checkout/src/test/ui/nll/closure-malformed-projection-input-issue-102800.rs:18:48
   |
   |
LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};

error: higher-ranked subtype error
  --> /checkout/src/test/ui/nll/closure-malformed-projection-input-issue-102800.rs:18:48
   |
   |
LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};

error: implementation of `Trait` is not general enough
  --> /checkout/src/test/ui/nll/closure-malformed-projection-input-issue-102800.rs:18:48
   |
   |
LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};
   |                                                ^^^^^^ implementation of `Trait` is not general enough
   |
   = note: `&'0 ()` must implement `Trait`, for any lifetime `'0`...
   = note: ...but `Trait` is actually implemented for the type `&'static ()`
error: implementation of `Trait` is not general enough
  --> /checkout/src/test/ui/nll/closure-malformed-projection-input-issue-102800.rs:18:12
   |
   |
LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `Trait` is not general enough
   |
   = note: `&'0 ()` must implement `Trait`, for any lifetime `'0`...
   = note: ...but `Trait` is actually implemented for the type `&'static ()`
error: implementation of `Trait` is not general enough
  --> /checkout/src/test/ui/nll/closure-malformed-projection-input-issue-102800.rs:18:12
   |
   |
LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `Trait` is not general enough
   |
   = note: `&'0 ()` must implement `Trait`, for any lifetime `'0`...
   = note: ...but `Trait` is actually implemented for the type `&'static ()`
error: implementation of `Trait` is not general enough
  --> /checkout/src/test/ui/nll/closure-malformed-projection-input-issue-102800.rs:18:12
   |
   |
LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `Trait` is not general enough
   |
   = note: `&'0 ()` must implement `Trait`, for any lifetime `'0`...
   = note: ...but `Trait` is actually implemented for the type `&'static ()`
error: implementation of `Trait` is not general enough
  --> /checkout/src/test/ui/nll/closure-malformed-projection-input-issue-102800.rs:18:12
   |
   |
LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `Trait` is not general enough
   |
   = note: `&'0 ()` must implement `Trait`, for any lifetime `'0`...
   = note: ...but `Trait` is actually implemented for the type `&'static ()`
error: implementation of `Trait` is not general enough
  --> /checkout/src/test/ui/nll/closure-malformed-projection-input-issue-102800.rs:18:48
   |
   |
LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};
   |                                                ^^^^^^ implementation of `Trait` is not general enough
   |
   = note: `&'0 ()` must implement `Trait`, for any lifetime `'0`...
   = note: ...but `Trait` is actually implemented for the type `&'static ()`
error: implementation of `Trait` is not general enough
  --> /checkout/src/test/ui/nll/closure-malformed-projection-input-issue-102800.rs:18:48
   |
   |
LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};
   |                                                ^^^^^^ implementation of `Trait` is not general enough
   |
   = note: `&'0 ()` must implement `Trait`, for any lifetime `'0`...
   = note: ...but `Trait` is actually implemented for the type `&'static ()`
error: implementation of `Trait` is not general enough
  --> /checkout/src/test/ui/nll/closure-malformed-projection-input-issue-102800.rs:18:48
   |
   |
LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};
   |                                                ^^^^^^ implementation of `Trait` is not general enough
   |
   = note: `&'0 ()` must implement `Trait`, for any lifetime `'0`...
   = note: ...but `Trait` is actually implemented for the type `&'static ()`
error: implementation of `Trait` is not general enough
  --> /checkout/src/test/ui/nll/closure-malformed-projection-input-issue-102800.rs:18:48
   |
   |
LL |     let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};
   |                                                ^^^^^^ implementation of `Trait` is not general enough
   |
   = note: `&'0 ()` must implement `Trait`, for any lifetime `'0`...
   = note: ...but `Trait` is actually implemented for the type `&'static ()`
error: aborting due to 12 previous errors
------------------------------------------


