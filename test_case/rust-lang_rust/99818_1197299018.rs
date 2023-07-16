plain
.......................iii.............................................................. 13200/13242
..........................................
failures:

---- [ui] src/test/ui/nll/closure-malformed-projection-input-issue-99665.rs stdout ----

2   --> $DIR/closure-malformed-projection-input-issue-99665.rs:32:5
3    |
3    |
4 LL |     |_: <Ty1<&u8> as MyComponent>::Properties| {};
-    |     ^-^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    |     ^-^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |     ||
7    |     |has type `<Ty1<&'1 u8> as MyComponent>::Properties`
8    |     requires that `'1` must outlive `'static`
11   --> $DIR/closure-malformed-projection-input-issue-99665.rs:32:5
12    |
12    |
13 LL |     |_: <Ty1<&u8> as MyComponent>::Properties| {};
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
15 
16 error: higher-ranked lifetime error
17   --> $DIR/closure-malformed-projection-input-issue-99665.rs:32:5
17   --> $DIR/closure-malformed-projection-input-issue-99665.rs:32:5

18    |
19 LL |     |_: <Ty1<&u8> as MyComponent>::Properties| {};
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
21    |
21    |
-    = note: could not normalize `&[closure@$DIR/closure-malformed-projection-input-issue-99665.rs:32:5: 32:50]`
+    = note: could not normalize `&[closure@$DIR/closure-malformed-projection-input-issue-99665.rs:32:5: 32:47]`
24 error: higher-ranked subtype error
25   --> $DIR/closure-malformed-projection-input-issue-99665.rs:39:5

26    |
26    |
27 LL |     |_: <Ty2<&u8> as MyComponent>::Properties| {};
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
29 
30 error: higher-ranked lifetime error
31   --> $DIR/closure-malformed-projection-input-issue-99665.rs:39:5
31   --> $DIR/closure-malformed-projection-input-issue-99665.rs:39:5

32    |
33 LL |     |_: <Ty2<&u8> as MyComponent>::Properties| {};
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
35    |
35    |
-    = note: could not normalize `&[closure@$DIR/closure-malformed-projection-input-issue-99665.rs:39:5: 39:50]`
+    = note: could not normalize `&[closure@$DIR/closure-malformed-projection-input-issue-99665.rs:39:5: 39:47]`
38 error: higher-ranked lifetime error
39   --> $DIR/closure-malformed-projection-input-issue-99665.rs:32:5
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


41 LL |     |_: <Ty1<&u8> as MyComponent>::Properties| {};
43    |
43    |
-    = note: could not normalize `[closure@$DIR/closure-malformed-projection-input-issue-99665.rs:32:5: 32:50]`
+    = note: could not normalize `[closure@$DIR/closure-malformed-projection-input-issue-99665.rs:32:5: 32:47]`
46 error: higher-ranked lifetime error
47   --> $DIR/closure-malformed-projection-input-issue-99665.rs:32:5


49 LL |     |_: <Ty1<&u8> as MyComponent>::Properties| {};
51    |
51    |
-    = note: could not normalize `[closure@$DIR/closure-malformed-projection-input-issue-99665.rs:32:5: 32:50]`
+    = note: could not normalize `[closure@$DIR/closure-malformed-projection-input-issue-99665.rs:32:5: 32:47]`
54 error: higher-ranked lifetime error
55   --> $DIR/closure-malformed-projection-input-issue-99665.rs:39:5


57 LL |     |_: <Ty2<&u8> as MyComponent>::Properties| {};
59    |
59    |
-    = note: could not normalize `[closure@$DIR/closure-malformed-projection-input-issue-99665.rs:39:5: 39:50]`
+    = note: could not normalize `[closure@$DIR/closure-malformed-projection-input-issue-99665.rs:39:5: 39:47]`
62 error: higher-ranked lifetime error
63   --> $DIR/closure-malformed-projection-input-issue-99665.rs:39:5


65 LL |     |_: <Ty2<&u8> as MyComponent>::Properties| {};
67    |
67    |
-    = note: could not normalize `[closure@$DIR/closure-malformed-projection-input-issue-99665.rs:39:5: 39:50]`
+    = note: could not normalize `[closure@$DIR/closure-malformed-projection-input-issue-99665.rs:39:5: 39:47]`
70 error: aborting due to 9 previous errors
71 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-malformed-projection-input-issue-99665/closure-malformed-projection-input-issue-99665.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-malformed-projection-input-issue-99665.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-malformed-projection-input-issue-99665.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-malformed-projection-input-issue-99665" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-malformed-projection-input-issue-99665/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/closure-malformed-projection-input-issue-99665.rs:32:5
   |
LL |     |_: <Ty1<&u8> as MyComponent>::Properties| {};
   |     ^-^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     ||
   |     |has type `<Ty1<&'1 u8> as MyComponent>::Properties`
   |     requires that `'1` must outlive `'static`
error: higher-ranked subtype error
  --> /checkout/src/test/ui/nll/closure-malformed-projection-input-issue-99665.rs:32:5
   |
   |
LL |     |_: <Ty1<&u8> as MyComponent>::Properties| {};

error: higher-ranked lifetime error
  --> /checkout/src/test/ui/nll/closure-malformed-projection-input-issue-99665.rs:32:5
   |
   |
LL |     |_: <Ty1<&u8> as MyComponent>::Properties| {};
   |
   |
   = note: could not normalize `&[closure@/checkout/src/test/ui/nll/closure-malformed-projection-input-issue-99665.rs:32:5: 32:47]`
error: higher-ranked subtype error
  --> /checkout/src/test/ui/nll/closure-malformed-projection-input-issue-99665.rs:39:5
   |
   |
LL |     |_: <Ty2<&u8> as MyComponent>::Properties| {};

error: higher-ranked lifetime error
  --> /checkout/src/test/ui/nll/closure-malformed-projection-input-issue-99665.rs:39:5
   |
   |
LL |     |_: <Ty2<&u8> as MyComponent>::Properties| {};
   |
   |
   = note: could not normalize `&[closure@/checkout/src/test/ui/nll/closure-malformed-projection-input-issue-99665.rs:39:5: 39:47]`
error: higher-ranked lifetime error
  --> /checkout/src/test/ui/nll/closure-malformed-projection-input-issue-99665.rs:32:5
   |
   |
LL |     |_: <Ty1<&u8> as MyComponent>::Properties| {};
   |
   |
   = note: could not normalize `[closure@/checkout/src/test/ui/nll/closure-malformed-projection-input-issue-99665.rs:32:5: 32:47]`
error: higher-ranked lifetime error
  --> /checkout/src/test/ui/nll/closure-malformed-projection-input-issue-99665.rs:32:5
   |
   |
LL |     |_: <Ty1<&u8> as MyComponent>::Properties| {};
   |
   |
   = note: could not normalize `[closure@/checkout/src/test/ui/nll/closure-malformed-projection-input-issue-99665.rs:32:5: 32:47]`
error: higher-ranked lifetime error
  --> /checkout/src/test/ui/nll/closure-malformed-projection-input-issue-99665.rs:39:5
   |
   |
LL |     |_: <Ty2<&u8> as MyComponent>::Properties| {};
   |
   |
   = note: could not normalize `[closure@/checkout/src/test/ui/nll/closure-malformed-projection-input-issue-99665.rs:39:5: 39:47]`
error: higher-ranked lifetime error
  --> /checkout/src/test/ui/nll/closure-malformed-projection-input-issue-99665.rs:39:5
   |
   |
LL |     |_: <Ty2<&u8> as MyComponent>::Properties| {};
   |
   |
   = note: could not normalize `[closure@/checkout/src/test/ui/nll/closure-malformed-projection-input-issue-99665.rs:39:5: 39:47]`
error: aborting due to 9 previous errors
------------------------------------------


