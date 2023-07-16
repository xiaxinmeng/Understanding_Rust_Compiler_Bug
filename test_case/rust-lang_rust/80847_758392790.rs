plain
.................................................................................................... 2300/11250
.................................................................................................... 2400/11250
.............................................................i..i................................... 2500/11250
.................................................................................................... 2600/11250
............F...F..........F....................................iiiii.............................F. 2700/11250
.................................................................................................... 2900/11250
.................................................................................................... 3000/11250
.................................................................................................... 3100/11250
.................................................................................................... 3200/11250
.................................................................................................... 3200/11250
.................................................................................................... 3300/11250
.................................................................................................... 3400/11250
............................................................F....................................... 3500/11250
.................................................................................................... 3600/11250
.................................................................................................... 3700/11250
.......................................FFF....F..................................................i.. 3800/11250
.................................................................................................... 4000/11250
.................................................................................................... 4100/11250
............................................ii...................................................... 4200/11250
..............................i..................................................................... 4300/11250
..............................i..................................................................... 4300/11250
..........F...................F...............................F..................................... 4400/11250
.................................................................................................... 4600/11250
.................................................................................................... 4700/11250
.................................................................................................... 4800/11250
.................................................................................................... 4900/11250
---
.........................................................................i...........i.............. 5700/11250
.................................................................................................... 5800/11250
.................................................................................................... 5900/11250
...............i.................................................................................... 6000/11250
...........iF................................F......................i............................... 6100/11250
.................................................................................................... 6300/11250
..............................i.........................i........................................... 6400/11250
.................................................................i.................................. 6500/11250
.................................................................................................... 6600/11250
---
.................................................................................................... 7300/11250
.................................................................................................... 7400/11250
..............................................................................i..ii................. 7500/11250
..........................................ii........................................................ 7600/11250
.....................................F.............F...........i.................................... 7700/11250
.......................................i............................................................ 7900/11250
.................................................................................................... 8000/11250
..........................................................................................i......... 8100/11250
.................................................................................................... 8200/11250
---
.................................................................................................... 9000/11250
.......................................................................................F............ 9100/11250
.................................................................................................... 9200/11250
..............................................i......i.............................................. 9300/11250
........................F............................................................iiiiii..iiiiii. 9400/11250
..................................................................................F................. 9600/11250
.................................................................................................... 9700/11250
.................................................................................................... 9800/11250
.................................................................................................... 9900/11250
---
diff of stderr:

2   --> $DIR/default-match-bindings-forbidden.rs:6:5
3    |
4 LL |     (x, y) = &(1, 2);
-    |     ^^^^^^   ------- this expression has type `&({integer}, {integer})`
-    |     |
-    |     expected reference, found tuple
+    |     ^^^^^^ expected reference, found tuple
8    |
9    = note: expected type `&({integer}, {integer})`
10              found tuple `(_, _)`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/default-match-bindings-forbidden/default-match-bindings-forbidden.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args destructuring-assignment/default-match-bindings-forbidden.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/destructuring-assignment/default-match-bindings-forbidden.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/default-match-bindings-forbidden" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/default-match-bindings-forbidden/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/destructuring-assignment/default-match-bindings-forbidden.rs:6:5
   |
LL |     (x, y) = &(1, 2); //~ ERROR mismatched types
   |     ^^^^^^ expected reference, found tuple
   |
   = note: expected type `&({integer}, {integer})`
             found tuple `(_, _)`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.

---
diff of stderr:

20   --> $DIR/destructure-trait-ref.rs:32:10
21    |
22 LL |     let &&x = &1isize as &dyn T;
-    |          ^^   ----------------- this expression has type `&dyn T`
24    |          |
24    |          |
25    |          expected trait object `dyn T`, found reference
26    |          help: you can probably remove the explicit borrow: `x`
32   --> $DIR/destructure-trait-ref.rs:36:11
33    |
33    |
34 LL |     let &&&x = &(&1isize as &dyn T);
-    |           ^^   -------------------- this expression has type `&&dyn T`
36    |           |
36    |           |
37    |           expected trait object `dyn T`, found reference
38    |           help: you can probably remove the explicit borrow: `x`
44   --> $DIR/destructure-trait-ref.rs:40:13
45    |
45    |
46 LL |     let box box x = box 1isize as Box<dyn T>;
-    |             ^^^^^   ------------------------ this expression has type `Box<dyn T>`
-    |             |
-    |             expected trait object `dyn T`, found struct `Box`
+    |             ^^^^^ expected trait object `dyn T`, found struct `Box`
51    = note: expected trait object `dyn T`
51    = note: expected trait object `dyn T`
52                     found struct `Box<_>`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructure-trait-ref/destructure-trait-ref.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructure-trait-ref/destructure-trait-ref.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args destructure-trait-ref.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/destructure-trait-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructure-trait-ref" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructure-trait-ref/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0033]: type `&dyn T` cannot be dereferenced
  --> /checkout/src/test/ui/destructure-trait-ref.rs:26:9
   |
LL |     let &x = &1isize as &dyn T;      //~ ERROR type `&dyn T` cannot be dereferenced
   |         ^^ type `&dyn T` cannot be dereferenced

error[E0033]: type `&dyn T` cannot be dereferenced
  --> /checkout/src/test/ui/destructure-trait-ref.rs:27:10
   |
LL |     let &&x = &(&1isize as &dyn T);  //~ ERROR type `&dyn T` cannot be dereferenced
   |          ^^ type `&dyn T` cannot be dereferenced

error[E0033]: type `Box<dyn T>` cannot be dereferenced
  --> /checkout/src/test/ui/destructure-trait-ref.rs:28:9
   |
LL |     let box x = box 1isize as Box<dyn T>;
   |         ^^^^^ type `Box<dyn T>` cannot be dereferenced
error[E0308]: mismatched types
  --> /checkout/src/test/ui/destructure-trait-ref.rs:32:10
   |
   |
LL |     let &&x = &1isize as &dyn T;
   |          |
   |          |
   |          expected trait object `dyn T`, found reference
   |          help: you can probably remove the explicit borrow: `x`
   = note: expected trait object `dyn T`
                 found reference `&_`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/destructure-trait-ref.rs:36:11
   |
LL |     let &&&x = &(&1isize as &dyn T);
   |           |
   |           |
   |           expected trait object `dyn T`, found reference
   |           help: you can probably remove the explicit borrow: `x`
   = note: expected trait object `dyn T`
                 found reference `&_`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/destructure-trait-ref.rs:40:13
   |
LL |     let box box x = box 1isize as Box<dyn T>;
   |             ^^^^^ expected trait object `dyn T`, found struct `Box`
   = note: expected trait object `dyn T`
   = note: expected trait object `dyn T`
                    found struct `Box<_>`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0033, E0308.
For more information about an error, try `rustc --explain E0033`.
---
diff of stderr:

10   --> $DIR/tuple_destructure_fail.rs:8:5
11    |
12 LL |     (a, a, b) = (1, 2);
-    |     ^^^^^^^^^   ------ this expression has type `({integer}, {integer})`
-    |     |
-    |     expected a tuple with 2 elements, found one with 3 elements
+    |     ^^^^^^^^^ expected a tuple with 2 elements, found one with 3 elements
16    |
17    = note: expected type `({integer}, {integer})`
18              found tuple `(_, _, _)`
29   --> $DIR/tuple_destructure_fail.rs:10:5
30    |
30    |
31 LL |     (_,) = (1, 2);
-    |     ^^^^   ------ this expression has type `({integer}, {integer})`
-    |     |
-    |     expected a tuple with 2 elements, found one with 1 element
+    |     ^^^^ expected a tuple with 2 elements, found one with 1 element
35    |
36    = note: expected type `({integer}, {integer})`
37              found tuple `(_,)`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/tuple_destructure_fail/tuple_destructure_fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args destructuring-assignment/tuple_destructure_fail.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/destructuring-assignment/tuple_destructure_fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/tuple_destructure_fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/tuple_destructure_fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `..` can only be used once per tuple pattern
  --> /checkout/src/test/ui/destructuring-assignment/tuple_destructure_fail.rs:7:16
   |
LL |     (a, .., b, ..) = (0, 1); //~ ERROR `..` can only be used once per tuple pattern
   |         --     ^^ can only be used once per tuple pattern
   |         previously used here

error[E0308]: mismatched types
  --> /checkout/src/test/ui/destructuring-assignment/tuple_destructure_fail.rs:8:5
  --> /checkout/src/test/ui/destructuring-assignment/tuple_destructure_fail.rs:8:5
   |
LL |     (a, a, b) = (1, 2); //~ ERROR mismatched types
   |     ^^^^^^^^^ expected a tuple with 2 elements, found one with 3 elements
   |
   = note: expected type `({integer}, {integer})`
             found tuple `(_, _, _)`

error[E0070]: invalid left-hand side of assignment
  --> /checkout/src/test/ui/destructuring-assignment/tuple_destructure_fail.rs:9:13
   |
LL |     (C, ..) = (0,1); //~ ERROR invalid left-hand side of assignment
   |      -      ^
   |      cannot assign to this expression

error[E0308]: mismatched types
  --> /checkout/src/test/ui/destructuring-assignment/tuple_destructure_fail.rs:10:5
  --> /checkout/src/test/ui/destructuring-assignment/tuple_destructure_fail.rs:10:5
   |
LL |     (_,) = (1, 2); //~ ERROR mismatched types
   |     ^^^^ expected a tuple with 2 elements, found one with 1 element
   |
   = note: expected type `({integer}, {integer})`
             found tuple `(_,)`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0070, E0308.
For more information about an error, try `rustc --explain E0070`.
---

1 error[E0308]: mismatched types
2   --> $DIR/diverging-tuple-parts-39485.rs:8:5
3    |
+ LL | fn g() {
+    |        - possibly return type missing here?
4 LL |     &panic!()
-    |     ^^^^^^^^^ expected `()`, found reference
+    |     |
+    |     |
+    |     expected `()`, found reference
+    |     help: consider removing the borrow: `panic!()`
7    = note: expected unit type `()`
8               found reference `&_`

- help: try adding a return type
- help: try adding a return type
-    |
- LL | fn g() -> &_ {
- help: consider removing the borrow
-    |
-    |
- LL |     panic!()
17 
18 error[E0308]: mismatched types
19   --> $DIR/diverging-tuple-parts-39485.rs:12:5



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/diverging-tuple-parts-39485/diverging-tuple-parts-39485.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args diverging-tuple-parts-39485.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/diverging-tuple-parts-39485.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/diverging-tuple-parts-39485" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/diverging-tuple-parts-39485/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/diverging-tuple-parts-39485.rs:8:5
   |
LL | fn g() {
   |        - possibly return type missing here?
LL |     &panic!() //~ ERROR mismatched types
   |     |
   |     |
   |     expected `()`, found reference
   |     help: consider removing the borrow: `panic!()`
   = note: expected unit type `()`
              found reference `&_`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/diverging-tuple-parts-39485.rs:12:5
   |
LL | fn f() -> isize {
   |           ----- expected `isize` because of return type
LL |     (return 1, return 2) //~ ERROR mismatched types
   |     ^^^^^^^^^^^^^^^^^^^^ expected `isize`, found tuple
   = note: expected type `isize`
   = note: expected type `isize`
             found tuple `(!, !)`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.


------------------------------------------


---- [ui] ui/fn/issue-80844-e0308.rs stdout ----

78    = note: expected unit type `()`
79                  found struct `Container<i32>`
80 
80 
- error[E0308]: mismatched types
-   --> $DIR/issue-80844-e0308.rs:63:5
-    |
- LL | fn expected_unit_got_container_bool() {
-    |                                       - possibly return type missing here?
- LL |
- LL |     Container(true)
-    |     ^^^^^^^^^^^^^^^- help: try adding a semicolon: `;`
-    |     |
-    |     expected `()`, found struct `Container`
-    = note: expected unit type `()`
-                  found struct `Container<bool>`
- 
- error: aborting due to 7 previous errors
- error: aborting due to 7 previous errors
+ error: aborting due to 6 previous errors
96 
97 For more information about this error, try `rustc --explain E0308`.
98 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/issue-80844-e0308/issue-80844-e0308.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fn/issue-80844-e0308.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fn/issue-80844-e0308.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/issue-80844-e0308" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/issue-80844-e0308/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/fn/issue-80844-e0308.rs:11:5
   |
LL | fn expected_unit_got_generator() {
   |                                  - possibly return type missing here?
LL | //~^ NOTE possibly return type missing here?
LL |     || yield 0i32
   |     ^^^^^^^^^^^^^ expected `()`, found generator
   = note: expected unit type `()`
   = note: expected unit type `()`
              found generator `[generator@/checkout/src/test/ui/fn/issue-80844-e0308.rs:11:5: 11:18]`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/fn/issue-80844-e0308.rs:19:5
   |
   |
LL | fn expected_unit_got_closure() {
   |                                - possibly return type missing here?
LL | //~^ NOTE possibly return type missing here?
LL |     || 0i32
   |     ^^^^^^^ expected `()`, found closure
   = note: expected unit type `()`
   = note: expected unit type `()`
                found closure `[closure@/checkout/src/test/ui/fn/issue-80844-e0308.rs:19:5: 19:12]`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/fn/issue-80844-e0308.rs:27:5
   |
   |
LL | fn expected_unit_got_option_closure() {
   |                                       - possibly return type missing here?
LL | //~^ NOTE possibly return type missing here?
LL |     Some(|| 0i32)
   |     ^^^^^^^^^^^^^- help: try adding a semicolon: `;`
   |     |
   |     expected `()`, found enum `Option`
   = note: expected unit type `()`
   = note: expected unit type `()`
                   found enum `Option<[closure@/checkout/src/test/ui/fn/issue-80844-e0308.rs:27:10: 27:17]>`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/fn/issue-80844-e0308.rs:36:5
   |
   |
LL | fn expected_unit_got_option_i32() {
   |                                   - possibly return type missing here?
LL | //~^ NOTE possibly return type missing here?
LL |     Some(0i32)
   |     ^^^^^^^^^^- help: try adding a semicolon: `;`
   |     |
   |     expected `()`, found enum `Option`
   = note: expected unit type `()`
   = note: expected unit type `()`
                   found enum `Option<i32>`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/fn/issue-80844-e0308.rs:45:5
   |
   |
LL | fn expected_unit_got_container_closure() {
   |                                          - possibly return type missing here?
LL | //~^ NOTE possibly return type missing here?
LL |     Container(|| 0i32)
   |     ^^^^^^^^^^^^^^^^^^- help: try adding a semicolon: `;`
   |     |
   |     expected `()`, found struct `Container`
   = note: expected unit type `()`
   = note: expected unit type `()`
                 found struct `Container<[closure@/checkout/src/test/ui/fn/issue-80844-e0308.rs:45:15: 45:22]>`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/fn/issue-80844-e0308.rs:54:5
   |
   |
LL | fn expected_unit_got_container_i32() {
   |                                      - possibly return type missing here?
LL | //~^ NOTE possibly return type missing here?
LL |     Container(0i32)
   |     ^^^^^^^^^^^^^^^- help: try adding a semicolon: `;`
   |     |
   |     expected `()`, found struct `Container`
   = note: expected unit type `()`
                 found struct `Container<i32>`

error: aborting due to 6 previous errors
---

1 error[E0308]: mismatched types
2   --> $DIR/exclusive_range_pattern_syntax_collision.rs:6:13
3    |
- LL |     match [5..4, 99..105, 43..44] {
-    |           ----------------------- this expression has type `[std::ops::Range<{integer}>; 3]`
6 LL |         [_, 99.., _] => {},
7    |             ^^ expected struct `std::ops::Range`, found integer


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision/exclusive_range_pattern_syntax_collision.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision/exclusive_range_pattern_syntax_collision.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args half-open-range-patterns/exclusive_range_pattern_syntax_collision.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision.rs:6:13
   |
LL |         [_, 99.., _] => {},
   |             ^^ expected struct `std::ops::Range`, found integer
   |
   = note: expected struct `std::ops::Range<{integer}>`
                found type `{integer}`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.

---

7 error[E0308]: mismatched types
8   --> $DIR/exclusive_range_pattern_syntax_collision2.rs:6:13
9    |
- LL |     match [5..4, 99..105, 43..44] {
-    |           ----------------------- this expression has type `[std::ops::Range<{integer}>; 3]`
12 LL |         [_, 99..] => {},
13    |             ^^ expected struct `std::ops::Range`, found integer


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision2/exclusive_range_pattern_syntax_collision2.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision2/exclusive_range_pattern_syntax_collision2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args half-open-range-patterns/exclusive_range_pattern_syntax_collision2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0527]: pattern requires 2 elements but array has 3
  --> /checkout/src/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision2.rs:6:9
   |
LL |         [_, 99..] => {},
   |         ^^^^^^^^^ expected 3 elements
error[E0308]: mismatched types
  --> /checkout/src/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision2.rs:6:13
   |
   |
LL |         [_, 99..] => {},
   |             ^^ expected struct `std::ops::Range`, found integer
   |
   = note: expected struct `std::ops::Range<{integer}>`
                found type `{integer}`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0527.
For more information about an error, try `rustc --explain E0308`.
---

1 error[E0308]: mismatched types
2   --> $DIR/exclusive_range_pattern_syntax_collision3.rs:6:12
3    |
- LL |     match [5..4, 99..105, 43..44] {
-    |           ----------------------- this expression has type `[std::ops::Range<{integer}>; 3]`
6 LL |         [..9, 99..100, _] => {},
7    |            ^ expected struct `std::ops::Range`, found integer

12 error[E0308]: mismatched types
13   --> $DIR/exclusive_range_pattern_syntax_collision3.rs:6:15
14    |
14    |
- LL |     match [5..4, 99..105, 43..44] {
-    |           ----------------------- this expression has type `[std::ops::Range<{integer}>; 3]`
17 LL |         [..9, 99..100, _] => {},
18    |               ^^  --- this is of type `{integer}`

25 error[E0308]: mismatched types
26   --> $DIR/exclusive_range_pattern_syntax_collision3.rs:6:19
27    |
27    |
- LL |     match [5..4, 99..105, 43..44] {
-    |           ----------------------- this expression has type `[std::ops::Range<{integer}>; 3]`
30 LL |         [..9, 99..100, _] => {},
31    |               --  ^^^ expected struct `std::ops::Range`, found integer


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision3/exclusive_range_pattern_syntax_collision3.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision3/exclusive_range_pattern_syntax_collision3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args half-open-range-patterns/exclusive_range_pattern_syntax_collision3.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision3.rs:6:12
   |
LL |         [..9, 99..100, _] => {},
   |            ^ expected struct `std::ops::Range`, found integer
   |
   = note: expected struct `std::ops::Range<{integer}>`
                found type `{integer}`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision3.rs:6:15
   |
   |
LL |         [..9, 99..100, _] => {},
   |               ^^  --- this is of type `{integer}`
   |               expected struct `std::ops::Range`, found integer
   |
   |
   = note: expected struct `std::ops::Range<{integer}>`
                found type `{integer}`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision3.rs:6:19
   |
   |
LL |         [..9, 99..100, _] => {},
   |               --  ^^^ expected struct `std::ops::Range`, found integer
   |               |
   |               this is of type `{integer}`
   |
   = note: expected struct `std::ops::Range<{integer}>`
                found type `{integer}`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.

---

1 error[E0308]: mismatched types
2   --> $DIR/pat-tuple-5.rs:8:10
3    |
- LL |     match (0, 1) {
-    |           ------ this expression has type `({integer}, {integer})`
6 LL |         (PAT ..) => {}
7    |          ^^^ expected tuple, found `u8`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/pat-tuple-5/pat-tuple-5.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args half-open-range-patterns/pat-tuple-5.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/half-open-range-patterns/pat-tuple-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/pat-tuple-5" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/pat-tuple-5/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/half-open-range-patterns/pat-tuple-5.rs:8:10
   |
LL |         (PAT ..) => {} //~ ERROR mismatched types
   |          ^^^ expected tuple, found `u8`
   |
   = note: expected tuple `({integer}, {integer})`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
---

1 error[E0308]: mismatched types
2   --> $DIR/issue-11844.rs:6:9
3    |
- LL |     match a {
-    |           - this expression has type `Option<Box<{integer}>>`
6 LL |         Ok(a) =>
7    |         ^^^^^ expected enum `Option`, found enum `std::result::Result`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11844/issue-11844.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11844/issue-11844.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-11844.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-11844.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11844" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11844/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-11844.rs:6:9
   |
LL |         Ok(a) => //~ ERROR: mismatched types
   |         ^^^^^ expected enum `Option`, found enum `std::result::Result`
   |
   = note: expected enum `Option<Box<{integer}>>`
              found enum `std::result::Result<_, _>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.

---

1 error[E0308]: mismatched types
2   --> $DIR/issue-12552.rs:6:5
3    |
- LL |   match t {
-    |         - this expression has type `std::result::Result<_, {integer}>`
6 LL |     Some(k) => match k {
7    |     ^^^^^^^ expected enum `std::result::Result`, found enum `Option`

12 error[E0308]: mismatched types
13   --> $DIR/issue-12552.rs:9:5
14    |
14    |
- LL |   match t {
-    |         - this expression has type `std::result::Result<_, {integer}>`
18 LL |     None => ()
18 LL |     None => ()
19    |     ^^^^ expected enum `std::result::Result`, found enum `Option`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12552/issue-12552.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12552/issue-12552.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-12552.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12552.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12552" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12552/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-12552.rs:6:5
   |
LL |     Some(k) => match k { //~ ERROR mismatched types
   |     ^^^^^^^ expected enum `std::result::Result`, found enum `Option`
   |
   = note: expected enum `std::result::Result<_, {integer}>`
              found enum `Option<_>`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-12552.rs:9:5
   |
   |
LL |     None => () //~ ERROR mismatched types
   |     ^^^^ expected enum `std::result::Result`, found enum `Option`
   |
   = note: expected enum `std::result::Result<_, {integer}>`
              found enum `Option<_>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.

---

1 error[E0308]: mismatched types
2   --> $DIR/issue-13466.rs:8:9
3    |
- LL |     let _x: usize = match Some(1) {
-    |                           ------- this expression has type `Option<{integer}>`
6 LL |         Ok(u) => u,
7    |         ^^^^^ expected enum `Option`, found enum `std::result::Result`

12 error[E0308]: mismatched types
13   --> $DIR/issue-13466.rs:14:9
14    |
14    |
- LL |     let _x: usize = match Some(1) {
-    |                           ------- this expression has type `Option<{integer}>`
- ...
18 LL |         Err(e) => panic!(e)
19    |         ^^^^^^ expected enum `Option`, found enum `std::result::Result`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-13466/issue-13466.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-13466/issue-13466.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-13466.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-13466.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-13466" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-13466/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-13466.rs:8:9
   |
LL |         Ok(u) => u,
   |         ^^^^^ expected enum `Option`, found enum `std::result::Result`
   |
   = note: expected enum `Option<{integer}>`
              found enum `std::result::Result<_, _>`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-13466.rs:14:9
   |
   |
LL |         Err(e) => panic!(e)
   |         ^^^^^^ expected enum `Option`, found enum `std::result::Result`
   |
   = note: expected enum `Option<{integer}>`
              found enum `std::result::Result<_, _>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.

---

1 error[E0308]: mismatched types
2   --> $DIR/issue-3680.rs:3:9
3    |
- LL |     match None {
-    |           ---- this expression has type `Option<_>`
6 LL |         Err(_) => ()
7    |         ^^^^^^ expected enum `Option`, found enum `std::result::Result`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3680/issue-3680.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3680/issue-3680.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-3680.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3680.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3680" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3680/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-3680.rs:3:9
   |
LL |         Err(_) => ()
   |         ^^^^^^ expected enum `Option`, found enum `std::result::Result`
   |
   = note: expected enum `Option<_>`
              found enum `std::result::Result<_, _>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.

---
diff of stderr:

36   --> $DIR/issue-66706.rs:2:5
37    |
38 LL | fn a() {
-    |        - help: try adding a return type: `-> [{integer}; _]`
+    |        - possibly return type missing here?
40 LL |     [0; [|_: _ &_| ()].len()]
41    |     ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found array `[{integer}; _]`

44   --> $DIR/issue-66706.rs:14:5
45    |
45    |
46 LL | fn c() {
-    |        - help: try adding a return type: `-> [{integer}; _]`
+    |        - possibly return type missing here?
48 LL |     [0; [|&_: _ &_| {}; 0 ].len()]
49    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found array `[{integer}; _]`

52   --> $DIR/issue-66706.rs:20:5
53    |
53    |
54 LL | fn d() {
-    |        - help: try adding a return type: `-> [{integer}; _]`
+    |        - possibly return type missing here?
56 LL |     [0; match [|f @ &ref _| () ] {} ]
57    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found array `[{integer}; _]`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66706/issue-66706.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66706/issue-66706.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-66706.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-66706.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66706" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66706/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected `,`, found `&`
   |
   |
LL |     [0; [|_: _ &_| ()].len()]
   |               -^ expected `,`
   |               help: missing `,`

error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/issues/issue-66706.rs:9:20
  --> /checkout/src/test/ui/issues/issue-66706.rs:9:20
   |
LL |     [0; [|f @ &ref _| {} ; 0 ].len() ];
   |                    ^ expected identifier, found reserved identifier

error: expected `,`, found `&`
   |
   |
LL |     [0; [|&_: _ &_| {}; 0 ].len()]
   |                -^ expected `,`
   |                help: missing `,`

error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/issues/issue-66706.rs:20:26
  --> /checkout/src/test/ui/issues/issue-66706.rs:20:26
   |
LL |     [0; match [|f @ &ref _| () ] {} ]
   |                          ^ expected identifier, found reserved identifier
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-66706.rs:2:11
   |
   |
LL |     [0; [|_: _ &_| ()].len()]
   |           ^ consider giving this closure parameter a type
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-66706.rs:2:5
   |
   |
LL | fn a() {
   |        - possibly return type missing here?
LL |     [0; [|_: _ &_| ()].len()]
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found array `[{integer}; _]`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-66706.rs:14:5
   |
   |
LL | fn c() {
   |        - possibly return type missing here?
LL |     [0; [|&_: _ &_| {}; 0 ].len()]
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found array `[{integer}; _]`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-66706.rs:20:5
   |
   |
LL | fn d() {
   |        - possibly return type missing here?
LL |     [0; match [|f @ &ref _| () ] {} ]
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found array `[{integer}; _]`
error: aborting due to 8 previous errors

Some errors have detailed explanations: E0282, E0308.
For more information about an error, try `rustc --explain E0282`.
---

21 error[E0308]: mismatched types
22   --> $DIR/issue-72574-1.rs:4:9
23    |
- LL |     match x {
-    |           - this expression has type `({integer}, {integer}, {integer})`
26 LL |         (_a, _x @ ..) => {}
27    |         ^^^^^^^^^^^^^ expected a tuple with 3 elements, found one with 2 elements


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-72574-1/issue-72574-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-72574-1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-72574-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-72574-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-72574-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `_x @` is not allowed in a tuple
   |
   |
LL |         (_a, _x @ ..) => {}
   |              ^^^^^^^ this is only allowed in slice patterns
   |
   = help: remove this and bind each tuple field independently
help: if you don't need to use the contents of _x, discard the tuple's remaining fields
   |
LL |         (_a, ..) => {}


error: `..` patterns are not allowed here
   |
   |
LL |         (_a, _x @ ..) => {}
   |
   |
   = note: only allowed in tuple, tuple struct, and slice patterns
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-72574-1.rs:4:9
   |
   |
LL |         (_a, _x @ ..) => {}
   |         ^^^^^^^^^^^^^ expected a tuple with 3 elements, found one with 2 elements
   |
   = note: expected tuple `({integer}, {integer}, {integer})`
              found tuple `(_, _)`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.

---

9 error[E0308]: mismatched types
10   --> $DIR/E0409.rs:5:23
11    |
- LL |     match x {
-    |           - this expression has type `({integer}, {integer})`
14 LL |         (0, ref y) | (y, 0) => {}
15    |             -----     ^ expected `&{integer}`, found integer


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/E0409/E0409.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mismatched_types/E0409.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/E0409.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/E0409" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/E0409/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0409]: variable `y` is bound inconsistently across alternatives separated by `|`
   |
   |
LL |         (0, ref y) | (y, 0) => {} //~ ERROR E0409
   |                 -     ^ bound in different ways
   |                 first binding

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/E0409.rs:5:23
  --> /checkout/src/test/ui/mismatched_types/E0409.rs:5:23
   |
LL |         (0, ref y) | (y, 0) => {} //~ ERROR E0409
   |             -----     ^ expected `&{integer}`, found integer
   |             |
   |             first introduced with type `&{integer}` here
   |
   = note: in the same arm, a binding must have the same type in all alternatives
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0409.
For more information about an error, try `rustc --explain E0308`.
---
diff of stderr:

2   --> $DIR/issue-19109.rs:4:5
3    |
4 LL | fn function(t: &mut dyn Trait) {
-    |                                - help: try adding a return type: `-> *mut dyn Trait`
+    |                                - possibly return type missing here?
6 LL |     t as *mut dyn Trait
7    |     ^^^^^^^^^^^^^^^^^^^ expected `()`, found *-ptr


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-19109/issue-19109.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-19109/issue-19109.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mismatched_types/issue-19109.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/issue-19109.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-19109" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-19109/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/issue-19109.rs:4:5
   |
LL | fn function(t: &mut dyn Trait) {
   |                                - possibly return type missing here?
LL |     t as *mut dyn Trait
   |     ^^^^^^^^^^^^^^^^^^^ expected `()`, found *-ptr
   = note: expected unit type `()`
   = note: expected unit type `()`
            found raw pointer `*mut dyn Trait`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.


------------------------------------------


---- [ui] ui/mut/mut-pattern-mismatched.rs stdout ----
diff of stderr:

3    |
4 LL |      let &_
5    |          ^^ types differ in mutability
- LL |         = foo;
- LL |         = foo;
-    |           --- this expression has type `&mut {integer}`
9    |
10    = note: expected mutable reference `&mut {integer}`
11                       found reference `&_`
15    |
16 LL |     let &mut _
17    |         ^^^^^^ types differ in mutability
- ...
- ...
- LL |          = bar;
-    |            --- this expression has type `&{integer}`
21    |
22    = note:      expected reference `&{integer}`
23            found mutable reference `&mut _`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mut/mut-pattern-mismatched/mut-pattern-mismatched.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mut/mut-pattern-mismatched/mut-pattern-mismatched.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mut/mut-pattern-mismatched.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mut/mut-pattern-mismatched.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mut/mut-pattern-mismatched" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mut/mut-pattern-mismatched/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/mut/mut-pattern-mismatched.rs:6:10
   |
LL |      let &_ //~  ERROR mismatched types
   |          ^^ types differ in mutability
   |
   = note: expected mutable reference `&mut {integer}`
                      found reference `&_`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/mut/mut-pattern-mismatched.rs:15:9
   |
   |
LL |     let &mut _ //~  ERROR mismatched types
   |         ^^^^^^ types differ in mutability
   |
   = note:      expected reference `&{integer}`
           found mutable reference `&mut _`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.

---
diff of stderr:

86   --> $DIR/already-bound-name.rs:32:31
87    |
88 LL |     let B(A(a, _) | B(a)) | A(a, A(a, _) | B(a)) = B(B(1));
-    |             -                 ^                    ------- this expression has type `E<E<{integer}>>`
-    |             |                 expected integer, found enum `E`
-    |             |                 expected integer, found enum `E`
+    |             -                 ^ expected integer, found enum `E`
+    |             |
92    |             first introduced with type `{integer}` here
93    |
94    = note: expected type `{integer}`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/already-bound-name/already-bound-name.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args or-patterns/already-bound-name.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/already-bound-name.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/already-bound-name" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/already-bound-name/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:11:13
   |
LL |     let (a, a) = (0, 1); // Standard duplication without an or-pattern.
   |             ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:14:15
   |
LL |     let (a, A(a, _) | B(a)) = (0, A(1, 2));
   |               ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:14:25
   |
LL |     let (a, A(a, _) | B(a)) = (0, A(1, 2));
   |                         ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:18:26
   |
LL |     let (A(a, _) | B(a), a) = (A(0, 1), 2);
   |                          ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:21:14
   |
LL |     let A(a, a) | B(a) = A(0, 1);
   |              ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:24:21
   |
LL |     let B(a) | A(a, a) = A(0, 1);
   |                     ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:28:21
   |
LL |         B(a) | A(a, a) => {} // Let's ensure `match` has no funny business.
   |                     ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:32:36
   |
LL |     let B(A(a, _) | B(a)) | A(a, A(a, _) | B(a)) = B(B(1));
   |                                    ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:32:46
   |
LL |     let B(A(a, _) | B(a)) | A(a, A(a, _) | B(a)) = B(B(1));
   |                                              ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:37:36
   |
LL |     let B(_) | A(A(a, _) | B(a), A(a, _) | B(a)) = B(B(1));
   |                                    ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:37:46
   |
LL |     let B(_) | A(A(a, _) | B(a), A(a, _) | B(a)) = B(B(1));
   |                                              ^ used in a pattern more than once

error[E0408]: variable `a` is not bound in all patterns
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:37:9
   |
LL |     let B(_) | A(A(a, _) | B(a), A(a, _) | B(a)) = B(B(1));
   |         ^^^^ pattern doesn't bind `a`        - variable not in all patterns

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:42:49
   |
LL |     let B(A(a, _) | B(a)) | A(A(a, _) | B(a), A(a, _) | B(a)) = B(B(1));
   |                                                 ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:42:59
   |
LL |     let B(A(a, _) | B(a)) | A(A(a, _) | B(a), A(a, _) | B(a)) = B(B(1));
   |                                                           ^ used in a pattern more than once
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/already-bound-name.rs:32:31
   |
   |
LL |     let B(A(a, _) | B(a)) | A(a, A(a, _) | B(a)) = B(B(1));
   |             -                 ^ expected integer, found enum `E`
   |             |
   |             first introduced with type `{integer}` here
   |
   = note: expected type `{integer}`
              found type `E<{integer}>`
   = note: a binding must have the same type in all alternatives
error: aborting due to 15 previous errors

Some errors have detailed explanations: E0308, E0408, E0416.
For more information about an error, try `rustc --explain E0308`.
---
diff of stderr:

65   --> $DIR/inconsistent-modes.rs:14:31
66    |
67 LL |     let Ok((ref a, b)) | Err((ref mut a, ref b)) = Ok((0, &0));
-    |             -----             ^^^^^^^^^            ----------- this expression has type `std::result::Result<({integer}, &{integer}), (_, _)>`
-    |             |                 types differ in mutability
+    |             -----             ^^^^^^^^^ types differ in mutability
+    |             |
+    |             |
71    |             first introduced with type `&{integer}` here
72    |
73    = note: expected type `&{integer}`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/inconsistent-modes/inconsistent-modes.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/inconsistent-modes/inconsistent-modes.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args or-patterns/inconsistent-modes.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/inconsistent-modes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/inconsistent-modes" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/inconsistent-modes/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0409]: variable `a` is bound inconsistently across alternatives separated by `|`
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:7:25
   |
LL |     let Ok(a) | Err(ref a): Result<&u8, u8> = Ok(&0);
   |            -            ^ bound in different ways
   |            first binding


error[E0409]: variable `a` is bound inconsistently across alternatives separated by `|`
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:9:29
   |
LL |     let Ok(ref mut a) | Err(a): Result<u8, &mut u8> = Ok(0);
   |                    -        ^ bound in different ways
   |                    first binding


error[E0409]: variable `a` is bound inconsistently across alternatives separated by `|`
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:11:33
   |
LL |     let Ok(ref a) | Err(ref mut a): Result<&u8, &mut u8> = Ok(&0);
   |                - first binding  ^ bound in different ways

error[E0409]: variable `a` is bound inconsistently across alternatives separated by `|`
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:14:39
   |
LL |     let Ok((ref a, b)) | Err((ref mut a, ref b)) = Ok((0, &0));
   |                 - first binding       ^ bound in different ways

error[E0409]: variable `b` is bound inconsistently across alternatives separated by `|`
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:14:46
   |
LL |     let Ok((ref a, b)) | Err((ref mut a, ref b)) = Ok((0, &0));
   |                    - first binding           ^ bound in different ways

error[E0409]: variable `a` is bound inconsistently across alternatives separated by `|`
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:20:38
   |
LL |     let Ok(Ok(a) | Err(a)) | Err(ref a) = Err(0);
   |                        -             ^ bound in different ways
   |                        first binding


error[E0409]: variable `a` is bound inconsistently across alternatives separated by `|`
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:24:33
   |
LL |     let Ok([Ok((Ok(ref a) | Err(a),)) | Err(a)]) | Err(a) = Err(&1);
   |                        -        ^ bound in different ways
   |                        first binding

error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:11:25
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:11:25
   |
LL |     let Ok(ref a) | Err(ref mut a): Result<&u8, &mut u8> = Ok(&0);
   |            -----        ^^^^^^^^^   -------------------- expected due to this
   |            |            types differ in mutability
   |            |            types differ in mutability
   |            first introduced with type `&&u8` here
   = note: expected type `&&u8`
              found type `&mut &mut u8`
              found type `&mut &mut u8`
   = note: a binding must have the same type in all alternatives
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/inconsistent-modes.rs:14:31
   |
   |
LL |     let Ok((ref a, b)) | Err((ref mut a, ref b)) = Ok((0, &0));
   |             -----             ^^^^^^^^^ types differ in mutability
   |             |
   |             first introduced with type `&{integer}` here
   |
   = note: expected type `&{integer}`
              found type `&mut _`
   = note: a binding must have the same type in all alternatives
error: aborting due to 9 previous errors

Some errors have detailed explanations: E0308, E0409.
For more information about an error, try `rustc --explain E0308`.
---
diff of stderr:

22   --> $DIR/issue-74702.rs:2:9
23    |
24 LL |     let (foo @ ..,) = (0, 0);
-    |         ^^^^^^^^^^^   ------ this expression has type `({integer}, {integer})`
-    |         |
-    |         expected a tuple with 2 elements, found one with 1 element
+    |         ^^^^^^^^^^^ expected a tuple with 2 elements, found one with 1 element
28    |
29    = note: expected tuple `({integer}, {integer})`
30               found tuple `(_,)`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/issue-74702/issue-74702.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/issue-74702/issue-74702.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/issue-74702.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/issue-74702.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/issue-74702" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/issue-74702/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `foo @` is not allowed in a tuple
   |
   |
LL |     let (foo @ ..,) = (0, 0);
   |          ^^^^^^^^ this is only allowed in slice patterns
   |
   = help: remove this and bind each tuple field independently
help: if you don't need to use the contents of foo, discard the tuple's remaining fields
   |
LL |     let (..,) = (0, 0);


error: `..` patterns are not allowed here
   |
   |
LL |     let (foo @ ..,) = (0, 0);
   |
   |
   = note: only allowed in tuple, tuple struct, and slice patterns
error[E0308]: mismatched types
  --> /checkout/src/test/ui/pattern/issue-74702.rs:2:9
   |
   |
LL |     let (foo @ ..,) = (0, 0);
   |         ^^^^^^^^^^^ expected a tuple with 2 elements, found one with 1 element
   |
   = note: expected tuple `({integer}, {integer})`
              found tuple `(_,)`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.

---

1 error[E0308]: mismatched types
2   --> $DIR/pat-tuple-overfield.rs:5:9
3    |
- LL |     match (1, 2, 3) {
-    |           --------- this expression has type `({integer}, {integer}, {integer})`
6 LL |         (1, 2, 3, 4) => {}
7    |         ^^^^^^^^^^^^ expected a tuple with 3 elements, found one with 4 elements

12 error[E0308]: mismatched types
13   --> $DIR/pat-tuple-overfield.rs:6:9
14    |
14    |
- LL |     match (1, 2, 3) {
-    |           --------- this expression has type `({integer}, {integer}, {integer})`
- LL |         (1, 2, 3, 4) => {}
18 LL |         (1, 2, .., 3, 4) => {}
19    |         ^^^^^^^^^^^^^^^^ expected a tuple with 3 elements, found one with 4 elements


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-overfield/pat-tuple-overfield.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-overfield/pat-tuple-overfield.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/pat-tuple-overfield.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/pat-tuple-overfield.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-overfield" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-overfield/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:5:9
   |
LL |         (1, 2, 3, 4) => {} //~ ERROR mismatched types
   |         ^^^^^^^^^^^^ expected a tuple with 3 elements, found one with 4 elements
   |
   = note: expected tuple `({integer}, {integer}, {integer})`
              found tuple `(_, _, _, _)`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:6:9
   |
   |
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
   |         ^^^^^^^^^^^^^ expected 3 fields, found 4

error[E0023]: this pattern has 4 fields, but the corresponding tuple struct has 3 fields
   |
   |
LL | struct S(u8, u8, u8);
   | --------------------- tuple struct defined here
...
LL |         S(1, 2, .., 3, 4) => {}
   |         ^^^^^^^^^^^^^^^^^ expected 3 fields, found 4
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0023, E0308.
For more information about an error, try `rustc --explain E0023`.
---

1 error[E0308]: mismatched types
2   --> $DIR/return-type.rs:10:5
3    |
+ LL | fn bar() {
+    |          - possibly return type missing here?
4 LL |     foo(4 as usize)
-    |     ^^^^^^^^^^^^^^^ expected `()`, found struct `S`
+    |     ^^^^^^^^^^^^^^^- help: try adding a semicolon: `;`
+    |     |
+    |     expected `()`, found struct `S`
7    = note: expected unit type `()`
7    = note: expected unit type `()`
8                  found struct `S<usize>`

- help: try adding a semicolon
-    |
- LL |     foo(4 as usize);
- help: try adding a return type
-    |
-    |
- LL | fn bar() -> S<usize> {
17 
18 error: aborting due to previous error
19 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/return-type/return-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args return/return-type.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/return/return-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/return-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/return-type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/return/return-type.rs:10:5
   |
LL | fn bar() {
   |          - possibly return type missing here?
LL |     foo(4 as usize)
   |     ^^^^^^^^^^^^^^^- help: try adding a semicolon: `;`
   |     |
   |     expected `()`, found struct `S`
   = note: expected unit type `()`
   = note: expected unit type `()`
                 found struct `S<usize>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.

---
diff of stderr:

660   --> $DIR/disallowed-positions.rs:78:12
661    |
662 LL |     if let Range { start: F, end } = F..|| true {}
-    |            ^^^^^^^^^^^^^^^^^^^^^^^ expected fn pointer, found struct `std::ops::Range`
+    |            ^^^^^^^^^^^^^^^^^^^^^^^   - this expression has type `fn() -> bool`
+    |            expected fn pointer, found struct `std::ops::Range`
664    |
664    |
665    = note: expected fn pointer `fn() -> bool`
666                   found struct `std::ops::Range<_>`
848   --> $DIR/disallowed-positions.rs:142:15
849    |
849    |
850 LL |     while let Range { start: F, end } = F..|| true {}
-    |               ^^^^^^^^^^^^^^^^^^^^^^^ expected fn pointer, found struct `std::ops::Range`
+    |               ^^^^^^^^^^^^^^^^^^^^^^^   - this expression has type `fn() -> bool`
+    |               expected fn pointer, found struct `std::ops::Range`
852    |
852    |
853    = note: expected fn pointer `fn() -> bool`
854                   found struct `std::ops::Range<_>`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions/disallowed-positions.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions/disallowed-positions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2497-if-let-chains/disallowed-positions.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expressions must be enclosed in braces to be used as const generic arguments
   |
   |
LL |         true && let 1 = 1
   |
   |
help: enclose the `const` expression in braces
   |
LL |         { true && let 1 = 1 }
   |         ^                   ^

error: `let` expressions are not supported here
   |
   |
LL |     if &let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if !let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if *let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if -let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if (let 0 = 0)? {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if true || let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if true && (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if true || (true && let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if x = let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if true..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if ..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if (let 0 = 0).. {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if let Range { start: _, end: _ } = true..true && false {}
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if let Range { start: _, end: _ } = true..true || false {}
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if let Range { start: F, end } = F..|| true {}
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if let Range { start: true, end } = t..&&false {}
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     if let true = let true = true {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while &let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while !let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while *let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while -let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while (let 0 = 0)? {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while true || let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while true && (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while true || (true && let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while x = let 0 = 0 {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while true..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while ..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while (let 0 = 0).. {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while let Range { start: _, end: _ } = true..true && false {}
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while let Range { start: _, end: _ } = true..true || false {}
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while let Range { start: F, end } = F..|| true {}
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while let Range { start: true, end } = t..&&false {}
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     while let true = let true = true {} //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     &let 0 = 0; //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     !let 0 = 0; //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     *let 0 = 0; //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     -let 0 = 0; //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     (let 0 = 0)?; //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     true || let 0 = 0; //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     (true || let 0 = 0); //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     true && (true || let 0 = 0); //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     x = let 0 = 0; //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     true..(let 0 = 0); //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     ..(let 0 = 0); //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     (let 0 = 0)..; //~ ERROR `let` expressions are not supported here
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     (let Range { start: _, end: _ } = true..true || false);
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

error: `let` expressions are not supported here
   |
   |
LL |     (let true = let true = true);
   |
   |
   = note: only supported directly in conditions of `if`- and `while`-expressions
   = note: as well as when nested within `&&` and parenthesis in those conditions

---

1 error[E0308]: mismatched types
2   --> $DIR/slightly-nice-generic-literal-messages.rs:7:9
3    |
- LL |     match Foo(1.1, marker::PhantomData) {
-    |           ----------------------------- this expression has type `Foo<{float}, _>`
6 LL |         1 => {}
7    |         ^ expected struct `Foo`, found integer


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/slightly-nice-generic-literal-messages/slightly-nice-generic-literal-messages.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/slightly-nice-generic-literal-messages/slightly-nice-generic-literal-messages.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args slightly-nice-generic-literal-messages.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/slightly-nice-generic-literal-messages.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/slightly-nice-generic-literal-messages" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/slightly-nice-generic-literal-messages/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/slightly-nice-generic-literal-messages.rs:7:9
   |
LL |         1 => {}
   |         ^ expected struct `Foo`, found integer
   |
   = note: expected struct `Foo<{float}, _>`
                found type `{integer}`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.

---

85 error[E0308]: mismatched types
86   --> $DIR/structure-constructor-type-mismatch.rs:54:9
87    |
- LL |     match (Point { x: 1, y: 2 }) {
-    |           ---------------------- this expression has type `Point<{integer}>`
90 LL |         PointF::<u32> { .. } => {}
91    |         ^^^^^^^^^^^^^^^^^^^^ expected integer, found `f32`

96 error[E0308]: mismatched types
97   --> $DIR/structure-constructor-type-mismatch.rs:59:9
98    |
98    |
- LL |     match (Point { x: 1, y: 2 }) {
-    |           ---------------------- this expression has type `Point<{integer}>`
101 LL |         PointF { .. } => {}
102    |         ^^^^^^^^^^^^^ expected integer, found `f32`

107 error[E0308]: mismatched types
108   --> $DIR/structure-constructor-type-mismatch.rs:67:9
109    |
109    |
- LL |     match (Pair { x: 1, y: 2 }) {
-    |           --------------------- this expression has type `Pair<{integer}, {integer}>`
112 LL |         PairF::<u32> { .. } => {}
113    |         ^^^^^^^^^^^^^^^^^^^ expected integer, found `f32`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/structure-constructor-type-mismatch/structure-constructor-type-mismatch.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/structure-constructor-type-mismatch/structure-constructor-type-mismatch.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args structs/structure-constructor-type-mismatch.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/structs/structure-constructor-type-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/structure-constructor-type-mismatch" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/structure-constructor-type-mismatch/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/structs/structure-constructor-type-mismatch.rs:17:12
   |
LL |         x: 1,
   |            ^
   |            |
   |            expected `f32`, found integer
   |            help: use a float literal: `1.0`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/structs/structure-constructor-type-mismatch.rs:20:12
   |
LL |         y: 2,
LL |         y: 2,
   |            ^
   |            |
   |            expected `f32`, found integer
   |            help: use a float literal: `2.0`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/structs/structure-constructor-type-mismatch.rs:26:12
   |
LL |         x: 3,
LL |         x: 3,
   |            ^
   |            |
   |            expected `f32`, found integer
   |            help: use a float literal: `3.0`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/structs/structure-constructor-type-mismatch.rs:29:12
   |
LL |         y: 4,
LL |         y: 4,
   |            ^
   |            |
   |            expected `f32`, found integer
   |            help: use a float literal: `4.0`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/structs/structure-constructor-type-mismatch.rs:35:12
   |
LL |         x: 5,
LL |         x: 5,
   |            ^
   |            |
   |            expected `f32`, found integer
   |            help: use a float literal: `5.0`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/structs/structure-constructor-type-mismatch.rs:42:12
   |
LL |         x: 7,
LL |         x: 7,
   |            ^
   |            |
   |            expected `f32`, found integer
   |            help: use a float literal: `7.0`

error[E0107]: wrong number of type arguments: expected 0, found 1
  --> /checkout/src/test/ui/structs/structure-constructor-type-mismatch.rs:48:24
   |
LL |     let pt3 = PointF::<i32> { //~ ERROR wrong number of type arguments
   |                        ^^^ unexpected type argument
error[E0308]: mismatched types
  --> /checkout/src/test/ui/structs/structure-constructor-type-mismatch.rs:49:12
   |
   |
LL |         x: 9,  //~ ERROR mismatched types
   |            |
   |            |
   |            expected `f32`, found integer
   |            help: use a float literal: `9.0`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/structs/structure-constructor-type-mismatch.rs:50:12
   |
   |
LL |         y: 10, //~ ERROR mismatched types
   |            |
   |            |
   |            expected `f32`, found integer
   |            help: use a float literal: `10.0`

error[E0107]: wrong number of type arguments: expected 0, found 1
  --> /checkout/src/test/ui/structs/structure-constructor-type-mismatch.rs:54:18
   |
LL |         PointF::<u32> { .. } => {} //~ ERROR wrong number of type arguments
   |                  ^^^ unexpected type argument
error[E0308]: mismatched types
  --> /checkout/src/test/ui/structs/structure-constructor-type-mismatch.rs:54:9
   |
   |
LL |         PointF::<u32> { .. } => {} //~ ERROR wrong number of type arguments
   |         ^^^^^^^^^^^^^^^^^^^^ expected integer, found `f32`
   |
   = note: expected struct `Point<{integer}>`
              found struct `Point<f32>`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/structs/structure-constructor-type-mismatch.rs:59:9
   |
   |
LL |         PointF { .. } => {} //~ ERROR mismatched types
   |         ^^^^^^^^^^^^^ expected integer, found `f32`
   |
   = note: expected struct `Point<{integer}>`
              found struct `Point<f32>`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/structs/structure-constructor-type-mismatch.rs:67:9
   |
   |
LL |         PairF::<u32> { .. } => {} //~ ERROR mismatched types
   |         ^^^^^^^^^^^^^^^^^^^ expected integer, found `f32`
   |
   = note: expected struct `Pair<{integer}, {integer}>`
              found struct `Pair<f32, u32>`
error: aborting due to 13 previous errors

Some errors have detailed explanations: E0107, E0308.
For more information about an error, try `rustc --explain E0107`.
---
test result: FAILED. 11138 passed; 26 failed; 86 ignored; 0 measured; 0 filtered out; finished in 111.73s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:30
