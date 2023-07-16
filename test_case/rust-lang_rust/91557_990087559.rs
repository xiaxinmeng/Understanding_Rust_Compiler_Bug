plain
..........................i......................................................................... 6100/12472
.................................................................................................... 6200/12472
.......i............................................................................................ 6300/12472
.................................................................................................... 6400/12472
....................ii.ii........i...i..........................................F.F................. 6500/12472
......................................i................i.............i.............................. 6700/12472
......................i............................................................................. 6800/12472
........................i........................................................................... 6900/12472
..................................................................i................................. 7000/12472
---
---- [ui] ui/issues/issue-10412.rs stdout ----
diff of stderr:

48    |
49    = note: assuming a `'static` lifetime...
+ error[E0106]: missing lifetime specifier
+   --> $DIR/issue-10412.rs:6:26
+    |
+    |
+ LL | impl<'self> Serializable<str> for &'self str {
+    |                          ^ expected named lifetime parameter
+    |
+ help: consider using the `'self` lifetime
+    |
+ LL | impl<'self> Serializable<'self, str> for &'self str {
+ 
51 error[E0277]: the size for values of type `str` cannot be known at compilation time
52   --> $DIR/issue-10412.rs:6:13
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
53    |

65 LL | trait Serializable<'self, T: ?Sized> {
67 
- error: aborting due to 9 previous errors
+ error: aborting due to 10 previous errors
69 
---
To only update this specific test, also pass `--test-args issues/issue-10412.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-10412.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10412" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10412/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetimes cannot use keyword names
  --> /checkout/src/test/ui/issues/issue-10412.rs:1:20
   |
LL | trait Serializable<'self, T> { //~ ERROR lifetimes cannot use keyword names

error: lifetimes cannot use keyword names
  --> /checkout/src/test/ui/issues/issue-10412.rs:2:25
   |
   |
LL |     fn serialize(val : &'self T) -> Vec<u8>; //~ ERROR lifetimes cannot use keyword names

error: lifetimes cannot use keyword names
  --> /checkout/src/test/ui/issues/issue-10412.rs:3:38
   |
   |
LL |     fn deserialize(repr : &[u8]) -> &'self T; //~ ERROR lifetimes cannot use keyword names

error: lifetimes cannot use keyword names
  --> /checkout/src/test/ui/issues/issue-10412.rs:6:6
   |
   |
LL | impl<'self> Serializable<str> for &'self str { //~ ERROR lifetimes cannot use keyword names

error: lifetimes cannot use keyword names
  --> /checkout/src/test/ui/issues/issue-10412.rs:6:36
   |
   |
LL | impl<'self> Serializable<str> for &'self str { //~ ERROR lifetimes cannot use keyword names

error: lifetimes cannot use keyword names
  --> /checkout/src/test/ui/issues/issue-10412.rs:10:25
   |
   |
LL |     fn serialize(val : &'self str) -> Vec<u8> { //~ ERROR lifetimes cannot use keyword names

error: lifetimes cannot use keyword names
  --> /checkout/src/test/ui/issues/issue-10412.rs:13:37
   |
   |
LL |     fn deserialize(repr: &[u8]) -> &'self str { //~ ERROR lifetimes cannot use keyword names


error[E0726]: implicit elided lifetime not allowed here
   |
   |
LL | impl<'self> Serializable<str> for &'self str { //~ ERROR lifetimes cannot use keyword names
   |             ^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `Serializable<'_, str>`
   |
   = note: assuming a `'static` lifetime...
error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/issues/issue-10412.rs:6:26
   |
   |
LL | impl<'self> Serializable<str> for &'self str { //~ ERROR lifetimes cannot use keyword names
   |                          ^ expected named lifetime parameter
   |
help: consider using the `'self` lifetime
   |
LL | impl<'self> Serializable<'self, str> for &'self str { //~ ERROR lifetimes cannot use keyword names

error[E0277]: the size for values of type `str` cannot be known at compilation time
  --> /checkout/src/test/ui/issues/issue-10412.rs:6:13
   |
   |
LL | impl<'self> Serializable<str> for &'self str { //~ ERROR lifetimes cannot use keyword names
   |
   = help: the trait `Sized` is not implemented for `str`
note: required by a bound in `Serializable`
  --> /checkout/src/test/ui/issues/issue-10412.rs:1:27
  --> /checkout/src/test/ui/issues/issue-10412.rs:1:27
   |
LL | trait Serializable<'self, T> { //~ ERROR lifetimes cannot use keyword names
   |                           ^ required by this bound in `Serializable`
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Serializable<'self, T: ?Sized> { //~ ERROR lifetimes cannot use keyword names

error: aborting due to 10 previous errors

Some errors have detailed explanations: E0106, E0277.
---
diff of stderr:

2   --> $DIR/label_misspelled.rs:6:9
3    |
4 LL |     'while_loop: while true {
-    |     ----------- a label with a similar name exists
+    |     |
+    |     a label with a similar name exists
+    |     a label with a similar name exists
6 LL |
6 LL |
7 LL |         while_loop;
8    |         ^^^^^^^^^^ not found in this scope

11   --> $DIR/label_misspelled.rs:11:9
12    |
13 LL |     'while_let: while let Some(_) = Some(()) {
-    |     ---------- a label with a similar name exists
+    |     |
+    |     a label with a similar name exists
+    |     a label with a similar name exists
15 LL |
15 LL |
16 LL |         while_let;
17    |         ^^^^^^^^^ not found in this scope

20   --> $DIR/label_misspelled.rs:16:9
21    |
22 LL |     'for_loop: for _ in 0..3 {
-    |     --------- a label with a similar name exists
+    |     |
+    |     a label with a similar name exists
+    |     a label with a similar name exists
24 LL |
24 LL |
25 LL |         for_loop;
26    |         ^^^^^^^^ not found in this scope

29   --> $DIR/label_misspelled.rs:21:9
30    |
31 LL |     'LOOP: loop {
-    |     ----- a label with a similar name exists
+    |     |
+    |     a label with a similar name exists
+    |     a label with a similar name exists
33 LL |
33 LL |
34 LL |         LOOP;
35    |         ^^^^ not found in this scope

38   --> $DIR/label_misspelled.rs:28:15
39    |
40 LL |     'LOOP: loop {
-    |     ----- a label with a similar name exists
+    |     |
+    |     a label with a similar name exists
+    |     a label with a similar name exists
42 LL |         break LOOP;
42 LL |         break LOOP;
-    |               ^^^^
-    |               |
-    |               not found in this scope
-    |               help: use the similarly named label: `'LOOP`
+    |
+ help: use the similarly named label
+    |
+    |
+ LL |         break 'LOOP;
+ help: use the similarly named label
+    |
+    |
+ LL |         break 'LOOP;
47 
47 
48 error[E0425]: cannot find value `while_loop` in this scope

50    |
50    |
51 LL |     'while_loop: while true {
-    |     ----------- a label with a similar name exists
+    |     |
+    |     a label with a similar name exists
+    |     a label with a similar name exists
53 LL |         break while_loop;
53 LL |         break while_loop;
-    |               ^^^^^^^^^^
-    |               |
-    |               not found in this scope
-    |               help: use the similarly named label: `'while_loop`
+    |
+ help: use the similarly named label
+    |
+    |
+ LL |         break 'while_loop;
+ help: use the similarly named label
+    |
+    |
+ LL |         break 'while_loop;
58 
58 
59 error[E0425]: cannot find value `while_let` in this scope

61    |
61    |
62 LL |     'while_let: while let Some(_) = Some(()) {
-    |     ---------- a label with a similar name exists
+    |     |
+    |     a label with a similar name exists
+    |     a label with a similar name exists
64 LL |         break while_let;
64 LL |         break while_let;
-    |               ^^^^^^^^^
-    |               |
-    |               not found in this scope
-    |               help: use the similarly named label: `'while_let`
+    |
+ help: use the similarly named label
+    |
+    |
+ LL |         break 'while_let;
+ help: use the similarly named label
+    |
+    |
+ LL |         break 'while_let;
69 
69 
70 error[E0425]: cannot find value `for_loop` in this scope

72    |
72    |
73 LL |     'for_loop: for _ in 0..3 {
-    |     --------- a label with a similar name exists
+    |     |
+    |     a label with a similar name exists
+    |     a label with a similar name exists
75 LL |         break for_loop;
75 LL |         break for_loop;
-    |               ^^^^^^^^
-    |               |
-    |               not found in this scope
-    |               help: use the similarly named label: `'for_loop`
+    |
+ help: use the similarly named label
+    |
+    |
+ LL |         break 'for_loop;
+ help: use the similarly named label
+    |
+    |
+ LL |         break 'for_loop;
80 
81 warning: unused label
82   --> $DIR/label_misspelled.rs:4:5

---
To only update this specific test, also pass `--test-args label/label_misspelled.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/label/label_misspelled.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/label/label_misspelled" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/label/label_misspelled/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0425]: cannot find value `while_loop` in this scope
   |
   |
LL |     'while_loop: while true { //~ WARN denote infinite loops with
   |     |
   |     a label with a similar name exists
   |     a label with a similar name exists
   |     a label with a similar name exists
LL |         //~^ WARN unused label
LL |         while_loop;


error[E0425]: cannot find value `while_let` in this scope
   |
   |
LL |     'while_let: while let Some(_) = Some(()) {
   |     |
   |     a label with a similar name exists
   |     a label with a similar name exists
   |     a label with a similar name exists
LL |         //~^ WARN unused label
LL |         while_let;


error[E0425]: cannot find value `for_loop` in this scope
   |
   |
LL |     'for_loop: for _ in 0..3 {
   |     |
   |     a label with a similar name exists
   |     a label with a similar name exists
   |     a label with a similar name exists
LL |         //~^ WARN unused label
LL |         for_loop;


error[E0425]: cannot find value `LOOP` in this scope
   |
   |
LL |     'LOOP: loop {
   |     |
   |     a label with a similar name exists
   |     a label with a similar name exists
   |     a label with a similar name exists
LL |         //~^ WARN unused label
LL |         LOOP;


error[E0425]: cannot find value `LOOP` in this scope
   |
   |
LL |     'LOOP: loop {
   |     |
   |     a label with a similar name exists
   |     a label with a similar name exists
LL |         break LOOP;
---
   |
LL |         break 'LOOP;
   |               ~~~~~

error[E0425]: cannot find value `while_loop` in this scope
   |
   |
LL |     'while_loop: while true { //~ WARN denote infinite loops with
   |     |
   |     a label with a similar name exists
   |     a label with a similar name exists
LL |         break while_loop;
LL |         break while_loop;
   |               ^^^^^^^^^^ not found in this scope
   |
help: use the similarly named label
   |
LL |         break 'while_loop;
help: use the similarly named label
   |
   |
LL |         break 'while_loop;


error[E0425]: cannot find value `while_let` in this scope
   |
   |
LL |     'while_let: while let Some(_) = Some(()) {
   |     |
   |     a label with a similar name exists
   |     a label with a similar name exists
LL |         break while_let;
LL |         break while_let;
   |               ^^^^^^^^^ not found in this scope
   |
help: use the similarly named label
   |
LL |         break 'while_let;
help: use the similarly named label
   |
   |
LL |         break 'while_let;


error[E0425]: cannot find value `for_loop` in this scope
   |
   |
LL |     'for_loop: for _ in 0..3 {
   |     |
   |     a label with a similar name exists
   |     a label with a similar name exists
LL |         break for_loop;
LL |         break for_loop;
   |               ^^^^^^^^ not found in this scope
   |
help: use the similarly named label
   |
LL |         break 'for_loop;
help: use the similarly named label
   |
   |
LL |         break 'for_loop;

warning: unused label
  --> /checkout/src/test/ui/label/label_misspelled.rs:4:5
   |
   |
LL |     'while_loop: while true { //~ WARN denote infinite loops with
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/label/label_misspelled.rs:1:9
   |
   |
LL | #![warn(unused_labels)]
   |         ^^^^^^^^^^^^^

warning: denote infinite loops with `loop { ... }`
   |
   |
LL |     'while_loop: while true { //~ WARN denote infinite loops with
   |     ^^^^^^^^^^^^^^^^^^^^^^^ help: use `loop`
   |
   = note: `#[warn(while_true)]` on by default
warning: unused label
  --> /checkout/src/test/ui/label/label_misspelled.rs:9:5
   |
   |
LL |     'while_let: while let Some(_) = Some(()) {

warning: unused label
  --> /checkout/src/test/ui/label/label_misspelled.rs:14:5
   |
   |
LL |     'for_loop: for _ in 0..3 {

warning: unused label
  --> /checkout/src/test/ui/label/label_misspelled.rs:19:5
   |
   |
LL |     'LOOP: loop {


warning: denote infinite loops with `loop { ... }`
   |
   |
LL |     'while_loop: while true { //~ WARN denote infinite loops with
   |     ^^^^^^^^^^^^^^^^^^^^^^^ help: use `loop`
warning: unused label
  --> /checkout/src/test/ui/label/label_misspelled.rs:47:5
   |
   |
LL |     'while_loop: while true { //~ WARN denote infinite loops with


warning: denote infinite loops with `loop { ... }`
   |
   |
LL |     'while_loop: while true { //~ WARN denote infinite loops with
   |     ^^^^^^^^^^^^^^^^^^^^^^^ help: use `loop`
warning: unused label
  --> /checkout/src/test/ui/label/label_misspelled.rs:52:5
   |
   |
LL |     'while_let: while let Some(_) = Some(()) {

warning: unused label
  --> /checkout/src/test/ui/label/label_misspelled.rs:57:5
   |
   |
LL |     'for_loop: for _ in 0..3 {


error[E0571]: `break` with value from a `while` loop
   |
   |
LL |     'while_loop: while true { //~ WARN denote infinite loops with
   |     ----------------------- you can't `break` with a value in a `while` loop
LL |         //~^ WARN unused label
LL |         break foo;
   |         ^^^^^^^^^ can only break with a value inside `loop` or breakable block
   |
help: use `break` on its own without a value inside this `while` loop
LL |         break;
   |         ~~~~~
   |         ~~~~~
help: alternatively, you might have meant to use the available loop label
   |
LL |         break 'while_loop;


error[E0571]: `break` with value from a `while` loop
   |
   |
LL |     'while_let: while let Some(_) = Some(()) {
   |     ---------------------------------------- you can't `break` with a value in a `while` loop
LL |         //~^ WARN unused label
LL |         break foo;
   |         ^^^^^^^^^ can only break with a value inside `loop` or breakable block
   |
help: use `break` on its own without a value inside this `while` loop
LL |         break;
   |         ~~~~~
   |         ~~~~~
help: alternatively, you might have meant to use the available loop label
   |
LL |         break 'while_let;


error[E0571]: `break` with value from a `for` loop
   |
   |
LL |     'for_loop: for _ in 0..3 {
   |     ------------------------ you can't `break` with a value in a `for` loop
LL |         //~^ WARN unused label
LL |         break foo;
   |         ^^^^^^^^^ can only break with a value inside `loop` or breakable block
   |
help: use `break` on its own without a value inside this `for` loop
LL |         break;
   |         ~~~~~
   |         ~~~~~
help: alternatively, you might have meant to use the available loop label
   |
LL |         break 'for_loop;

error: aborting due to 11 previous errors; 10 warnings emitted

Some errors have detailed explanations: E0425, E0571.
---
diff of stderr:

14   --> $DIR/label_misspelled_2.rs:8:15
15    |
16 LL |     'b: for _ in 0..1 {
-    |     -- a label with a similar name exists
+    |     |
+    |     a label with a similar name exists
+    |     a label with a similar name exists
18 LL |         break b;
18 LL |         break b;
-    |               ^
-    |               |
-    |               not found in this scope
-    |               help: use the similarly named label: `'b`
+    |
+ help: use the similarly named label
+    |
+ LL |         break 'b;
---
24 error[E0425]: cannot find value `d` in this scope
25   --> $DIR/label_misspelled_2.rs:14:15

26    |
27 LL |     d: for _ in 0..1 {
-    |     - a label with a similar name exists
+    |     |
+    |     a label with a similar name exists
+    |     a label with a similar name exists
29 LL |         break d;
29 LL |         break d;
-    |               ^
-    |               |
-    |               not found in this scope
-    |               help: use the similarly named label: `'d`
+    |
+ help: use the similarly named label
+    |
+ LL |         break 'd;
---
36 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/label/label_misspelled_2/label_misspelled_2.stderr
To only update this specific test, also pass `--test-args label/label_misspelled_2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/label/label_misspelled_2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/label/label_misspelled_2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/label/label_misspelled_2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: malformed loop label
   |
   |
LL |     c: for _ in 0..1 { //~ ERROR malformed loop label
   |     ^ help: use the correct loop label format: `'c`

error: malformed loop label
   |
   |
LL |     d: for _ in 0..1 { //~ ERROR malformed loop label
   |     ^ help: use the correct loop label format: `'d`
error[E0425]: cannot find value `b` in this scope
  --> /checkout/src/test/ui/label/label_misspelled_2.rs:8:15
   |
   |
LL |     'b: for _ in 0..1 {
   |     |
   |     a label with a similar name exists
   |     a label with a similar name exists
   |     a label with a similar name exists
LL |         break b; //~ ERROR cannot find value `b` in this scope
   |
help: use the similarly named label
   |
   |
LL |         break 'b; //~ ERROR cannot find value `b` in this scope
help: use the similarly named label
   |
   |
LL |         break 'b; //~ ERROR cannot find value `b` in this scope

error[E0425]: cannot find value `d` in this scope
  --> /checkout/src/test/ui/label/label_misspelled_2.rs:14:15
   |
   |
LL |     d: for _ in 0..1 { //~ ERROR malformed loop label
   |     |
   |     a label with a similar name exists
   |     a label with a similar name exists
   |     a label with a similar name exists
LL |         break d; //~ ERROR cannot find value `d` in this scope
   |
help: use the similarly named label
   |
   |
LL |         break 'd; //~ ERROR cannot find value `d` in this scope
help: use the similarly named label
   |
   |
LL |         break 'd; //~ ERROR cannot find value `d` in this scope

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0425`.
---
diff of stderr:

2   --> $DIR/loop-break-value.rs:95:15
3    |
4 LL |     'LOOP: for _ in 0 .. 9 {
-    |     ----- a label with a similar name exists
+    |     |
+    |     a label with a similar name exists
+    |     a label with a similar name exists
6 LL |         break LOOP;
6 LL |         break LOOP;
-    |               ^^^^
-    |               |
-    |               not found in this scope
-    |               help: use the similarly named label: `'LOOP`
+    |
+ help: use the similarly named label
+    |
+    |
+ LL |         break 'LOOP;
+ help: use the similarly named label
+    |
+    |
+ LL |         break 'LOOP;
11 
11 
12 warning: denote infinite loops with `loop { ... }`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/loop-break-value/loop-break-value.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/loop-break-value/loop-break-value.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args loops/loop-break-value.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/loops/loop-break-value.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/loop-break-value" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/loop-break-value/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0425]: cannot find value `LOOP` in this scope
   |
   |
LL |     'LOOP: for _ in 0 .. 9 {
   |     |
   |     a label with a similar name exists
   |     a label with a similar name exists
LL |         break LOOP;
---
   |
LL |         break 'LOOP;
   |               ~~~~~

warning: denote infinite loops with `loop { ... }`
   |
   |
LL |     'while_loop: while true { //~ WARN denote infinite loops with
   |     ^^^^^^^^^^^^^^^^^^^^^^^ help: use `loop`
   |
   = note: `#[warn(while_true)]` on by default

error[E0571]: `break` with value from a `while` loop
   |
   |
LL |     'while_loop: while true { //~ WARN denote infinite loops with
   |     ----------------------- you can't `break` with a value in a `while` loop
LL |         break;
LL |         break (); //~ ERROR `break` with value from a `while` loop
   |         ^^^^^^^^ can only break with a value inside `loop` or breakable block
   |
help: use `break` on its own without a value inside this `while` loop
   |
LL |         break; //~ ERROR `break` with value from a `while` loop
   |         ~~~~~
help: alternatively, you might have meant to use the available loop label
   |
LL |         break 'while_loop; //~ ERROR `break` with value from a `while` loop


error[E0571]: `break` with value from a `while` loop
   |
   |
LL |     'while_loop: while true { //~ WARN denote infinite loops with
   |     ----------------------- you can't `break` with a value in a `while` loop
...
LL |             break 'while_loop 123;
   |             ^^^^^^^^^^^^^^^^^^^^^ can only break with a value inside `loop` or breakable block
   |
help: use `break` on its own without a value inside this `while` loop
   |
LL |             break 'while_loop;


error[E0571]: `break` with value from a `while` loop
   |
   |
LL |     while let Some(_) = Some(()) {
   |     ---------------------------- you can't `break` with a value in a `while` loop
LL |         if break () { //~ ERROR `break` with value from a `while` loop
   |            ^^^^^^^^ can only break with a value inside `loop` or breakable block
   |
help: use `break` on its own without a value inside this `while` loop
   |
LL |         if break { //~ ERROR `break` with value from a `while` loop


error[E0571]: `break` with value from a `while` loop
   |
   |
LL |     while let Some(_) = Some(()) {
   |     ---------------------------- you can't `break` with a value in a `while` loop
LL |         break None;
   |         ^^^^^^^^^^ can only break with a value inside `loop` or breakable block
   |
help: use `break` on its own without a value inside this `while` loop
LL |         break;
   |         ~~~~~


error[E0571]: `break` with value from a `while` loop
   |
   |
LL |     'while_let_loop: while let Some(_) = Some(()) {
   |     --------------------------------------------- you can't `break` with a value in a `while` loop
LL |         loop {
LL |             break 'while_let_loop "nope";
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can only break with a value inside `loop` or breakable block
   |
help: use `break` on its own without a value inside this `while` loop
   |
LL |             break 'while_let_loop;


error[E0571]: `break` with value from a `for` loop
   |
   |
LL |     for _ in &[1,2,3] {
   |     ----------------- you can't `break` with a value in a `for` loop
LL |         break (); //~ ERROR `break` with value from a `for` loop
   |         ^^^^^^^^ can only break with a value inside `loop` or breakable block
   |
help: use `break` on its own without a value inside this `for` loop
   |
LL |         break; //~ ERROR `break` with value from a `for` loop


error[E0571]: `break` with value from a `for` loop
   |
   |
LL |     for _ in &[1,2,3] {
   |     ----------------- you can't `break` with a value in a `for` loop
LL |         break (); //~ ERROR `break` with value from a `for` loop
LL |         break [()];
   |         ^^^^^^^^^^ can only break with a value inside `loop` or breakable block
   |
help: use `break` on its own without a value inside this `for` loop
LL |         break;
   |         ~~~~~


error[E0571]: `break` with value from a `for` loop
   |
   |
LL |     'for_loop: for _ in &[1,2,3] {
   |     ---------------------------- you can't `break` with a value in a `for` loop
...
LL |             break 'for_loop Some(17);
   |             ^^^^^^^^^^^^^^^^^^^^^^^^ can only break with a value inside `loop` or breakable block
   |
help: use `break` on its own without a value inside this `for` loop
   |
LL |             break 'for_loop;

error[E0308]: mismatched types
  --> /checkout/src/test/ui/loops/loop-break-value.rs:4:31
   |
   |
LL |     let val: ! = loop { break break; };
   |                               ^^^^^ expected `!`, found `()`
   = note:   expected type `!`
           found unit type `()`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/loops/loop-break-value.rs:11:19
   |
LL |             break 123; //~ ERROR mismatched types
   |                   ^^^ expected `&str`, found integer
error[E0308]: mismatched types
  --> /checkout/src/test/ui/loops/loop-break-value.rs:16:15
   |
   |
LL |         break "asdf"; //~ ERROR mismatched types
   |               ^^^^^^ expected `i32`, found `&str`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/loops/loop-break-value.rs:21:31
   |
   |
LL |             break 'outer_loop "nope"; //~ ERROR mismatched types
   |                               ^^^^^^ expected `i32`, found `&str`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/loops/loop-break-value.rs:73:26
   |
   |
LL |                 break 'c 123; //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/loops/loop-break-value.rs:80:15
   |
   |
LL |         break (break, break); //~ ERROR mismatched types
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                  found tuple `(!, !)`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/loops/loop-break-value.rs:85:15
   |
   |
LL |         break 2; //~ ERROR mismatched types
   |               ^ expected `()`, found integer
error[E0308]: mismatched types
  --> /checkout/src/test/ui/loops/loop-break-value.rs:90:9
   |
   |
LL |         break; //~ ERROR mismatched types
   |         |
   |         expected integer, found `()`
   |         expected integer, found `()`
   |         help: give it a value of the expected type: `break value`
error: aborting due to 17 previous errors; 1 warning emitted

Some errors have detailed explanations: E0308, E0425, E0571.
For more information about an error, try `rustc --explain E0308`.
---
test result: FAILED. 12350 passed; 4 failed; 118 ignored; 0 measured; 0 filtered out; finished in 115.36s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:11:08
