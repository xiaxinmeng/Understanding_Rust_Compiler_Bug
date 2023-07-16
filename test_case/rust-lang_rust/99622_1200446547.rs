plain
........................................................................................ 704/13259
...........................................................i............................ 792/13259
........F..............................................i................................ 880/13259
........................................................................................ 968/13259
.............................FF.F..F..F.F....F.......F.................................. 1056/13259
........................................................................................ 1232/13259
.............................................i.......................................... 1320/13259
........................................................................................ 1408/13259
..............i......................................................................... 1496/13259
---
.......................................i..................i.............i............... 7040/13259
.........................................i.............................................. 7128/13259
..........................................................i............................. 7216/13259
........................................................................................ 7304/13259
...........F...........................................................F................ 7392/13259
...................................................iiF.................................. 7480/13259
........................................................................................ 7656/13259
............................................................................ii.......... 7744/13259
........................................................................................ 7832/13259
........................................................................................ 7920/13259
........................................................................................ 7920/13259
..............................................................F....F.................ii. 8008/13259
.........................................F.............................................. 8184/13259
........................................................................................ 8272/13259
........................................................................................ 8360/13259
........................................................................................ 8448/13259
---
........................................................................................ 9504/13259
........................................................................................ 9592/13259
........................................................................................ 9680/13259
........................................................................................ 9768/13259
..................................................F..........................ii......... 9856/13259
......i..............................F.................F...........ii................... 9944/13259
........................................................................................ 10120/13259
........................................................................................ 10208/13259
........................................................................................ 10296/13259
........................................................................................ 10384/13259
........................................................................................ 10384/13259
..........F.F............F.............................................................. 10472/13259
..............i..i.i.................................................................... 10648/13259
.........................................................i....................F......... 10736/13259
...................................................................iiiiii.i..iiiiii.i... 10824/13259
........................................................................................ 10912/13259
........................................................................................ 10912/13259
........................................................................................ 11000/13259
........................................................................................ 11088/13259
........................................................................................ 11176/13259
.........F.............................................................................. 11264/13259
........................................................................................ 11352/13259
........................................................................................ 11440/13259
........................................................................................ 11528/13259
....................................F................................................F.. 11616/13259
..................................................................................F..... 11704/13259
........................................................................................ 11880/13259
........................................................................................ 11968/13259
........................................................................................ 12056/13259
........................................................................................ 12144/13259
........................................................................................ 12144/13259
........................................................................................ 12232/13259
........................................................................................ 12320/13259
.......................................................................................i 12408/13259
..............................................F......................................... 12496/13259
..........................F............................................................. 12584/13259
..F...............................................F.F..F............F................... 12672/13259
........................................................................................ 12848/13259
........................................................................................ 12936/13259
........................................................................................ 13024/13259
........................................................................................ 13112/13259
---

1 error[E0308]: mismatched types
2   --> $DIR/issue-71443-1.rs:6:5
3    |
- LL | fn hello<F: for<'a> Iterator<Item: 'a>>() {
-    |                                           - help: try adding a return type: `-> Incorrect`
7    |     ^^^^^^^^^ expected `()`, found struct `Incorrect`
+    |
+ help: consider using a semicolon at the end of the expression
+    |
+    |
+ LL |     Incorrect;
+    |              +
+ help: try adding a return type
+    |
+ LL | fn hello<F: for<'a> Iterator<Item: 'a>>() -> Incorrect {
8 
9 error: aborting due to previous error
10 

---
To only update this specific test, also pass `--test-args associated-type-bounds/issue-71443-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/issue-71443-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/issue-71443-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/issue-71443-1/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/associated-type-bounds/issue-71443-1.rs:6:5
   |
   |
LL |     Incorrect //~ERROR: mismatched types
   |     ^^^^^^^^^ expected `()`, found struct `Incorrect`
help: consider using a semicolon at the end of the expression
   |
   |
LL |     Incorrect; //~ERROR: mismatched types
help: try adding a return type
   |
   |
LL | fn hello<F: for<'a> Iterator<Item: 'a>>() -> Incorrect {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/async-await/suggest-missing-await.rs stdout ----
diff of stderr:

36    |                  ^ checked the `Output` of this `async fn`, found opaque type
38             found opaque type `impl Future<Output = ()>`
+ help: consider using a semicolon at the end of the expression
+    |
+    |
+ LL |     dummy();
+    |            +
39 help: consider `await`ing on the `Future`
40    |
41 LL |     dummy().await

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await/suggest-missing-await.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/suggest-missing-await.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/suggest-missing-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/async-await/suggest-missing-await.rs:12:14
   |
LL |     take_u32(x)
LL |     take_u32(x)
   |     -------- ^ expected `u32`, found opaque type
   |     arguments to this function are incorrect
   |
note: while checking the return type of the `async fn`
  --> /checkout/src/test/ui/async-await/suggest-missing-await.rs:5:24
  --> /checkout/src/test/ui/async-await/suggest-missing-await.rs:5:24
   |
LL | async fn make_u32() -> u32 {
   |                        ^^^ checked the `Output` of this `async fn`, found opaque type
           found opaque type `impl Future<Output = u32>`
note: function defined here
  --> /checkout/src/test/ui/async-await/suggest-missing-await.rs:3:4
   |
   |
LL | fn take_u32(_x: u32) {}
   |    ^^^^^^^^ -------
help: consider `await`ing on the `Future`
   |
LL |     take_u32(x.await)

error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/suggest-missing-await.rs:22:5
   |
   |
LL |     dummy()
   |     ^^^^^^^ expected `()`, found opaque type
   |
note: while checking the return type of the `async fn`
  --> /checkout/src/test/ui/async-await/suggest-missing-await.rs:18:18
   |
LL | async fn dummy() {}
   |                  ^ checked the `Output` of this `async fn`, found opaque type
            found opaque type `impl Future<Output = ()>`
help: consider using a semicolon at the end of the expression
   |
LL |     dummy();
LL |     dummy();
   |            +
help: consider `await`ing on the `Future`
   |
LL |     dummy().await
help: consider using a semicolon here
   |
LL |     dummy();
   |            +
   |            +

error[E0308]: `if` and `else` have incompatible types
   |
LL |       let _x = if true {
   |  ______________-
LL | |         dummy()
LL | |         dummy()
   | |         ------- expected because of this
LL | |         //~^ HELP consider `await`ing on the `Future`
LL | |     } else {
LL | |         dummy().await
   | |         ^^^^^^^^^^^^^ expected opaque type, found `()`
LL | |         //~^ ERROR `if` and `else` have incompatible types [E0308]
LL | |     };
   | |_____- `if` and `else` have incompatible types
note: while checking the return type of the `async fn`
  --> /checkout/src/test/ui/async-await/suggest-missing-await.rs:18:18
   |
   |
LL | async fn dummy() {}
   |                  ^ checked the `Output` of this `async fn`, expected opaque type
   = note: expected opaque type `impl Future<Output = ()>`
                found unit type `()`
help: consider `await`ing on the `Future`
   |
LL |         dummy().await

error[E0308]: `match` arms have incompatible types
  --> /checkout/src/test/ui/async-await/suggest-missing-await.rs:45:14
   |
   |
LL |       let _x = match 0usize {
   |  ______________-
LL | |         0 => dummy(), //~ HELP consider `await`ing on the `Future`
   | |              ------- this is found to be of type `impl Future<Output = ()>`
LL | |         1 => dummy(),
   | |              ------- this is found to be of type `impl Future<Output = ()>`
LL | |         2 => dummy().await,
   | |              ^^^^^^^^^^^^^ expected opaque type, found `()`
LL | |         //~^ `match` arms have incompatible types [E0308]
LL | |     };
   |
note: while checking the return type of the `async fn`
  --> /checkout/src/test/ui/async-await/suggest-missing-await.rs:18:18
   |
   |
LL | async fn dummy() {}
   |                  ^ checked the `Output` of this `async fn`, expected opaque type
   = note: expected opaque type `impl Future<Output = ()>`
                found unit type `()`
help: consider `await`ing on the `Future`
   |
LL ~         0 => dummy().await, //~ HELP consider `await`ing on the `Future`
LL ~         1 => dummy().await,

error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/suggest-missing-await.rs:53:9
   |
   |
LL |     let _x = match dummy() { //~ HELP consider `await`ing on the `Future`
   |                    ------- this expression has type `impl Future<Output = ()>`
LL |         () => {} //~ ERROR mismatched types [E0308]
   |
note: while checking the return type of the `async fn`
  --> /checkout/src/test/ui/async-await/suggest-missing-await.rs:18:18
   |
   |
LL | async fn dummy() {}
   |                  ^ checked the `Output` of this `async fn`, expected opaque type
   = note: expected opaque type `impl Future<Output = ()>`
                found unit type `()`
help: consider `await`ing on the `Future`
   |
LL |     let _x = match dummy().await { //~ HELP consider `await`ing on the `Future`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/suggest-missing-await.rs:67:9
   |
---
   |
note: while checking the return type of the `async fn`
  --> /checkout/src/test/ui/async-await/suggest-missing-await.rs:57:28
   |
LL | async fn dummy_result() -> Result<(), ()> {
   |                            ^^^^^^^^^^^^^^ checked the `Output` of this `async fn`, expected opaque type
   = note: expected opaque type `impl Future<Output = Result<(), ()>>`
                     found enum `Result<_, _>`
help: consider `await`ing on the `Future`
   |
LL |     match dummy_result().await {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/suggest-missing-await.rs:69:9
   |
---
   |
note: while checking the return type of the `async fn`
  --> /checkout/src/test/ui/async-await/suggest-missing-await.rs:57:28
   |
LL | async fn dummy_result() -> Result<(), ()> {
   |                            ^^^^^^^^^^^^^^ checked the `Output` of this `async fn`, expected opaque type
   = note: expected opaque type `impl Future<Output = Result<(), ()>>`
                     found enum `Result<_, _>`
help: consider `await`ing on the `Future`
   |
LL |     match dummy_result().await {

error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0308`.
---
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/block-must-not-have-result-do/block-must-not-have-result-do.stderr
To only update this specific test, also pass `--test-args block-result/block-must-not-have-result-do.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/block-result/block-must-not-have-result-do.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/block-must-not-have-result-do" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/block-must-not-have-result-do/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/block-result/block-must-not-have-result-do.rs:3:9
   |
   |
LL |         true //~  ERROR mismatched types
   |         ^^^^ expected `()`, found `bool`
help: consider using a semicolon at the end of the expression
   |
   |
LL |         true; //~  ERROR mismatched types

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
---
To only update this specific test, also pass `--test-args block-result/block-must-not-have-result-while.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/block-result/block-must-not-have-result-while.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/block-must-not-have-result-while" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/block-must-not-have-result-while/auxiliary"
stdout: none
--- stderr -------------------------------
warning: denote infinite loops with `loop { ... }`
   |
   |
LL |     while true { //~ WARN denote infinite loops with
   |     ^^^^^^^^^^ help: use `loop`
   = note: `#[warn(while_true)]` on by default

error[E0308]: mismatched types
  --> /checkout/src/test/ui/block-result/block-must-not-have-result-while.rs:3:9
  --> /checkout/src/test/ui/block-result/block-must-not-have-result-while.rs:3:9
   |
LL | /     while true { //~ WARN denote infinite loops with
LL | |         true //~  ERROR mismatched types
   | |         ^^^^ expected `()`, found `bool`
LL | |              //~| expected `()`, found `bool`
LL | |     }
   | |_____- expected this to be `()`
help: consider using a semicolon at the end of the expression
   |
   |
LL |         true; //~  ERROR mismatched types

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0308`.
---
9    = note: expected unit type `()`
10               found reference `&_`
+ help: consider using a semicolon at the end of the expression
+    |
+ LL |     &panic!();
11 help: consider removing the borrow
12    |
12    |
13 LL -     &panic!()

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/issue-5500/issue-5500.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args block-result/issue-5500.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/block-result/issue-5500.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/issue-5500" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/issue-5500/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/block-result/issue-5500.rs:2:5
   |
LL | fn main() {
LL | fn main() {
   |           - expected `()` because of default return type
LL |     &panic!()
   |
   = note: expected unit type `()`
              found reference `&_`
help: consider using a semicolon at the end of the expression
help: consider using a semicolon at the end of the expression
   |
LL |     &panic!();
   |              +
help: consider removing the borrow
   |
LL -     &panic!()
LL +     panic!()

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/block-result/block-must-not-have-result-res.rs stdout ----
diff of stderr:

5    |                        - expected `()` because of default return type
7    |         ^^^^ expected `()`, found `bool`
+    |
+ help: consider using a semicolon at the end of the expression
+    |
---
To only update this specific test, also pass `--test-args block-result/block-must-not-have-result-res.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/block-result/block-must-not-have-result-res.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/block-must-not-have-result-res" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/block-must-not-have-result-res/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/block-result/block-must-not-have-result-res.rs:5:9
   |
   |
LL |     fn drop(&mut self) {
   |                        - expected `()` because of default return type
LL |         true //~  ERROR mismatched types
   |         ^^^^ expected `()`, found `bool`
help: consider using a semicolon at the end of the expression
   |
   |
LL |         true; //~  ERROR mismatched types

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
---

1 error[E0308]: mismatched types
2   --> $DIR/issue-20862.rs:2:5
3    |
- LL | fn foo(x: i32) {
-    |                - help: a return type might be missing here: `-> _`
6 LL |     |y| x + y
8    |

9    = note: expected unit type `()`
10                 found closure `[closure@$DIR/issue-20862.rs:2:5: 2:8]`
10                 found closure `[closure@$DIR/issue-20862.rs:2:5: 2:8]`
+ help: consider using a semicolon at the end of the expression
+    |
+ LL |     |y| x + y;
+ help: a return type might be missing here
+    |
+    |
+ LL | fn foo(x: i32) -> _ {
11 
12 error[E0618]: expected function, found `()`
13   --> $DIR/issue-20862.rs:7:13

---
To only update this specific test, also pass `--test-args block-result/issue-20862.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/block-result/issue-20862.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/issue-20862" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/issue-20862/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/block-result/issue-20862.rs:2:5
   |
   |
LL |     |y| x + y
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found closure `[closure@/checkout/src/test/ui/block-result/issue-20862.rs:2:5: 2:8]`
help: consider using a semicolon at the end of the expression
   |
LL |     |y| x + y;
help: a return type might be missing here
   |
   |
LL | fn foo(x: i32) -> _ {

error[E0618]: expected function, found `()`
  --> /checkout/src/test/ui/block-result/issue-20862.rs:7:13
   |
   |
LL | fn foo(x: i32) {
   | -------------- `foo` defined here returns `()`
LL |     let x = foo(5)(2);
   |             ^^^^^^---
   |             |
   |             call expression requires function
---

---- [ui] src/test/ui/block-result/issue-13624.rs stdout ----
diff of stderr:

5    |                                       -- expected `()` because of return type
6 LL |     Enum::EnumStructVariant { x: 1, y: 2, z: 3 }
7    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found enum `Enum`
+ help: consider using a semicolon at the end of the expression
+    |
+    |
+ LL |     Enum::EnumStructVariant { x: 1, y: 2, z: 3 };
8 
9 error[E0308]: mismatched types
10   --> $DIR/issue-13624.rs:20:9

---
To only update this specific test, also pass `--test-args block-result/issue-13624.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/block-result/issue-13624.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/issue-13624" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/issue-13624/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/block-result/issue-13624.rs:7:5
   |
LL |   pub fn get_enum_struct_variant() -> () {
   |                                       -- expected `()` because of return type
   |                                       -- expected `()` because of return type
LL |     Enum::EnumStructVariant { x: 1, y: 2, z: 3 }
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found enum `Enum`
help: consider using a semicolon at the end of the expression
   |
   |
LL |     Enum::EnumStructVariant { x: 1, y: 2, z: 3 };

error[E0308]: mismatched types
  --> /checkout/src/test/ui/block-result/issue-13624.rs:20:9
   |
   |
LL |       match enum_struct_variant {
   |             ------------------- this expression has type `()`
LL |         a::Enum::EnumStructVariant { x, y, z } => {
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found enum `Enum`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/block-result/issue-22645.rs stdout ----
diff of stderr:

19 LL |   let b = Bob + 3.5;
20 LL |   b + 3
21    |   ^^^^^ expected `()`, found struct `Bob`
+ help: consider using a semicolon at the end of the expression
+    |
+ LL |   b + 3;
+    |        +
---
To only update this specific test, also pass `--test-args block-result/issue-22645.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/block-result/issue-22645.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/issue-22645" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/issue-22645/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `{integer}: Scalar` is not satisfied
   |
   |
LL |   b + 3 //~ ERROR E0277
   |     ^ the trait `Scalar` is not implemented for `{integer}`
   = help: the trait `Scalar` is implemented for `f64`
   = help: the trait `Scalar` is implemented for `f64`
note: required because of the requirements on the impl of `Add<{integer}>` for `Bob`
   |
   |
LL | impl<RHS: Scalar> Add <RHS> for Bob {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/block-result/issue-22645.rs:15:3
   |
   |
LL | fn main() {
   |           - expected `()` because of default return type
LL |   let b = Bob + 3.5;
LL |   b + 3 //~ ERROR E0277
   |   ^^^^^ expected `()`, found struct `Bob`
help: consider using a semicolon at the end of the expression
   |
   |
LL |   b + 3; //~ ERROR E0277

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0308.
---
To only update this specific test, also pass `--test-args block-result/unexpected-return-on-unit.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/block-result/unexpected-return-on-unit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/unexpected-return-on-unit" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/unexpected-return-on-unit/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/block-result/unexpected-return-on-unit.rs:9:5
   |
   |
LL |     foo() //~ ERROR mismatched types
   |
help: consider using a semicolon at the end of the expression
   |
   |
LL |     foo(); //~ ERROR mismatched types
help: consider using a semicolon here
   |
   |
LL |     foo(); //~ ERROR mismatched types
help: try adding a return type
   |
LL | fn bar() -> usize {
   |          ++++++++
---

---- [ui] src/test/ui/closures/issue-52437.rs stdout ----
diff of stderr:

22    |           - expected `()` because of default return type
23 LL |     [(); &(&'static: loop { |x| {}; }) as *const _ as usize]
24    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found array `[(); _]`
+ help: consider using a semicolon at the end of the expression
+    |
+    |
+ LL |     [(); &(&'static: loop { |x| {}; }) as *const _ as usize];
25 
26 error: aborting due to 3 previous errors
27 

---
To only update this specific test, also pass `--test-args closures/issue-52437.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/issue-52437.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/issue-52437" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/issue-52437/auxiliary"
stdout: none
--- stderr -------------------------------
error: invalid label name `'static`
   |
   |
LL |     [(); &(&'static: loop { |x| {}; }) as *const _ as usize]

error[E0282]: type annotations needed
  --> /checkout/src/test/ui/closures/issue-52437.rs:2:30
   |
   |
LL |     [(); &(&'static: loop { |x| {}; }) as *const _ as usize]
   |
   |
help: consider giving this closure parameter an explicit type
   |
LL |     [(); &(&'static: loop { |x: _| {}; }) as *const _ as usize]

error[E0308]: mismatched types
  --> /checkout/src/test/ui/closures/issue-52437.rs:2:5
   |
   |
LL | fn main() {
   |           - expected `()` because of default return type
LL |     [(); &(&'static: loop { |x| {}; }) as *const _ as usize]
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found array `[(); _]`
help: consider using a semicolon at the end of the expression
   |
   |
LL |     [(); &(&'static: loop { |x| {}; }) as *const _ as usize];

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0282, E0308.
---

7 error[E0308]: mismatched types
8   --> $DIR/tab.rs:8:2
9    |
- LL | fn foo() {
-    |          - help: try adding a return type: `-> &'static str`
12 LL |     "bar            boo"
13    |     ^^^^^^^^^^^^^^^^^^^^ expected `()`, found `&str`
+ help: consider using a semicolon at the end of the expression
+    |
+    |
+ LL |     "bar            boo";
+ help: try adding a return type
+    |
+    |
+ LL | fn foo() -> &'static str {
14 
15 error: aborting due to 2 previous errors
16 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/tab/tab.stderr
To only update this specific test, also pass `--test-args codemap_tests/tab.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/codemap_tests/tab.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/tab" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/tab/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find value `bar` in this scope
   |
   |
LL |     bar; //~ ERROR cannot find value `bar`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/codemap_tests/tab.rs:8:2
   |
   |
LL |     "bar            boo" //~ ERROR mismatched types
   |     ^^^^^^^^^^^^^^^^^^^^ expected `()`, found `&str`
help: consider using a semicolon at the end of the expression
   |
   |
LL |     "bar            boo"; //~ ERROR mismatched types
help: try adding a return type
   |
LL | fn foo() -> &'static str {
   |          +++++++++++++++
---

---- [ui] src/test/ui/impl-trait/issues/issue-74282.rs stdout ----
diff of stderr:

31 LL | /     Anonymous(|| {
32 LL | |         3
33 LL | |     })
-    | |      ^- help: consider using a semicolon here: `;`
-    |        expected `()`, found struct `Anonymous`
+    | |______^ expected `()`, found struct `Anonymous`
+    |
+ help: consider using a semicolon at the end of the expression
---
To only update this specific test, also pass `--test-args impl-trait/issues/issue-74282.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/issue-74282.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-74282" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-74282/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/impl-trait/issues/issue-74282.rs:8:15
   |
LL |   type Closure = impl Fn() -> u64;
   |                  ---------------- the expected opaque type
   |                  ---------------- the expected opaque type
...
LL |       Anonymous(|| { //~ ERROR mismatched types
   |  _____---------_^
   | |     arguments to this struct are incorrect
   | |     arguments to this struct are incorrect
LL | |         3 //~^ ERROR mismatched types
LL | |     })
   | |_____^ expected closure, found a different closure
   = note: expected opaque type `Closure`
   = note: expected opaque type `Closure`
                  found closure `[closure@/checkout/src/test/ui/impl-trait/issues/issue-74282.rs:8:15: 8:17]`
   = note: no two closures, even if identical, have the same type
   = help: consider boxing your closure and/or using it as a trait object
  --> /checkout/src/test/ui/impl-trait/issues/issue-74282.rs:4:8
   |
   |
LL | struct Anonymous(Closure);

error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/issues/issue-74282.rs:8:5
   |
   |
LL |   fn main() {
   |             - expected `()` because of default return type
LL |       let y = || -> Closure { || 3 };
LL | /     Anonymous(|| { //~ ERROR mismatched types
LL | |         3 //~^ ERROR mismatched types
LL | |     })
   | |______^ expected `()`, found struct `Anonymous`
help: consider using a semicolon at the end of the expression
   |
LL |     });
   |       +
---

35 error[E0308]: mismatched types
36   --> $DIR/issue-66706.rs:2:5
37    |
- LL | fn a() {
-    |        - help: try adding a return type: `-> [i32; _]`
40 LL |     [0; [|_: _ &_| ()].len()]
41    |     ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found array `[{integer}; _]`
+ help: consider using a semicolon at the end of the expression
+    |
+    |
+ LL |     [0; [|_: _ &_| ()].len()];
+ help: try adding a return type
+    |
+    |
+ LL | fn a() -> [i32; _] {
42 
43 error[E0308]: mismatched types
44   --> $DIR/issue-66706.rs:14:5


45    |
- LL | fn c() {
-    |        - help: try adding a return type: `-> [i32; _]`
48 LL |     [0; [|&_: _ &_| {}; 0 ].len()]
49    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found array `[{integer}; _]`
+ help: consider using a semicolon at the end of the expression
+    |
+    |
+ LL |     [0; [|&_: _ &_| {}; 0 ].len()];
+ help: try adding a return type
+    |
+    |
+ LL | fn c() -> [i32; _] {
50 
51 error[E0308]: mismatched types
52   --> $DIR/issue-66706.rs:20:5


53    |
- LL | fn d() {
-    |        - help: try adding a return type: `-> [i32; _]`
56 LL |     [0; match [|f @ &ref _| () ] {} ]
57    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found array `[{integer}; _]`
+ help: consider using a semicolon at the end of the expression
+    |
+    |
+ LL |     [0; match [|f @ &ref _| () ] {} ];
+ help: try adding a return type
+    |
+    |
+ LL | fn d() -> [i32; _] {
58 
59 error: aborting due to 8 previous errors
60 

---
To only update this specific test, also pass `--test-args issues/issue-66706.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-66706.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66706" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66706/auxiliary"
stdout: none
--- stderr -------------------------------
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

error[E0282]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-66706.rs:2:11
   |
   |
LL |     [0; [|_: _ &_| ()].len()]
   |           ^ cannot infer type
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-66706.rs:2:5
   |
   |
LL |     [0; [|_: _ &_| ()].len()]
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found array `[{integer}; _]`
help: consider using a semicolon at the end of the expression
   |
   |
LL |     [0; [|_: _ &_| ()].len()];
help: try adding a return type
   |
   |
LL | fn a() -> [i32; _] {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-66706.rs:14:5
   |
   |
LL |     [0; [|&_: _ &_| {}; 0 ].len()]
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found array `[{integer}; _]`
help: consider using a semicolon at the end of the expression
   |
   |
LL |     [0; [|&_: _ &_| {}; 0 ].len()];
help: try adding a return type
   |
   |
LL | fn c() -> [i32; _] {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-66706.rs:20:5
   |
   |
LL |     [0; match [|f @ &ref _| () ] {} ]
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found array `[{integer}; _]`
help: consider using a semicolon at the end of the expression
   |
   |
LL |     [0; match [|f @ &ref _| () ] {} ];
help: try adding a return type
   |
   |
LL | fn d() -> [i32; _] {

error: aborting due to 8 previous errors

Some errors have detailed explanations: E0282, E0308.
---
To only update this specific test, also pass `--test-args lint/unused/unused-doc-comments-edge-cases.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused/unused-doc-comments-edge-cases.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-doc-comments-edge-cases" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-doc-comments-edge-cases/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected expression, found keyword `else`
   |
   |
LL |     else { //~ ERROR: expected expression, found keyword `else`

error[E0658]: attributes on expressions are experimental
  --> /checkout/src/test/ui/lint/unused/unused-doc-comments-edge-cases.rs:23:5
   |
   |
LL |     /// useless doc comment
   |     ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #15701 <https://github.com/rust-lang/rust/issues/15701> for more information
   = help: add `#![feature(stmt_expr_attributes)]` to the crate attributes to enable
   = help: `///` is for documentation comments. For a plain comment, use `//`.
error: unused doc comment
  --> /checkout/src/test/ui/lint/unused/unused-doc-comments-edge-cases.rs:6:9
   |
LL |         /// useless doc comment
LL |         /// useless doc comment
   |         ^^^^^^^^^^^^^^^^^^^^^^^
LL |         //~^ ERROR: unused doc comment
   |         ---------- rustdoc does not generate documentation for match arms
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unused/unused-doc-comments-edge-cases.rs:1:9
  --> /checkout/src/test/ui/lint/unused/unused-doc-comments-edge-cases.rs:1:9
   |
LL | #![deny(unused_doc_comments)]
   |         ^^^^^^^^^^^^^^^^^^^
   = help: use `//` for a plain comment
error: unused doc comment
  --> /checkout/src/test/ui/lint/unused/unused-doc-comments-edge-cases.rs:23:5
   |
LL |     /// useless doc comment
LL |     /// useless doc comment
   |     ^^^^^^^^^^^^^^^^^^^^^^^
...
LL |     num == 3
   |     --- rustdoc does not generate documentation for expressions
   |
   = help: use `//` for a plain comment
error: unused doc comment
  --> /checkout/src/test/ui/lint/unused/unused-doc-comments-edge-cases.rs:29:27
   |
   |
LL | fn doc_comment_on_generic<#[doc = "x"] T>(val: T) {}
   |                           ^^^^^^^^^^^^ - rustdoc does not generate documentation for generic parameters
   |
   = help: use `//` for a plain comment
error: unused doc comment
  --> /checkout/src/test/ui/lint/unused/unused-doc-comments-edge-cases.rs:33:5
   |
LL |       /// unused doc comment
LL |       /// unused doc comment
   |       ^^^^^^^^^^^^^^^^^^^^^^
LL |       //~^ ERROR: unused doc comment
LL | /     {
LL | |         let x = 12;
LL | |     }
   | |_____- rustdoc does not generate documentation for expressions
   |
   = help: use `//` for a plain comment
error: unused doc comment
  --> /checkout/src/test/ui/lint/unused/unused-doc-comments-edge-cases.rs:40:1
   |
LL |   /// unused doc comment
LL |   /// unused doc comment
   |   ^^^^^^^^^^^^^^^^^^^^^^
LL |   //~^ ERROR: unused doc comment
LL | / extern "C" {
LL | |     fn foo();
LL | | }
   | |_- rustdoc does not generate documentation for extern blocks
   |
   = help: use `//` for a plain comment
error[E0308]: mismatched types
  --> /checkout/src/test/ui/lint/unused/unused-doc-comments-edge-cases.rs:14:9
   |
LL | /     if num == 3 {
LL | /     if num == 3 {
LL | |         true //~ ERROR: mismatched types
   | |         ^^^^ expected `()`, found `bool`
LL | |     }
   | |_____- expected this to be `()`
help: consider using a semicolon at the end of the expression
   |
   |
LL |         true; //~ ERROR: mismatched types
help: you might have meant to return this value
   |
   |
LL |         return true; //~ ERROR: mismatched types

error: aborting due to 8 previous errors

Some errors have detailed explanations: E0308, E0658.
---
+ help: consider using a semicolon at the end of the expression
+    |
+ LL |         1;
+    |          +
7 help: you might have meant to break the loop with this value
9 LL |         break 1;

15 LL |         1
16    |         ^ expected `()`, found integer
16    |         ^ expected `()`, found integer
17    |
+ help: consider using a semicolon at the end of the expression
+    |
+ LL |         1;
+    |          +
18 help: you might have meant to break the loop with this value
20 LL |         break 1;

26 LL |         1
27    |         ^ expected `()`, found integer
---
To only update this specific test, also pass `--test-args loops/loop-no-implicit-break.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/loops/loop-no-implicit-break.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/loop-no-implicit-break" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/loop-no-implicit-break/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/loops/loop-no-implicit-break.rs:3:9
   |
   |
LL |         1 //~ ERROR mismatched types
   |
help: consider using a semicolon at the end of the expression
   |
   |
LL |         1; //~ ERROR mismatched types
   |          +
help: you might have meant to break the loop with this value
   |
LL |         break 1; //~ ERROR mismatched types
   |         +++++  +
error[E0308]: mismatched types
  --> /checkout/src/test/ui/loops/loop-no-implicit-break.rs:13:9
   |
   |
LL |         1 //~ ERROR mismatched types
   |
help: consider using a semicolon at the end of the expression
   |
   |
LL |         1; //~ ERROR mismatched types
   |          +
help: you might have meant to break the loop with this value
   |
LL |         break 1; //~ ERROR mismatched types
   |         +++++  +
error[E0308]: mismatched types
  --> /checkout/src/test/ui/loops/loop-no-implicit-break.rs:21:9
   |
   |
LL |         1 //~ ERROR mismatched types
   |
help: consider using a semicolon at the end of the expression
   |
   |
LL |         1; //~ ERROR mismatched types
help: you might have meant to return this value
   |
   |
LL |         return 1; //~ ERROR mismatched types
   |         ++++++  +
error[E0308]: mismatched types
  --> /checkout/src/test/ui/loops/loop-no-implicit-break.rs:29:9
   |
   |
LL |         1 //~ ERROR mismatched types
   |
help: consider using a semicolon at the end of the expression
   |
   |
LL |         1; //~ ERROR mismatched types
help: you might have meant to return this value
   |
   |
LL |         return 1; //~ ERROR mismatched types
   |         ++++++  +
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
---
To only update this specific test, also pass `--test-args macros/empty-trailing-stmt.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/empty-trailing-stmt.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/empty-trailing-stmt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/empty-trailing-stmt/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/macros/empty-trailing-stmt.rs:6:7
   |
   |
LL |     { true } //~ ERROR mismatched
   |       ^^^^ expected `()`, found `bool`
help: consider using a semicolon at the end of the expression
   |
   |
LL |     { true; } //~ ERROR mismatched
help: you might have meant to return this value
   |
   |
LL |     { return true; } //~ ERROR mismatched

error[E0308]: mismatched types
  --> /checkout/src/test/ui/macros/empty-trailing-stmt.rs:5:13
   |
   |
LL | fn foo() -> bool { //~ ERROR mismatched
   |    ---      ^^^^ expected `bool`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
---

1 error[E0308]: mismatched types
2   --> $DIR/issue-19109.rs:4:5
3    |
- LL | fn function(t: &mut dyn Trait) {
-    |                                - help: try adding a return type: `-> *mut dyn Trait`
6 LL |     t as *mut dyn Trait
7    |     ^^^^^^^^^^^^^^^^^^^ expected `()`, found *-ptr

9    = note: expected unit type `()`
9    = note: expected unit type `()`
10             found raw pointer `*mut dyn Trait`
+ help: consider using a semicolon at the end of the expression
+ LL |     t as *mut dyn Trait;
+    |                        +
+ help: try adding a return type
+    |
+    |
+ LL | fn function(t: &mut dyn Trait) -> *mut dyn Trait {
11 
12 error: aborting due to previous error
13 

---
To only update this specific test, also pass `--test-args mismatched_types/issue-19109.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/issue-19109.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-19109" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-19109/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/mismatched_types/issue-19109.rs:4:5
   |
LL |     t as *mut dyn Trait
LL |     t as *mut dyn Trait
   |     ^^^^^^^^^^^^^^^^^^^ expected `()`, found *-ptr
   = note: expected unit type `()`
   = note: expected unit type `()`
            found raw pointer `*mut dyn Trait`
help: consider using a semicolon at the end of the expression
LL |     t as *mut dyn Trait;
   |                        +
help: try adding a return type
   |
   |
LL | fn function(t: &mut dyn Trait) -> *mut dyn Trait {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
---
To only update this specific test, also pass `--test-args mismatched_types/for-loop-has-unit-body.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/for-loop-has-unit-body.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/for-loop-has-unit-body" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/for-loop-has-unit-body/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/mismatched_types/for-loop-has-unit-body.rs:3:9
   |
   |
LL |         x //~ ERROR mismatched types
   |
help: consider using a semicolon at the end of the expression
   |
   |
LL |         x; //~ ERROR mismatched types

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
---
7    = note: expected unit type `()`
8               found reference `&_`
+ help: consider using a semicolon at the end of the expression
+    |
+ LL |     &panic!();
9 help: a return type might be missing here
10    |
10    |
11 LL | fn g() -> _ {

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/diverging-tuple-parts-39485/diverging-tuple-parts-39485.stderr
To only update this specific test, also pass `--test-args never_type/diverging-tuple-parts-39485.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/never_type/diverging-tuple-parts-39485.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/diverging-tuple-parts-39485" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/diverging-tuple-parts-39485/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/never_type/diverging-tuple-parts-39485.rs:8:5
   |
   |
LL |     &panic!() //~ ERROR mismatched types
   |
   = note: expected unit type `()`
              found reference `&_`
help: consider using a semicolon at the end of the expression
help: consider using a semicolon at the end of the expression
   |
LL |     &panic!(); //~ ERROR mismatched types
help: a return type might be missing here
   |
   |
LL | fn g() -> _ {
help: consider removing the borrow
   |
   |
LL -     &panic!() //~ ERROR mismatched types
LL +     panic!() //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/never_type/diverging-tuple-parts-39485.rs:12:5
   |
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
---
8    |     expected this to be `()`
9    |
+ help: consider using a semicolon at the end of the expression
+    |
+ LL |     if let Some(x) = a { true; } else { false }
10 help: you might have meant to return this value
11    |
11    |
12 LL |     if let Some(x) = a { return true; } else { false }
21    |     |                                  expected `()`, found `bool`
22    |     expected this to be `()`
23    |
+ help: consider using a semicolon at the end of the expression
+ help: consider using a semicolon at the end of the expression
+    |
+ LL |     if let Some(x) = a { true } else { false; }
24 help: you might have meant to return this value
25    |
25    |
26 LL |     if let Some(x) = a { true } else { return false; }

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/expr-as-stmt-2/expr-as-stmt-2.stderr
To only update this specific test, also pass `--test-args parser/expr-as-stmt-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/expr-as-stmt-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/expr-as-stmt-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/expr-as-stmt-2/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/parser/expr-as-stmt-2.rs:3:26
   |
   |
LL |     if let Some(x) = a { true } else { false }
   |     |                    |
   |     |                    expected `()`, found `bool`
   |     expected this to be `()`
   |
   |
help: consider using a semicolon at the end of the expression
   |
LL |     if let Some(x) = a { true; } else { false }
help: you might have meant to return this value
   |
   |
LL |     if let Some(x) = a { return true; } else { false }

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/expr-as-stmt-2.rs:3:40
   |
   |
LL |     if let Some(x) = a { true } else { false }
   |     |                                  |
   |     |                                  expected `()`, found `bool`
   |     expected this to be `()`
   |
   |
help: consider using a semicolon at the end of the expression
   |
LL |     if let Some(x) = a { true } else { false; }
help: you might have meant to return this value
   |
   |
LL |     if let Some(x) = a { true } else { return false; }

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/expr-as-stmt-2.rs:6:5
   |
   |
LL |   fn foo(a: Option<u32>, b: Option<u32>) -> bool {
   |                                             ---- expected `bool` because of return type
...
LL | /     && //~ ERROR mismatched types
LL | |     if let Some(y) = a { true } else { false }
   | |______________________________________________^ expected `bool`, found `&&bool`
help: consider removing the `&&`
   |
   |
LL -     && //~ ERROR mismatched types
LL +     if let Some(y) = a { true } else { false }
help: parentheses are required to parse this as an expression
   |
   |
LL |     (if let Some(x) = a { true } else { false })

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.
---
diff of stderr:

59   --> $DIR/expr-as-stmt.rs:64:7
60    |
61 LL |     { foo() } || { true }
-    |       ^^^^^- help: consider using a semicolon here: `;`
-    |       expected `()`, found `i32`
+    |       ^^^^^ expected `()`, found `i32`
+    |
+ help: consider using a semicolon at the end of the expression
+ help: consider using a semicolon at the end of the expression
+    |
+ LL |     { foo(); } || { true }
+ help: consider using a semicolon here
+    |
+    |
+ LL |     { foo(); } || { true }
65 
66 error[E0308]: mismatched types
67   --> $DIR/expr-as-stmt.rs:8:6


69 LL |     {2} + {2}
70    |      ^ expected `()`, found integer
71    |
+ help: consider using a semicolon at the end of the expression
+    |
+ LL |     {2;} + {2}
72 help: you might have meant to return this value
73    |
73    |
74 LL |     {return 2;} + {2}
80 LL |     {2} + 2
81    |      ^ expected `()`, found integer
82    |
+ help: consider using a semicolon at the end of the expression
+ help: consider using a semicolon at the end of the expression
+    |
+ LL |     {2;} + 2
83 help: you might have meant to return this value
84    |
84    |
85 LL |     {return 2;} + 2

91 LL |     { 42 } + foo;
93    |
+ help: consider using a semicolon at the end of the expression
+    |
+    |
+ LL |     { 42; } + foo;
94 help: you might have meant to return this value
95    |
95    |
96 LL |     { return 42; } + foo;

102 LL |     { 3 } * 3
104    |
+ help: consider using a semicolon at the end of the expression
+    |
+    |
+ LL |     { 3; } * 3
105 help: you might have meant to return this value
106    |
106    |
107 LL |     { return 3; } * 3
124 LL |     {2} - 2
125    |      ^ expected `()`, found integer
126    |
+ help: consider using a semicolon at the end of the expression
+ help: consider using a semicolon at the end of the expression
+    |
+ LL |     {2;} - 2
127 help: you might have meant to return this value
128    |
128    |
129 LL |     {return 2;} - 2

146 LL |     { true } | { true }
147    |       ^^^^ expected `()`, found `bool`
+ help: consider using a semicolon at the end of the expression
+    |
+    |
+ LL |     { true; } | { true }
149 help: you might have meant to return this value
150    |
150    |
151 LL |     { return true; } | { true }

157 LL |     { true } && { true }
158    |       ^^^^ expected `()`, found `bool`
+ help: consider using a semicolon at the end of the expression
+    |
+    |
+ LL |     { true; } && { true }
160 help: you might have meant to return this value
161    |
161    |
162 LL |     { return true; } && { true }

186 LL |     { true } || { true }
187    |       ^^^^ expected `()`, found `bool`
+ help: consider using a semicolon at the end of the expression
+    |
+    |
+ LL |     { true; } || { true }
189 help: you might have meant to return this value
190    |
190    |
191 LL |     { return true; } || { true }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/expr-as-stmt/expr-as-stmt.stderr
diff of fixed:
diff of fixed:

5 #![allow(unused_must_use)]
6 
7 fn foo() -> i32 {
-     ({2}) + {2} //~ ERROR expected expression, found `+`
+     ({2;}) + {2} //~ ERROR expected expression, found `+`
9     //~^ ERROR mismatched types
11 

12 fn bar() -> i32 {
12 fn bar() -> i32 {
-     ({2}) + 2 //~ ERROR leading `+` is not supported
+     ({2;}) + 2 //~ ERROR leading `+` is not supported
14     //~^ ERROR mismatched types
16 


17 fn zul() -> u32 {
18     let foo = 3;
-     ({ 42 }) + foo; //~ ERROR expected expression, found `+`
+     ({ 42; }) + foo; //~ ERROR expected expression, found `+`
20     //~^ ERROR mismatched types
22 }

23 
23 
24 fn baz() -> i32 {
-     ({ 3 }) * 3 //~ ERROR type `{integer}` cannot be dereferenced
+     ({ 3; }) * 3 //~ ERROR type `{integer}` cannot be dereferenced
26     //~^ ERROR mismatched types
28 

33 }
34 
34 
35 fn qux() -> u32 {
-     ({2}) - 2 //~ ERROR cannot apply unary operator `-` to type `u32`
+     ({2;}) - 2 //~ ERROR cannot apply unary operator `-` to type `u32`
37     //~^ ERROR mismatched types
39 


40 fn space_cadet() -> bool {
-     ({ true }) | { true } //~ ERROR E0308
+     ({ true; }) | { true } //~ ERROR E0308
42     //~^ ERROR expected parameter name
44 


45 fn revenge_from_mars() -> bool {
-     ({ true }) && { true } //~ ERROR E0308
+     ({ true; }) && { true } //~ ERROR E0308
47     //~^ ERROR mismatched types
49 

50 fn attack_from_mars() -> bool {
50 fn attack_from_mars() -> bool {
-     ({ true }) || { true } //~ ERROR E0308
+     ({ true; }) || { true } //~ ERROR E0308
52     //~^ ERROR mismatched types
54 


61 // all the ones above use? Nothing. It makes neither suggestion in
62 // that case.
63 fn asteroids() -> impl FnOnce() -> bool {
-     { foo(); } || { true } //~ ERROR E0308
+     { foo();; } || { true } //~ ERROR E0308
66 
67 fn main() {}



The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/expr-as-stmt/expr-as-stmt.fixed
To only update this specific test, also pass `--test-args parser/expr-as-stmt.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/expr-as-stmt.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/expr-as-stmt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/expr-as-stmt/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected expression, found `+`
   |
   |
LL |     {2} + {2} //~ ERROR expected expression, found `+`
   |
help: parentheses are required to parse this as an expression
   |
   |
LL |     ({2}) + {2} //~ ERROR expected expression, found `+`
   |     +   +

error: leading `+` is not supported
   |
   |
LL |     {2} + 2 //~ ERROR leading `+` is not supported
   |         ^ unexpected `+`
help: parentheses are required to parse this as an expression
   |
   |
LL |     ({2}) + 2 //~ ERROR leading `+` is not supported
   |     +   +
error: expected expression, found `+`
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:19:12
   |
   |
LL |     { 42 } + foo; //~ ERROR expected expression, found `+`
   |
help: parentheses are required to parse this as an expression
   |
   |
LL |     ({ 42 }) + foo; //~ ERROR expected expression, found `+`

error: expected expression, found `>`
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:32:7
   |
   |
LL |     } > 0 //~ ERROR expected expression
   |
help: parentheses are required to parse this as an expression
   |
LL ~     (match x {
LL ~     (match x {
LL |         _ => 1,
LL ~     }) > 0 //~ ERROR expected expression

error: expected parameter name, found `{`
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:41:16
   |
   |
LL |     { true } | { true } //~ ERROR E0308
   |                ^ expected parameter name
help: parentheses are required to parse this as an expression
   |
   |
LL |     ({ true }) | { true } //~ ERROR E0308

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:64:7
   |
   |
LL |     { foo() } || { true } //~ ERROR E0308
   |       ^^^^^ expected `()`, found `i32`
help: consider using a semicolon at the end of the expression
   |
   |
LL |     { foo(); } || { true } //~ ERROR E0308
help: consider using a semicolon here
   |
   |
LL |     { foo(); } || { true } //~ ERROR E0308

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:8:6
   |
   |
LL |     {2} + {2} //~ ERROR expected expression, found `+`
   |
help: consider using a semicolon at the end of the expression
   |
   |
LL |     {2;} + {2} //~ ERROR expected expression, found `+`
help: you might have meant to return this value
   |
   |
LL |     {return 2;} + {2} //~ ERROR expected expression, found `+`
   |      ++++++  +
error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:13:6
   |
   |
LL |     {2} + 2 //~ ERROR leading `+` is not supported
   |
help: consider using a semicolon at the end of the expression
   |
   |
LL |     {2;} + 2 //~ ERROR leading `+` is not supported
help: you might have meant to return this value
   |
   |
LL |     {return 2;} + 2 //~ ERROR leading `+` is not supported
   |      ++++++  +
error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:19:7
   |
   |
LL |     { 42 } + foo; //~ ERROR expected expression, found `+`
   |
help: consider using a semicolon at the end of the expression
   |
   |
LL |     { 42; } + foo; //~ ERROR expected expression, found `+`
help: you might have meant to return this value
   |
   |
LL |     { return 42; } + foo; //~ ERROR expected expression, found `+`
   |       ++++++   +
error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:25:7
   |
   |
LL |     { 3 } * 3 //~ ERROR type `{integer}` cannot be dereferenced
   |
help: consider using a semicolon at the end of the expression
   |
   |
LL |     { 3; } * 3 //~ ERROR type `{integer}` cannot be dereferenced
help: you might have meant to return this value
   |
   |
LL |     { return 3; } * 3 //~ ERROR type `{integer}` cannot be dereferenced
   |       ++++++  +
error[E0614]: type `{integer}` cannot be dereferenced
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:25:11
   |
   |
LL |     { 3 } * 3 //~ ERROR type `{integer}` cannot be dereferenced
   |
help: parentheses are required to parse this as an expression
   |
   |
LL |     ({ 3 }) * 3 //~ ERROR type `{integer}` cannot be dereferenced

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:36:6
   |
   |
LL |     {2} - 2 //~ ERROR cannot apply unary operator `-` to type `u32`
   |
help: consider using a semicolon at the end of the expression
   |
   |
LL |     {2;} - 2 //~ ERROR cannot apply unary operator `-` to type `u32`
help: you might have meant to return this value
   |
   |
LL |     {return 2;} - 2 //~ ERROR cannot apply unary operator `-` to type `u32`
   |      ++++++  +

error[E0600]: cannot apply unary operator `-` to type `u32`
   |
   |
LL |     {2} - 2 //~ ERROR cannot apply unary operator `-` to type `u32`
   |         ^^^ cannot apply unary operator `-`
help: parentheses are required to parse this as an expression
   |
   |
LL |     ({2}) - 2 //~ ERROR cannot apply unary operator `-` to type `u32`
   |     +   +
error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:41:7
   |
   |
LL |     { true } | { true } //~ ERROR E0308
   |       ^^^^ expected `()`, found `bool`
help: consider using a semicolon at the end of the expression
   |
   |
LL |     { true; } | { true } //~ ERROR E0308
help: you might have meant to return this value
   |
   |
LL |     { return true; } | { true } //~ ERROR E0308

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:46:7
   |
   |
LL |     { true } && { true } //~ ERROR E0308
   |       ^^^^ expected `()`, found `bool`
help: consider using a semicolon at the end of the expression
   |
   |
LL |     { true; } && { true } //~ ERROR E0308
help: you might have meant to return this value
   |
   |
LL |     { return true; } && { true } //~ ERROR E0308

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:46:14
   |
   |
LL | fn revenge_from_mars() -> bool {
   |                           ---- expected `bool` because of return type
LL |     { true } && { true } //~ ERROR E0308
   |              ^^^^^^^^^^^ expected `bool`, found `&&bool`
help: consider removing the `&&`
   |
   |
LL -     { true } && { true } //~ ERROR E0308
LL +     { true } { true } //~ ERROR E0308
help: parentheses are required to parse this as an expression
   |
   |
LL |     ({ true }) && { true } //~ ERROR E0308

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:51:7
   |
   |
LL |     { true } || { true } //~ ERROR E0308
   |       ^^^^ expected `()`, found `bool`
help: consider using a semicolon at the end of the expression
   |
   |
LL |     { true; } || { true } //~ ERROR E0308
help: you might have meant to return this value
   |
   |
LL |     { return true; } || { true } //~ ERROR E0308

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:51:14
   |
   |
LL | fn attack_from_mars() -> bool {
   |                          ---- expected `bool` because of return type
LL |     { true } || { true } //~ ERROR E0308
   |              ^^^^^^^^^^^ expected `bool`, found closure
   = note: expected type `bool`
   = note: expected type `bool`
           found closure `[closure@/checkout/src/test/ui/parser/expr-as-stmt.rs:51:14: 51:16]`
help: use parentheses to call this closure
   |
LL |     { true } (|| { true })() //~ ERROR E0308
help: parentheses are required to parse this as an expression
   |
   |
LL |     ({ true }) || { true } //~ ERROR E0308

error: aborting due to 18 previous errors

Some errors have detailed explanations: E0308, E0600, E0614.
---
63    |     expected this to be `()`
+    |
+ help: consider using a semicolon at the end of the expression
+    |
+ LL |     if x == E::V { field; } {}
64 
65 error[E0308]: mismatched types
66   --> $DIR/struct-literal-variant-in-if.rs:21:20



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/struct-literal-variant-in-if/struct-literal-variant-in-if.stderr
To only update this specific test, also pass `--test-args parser/struct-literal-variant-in-if.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/struct-literal-variant-in-if.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/struct-literal-variant-in-if" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/struct-literal-variant-in-if/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/parser/struct-literal-variant-in-if.rs:13:13
   |
   |
LL |     if x == E::I { field1: true, field2: 42 } {}
   |
   |
help: surround the struct literal with parentheses
   |
LL |     if x == (E::I { field1: true, field2: 42 }) {}

error: struct literals are not allowed here
  --> /checkout/src/test/ui/parser/struct-literal-variant-in-if.rs:15:13
   |
   |
LL |     if x == E::V { field: false } {}
   |
   |
help: surround the struct literal with parentheses
   |
LL |     if x == (E::V { field: false }) {}

error: struct literals are not allowed here
  --> /checkout/src/test/ui/parser/struct-literal-variant-in-if.rs:17:13
   |
   |
LL |     if x == E::J { field: -42 } {}
   |
   |
help: surround the struct literal with parentheses
   |
LL |     if x == (E::J { field: -42 }) {}

error: struct literals are not allowed here
  --> /checkout/src/test/ui/parser/struct-literal-variant-in-if.rs:19:13
   |
   |
LL |     if x == E::K { field: "" } {}
   |
   |
help: surround the struct literal with parentheses
   |
LL |     if x == (E::K { field: "" }) {}

error[E0423]: expected value, found struct variant `E::V`
  --> /checkout/src/test/ui/parser/struct-literal-variant-in-if.rs:10:13
   |
   |
LL |     if x == E::V { field } {}
   |
   |
help: surround the struct literal with parentheses
   |
LL |     if x == (E::V { field }) {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/struct-literal-variant-in-if.rs:10:20
   |
   |
LL |     if x == E::V { field } {}
   |     |              |
   |     |              expected `()`, found `bool`
   |     expected this to be `()`
   |
   |
help: consider using a semicolon at the end of the expression
   |
LL |     if x == E::V { field; } {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/struct-literal-variant-in-if.rs:21:20
   |
   |
LL |     let y: usize = ();
   |            |
   |            expected due to this

error: aborting due to 7 previous errors
---

---- [ui] src/test/ui/proc-macro/issue-37788.rs stdout ----
diff of stderr:

5    |           - expected `()` because of default return type
6 LL |     // Test that constructing the `visible_parent_map` (in `cstore_impl.rs`) does not ICE.
7 LL |     std::cell::Cell::new(0)
-    |     ^^^^^^^^^^^^^^^^^^^^^^^- help: consider using a semicolon here: `;`
-    |     expected `()`, found struct `Cell`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found struct `Cell`
11    |
12    = note: expected unit type `()`
12    = note: expected unit type `()`
13                  found struct `Cell<{integer}>`
+ help: consider using a semicolon at the end of the expression
+    |
+ LL |     std::cell::Cell::new(0);
+    |                            +
---
To only update this specific test, also pass `--test-args proc-macro/issue-37788.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/issue-37788.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-37788" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-37788/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/proc-macro/issue-37788.rs:8:5
   |
LL | fn main() {
LL | fn main() {
   |           - expected `()` because of default return type
LL |     // Test that constructing the `visible_parent_map` (in `cstore_impl.rs`) does not ICE.
LL |     std::cell::Cell::new(0) //~ ERROR mismatched types
   |     ^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found struct `Cell`
   = note: expected unit type `()`
   = note: expected unit type `()`
                 found struct `Cell<{integer}>`
help: consider using a semicolon at the end of the expression
LL |     std::cell::Cell::new(0); //~ ERROR mismatched types
   |                            +
help: consider using a semicolon here
   |
---
16    |
17    = note: this error originates in the macro `resolve_located_at` (in Nightly builds, run with -Z macro-backtrace for more info)
+ help: consider using a semicolon at the end of the expression
+    |
+ LL |     resolve_located_at!(a b;)
18 
19 error: aborting due to 2 previous errors
20 

---
To only update this specific test, also pass `--test-args proc-macro/resolved-located-at.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/resolved-located-at.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/resolved-located-at" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/resolved-located-at/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/proc-macro/resolved-located-at.rs:7:25
   |
   |
LL |     resolve_located_at!(a b)
   |
   = note: this error originates in the macro `resolve_located_at` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/proc-macro/resolved-located-at.rs:7:27
   |
LL | fn main() {
   |           - expected `()` because of default return type
LL |     resolve_located_at!(a b)
   |
   = note: this error originates in the macro `resolve_located_at` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider using a semicolon at the end of the expression
   |
   |
LL |     resolve_located_at!(a b;)

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
---

37 error[E0308]: mismatched types
38   --> $DIR/span-preservation.rs:39:5
39    |
- LL | extern "C" fn bar() {
-    |                     - help: try adding a return type: `-> i32`
43    |     ^ expected `()`, found integer
+    |
+ help: consider using a semicolon at the end of the expression
+    |
---
45 error[E0308]: mismatched types
46   --> $DIR/span-preservation.rs:44:5

47    |
- LL | extern "C" fn baz() {
-    |                     - help: try adding a return type: `-> i32`
51    |     ^ expected `()`, found integer
+    |
+ help: consider using a semicolon at the end of the expression
+    |
+    |
+ LL |     0;
+    |      +
+ help: try adding a return type
+    |
+ LL | extern "C" fn baz() -> i32 {
52 
53 error[E0308]: mismatched types
54   --> $DIR/span-preservation.rs:49:5


55    |
- LL | extern "Rust" fn rust_abi() {
-    |                             - help: try adding a return type: `-> i32`
59    |     ^ expected `()`, found integer
+    |
+ help: consider using a semicolon at the end of the expression
+    |
+    |
+ LL |     0;
+    |      +
+ help: try adding a return type
+    |
+ LL | extern "Rust" fn rust_abi() -> i32 {
60 
61 error[E0308]: mismatched types
62   --> $DIR/span-preservation.rs:54:5


63    |
- LL | extern "\x43" fn c_abi_escaped() {
-    |                                  - help: try adding a return type: `-> i32`
67    |     ^ expected `()`, found integer
+    |
+ help: consider using a semicolon at the end of the expression
+    |
+    |
+ LL |     0;
+    |      +
+ help: try adding a return type
+    |
+ LL | extern "\x43" fn c_abi_escaped() -> i32 {
68 
69 error: aborting due to 8 previous errors
70 

---
To only update this specific test, also pass `--test-args proc-macro/span-preservation.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/span-preservation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/span-preservation" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/span-preservation/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/proc-macro/span-preservation.rs:11:20
   |
   |
LL |     let x: usize = "hello"; //~ ERROR mismatched types
   |            -----   ^^^^^^^ expected `usize`, found `&str`
   |            expected due to this

error[E0308]: mismatched types
  --> /checkout/src/test/ui/proc-macro/span-preservation.rs:17:29
  --> /checkout/src/test/ui/proc-macro/span-preservation.rs:17:29
   |
LL | fn b(x: Option<isize>) -> usize {
   |                           ----- expected `usize` because of return type
LL |     match x {
LL |         Some(x) => { return x }, //~ ERROR mismatched types
   |                             ^ expected `usize`, found `isize`
   |
help: you can convert an `isize` to a `usize` and panic if the converted value doesn't fit
   |
LL |         Some(x) => { return x.try_into().unwrap() }, //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/proc-macro/span-preservation.rs:33:22
   |
   |
LL |     let x = Foo { a: 10isize }; //~ ERROR mismatched types
   |                      ^^^^^^^ expected `usize`, found `isize`
error[E0560]: struct `Foo` has no field named `b`
  --> /checkout/src/test/ui/proc-macro/span-preservation.rs:34:26
   |
   |
LL |     let y = Foo { a: 10, b: 10isize }; //~ ERROR has no field named `b`
   |                          ^ `Foo` does not have this field
   = note: available fields are: `a`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/proc-macro/span-preservation.rs:39:5
  --> /checkout/src/test/ui/proc-macro/span-preservation.rs:39:5
   |
LL |     0 //~ ERROR mismatched types
   |
help: consider using a semicolon at the end of the expression
   |
   |
LL |     0; //~ ERROR mismatched types
help: try adding a return type
   |
LL | extern "C" fn bar() -> i32 {
   |                     ++++++
   |                     ++++++

error[E0308]: mismatched types
  --> /checkout/src/test/ui/proc-macro/span-preservation.rs:44:5
   |
LL |     0 //~ ERROR mismatched types
   |
help: consider using a semicolon at the end of the expression
   |
   |
LL |     0; //~ ERROR mismatched types
help: try adding a return type
   |
   |
LL | extern "C" fn baz() -> i32 {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/proc-macro/span-preservation.rs:49:5
   |
   |
LL |     0 //~ ERROR mismatched types
   |
help: consider using a semicolon at the end of the expression
   |
   |
LL |     0; //~ ERROR mismatched types
help: try adding a return type
   |
   |
LL | extern "Rust" fn rust_abi() -> i32 {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/proc-macro/span-preservation.rs:54:5
   |
   |
LL |     0 //~ ERROR mismatched types
   |
help: consider using a semicolon at the end of the expression
   |
   |
LL |     0; //~ ERROR mismatched types
help: try adding a return type
   |
   |
LL | extern "\x43" fn c_abi_escaped() -> i32 {

error: aborting due to 8 previous errors

Some errors have detailed explanations: E0308, E0560.
---
To only update this specific test, also pass `--test-args return/issue-82612-return-mutable-reference.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/return/issue-82612-return-mutable-reference.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/issue-82612-return-mutable-reference" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/issue-82612-return-mutable-reference/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/return/issue-82612-return-mutable-reference.rs:18:13
   |
   |
LL | /         if index < self.values.len() {
LL | |             let value = unsafe { self.values.get_unchecked_mut(index) };
LL | |             value.get_or_insert_with(func) //~ ERROR mismatched types
   | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found `&mut V`
LL | |         }
   | |_________- expected this to be `()`
   = note:      expected unit type `()`
           found mutable reference `&mut V`
help: consider using a semicolon at the end of the expression
   |
   |
LL |             value.get_or_insert_with(func); //~ ERROR mismatched types
help: consider using a semicolon here
   |
   |
LL |             value.get_or_insert_with(func); //~ ERROR mismatched types
help: consider using a semicolon here
   |
LL |         };
   |          +
   |          +
help: you might have meant to return this value
   |
LL |             return value.get_or_insert_with(func); //~ ERROR mismatched types

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
---
To only update this specific test, also pass `--test-args return/return-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/return/return-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/return-type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/return-type/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/return/return-type.rs:10:5
   |
LL |     foo(4 as usize)
   |     ^^^^^^^^^^^^^^^ expected `()`, found struct `S`
---
LL |     foo(4 as usize);
   |                    +
help: try adding a return type
   |
LL | fn bar() -> S<usize> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
---
To only update this specific test, also pass `--test-args return/tail-expr-as-potential-return.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/return/tail-expr-as-potential-return.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/tail-expr-as-potential-return" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/tail-expr-as-potential-return/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/return/tail-expr-as-potential-return.rs:28:9
   |
LL | /     if x {
LL | /     if x {
LL | |         Err(42) //~ ERROR mismatched types
   | |         ^^^^^^^ expected `()`, found enum `Result`
LL | |                 //| HELP you might have meant to return this value
LL | |     }
   | |_____- expected this to be `()`
   = note: expected unit type `()`
                   found enum `Result<_, {integer}>`
help: consider using a semicolon at the end of the expression
   |
---

error[E0308]: mismatched types
  --> /checkout/src/test/ui/return/tail-expr-as-potential-return.rs:20:9
   |
LL | /     if x {
LL | |         Err(42) //~ ERROR mismatched types
   | |         ^^^^^^^ expected `()`, found enum `Result`
LL | |                 //| HELP you might have meant to return this value
LL | |     }
   | |_____- expected this to be `()`
   = note: expected unit type `()`
                   found enum `Result<_, {integer}>`
help: consider using a semicolon at the end of the expression
   |
---

1867 error[E0308]: mismatched types
1868   --> $DIR/disallowed-positions.rs:377:5
1869    |
- LL | fn outside_if_and_while_expr() {
-    |                                - help: try adding a return type: `-> &bool`
- ...
1873 LL |     &let 0 = 0
1874    |     ^^^^^^^^^^ expected `()`, found `&bool`
+ help: consider using a semicolon at the end of the expression
+    |
+    |
+ LL |     &let 0 = 0;
+ help: try adding a return type
+    |
+    |
+ LL | fn outside_if_and_while_expr() -> &bool {
1875 
1876 error[E0277]: the `?` operator can only be applied to values that implement `Try`
1877   --> $DIR/disallowed-positions.rs:319:17

---
To only update this specific test, also pass `--test-args rfc-2497-if-let-chains/disallowed-positions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected expression, found `let` statement
   |
   |
LL |     if (let 0 = 1) {}


error: expected expression, found `let` statement
   |
   |
LL |     if (((let 0 = 1))) {}


error: expected expression, found `let` statement
   |
   |
LL |     if (let 0 = 1) && true {}


error: expected expression, found `let` statement
   |
   |
LL |     if true && (let 0 = 1) {}


error: expected expression, found `let` statement
   |
   |
LL |     if (let 0 = 1) && (let 0 = 1) {}


error: expected expression, found `let` statement
   |
   |
LL |     if (let 0 = 1) && (let 0 = 1) {}


error: expected expression, found `let` statement
   |
   |
LL |     if let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}


error: expected expression, found `let` statement
   |
   |
LL |     while (let 0 = 1) {}


error: expected expression, found `let` statement
   |
   |
LL |     while (((let 0 = 1))) {}


error: expected expression, found `let` statement
   |
   |
LL |     while (let 0 = 1) && true {}


error: expected expression, found `let` statement
   |
   |
LL |     while true && (let 0 = 1) {}


error: expected expression, found `let` statement
   |
   |
LL |     while (let 0 = 1) && (let 0 = 1) {}


error: expected expression, found `let` statement
   |
   |
LL |     while (let 0 = 1) && (let 0 = 1) {}


error: expected expression, found `let` statement
   |
   |
LL |     while let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}


error: expected expression, found `let` statement
   |
   |
LL |     if &let 0 = 0 {}


error: expected expression, found `let` statement
   |
   |
LL |     if !let 0 = 0 {}


error: expected expression, found `let` statement
   |
   |
LL |     if *let 0 = 0 {}


error: expected expression, found `let` statement
   |
   |
LL |     if -let 0 = 0 {}


error: expected expression, found `let` statement
   |
   |
LL |     if (let 0 = 0)? {}


error: expected expression, found `let` statement
   |
   |
LL |     if true || let 0 = 0 {}


error: expected expression, found `let` statement
   |
   |
LL |     if (true || let 0 = 0) {}


error: expected expression, found `let` statement
   |
   |
LL |     if true && (true || let 0 = 0) {}


error: expected expression, found `let` statement
   |
   |
LL |     if x = let 0 = 0 {}


error: expected expression, found `let` statement
   |
   |
LL |     if true..(let 0 = 0) {}


error: expected expression, found `let` statement
   |
   |
LL |     if ..(let 0 = 0) {}


error: expected expression, found `let` statement
   |
   |
LL |     if (let 0 = 0).. {}


error: expected expression, found `let` statement
   |
LL |     if let true = let true = true {}
   |                   ^^^


error: expected expression, found `let` statement
   |
   |
LL |     while &let 0 = 0 {}


error: expected expression, found `let` statement
   |
   |
LL |     while !let 0 = 0 {}


error: expected expression, found `let` statement
   |
   |
LL |     while *let 0 = 0 {}


error: expected expression, found `let` statement
   |
   |
LL |     while -let 0 = 0 {}


error: expected expression, found `let` statement
   |
   |
LL |     while (let 0 = 0)? {}


error: expected expression, found `let` statement
   |
   |
LL |     while true || let 0 = 0 {}


error: expected expression, found `let` statement
   |
   |
LL |     while (true || let 0 = 0) {}


error: expected expression, found `let` statement
   |
   |
LL |     while true && (true || let 0 = 0) {}


error: expected expression, found `let` statement
   |
   |
LL |     while x = let 0 = 0 {}


error: expected expression, found `let` statement
   |
   |
LL |     while true..(let 0 = 0) {}


error: expected expression, found `let` statement
   |
   |
LL |     while ..(let 0 = 0) {}


error: expected expression, found `let` statement
   |
   |
LL |     while (let 0 = 0).. {}


error: expected expression, found `let` statement
   |
LL |     while let true = let true = true {}
   |                      ^^^


error: expected expression, found `let` statement
   |
   |
LL |     &let 0 = 0;


error: expected expression, found `let` statement
   |
   |
LL |     !let 0 = 0;


error: expected expression, found `let` statement
   |
   |
LL |     *let 0 = 0;


error: expected expression, found `let` statement
   |
   |
LL |     -let 0 = 0;


error: expected expression, found `let` statement
   |
   |
LL |     (let 0 = 0)?;


error: expected expression, found `let` statement
   |
   |
LL |     true || let 0 = 0;


error: expected expression, found `let` statement
   |
   |
LL |     (true || let 0 = 0);


error: expected expression, found `let` statement
   |
   |
LL |     true && (true || let 0 = 0);


error: expected expression, found `let` statement
   |
   |
LL |     x = let 0 = 0;


error: expected expression, found `let` statement
   |
   |
LL |     true..(let 0 = 0);


error: expected expression, found `let` statement
   |
   |
LL |     ..(let 0 = 0);


error: expected expression, found `let` statement
   |
   |
LL |     (let 0 = 0)..;


error: expected expression, found `let` statement
   |
   |
LL |     (let Range { start: _, end: _ } = true..true || false);


error: expected expression, found `let` statement
   |
LL |     (let true = let true = true);
   |      ^^^


error: expected expression, found `let` statement
   |
LL |     (let true = let true = true);
   |                 ^^^


error: expected expression, found `let` statement
   |
   |
LL |         let x = true && let y = 1;


error: expected expression, found `let` statement
   |
   |
LL |         [1, 2, 3][let _ = ()]


error: expected expression, found `let` statement
   |
   |
LL |     &let 0 = 0


error: expressions must be enclosed in braces to be used as const generic arguments
   |
   |
LL |         true && let 1 = 1
   |
help: enclose the `const` expression in braces
   |
   |
LL |         { true && let 1 = 1 }


error: expected expression, found `let` statement
   |
   |
LL |     if (let Some(a) = opt && true) {


error: expected expression, found `let` statement
   |
   |
LL |     if (let Some(a) = opt) && true {


error: expected expression, found `let` statement
   |
   |
LL |     if (let Some(a) = opt) && (let Some(b) = a) {


error: expected expression, found `let` statement
   |
   |
LL |     if (let Some(a) = opt) && (let Some(b) = a) {


error: expected expression, found `let` statement
   |
   |
LL |     if (let Some(a) = opt && (let Some(b) = a)) && b == 1 {


error: expected expression, found `let` statement
   |
   |
LL |     if (let Some(a) = opt && (let Some(b) = a)) && b == 1 {


error: expected expression, found `let` statement
   |
   |
LL |     if (let Some(a) = opt && (let Some(b) = a)) && true {


error: expected expression, found `let` statement
   |
   |
LL |     if (let Some(a) = opt && (let Some(b) = a)) && true {


error: expected expression, found `let` statement
   |
   |
LL |     if (let Some(a) = opt && (true)) && true {


error: expected expression, found `let` statement
   |
   |
LL |     let x = (true && let y = 1);


error: expected expression, found `let` statement
   |
   |
LL |         ([1, 2, 3][let _ = ()])


error: expected expression, found `let` statement
   |
   |
LL |     #[cfg(FALSE)] (let 0 = 1);


error: expected expression, found `let` statement
   |
   |
LL |     use_expr!((let 0 = 1 && 0 == 0));


error: expected expression, found `let` statement
   |
   |
LL |     use_expr!((let 0 = 1));


error: expected expression, found `let` statement
   |
   |
LL |     use_expr!(true && let 0 = 1);


error: expected expression, found `let` statement
   |
   |
LL |     noop_expr!((let 0 = 1));

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:27:9
   |
   |
LL |     if (let 0 = 1) {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     if (let 0 = 1) {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:31:11
   |
   |
LL |     if (((let 0 = 1))) {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     if (((let 0 = 1))) {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:35:9
   |
   |
LL |     if (let 0 = 1) && true {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     if (let 0 = 1) && true {}

---
diff of stderr:

29    |                     -- expected `()` because of return type
30 ...
31 LL |     generic::<()>()
-    |     ^^^^^^^^^^^^^^^- help: consider using a semicolon here: `;`
-    |     expected `()`, found associated type
+    |     ^^^^^^^^^^^^^^^ expected `()`, found associated type
35    |
36    = note:    expected unit type `()`
36    = note:    expected unit type `()`
37            found associated type `<() as Foo>::Assoc`

38    = help: consider constraining the associated type `<() as Foo>::Assoc` to `()`
+ help: consider using a semicolon at the end of the expression
+    |
+    |
+ LL |     generic::<()>();
+ help: consider using a semicolon here
+    |
+    |
+ LL |     generic::<()>();
40 
41 error: aborting due to 2 previous errors; 1 warning emitted
42 

---
To only update this specific test, also pass `--test-args specialization/specialization-default-projection.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/specialization-default-projection.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-default-projection" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-default-projection/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(specialization)] //~ WARN the feature `specialization` is incomplete
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete
   = help: consider using `min_specialization` instead, which is more stable and complete

error[E0308]: mismatched types
  --> /checkout/src/test/ui/specialization/specialization-default-projection.rs:21:5
   |
LL | fn generic<T>() -> <T as Foo>::Assoc {
   |                    ----------------- expected `<T as Foo>::Assoc` because of return type
...
LL |     () //~ ERROR mismatched types
   |
   |
   = note: expected associated type `<T as Foo>::Assoc`
                    found unit type `()`
   = help: consider constraining the associated type `<T as Foo>::Assoc` to `()` or calling a method that returns `<T as Foo>::Assoc`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/specialization/specialization-default-projection.rs:28:5
   |
   |
LL | fn monomorphic() -> () {
   |                     -- expected `()` because of return type
...
LL |     generic::<()>() //~ ERROR mismatched types
   |
   = note:    expected unit type `()`
   = note:    expected unit type `()`
           found associated type `<() as Foo>::Assoc`
   = help: consider constraining the associated type `<() as Foo>::Assoc` to `()`
help: consider using a semicolon at the end of the expression
   |
   |
LL |     generic::<()>(); //~ ERROR mismatched types
help: consider using a semicolon here
   |
   |
LL |     generic::<()>(); //~ ERROR mismatched types

error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0308`.
---
diff of fixed:

6 
7 fn main() {
8     match () {
-         () => func() //~ ERROR mismatched types
+         () => func(); //~ ERROR mismatched types
11 }
12 



The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-83892/issue-83892.fixed
To only update this specific test, also pass `--test-args suggestions/issue-83892.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-83892.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-83892" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-83892/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/suggestions/issue-83892.rs:9:15
   |
LL | fn main() {
LL | fn main() {
   |           - expected `()` because of default return type
LL |     match () {
LL |         () => func() //~ ERROR mismatched types
   |               ^^^^^^ expected `()`, found `u8`
help: consider using a semicolon at the end of the expression
   |
   |
LL |         () => func(); //~ ERROR mismatched types
help: consider using a semicolon here
   |
LL |     };
   |      +
---
To only update this specific test, also pass `--test-args suggestions/match-needing-semi.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/match-needing-semi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-needing-semi" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-needing-semi/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/suggestions/match-needing-semi.rs:7:13
   |
LL | /     match 3 {
LL | |         4 => 1,
LL | |         4 => 1,
LL | |         3 => {
LL | |             foo() //~ ERROR mismatched types
   | |             ^^^^^ expected `()`, found `i32`
LL | |         }
LL | |         _ => 2
LL | |     }
   | |_____- expected this to be `()`
help: consider using a semicolon at the end of the expression
   |
   |
LL |             foo(); //~ ERROR mismatched types
help: consider using a semicolon here
   |
   |
LL |             foo(); //~ ERROR mismatched types
help: consider using a semicolon here
   |
LL |     };
   |      +
   |      +

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/match-needing-semi.rs:11:5
   |
LL | /     match 3 { //~ ERROR mismatched types
LL | |         4 => 1,
LL | |         3 => 2,
LL | |         _ => 2
LL | |     }
   | |     ^- help: consider using a semicolon here
   |       expected `()`, found integer

error: aborting due to 2 previous errors

---
diff of stderr:

2   --> $DIR/try-operator-dont-suggest-semicolon.rs:6:9
3    |
4 LL |         b()
-    |         ^^^- help: consider using a semicolon here: `;`
-    |         expected `()`, found `i32`
+    |         ^^^ expected `()`, found `i32`
+    |
+ help: consider using a semicolon at the end of the expression
---
To only update this specific test, also pass `--test-args suggestions/try-operator-dont-suggest-semicolon.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/try-operator-dont-suggest-semicolon.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/try-operator-dont-suggest-semicolon" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/try-operator-dont-suggest-semicolon/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/suggestions/try-operator-dont-suggest-semicolon.rs:6:9
   |
LL |         b()
   |         ^^^ expected `()`, found `i32`
---

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/try-operator-dont-suggest-semicolon.rs:16:9
   |
LL | /     if true {
LL | |     //~^ NOTE: expected this to be `()`
LL | |         x?
   | |         ^^ expected `()`, found integer
LL | |         //~^ ERROR: mismatched types [E0308]
LL | |         //~| NOTE: expected `()`, found integer
LL | |         //~| HELP: consider using a semicolon here
LL | |     }
   | |_____- expected this to be `()`
help: consider using a semicolon at the end of the expression
   |
LL |         x?;
   |           +
---
25    = note: expected unit type `()`
26                 found closure `[closure@$DIR/issue-63279.rs:9:21: 9:23]`
+ help: consider using a semicolon at the end of the expression
+    |
+ LL |     || -> Closure { || (); }
27 help: use parentheses to call this closure
28    |
28    |
29 LL |     || -> Closure { (|| ())() }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-63279/issue-63279.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-63279.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-63279.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-63279" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zsave-analysis" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-63279/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: expected a `FnOnce<()>` closure, found `()`
   |
   |
LL | fn c() -> Closure {
   |           ^^^^^^^ expected an `FnOnce<()>` closure, found `()`
   |
   = help: the trait `FnOnce<()>` is not implemented for `()`
   = note: wrap the `()` in a closure with no arguments: `|| { /* code */ }`

error[E0277]: expected a `FnOnce<()>` closure, found `()`
   |
   |
LL |     || -> Closure { || () }
   |           ^^^^^^^ expected an `FnOnce<()>` closure, found `()`
   |
   = help: the trait `FnOnce<()>` is not implemented for `()`
   = note: wrap the `()` in a closure with no arguments: `|| { /* code */ }`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/type-alias-impl-trait/issue-63279.rs:9:21
   |
   |
LL |     || -> Closure { || () }
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found closure `[closure@/checkout/src/test/ui/type-alias-impl-trait/issue-63279.rs:9:21: 9:23]`
help: consider using a semicolon at the end of the expression
   |
LL |     || -> Closure { || (); }
help: use parentheses to call this closure
   |
   |
LL |     || -> Closure { (|| ())() }

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type-alias-impl-trait/issue-63279.rs:9:5
   |
   |
LL |     || -> Closure { || () }
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found closure `[closure@/checkout/src/test/ui/type-alias-impl-trait/issue-63279.rs:9:5: 9:18]`
help: use parentheses to call this closure
   |
LL |     (|| -> Closure { || () })()

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0277, E0308.
Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/type/issue-91268.rs stdout ----
diff of stderr:

56    |           - expected `()` because of default return type
58    |     ^^^^^^^ expected `()`, found `u8`
+    |
+ help: consider using a semicolon at the end of the expression
+    |
---
To only update this specific test, also pass `--test-args type/issue-91268.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/issue-91268.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/issue-91268" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/issue-91268/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/type/issue-91268.rs:9:12
   |
LL | fn main() {
   |           - unclosed delimiter
---
error[E0412]: cannot find type `ţ` in this scope
  --> /checkout/src/test/ui/type/issue-91268.rs:9:11
   |
LL |     0: u8(ţ
   |           ^ expecting a type here because of type ascription

error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
   |
LL |     0: u8(ţ
LL |     0: u8(ţ
   |        ^^^^ only `Fn` traits may use parentheses
help: use angle brackets instead
   |
LL |     0: u8<ţ>
   |          ~ +
---
   |        -- ^ type argument not allowed
   |        |
   |        not allowed on builtin type `u8`
   |
help: primitive type `u8` doesn't have generic parameters
LL -     0: u8(ţ
LL +     0: u8
   |


error[E0308]: mismatched types
  --> /checkout/src/test/ui/type/issue-91268.rs:9:5
   |
LL | fn main() {
   |           - expected `()` because of default return type
   |     ^^^^^^^ expected `()`, found `u8`
   |
help: consider using a semicolon at the end of the expression
   |
---

1 error[E0308]: mismatched types
2   --> $DIR/issue-57673-ice-on-deref-of-boxed-trait.rs:5:5
3    |
- LL | fn ice(x: Box<dyn Iterator<Item=()>>) {
-    |                                       - help: try adding a return type: `-> (dyn Iterator<Item = ()> + 'static)`
6 LL |     *x
7    |     ^^ expected `()`, found trait object `dyn Iterator`

9    = note: expected unit type `()`
9    = note: expected unit type `()`
10            found trait object `(dyn Iterator<Item = ()> + 'static)`
+ help: consider using a semicolon at the end of the expression
+ LL |     *x;
+    |       +
+ help: try adding a return type
+    |
+    |
+ LL | fn ice(x: Box<dyn Iterator<Item=()>>) -> (dyn Iterator<Item = ()> + 'static) {
11 
12 error: aborting due to previous error
13 

---
To only update this specific test, also pass `--test-args typeck/issue-57673-ice-on-deref-of-boxed-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-57673-ice-on-deref-of-boxed-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-57673-ice-on-deref-of-boxed-trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-57673-ice-on-deref-of-boxed-trait/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/typeck/issue-57673-ice-on-deref-of-boxed-trait.rs:5:5
   |
   |
LL |     *x //~ ERROR mismatched types [E0308]
   |     ^^ expected `()`, found trait object `dyn Iterator`
   = note: expected unit type `()`
   = note: expected unit type `()`
           found trait object `(dyn Iterator<Item = ()> + 'static)`
help: consider using a semicolon at the end of the expression
   |
LL |     *x; //~ ERROR mismatched types [E0308]
help: try adding a return type
   |
   |
LL | fn ice(x: Box<dyn Iterator<Item=()>>) -> (dyn Iterator<Item = ()> + 'static) {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
---
diff of stderr:

38   --> $DIR/issue-91334.rs:10:8
39    |
40 LL | fn f(){||yield(((){),
-    |       -^^^^^^^^^^^^^^^ expected `()`, found generator
-    |       |
-    |       help: a return type might be missing here: `-> _`
+    |        ^^^^^^^^^^^^^^^ expected `()`, found generator
45    = note: expected unit type `()`
46               found generator `[generator@$DIR/issue-91334.rs:10:8: 10:10]`

+ help: consider using a semicolon at the end of the expression
+ help: consider using a semicolon at the end of the expression
+    |
+ LL | fn f(){||yield(((){),;
+ help: a return type might be missing here
+    |
+    |
+ LL | fn f()-> _ {||yield(((){),
47 
48 error: aborting due to 5 previous errors
49 

---
To only update this specific test, also pass `--test-args typeck/issue-91334.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-91334.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-91334" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-91334/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/typeck/issue-91334.rs:10:23
   |
   |
LL | fn f(){||yield(((){),
   |       -       -       ^
   |       |       unclosed delimiter
   |       unclosed delimiter

error: this file contains an unclosed delimiter
error: this file contains an unclosed delimiter
  --> /checkout/src/test/ui/typeck/issue-91334.rs:10:23
   |
LL | fn f(){||yield(((){),
   |       -       -       ^
   |       |       unclosed delimiter
   |       unclosed delimiter


error: expected one of `)`, `,`, `.`, `?`, or an operator, found `{`
   |
   |
LL | fn f(){||yield(((){),
   |                   |
   |                   |
   |                   expected one of `)`, `,`, `.`, `?`, or an operator
   |                   help: missing `,`
error: mismatched closing delimiter: `)`
  --> /checkout/src/test/ui/typeck/issue-91334.rs:10:19
   |
   |
LL | fn f(){||yield(((){),
   |                -  ^^ mismatched closing delimiter
   |                |  unclosed delimiter
   |                closing delimiter possibly meant for this

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/typeck/issue-91334.rs:10:8
   |
LL | fn f(){||yield(((){),
   |        ^^^^^^^^^^^^^^^ expected `()`, found generator
   = note: expected unit type `()`
   = note: expected unit type `()`
              found generator `[generator@/checkout/src/test/ui/typeck/issue-91334.rs:10:8: 10:10]`
help: consider using a semicolon at the end of the expression
   |
LL | fn f(){||yield(((){),;
help: a return type might be missing here
   |
   |
LL | fn f()-> _ {||yield(((){),

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/typeck/issue-91267.rs stdout ----
diff of stderr:

20    |           - expected `()` because of default return type
21 LL |     0: u8<e<5>=e>
22    |     ^^^^^^^^^^^^^ expected `()`, found `u8`
+ help: consider using a semicolon at the end of the expression
+    |
+    |
+ LL |     0: u8<e<5>=e>;
23 
24 error: aborting due to 3 previous errors
25 

---
To only update this specific test, also pass `--test-args typeck/issue-91267.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-91267.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-91267" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-91267/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0412]: cannot find type `e` in this scope
   |
   |
LL |     0: u8<e<5>=e>
   |                |
   |                not found in this scope
   |                not found in this scope
   |                help: maybe you meant to write an assignment here: `let e`
error[E0229]: associated type bindings are not allowed here
  --> /checkout/src/test/ui/typeck/issue-91267.rs:2:11
   |
   |
LL |     0: u8<e<5>=e>
   |           ^^^^^^ associated type not allowed here
error[E0308]: mismatched types
  --> /checkout/src/test/ui/typeck/issue-91267.rs:2:5
   |
LL | fn main() {
LL | fn main() {
   |           - expected `()` because of default return type
LL |     0: u8<e<5>=e>
   |     ^^^^^^^^^^^^^ expected `()`, found `u8`
help: consider using a semicolon at the end of the expression
   |
   |
LL |     0: u8<e<5>=e>;

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0229, E0308, E0412.
---
diff of stderr:

5    |         ^^^^^^^^^^ expected `()`, found `i32`
6    |
7    = note: this error originates in the macro `lib::d` (in Nightly builds, run with -Z macro-backtrace for more info)
+ help: consider using a semicolon at the end of the expression
+   --> $DIR/auxiliary/issue-81943-lib.rs:6:48
+    |
+ LL |   ($e:expr) => { match $e { x => { $crate::g(x); } } }
8 
9 error[E0308]: mismatched types
10   --> $DIR/issue-81943.rs:8:28


15    |         |                  expected `()`, found `i32`
16    |         expected this to be `()`
17    |
+ help: consider using a semicolon at the end of the expression
+    |
+ LL |   f(|x| match x { tmp => { g(tmp); } });
18 help: consider using a semicolon here
19    |
19    |
20 LL |   f(|x| match x { tmp => { g(tmp); } });
37    |         ----- in this macro invocation
38    |
39    = note: this error originates in the macro `d` (in Nightly builds, run with -Z macro-backtrace for more info)
+ help: consider using a semicolon at the end of the expression
+ help: consider using a semicolon at the end of the expression
+    |
+ LL |     ($e:expr) => { match $e { x => { g(x); } } }
40 help: consider using a semicolon here
41    |
41    |
42 LL |     ($e:expr) => { match $e { x => { g(x); } } }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-81943/issue-81943.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args typeck/issue-81943.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-81943.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-81943" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-81943/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/typeck/issue-81943.rs:7:9
   |
   |
LL |   f(|x| lib::d!(x)); //~ERROR
   |         ^^^^^^^^^^ expected `()`, found `i32`
   |
   = note: this error originates in the macro `lib::d` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider using a semicolon at the end of the expression
  --> /checkout/src/test/ui/typeck/auxiliary/issue-81943-lib.rs:6:48
   |
LL |   ($e:expr) => { match $e { x => { $crate::g(x); } } }

error[E0308]: mismatched types
  --> /checkout/src/test/ui/typeck/issue-81943.rs:8:28
   |
   |
LL |   f(|x| match x { tmp => { g(tmp) } }); //~ERROR
   |         |                  |
   |         |                  expected `()`, found `i32`
   |         expected this to be `()`
   |
   |
help: consider using a semicolon at the end of the expression
   |
LL |   f(|x| match x { tmp => { g(tmp); } }); //~ERROR
help: consider using a semicolon here
   |
   |
LL |   f(|x| match x { tmp => { g(tmp); } }); //~ERROR
help: consider using a semicolon here
   |
   |
LL |   f(|x| match x { tmp => { g(tmp) } };); //~ERROR

error[E0308]: mismatched types
  --> /checkout/src/test/ui/typeck/issue-81943.rs:10:38
   |
   |
LL |     ($e:expr) => { match $e { x => { g(x) } } } //~ERROR
   |                    |                 |
   |                    |                 expected `()`, found `i32`
   |                    expected this to be `()`
LL |   }
LL |   }
LL |   f(|x| d!(x));
   |
   = note: this error originates in the macro `d` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider using a semicolon at the end of the expression
   |
   |
LL |     ($e:expr) => { match $e { x => { g(x); } } } //~ERROR
help: consider using a semicolon here
   |
   |
LL |     ($e:expr) => { match $e { x => { g(x); } } } //~ERROR
help: consider using a semicolon here
   |
   |
LL |     ($e:expr) => { match $e { x => { g(x) } }; } //~ERROR

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.
---
7    = note: expected unit type `()`
8                  found struct `Map<std::slice::Iter<'_, char>, [closure@$DIR/return_type_containing_closure.rs:3:26: 3:29]>`
+ help: consider using a semicolon at the end of the expression
+    |
+ LL |     vec!['a'].iter().map(|c| c);
9 help: consider using a semicolon here
10    |
10    |
11 LL |     vec!['a'].iter().map(|c| c);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/return_type_containing_closure/return_type_containing_closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args typeck/return_type_containing_closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/return_type_containing_closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/return_type_containing_closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/return_type_containing_closure/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/typeck/return_type_containing_closure.rs:3:5
   |
   |
LL |     vec!['a'].iter().map(|c| c)
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                 found struct `Map<std::slice::Iter<'_, char>, [closure@/checkout/src/test/ui/typeck/return_type_containing_closure.rs:3:26: 3:29]>`
help: consider using a semicolon at the end of the expression
   |
LL |     vec!['a'].iter().map(|c| c);
help: consider using a semicolon here
   |
   |
LL |     vec!['a'].iter().map(|c| c);
help: a return type might be missing here
   |
   |
LL | fn foo() -> _ { //~ HELP a return type might be missing here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
