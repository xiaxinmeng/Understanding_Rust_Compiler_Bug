plain
.............................i...................................................................... 8500/12148
.................................................................................................... 8600/12148
.....i.............................................................................................. 8700/12148
.................................................................................................... 8800/12148
................F.........F.................FF....................F................................. 8900/12148
.................................................................................................... 9100/12148
.....iiii.iiiii.................................................................ii...............i.. 9200/12148
.................................................................................................... 9300/12148
.................................................................................................... 9400/12148
---

---- [ui] ui/issues/issue-72574-2.rs stdout ----
diff of stderr:

19    = note: only allowed in tuple, tuple struct, and slice patterns
20 
21 error[E0023]: this pattern has 2 fields, but the corresponding tuple struct has 3 fields
-   --> $DIR/issue-72574-2.rs:6:9
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
23    |
23    |
24 LL | struct Binder(i32, i32, i32);
25    | ----------------------------- tuple struct defined here
26 ...
26 ...
27 LL |         Binder(_a, _x @ ..) => {}
-    |         ^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
+    |         ------^^^^^^^^^^^^^ expected 3 fields, found 2
+    |         this tuple struct
29    |
29    |
30 help: use `_` to explicitly ignore each field


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-72574-2/issue-72574-2.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-72574-2/issue-72574-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-72574-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-72574-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-72574-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-72574-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `_x @` is not allowed in a tuple struct
   |
   |
LL |         Binder(_a, _x @ ..) => {}
   |                    ^^^^^^^ this is only allowed in slice patterns
   |
   = help: remove this and bind each tuple field independently
help: if you don't need to use the contents of _x, discard the tuple's remaining fields
   |
LL |         Binder(_a, ..) => {}


error: `..` patterns are not allowed here
   |
   |
LL |         Binder(_a, _x @ ..) => {}
   |
   |
   = note: only allowed in tuple, tuple struct, and slice patterns

error[E0023]: this pattern has 2 fields, but the corresponding tuple struct has 3 fields
   |
   |
LL | struct Binder(i32, i32, i32);
   | ----------------------------- tuple struct defined here
...
LL |         Binder(_a, _x @ ..) => {}
   |         ------^^^^^^^^^^^^^ expected 3 fields, found 2
   |         this tuple struct
   |
   |
help: use `_` to explicitly ignore each field
   |
LL |         Binder(_a, _x @ .., _) => {}

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0023`.
For more information about this error, try `rustc --explain E0023`.

------------------------------------------


---- [ui] ui/match/match-pattern-field-mismatch.rs stdout ----
diff of stderr:

1 error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
-   --> $DIR/match-pattern-field-mismatch.rs:10:11
3    |
3    |
4 LL |         Rgb(usize, usize, usize),
5    |         ------------------------ tuple variant defined here
6 ...
6 ...
7 LL |           Color::Rgb(_, _) => { }
-    |           ^^^^^^^^^^^^^^^^ expected 3 fields, found 2
+    |           ----------^^^^^^ expected 3 fields, found 2
+    |           this tuple variant
9    |
9    |
10 help: use `_` to explicitly ignore each field


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-pattern-field-mismatch/match-pattern-field-mismatch.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-pattern-field-mismatch/match-pattern-field-mismatch.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args match/match-pattern-field-mismatch.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/match/match-pattern-field-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-pattern-field-mismatch" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-pattern-field-mismatch/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
   |
   |
LL |         Rgb(usize, usize, usize),
   |         ------------------------ tuple variant defined here
...
LL |           Color::Rgb(_, _) => { }
   |           ----------^^^^^^ expected 3 fields, found 2
   |           this tuple variant
   |
   |
help: use `_` to explicitly ignore each field
   |
LL |           Color::Rgb(_, _, _) => { }
help: use `..` to ignore all fields
   |
   |
LL |           Color::Rgb(..) => { }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0023`.
---
diff of stderr:

10               found struct `P<_>`
11 
12 error[E0023]: this pattern has 0 fields, but the corresponding tuple struct has 1 field
-   --> $DIR/issue-67037-pat-tup-scrut-ty-diff-less-fields.rs:19:9
14    |
14    |
15 LL | struct P<T>(T); // 1 type parameter wanted
16    | --------------- tuple struct defined here
17 ...
17 ...
18 LL |     let P() = U {};
-    |         ^^^ expected 1 field, found 0
+    |         -^^ expected 1 field, found 0
+    |         this tuple struct
20    |
20    |
21 help: use `_` to explicitly ignore each field


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/issue-67037-pat-tup-scrut-ty-diff-less-fields/issue-67037-pat-tup-scrut-ty-diff-less-fields.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/issue-67037-pat-tup-scrut-ty-diff-less-fields/issue-67037-pat-tup-scrut-ty-diff-less-fields.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/issue-67037-pat-tup-scrut-ty-diff-less-fields.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/issue-67037-pat-tup-scrut-ty-diff-less-fields.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/issue-67037-pat-tup-scrut-ty-diff-less-fields" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/issue-67037-pat-tup-scrut-ty-diff-less-fields/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/pattern/issue-67037-pat-tup-scrut-ty-diff-less-fields.rs:19:9
   |
LL |     let P() = U {}; //~ ERROR mismatched types
   |         ^^^   ---- this expression has type `U`
   |         expected struct `U`, found struct `P`
   |
   = note: expected struct `U`
              found struct `P<_>`
              found struct `P<_>`

error[E0023]: this pattern has 0 fields, but the corresponding tuple struct has 1 field
   |
   |
LL | struct P<T>(T); // 1 type parameter wanted
   | --------------- tuple struct defined here
...
LL |     let P() = U {}; //~ ERROR mismatched types
   |         -^^ expected 1 field, found 0
   |         this tuple struct
   |
   |
help: use `_` to explicitly ignore each field
   |
LL |     let P(_) = U {}; //~ ERROR mismatched types
help: use `..` to ignore all fields
   |
   |
LL |     let P(..) = U {}; //~ ERROR mismatched types

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0023, E0308.
---

---- [ui] ui/pattern/issue-74539.rs stdout ----
diff of stderr:

19    = note: only allowed in tuple, tuple struct, and slice patterns
20 
21 error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
-   --> $DIR/issue-74539.rs:8:9
23    |
23    |
24 LL |     A(u8, u8),
25    |     --------- tuple variant defined here
26 ...
26 ...
27 LL |         E::A(x @ ..) => {
-    |         ^^^^^^^^^^^^ expected 2 fields, found 1
+    |         ----^^^^^^^^ expected 2 fields, found 1
+    |         this tuple variant
29    |
29    |
30 help: use `_` to explicitly ignore each field


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/issue-74539/issue-74539.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/issue-74539/issue-74539.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/issue-74539.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/issue-74539.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/issue-74539" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/issue-74539/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `x @` is not allowed in a tuple struct
   |
   |
LL |         E::A(x @ ..) => {
   |              ^^^^^^ this is only allowed in slice patterns
   |
   = help: remove this and bind each tuple field independently
help: if you don't need to use the contents of x, discard the tuple's remaining fields
   |
LL |         E::A(..) => {


error: `..` patterns are not allowed here
   |
   |
LL |         E::A(x @ ..) => {
   |
   |
   = note: only allowed in tuple, tuple struct, and slice patterns

error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
   |
   |
LL |     A(u8, u8),
   |     --------- tuple variant defined here
...
LL |         E::A(x @ ..) => {
   |         ----^^^^^^^^ expected 2 fields, found 1
   |         this tuple variant
   |
   |
help: use `_` to explicitly ignore each field
   |
LL |         E::A(x @ .., _) => {

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0023`.
For more information about this error, try `rustc --explain E0023`.

------------------------------------------


---- [ui] ui/pattern/pat-tuple-overfield.rs stdout ----
diff of stderr:

22               found tuple `(_, _, _, _)`
23 
24 error[E0023]: this pattern has 4 fields, but the corresponding tuple struct has 3 fields
-   --> $DIR/pat-tuple-overfield.rs:10:9
26    |
26    |
27 LL | struct S(u8, u8, u8);
28    | --------------------- tuple struct defined here
29 ...
29 ...
30 LL |         S(1, 2, 3, 4) => {}
-    |         ^^^^^^^^^^^^^ expected 3 fields, found 4
+    |         -^^^^^^^^^^^^ expected 3 fields, found 4
+    |         this tuple struct
32 
32 
33 error[E0023]: this pattern has 4 fields, but the corresponding tuple struct has 3 fields
-   --> $DIR/pat-tuple-overfield.rs:12:9
35    |
35    |
36 LL | struct S(u8, u8, u8);
37    | --------------------- tuple struct defined here
38 ...
38 ...
39 LL |         S(1, 2, .., 3, 4) => {}
-    |         ^^^^^^^^^^^^^^^^^ expected 3 fields, found 4
+    |         -^^^^^^^^^^^^^^^^ expected 3 fields, found 4
+    |         this tuple struct
41 
42 error: aborting due to 4 previous errors
43 
---
To only update this specific test, also pass `--test-args pattern/pat-tuple-overfield.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/pat-tuple-overfield.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-overfield" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-overfield/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:5:9
   |
LL |     match (1, 2, 3) {
   |           --------- this expression has type `({integer}, {integer}, {integer})`
LL |         (1, 2, 3, 4) => {} //~ ERROR mismatched types
   |         ^^^^^^^^^^^^ expected a tuple with 3 elements, found one with 4 elements
   |
   = note: expected tuple `({integer}, {integer}, {integer})`
              found tuple `(_, _, _, _)`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:6:9
   |
LL |     match (1, 2, 3) {
LL |     match (1, 2, 3) {
   |           --------- this expression has type `({integer}, {integer}, {integer})`
LL |         (1, 2, 3, 4) => {} //~ ERROR mismatched types
LL |         (1, 2, .., 3, 4) => {} //~ ERROR mismatched types
   |         ^^^^^^^^^^^^^^^^ expected a tuple with 3 elements, found one with 4 elements
   |
   = note: expected tuple `({integer}, {integer}, {integer})`
              found tuple `(_, _, _, _)`

error[E0023]: this pattern has 4 fields, but the corresponding tuple struct has 3 fields
   |
   |
LL | struct S(u8, u8, u8);
   | --------------------- tuple struct defined here
...
LL |         S(1, 2, 3, 4) => {}
   |         -^^^^^^^^^^^^ expected 3 fields, found 4
   |         this tuple struct


error[E0023]: this pattern has 4 fields, but the corresponding tuple struct has 3 fields
   |
   |
LL | struct S(u8, u8, u8);
   | --------------------- tuple struct defined here
...
LL |         S(1, 2, .., 3, 4) => {}
   |         -^^^^^^^^^^^^^^^^ expected 3 fields, found 4
   |         this tuple struct

error: aborting due to 4 previous errors

---

---- [ui] ui/pattern/pat-tuple-underfield.rs stdout ----
diff of stderr:

8    |         ^^^^ help: use the tuple variant pattern syntax instead: `E::S(_, _)`
9 
10 error[E0023]: this pattern has 1 field, but the corresponding tuple struct has 2 fields
-   --> $DIR/pat-tuple-underfield.rs:9:9
12    |
12    |
13 LL | struct S(i32, f32);
14    | ------------------- tuple struct defined here
15 ...
15 ...
16 LL |         S(x) => {}
-    |         ^^^^ expected 2 fields, found 1
+    |         -^^^ expected 2 fields, found 1
+    |         this tuple struct
18    |
18    |
19 help: use `_` to explicitly ignore each field

22    |            +++
23 
23 
24 error[E0023]: this pattern has 1 field, but the corresponding tuple struct has 2 fields
-   --> $DIR/pat-tuple-underfield.rs:14:9
26    |
26    |
27 LL | struct S(i32, f32);
28    | ------------------- tuple struct defined here
29 ...
29 ...
30 LL |         S(_) => {}
-    |         ^^^^ expected 2 fields, found 1
+    |         -^^^ expected 2 fields, found 1
+    |         this tuple struct
32    |
32    |
33 help: use `_` to explicitly ignore each field

40    |           ~~
41 
41 
42 error[E0023]: this pattern has 0 fields, but the corresponding tuple struct has 2 fields
-   --> $DIR/pat-tuple-underfield.rs:20:9
44    |
44    |
45 LL | struct S(i32, f32);
46    | ------------------- tuple struct defined here
47 ...
48 LL |         S() => {}
-    |         ^^^ expected 2 fields, found 0
-    |         ^^^ expected 2 fields, found 0
+    |         -^^ expected 2 fields, found 0
+    |         this tuple struct
50    |
50    |
51 help: use `_` to explicitly ignore each field

58    |           ++
59 
59 
60 error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
-   --> $DIR/pat-tuple-underfield.rs:27:9
62    |
62    |
63 LL |     S(i32, f32),
64    |     ----------- tuple variant defined here
65 ...
65 ...
66 LL |         E::S(x) => {}
-    |         ^^^^^^^ expected 2 fields, found 1
+    |         ----^^^ expected 2 fields, found 1
+    |         this tuple variant
68    |
68    |
69 help: use `_` to explicitly ignore each field

72    |               +++
73 
73 
74 error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
-   --> $DIR/pat-tuple-underfield.rs:32:9
76    |
76    |
77 LL |     S(i32, f32),
78    |     ----------- tuple variant defined here
79 ...
79 ...
80 LL |         E::S(_) => {}
-    |         ^^^^^^^ expected 2 fields, found 1
+    |         ----^^^ expected 2 fields, found 1
+    |         this tuple variant
82    |
82    |
83 help: use `_` to explicitly ignore each field

90    |              ~~
91 
91 
92 error[E0023]: this pattern has 0 fields, but the corresponding tuple variant has 2 fields
-   --> $DIR/pat-tuple-underfield.rs:38:9
94    |
94    |
95 LL |     S(i32, f32),
96    |     ----------- tuple variant defined here
97 ...
97 ...
98 LL |         E::S() => {}
-    |         ^^^^^^ expected 2 fields, found 0
+    |         ----^^ expected 2 fields, found 0
+    |         this tuple variant
100    |
100    |
101 help: use `_` to explicitly ignore each field

108    |              ++
109 
109 
110 error[E0023]: this pattern has 2 fields, but the corresponding tuple struct has 4 fields
-   --> $DIR/pat-tuple-underfield.rs:50:9
112    |
113 LL | struct Point4(i32, i32, i32, i32);
114    | ---------------------------------- tuple struct defined here


115 ...
116 LL |         Point4(   a   ,     _    ) => {}
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 4 fields, found 2
+    |         ------^^^^^^^^^^^^^^^^^^^^ expected 4 fields, found 2
+    |         this tuple struct
118    |
118    |
119 help: use `_` to explicitly ignore each field


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-underfield/pat-tuple-underfield.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-underfield/pat-tuple-underfield.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/pat-tuple-underfield.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/pat-tuple-underfield.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-underfield" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-underfield/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0532]: expected unit struct, unit variant or constant, found tuple variant `E::S`
  --> /checkout/src/test/ui/pattern/pat-tuple-underfield.rs:44:9
   |
LL |     S(i32, f32),
   |     ----------- `E::S` defined here
...
LL |         E::S => {}
   |         ^^^^ help: use the tuple variant pattern syntax instead: `E::S(_, _)`

error[E0023]: this pattern has 1 field, but the corresponding tuple struct has 2 fields
   |
   |
LL | struct S(i32, f32);
   | ------------------- tuple struct defined here
...
LL |         S(x) => {}
   |         -^^^ expected 2 fields, found 1
   |         this tuple struct
   |
   |
help: use `_` to explicitly ignore each field
   |
LL |         S(x, _) => {}


error[E0023]: this pattern has 1 field, but the corresponding tuple struct has 2 fields
   |
   |
LL | struct S(i32, f32);
   | ------------------- tuple struct defined here
...
LL |         S(_) => {}
   |         -^^^ expected 2 fields, found 1
   |         this tuple struct
   |
   |
help: use `_` to explicitly ignore each field
   |
LL |         S(_, _) => {}
help: use `..` to ignore all fields
   |
   |
LL |         S(..) => {}


error[E0023]: this pattern has 0 fields, but the corresponding tuple struct has 2 fields
   |
   |
LL | struct S(i32, f32);
   | ------------------- tuple struct defined here
LL |         S() => {}
LL |         S() => {}
   |         -^^ expected 2 fields, found 0
   |         this tuple struct
   |
   |
help: use `_` to explicitly ignore each field
   |
LL |         S(_, _) => {}
help: use `..` to ignore all fields
   |
   |
LL |         S(..) => {}


error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
   |
   |
LL |     S(i32, f32),
   |     ----------- tuple variant defined here
...
LL |         E::S(x) => {}
   |         ----^^^ expected 2 fields, found 1
   |         this tuple variant
   |
   |
help: use `_` to explicitly ignore each field
   |
LL |         E::S(x, _) => {}


error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
   |
   |
LL |     S(i32, f32),
   |     ----------- tuple variant defined here
...
LL |         E::S(_) => {}
   |         ----^^^ expected 2 fields, found 1
   |         this tuple variant
   |
   |
help: use `_` to explicitly ignore each field
   |
LL |         E::S(_, _) => {}
help: use `..` to ignore all fields
   |
   |
LL |         E::S(..) => {}


error[E0023]: this pattern has 0 fields, but the corresponding tuple variant has 2 fields
   |
   |
LL |     S(i32, f32),
   |     ----------- tuple variant defined here
...
LL |         E::S() => {}
   |         ----^^ expected 2 fields, found 0
   |         this tuple variant
   |
   |
help: use `_` to explicitly ignore each field
   |
LL |         E::S(_, _) => {}
help: use `..` to ignore all fields
   |
   |
LL |         E::S(..) => {}


error[E0023]: this pattern has 2 fields, but the corresponding tuple struct has 4 fields
   |
LL | struct Point4(i32, i32, i32, i32);
   | ---------------------------------- tuple struct defined here
...
...
LL |         Point4(   a   ,     _    ) => {}
   |         ------^^^^^^^^^^^^^^^^^^^^ expected 4 fields, found 2
   |         this tuple struct
   |
   |
help: use `_` to explicitly ignore each field
   |
LL |         Point4(   a   ,     _    , _, _) => {}
help: use `..` to ignore the rest of the fields
   |
   |
LL |         Point4(   a, ..) => {}

error: aborting due to 8 previous errors

Some errors have detailed explanations: E0023, E0532.
---
diff of stderr:

26    |            ~
27 
28 error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 2 fields
-   --> $DIR/pattern-error-continue.rs:17:9
30    |
30    |
31 LL |     B(isize, isize),
32    |     --------------- tuple variant defined here
33 ...
33 ...
34 LL |         A::B(_, _, _) => (),
-    |         ^^^^^^^^^^^^^ expected 2 fields, found 3
+    |         ----^^^^^^^^^ expected 2 fields, found 3
+    |         this tuple variant
36 
37 error[E0308]: mismatched types
38   --> $DIR/pattern-error-continue.rs:22:9
---
To only update this specific test, also pass `--test-args pattern/pattern-error-continue.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/pattern-error-continue.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pattern-error-continue" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pattern-error-continue/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0433]: failed to resolve: use of undeclared type `E`
  --> /checkout/src/test/ui/pattern/pattern-error-continue.rs:33:9
   |
LL |         E::V => {} //~ ERROR failed to resolve: use of undeclared type `E`
   |         ^ use of undeclared type `E`
error[E0532]: expected tuple struct or tuple variant, found unit variant `A::D`
  --> /checkout/src/test/ui/pattern/pattern-error-continue.rs:18:9
   |
   |
LL |     B(isize, isize),
   |     --------------- similarly named tuple variant `B` defined here
LL |     C(isize, isize, isize),
LL |     D
   |     - `A::D` defined here
...
LL |         A::D(_) => (), //~ ERROR expected tuple struct or tuple variant, found unit variant `A::D`
   |
help: use this syntax instead
   |
   |
LL |         A::D => (), //~ ERROR expected tuple struct or tuple variant, found unit variant `A::D`
help: a tuple variant with a similar name exists
   |
   |
LL |         A::B(_) => (), //~ ERROR expected tuple struct or tuple variant, found unit variant `A::D`


error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 2 fields
   |
   |
LL |     B(isize, isize),
   |     --------------- tuple variant defined here
...
LL |         A::B(_, _, _) => (), //~ ERROR this pattern has 3 fields, but
   |         ----^^^^^^^^^ expected 2 fields, found 3
   |         this tuple variant

error[E0308]: mismatched types
  --> /checkout/src/test/ui/pattern/pattern-error-continue.rs:22:9
  --> /checkout/src/test/ui/pattern/pattern-error-continue.rs:22:9
   |
LL |     match 'c' {
   |           --- this expression has type `char`
LL |         S { .. } => (),
   |         ^^^^^^^^ expected `char`, found struct `S`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/pattern/pattern-error-continue.rs:28:7
   |
LL |     f(true);
LL |     f(true);
   |       ^^^^ expected `char`, found `bool`
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0023, E0308, E0433, E0532.
For more information about an error, try `rustc --explain E0023`.
---
test result: FAILED. 12040 passed; 7 failed; 101 ignored; 0 measured; 0 filtered out; finished in 125.61s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:11
