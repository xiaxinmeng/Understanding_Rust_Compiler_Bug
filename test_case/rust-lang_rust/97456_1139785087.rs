plain
........................................................................................ 11440/13286
........................................................................................ 11528/13286
........................................................................................ 11616/13286
........................................................................................ 11704/13286
..........................................................i........iF.......i.....i..... 11792/13286
........................................................................................ 11968/13286
........................................................................................ 12056/13286
........................................................................................ 12144/13286
........................................................................................ 12232/13286
---

---- [ui] src/test/ui/enum/enum-discrim-autosizing.rs stdout ----
diff of stderr:

- error[E0081]: discriminant value `0` already exists
-   --> $DIR/enum-discrim-autosizing.rs:8:12
+ error[E0081]: discriminant value `0` assigned more than once
3    |
4 LL |     Au64 = 0,
-    |            - first use of `0`
+    |            - first assignment of `0`
+    |            - first assignment of `0`
+ LL |
6 LL |     Bu64 = 0x8000_0000_0000_0000
-    |            ^^^^^^^^^^^^^^^^^^^^^ enum already has `0` (overflowed from `9223372036854775808`)
+    |            ^^^^^^^^^^^^^^^^^^^^^ second assignment of `0` (overflowed from `9223372036854775808`)
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum/enum-discrim-autosizing/enum-discrim-autosizing.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args enum/enum-discrim-autosizing.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/enum/enum-discrim-autosizing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum/enum-discrim-autosizing" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum/enum-discrim-autosizing/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0081]: discriminant value `0` assigned more than once
   |
LL |     Au64 = 0,
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |            - first assignment of `0`
   |            - first assignment of `0`
LL |     //~^NOTE first assignment of `0`
LL |     Bu64 = 0x8000_0000_0000_0000
   |            ^^^^^^^^^^^^^^^^^^^^^ second assignment of `0` (overflowed from `9223372036854775808`)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0081`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/error-codes/E0081.rs stdout ----
diff of stderr:

- error[E0081]: discriminant value `3` already exists
+ error[E0081]: discriminant value `3` assigned more than once
2   --> $DIR/E0081.rs:4:9
4 LL |     P = 3,

-    |         - first use of `3`
+    |         - first assignment of `3`
+    |         - first assignment of `3`
6 LL |
7 LL |     X = 3,
-    |         ^ enum already has `3`
+    |         ^ second assignment of `3`
9 
- error[E0081]: discriminant value `1` already exists
+ error[E0081]: discriminant value `1` assigned more than once
11   --> $DIR/E0081.rs:14:9
13 LL |     P = 257,


-    |         --- first use of `1` (overflowed from `257`)
+    |         --- first assignment of `1` (overflowed from `257`)
16 LL |     X = 513,
16 LL |     X = 513,
-    |         ^^^ enum already has `1` (overflowed from `513`)
+    |         ^^^ second assignment of `1` (overflowed from `513`)
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error[E0081]: discriminant value `-1` assigned more than once
+   --> $DIR/E0081.rs:24:5
+ LL |     First = -1,
+    |             -- first assignment of `-1`
+ ...
+ LL |     Last,
+ LL |     Last,
+    |     ^^^^ second assignment of `-1`
+    |
+    = note: because `Last` has no explicit discriminant, it is being assigned the increment of the previous discriminant
+ error: aborting due to 3 previous errors
20 
21 For more information about this error, try `rustc --explain E0081`.
22 
---
To only update this specific test, also pass `--test-args error-codes/E0081.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0081.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0081" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0081/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0081]: discriminant value `3` assigned more than once
   |
LL |     P = 3,
   |         - first assignment of `3`
   |         - first assignment of `3`
LL |     //~^ NOTE first assignment of `3`
LL |     X = 3,
   |         ^ second assignment of `3`

error[E0081]: discriminant value `1` assigned more than once
   |
LL |     P = 257,
LL |     P = 257,
   |         --- first assignment of `1` (overflowed from `257`)
LL |     //~^ NOTE first assignment of `1` (overflowed from `257`)
LL |     X = 513,
   |         ^^^ second assignment of `1` (overflowed from `513`)

error[E0081]: discriminant value `-1` assigned more than once
   |
LL |     First = -1,
   |             -- first assignment of `-1`
...
...
LL |     Last,
   |     ^^^^ second assignment of `-1`
   |
   = note: because `Last` has no explicit discriminant, it is being assigned the increment of the previous discriminant
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0081`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/issues/issue-15524.rs stdout ----
diff of stderr:

- error[E0081]: discriminant value `1` already exists
+ error[E0081]: discriminant value `1` assigned more than once
3    |
4 LL |     A = 1,

-    |         - first use of `1`
-    |         - first use of `1`
+    |         - first assignment of `1`
6 LL |     B = 1,
-    |         ^ enum already has `1`
+    |         ^ second assignment of `1`
8 
- error[E0081]: discriminant value `1` already exists
+ error[E0081]: discriminant value `1` assigned more than once
11    |
12 LL |     A = 1,

-    |         - first use of `1`
-    |         - first use of `1`
+    |         - first assignment of `1`
14 ...
15 LL |     D,
-    |     ^ enum already has `1`
+    |     ^ second assignment of `1`
+    |
+    = note: because `D` has no explicit discriminant, it is being assigned the increment of the previous discriminant
17 
- error[E0081]: discriminant value `1` already exists
+ error[E0081]: discriminant value `1` assigned more than once
20    |
21 LL |     A = 1,

-    |         - first use of `1`
-    |         - first use of `1`
+    |         - first assignment of `1`
23 ...
24 LL |     E = N,
-    |         ^ enum already has `1`
+    |         ^ second assignment of `1`
27 error: aborting due to 3 previous errors
28 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-15524/issue-15524.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-15524.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-15524.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-15524" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-15524/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0081]: discriminant value `1` assigned more than once
   |
LL |     A = 1,
   |         - first assignment of `1`
LL |     B = 1,
LL |     B = 1,
   |         ^ second assignment of `1`

error[E0081]: discriminant value `1` assigned more than once
   |
LL |     A = 1,
   |         - first assignment of `1`
...
...
LL |     D,
   |     ^ second assignment of `1`
   |
   = note: because `D` has no explicit discriminant, it is being assigned the increment of the previous discriminant

error[E0081]: discriminant value `1` assigned more than once
   |
LL |     A = 1,
   |         - first assignment of `1`
...
---

---- [ui] src/test/ui/tag-variant-disr-dup.rs stdout ----
diff of stderr:

- error[E0081]: discriminant value `0` already exists
+ error[E0081]: discriminant value `0` assigned more than once
3    |
4 LL |     Black = 0x000000,

-    |             -------- first use of `0`
-    |             -------- first use of `0`
+    |             -------- first assignment of `0`
6 LL |     White = 0x000000,
-    |             ^^^^^^^^ enum already has `0`
+    |             ^^^^^^^^ second assignment of `0`
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tag-variant-disr-dup/tag-variant-disr-dup.stderr
To only update this specific test, also pass `--test-args tag-variant-disr-dup.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/tag-variant-disr-dup.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tag-variant-disr-dup" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tag-variant-disr-dup/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0081]: discriminant value `0` assigned more than once
   |
LL |     Black = 0x000000,
   |             -------- first assignment of `0`
   |             -------- first assignment of `0`
LL |     White = 0x000000, //~ ERROR discriminant value `0` assigned more than once
   |             ^^^^^^^^ second assignment of `0`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0081`.
------------------------------------------
