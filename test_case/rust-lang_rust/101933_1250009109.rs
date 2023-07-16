plain
........................................................................................ 5104/13539
........................................................................................ 5192/13539
...........................................i............................................ 5280/13539
........................i............................................................... 5368/13539
.F...............................................F...................................... 5456/13539
........................................................................................ 5632/13539
........................................................................................ 5720/13539
........................................................................................ 5808/13539
........................................................................................ 5896/13539
---
......................................i................................................. 9240/13539
........................................................................................ 9328/13539
..............................................i......................................... 9416/13539
........................................................................................ 9504/13539
................................................................F....................... 9592/13539
.........................................................F.................F............ 9680/13539
........................................................................................ 9856/13539
........................................................................................ 9944/13539
..............................................................ii...............i........ 10032/13539
........................................................................................ 10120/13539
---

---- [ui] src/test/ui/async-await/suggest-missing-await.rs stdout ----
diff of stderr:

136    |                            ^^^^^^^^^^^^^^ checked the `Output` of this `async fn`, expected opaque type
137    = note: expected opaque type `impl Future<Output = Result<(), ()>>`
138                      found enum `Result<_, _>`
+ help: Either change the return type or use `unwrap`
+    |
+ LL |         Ok(_).unwrap() => {}
+    |         ~~~~~~~~~~~~~~
139 help: consider `await`ing on the `Future`
140    |
141 LL |     match dummy_result().await {

157    |                            ^^^^^^^^^^^^^^ checked the `Output` of this `async fn`, expected opaque type
158    = note: expected opaque type `impl Future<Output = Result<(), ()>>`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
159                      found enum `Result<_, _>`
+ help: Either change the return type or use `unwrap`
+    |
+ LL |         Err(_).unwrap() => {}
+    |         ~~~~~~~~~~~~~~~
160 help: consider `await`ing on the `Future`
161    |
162 LL |     match dummy_result().await {

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
   |     -------- ^ expected `u32`, found opaque type
   |     -------- ^ expected `u32`, found opaque type
   |     |
   |     arguments to this function are incorrect
   |
note: while checking the return type of the `async fn`
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
            found opaque type `impl Future<Output = ()>`
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
help: Either change the return type or use `unwrap`
   |
   |
LL |         Ok(_).unwrap() => {}
   |         ~~~~~~~~~~~~~~
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
help: Either change the return type or use `unwrap`
   |
   |
LL |         Err(_).unwrap() => {}
   |         ~~~~~~~~~~~~~~~
help: consider `await`ing on the `Future`
   |
LL |     match dummy_result().await {

error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-11844.rs stdout ----
diff of stderr:

4 LL |     match a {
5    |           - this expression has type `Option<Box<{integer}>>`
6 LL |         Ok(a) =>
+    |         ^^^^^
+    |         |
+    |         expected enum `Option`, found enum `Result`
+    |         expected enum `Option`, found enum `Result`
+    |         help: Either change the return type or use `unwrap`: `Ok(a).unwrap()`
8    |
9    = note: expected enum `Option<Box<{integer}>>`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11844/issue-11844.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11844/issue-11844.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-11844.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-11844.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11844" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11844/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-11844.rs:4:9
   |
LL |     match a {
LL |     match a {
   |           - this expression has type `Option<Box<{integer}>>`
LL |         Ok(a) => //~ ERROR: mismatched types
   |         |
   |         expected enum `Option`, found enum `Result`
   |         expected enum `Option`, found enum `Result`
   |         help: Either change the return type or use `unwrap`: `Ok(a).unwrap()`
   |
   = note: expected enum `Option<Box<{integer}>>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-13466.rs stdout ----
diff of stderr:

4 LL |     let _x: usize = match Some(1) {
5    |                           ------- this expression has type `Option<{integer}>`
6 LL |         Ok(u) => u,
+    |         ^^^^^
+    |         |
+    |         expected enum `Option`, found enum `Result`
+    |         expected enum `Option`, found enum `Result`
+    |         help: Either change the return type or use `unwrap`: `Ok(u).unwrap()`
9    = note: expected enum `Option<{integer}>`
10               found enum `Result<_, _>`

16    |                           ------- this expression has type `Option<{integer}>`
16    |                           ------- this expression has type `Option<{integer}>`
17 ...
18 LL |         Err(e) => panic!(e)
+    |         ^^^^^^
+    |         |
+    |         expected enum `Option`, found enum `Result`
+    |         expected enum `Option`, found enum `Result`
+    |         help: Either change the return type or use `unwrap`: `Err(e).unwrap()`
21    = note: expected enum `Option<{integer}>`
22               found enum `Result<_, _>`



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-13466/issue-13466.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-13466.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-13466.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-13466" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-13466/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-13466.rs:8:9
   |
LL |     let _x: usize = match Some(1) {
   |                           ------- this expression has type `Option<{integer}>`
   |                           ------- this expression has type `Option<{integer}>`
LL |         Ok(u) => u,
   |         |
   |         expected enum `Option`, found enum `Result`
   |         expected enum `Option`, found enum `Result`
   |         help: Either change the return type or use `unwrap`: `Ok(u).unwrap()`
   = note: expected enum `Option<{integer}>`
              found enum `Result<_, _>`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-13466.rs:14:9
   |
LL |     let _x: usize = match Some(1) {
   |                           ------- this expression has type `Option<{integer}>`
...
LL |         Err(e) => panic!(e)
   |         |
   |         expected enum `Option`, found enum `Result`
   |         expected enum `Option`, found enum `Result`
   |         help: Either change the return type or use `unwrap`: `Err(e).unwrap()`
   = note: expected enum `Option<{integer}>`
              found enum `Result<_, _>`

error: aborting due to 2 previous errors
---

8    |
9    = note: expected enum `Option<_>`
10               found enum `Result<_, _>`
+ help: Either change the return type or use `unwrap`
+    |
+ LL |         Err(_).unwrap() => ()
11 help: try wrapping the pattern in `Some`
12    |
12    |
13 LL |         Some(Err(_)) => ()

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3680/issue-3680.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-3680.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3680.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3680" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3680/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-3680.rs:3:9
   |
LL |     match None {
   |           ---- this expression has type `Option<_>`
   |           ---- this expression has type `Option<_>`
LL |         Err(_) => ()
   |         ^^^^^^ expected enum `Option`, found enum `Result`
   |
   = note: expected enum `Option<_>`
              found enum `Result<_, _>`
help: Either change the return type or use `unwrap`
   |
LL |         Err(_).unwrap() => ()
help: try wrapping the pattern in `Some`
   |
   |
LL |         Some(Err(_)) => ()

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/mismatched_types/abridged.rs stdout ----
diff of stderr:

15 LL | fn a2() -> Foo {
16    |            --- expected `Foo` because of return type
17 LL |     Ok(Foo { bar: 1})
-    |     ^^^^^^^^^^^^^^^^^ expected struct `Foo`, found enum `Result`
+    |     |
+    |     expected struct `Foo`, found enum `Result`
+    |     expected struct `Foo`, found enum `Result`
+    |     help: Either change the return type or use `unwrap`: `Ok(Foo { bar: 1}).unwrap()`
20    = note: expected struct `Foo`
20    = note: expected struct `Foo`
21                 found enum `Result<Foo, _>`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/abridged/abridged.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mismatched_types/abridged.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/abridged.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/abridged" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/abridged/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/mismatched_types/abridged.rs:16:5
   |
   |
LL | fn a() -> Foo {
   |           --- expected `Foo` because of return type
LL |     Some(Foo { bar: 1 }) //~ ERROR mismatched types
   |     ^^^^^^^^^^^^^^^^^^^^ expected struct `Foo`, found enum `Option`
   = note: expected struct `Foo`
                found enum `Option<Foo>`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/abridged.rs:20:5
   |
LL | fn a2() -> Foo {
   |            --- expected `Foo` because of return type
LL |     Ok(Foo { bar: 1}) //~ ERROR mismatched types
   |     |
   |     expected struct `Foo`, found enum `Result`
   |     expected struct `Foo`, found enum `Result`
   |     help: Either change the return type or use `unwrap`: `Ok(Foo { bar: 1}).unwrap()`
   = note: expected struct `Foo`
   = note: expected struct `Foo`
                found enum `Result<Foo, _>`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/abridged.rs:24:5
   |
   |
LL | fn b() -> Option<Foo> {
   |           ----------- expected `Option<Foo>` because of return type
LL |     Foo { bar: 1 } //~ ERROR mismatched types
   |
   = note: expected enum `Option<Foo>`
            found struct `Foo`
help: try wrapping the expression in `Some`
help: try wrapping the expression in `Some`
   |
LL |     Some(Foo { bar: 1 }) //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/abridged.rs:28:5
   |
   |
LL | fn c() -> Result<Foo, Bar> {
   |           ---------------- expected `Result<Foo, Bar>` because of return type
LL |     Foo { bar: 1 } //~ ERROR mismatched types
   |
   |
   = note: expected enum `Result<Foo, Bar>`
help: try wrapping the expression in `Ok`
   |
   |
LL |     Ok(Foo { bar: 1 }) //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/abridged.rs:39:5
   |
   |
LL | fn d() -> X<X<String, String>, String> {
   |           ---------------------------- expected `X<X<String, String>, String>` because of return type
...
LL |     x //~ ERROR mismatched types
   |     ^ expected struct `String`, found integer
   |
   = note: expected struct `X<X<_, String>, String>`
              found struct `X<X<_, {integer}>, {integer}>`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/abridged.rs:50:5
   |
   |
LL | fn e() -> X<X<String, String>, String> {
   |           ---------------------------- expected `X<X<String, String>, String>` because of return type
...
LL |     x //~ ERROR mismatched types
   |     ^ expected struct `String`, found integer
   |
   = note: expected struct `X<X<_, String>, _>`
              found struct `X<X<_, {integer}>, _>`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/abridged.rs:54:5
   |
   |
LL | fn f() -> String {
   |           ------ expected `String` because of return type
LL |     1+2 //~ ERROR mismatched types
   |     ^^^ expected struct `String`, found integer
help: try using a conversion method
   |
   |
LL |     (1+2).to_string() //~ ERROR mismatched types
   |     +   +++++++++++++
error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/abridged.rs:59:5
   |
   |
LL | fn g() -> String {
   |           ------ expected `String` because of return type
LL |     -2 //~ ERROR mismatched types
   |     ^^ expected struct `String`, found integer
help: try using a conversion method
   |
   |
LL |     (-2).to_string() //~ ERROR mismatched types
   |     +  +++++++++++++
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/parser/unclosed-delimiter-in-dep.rs stdout ----
diff of stderr:

13   --> $DIR/unclosed-delimiter-in-dep.rs:4:20
14    |
15 LL |     let _: usize = unclosed_delim_mod::new();
-    |            |
+    |            -----   ^^^^^^^^^^^^^^^^^^^^^^^^^
+    |            |       |
+    |            |       expected `usize`, found enum `Result`
+    |            |       expected `usize`, found enum `Result`
+    |            |       help: Either change the return type or use `unwrap`: `unclosed_delim_mod::new().unwrap()`
19    |
20    = note: expected type `usize`



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unclosed-delimiter-in-dep/unclosed-delimiter-in-dep.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/unclosed-delimiter-in-dep.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/unclosed-delimiter-in-dep.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unclosed-delimiter-in-dep" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unclosed-delimiter-in-dep/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/parser/unclosed_delim_mod.rs:5:7
   |
   |
LL | pub fn new() -> Result<Value, ()> {
LL |     Ok(Value {
   |       ^ unclosed delimiter
LL |     }
LL | }
LL | }
   | ^ mismatched closing delimiter

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/unclosed-delimiter-in-dep.rs:4:20
   |
LL |     let _: usize = unclosed_delim_mod::new();
   |            |       |
   |            |       expected `usize`, found enum `Result`
   |            |       expected `usize`, found enum `Result`
   |            |       help: Either change the return type or use `unwrap`: `unclosed_delim_mod::new().unwrap()`
   |
   = note: expected type `usize`
              found enum `Result<Value, ()>`

---

---- [ui] src/test/ui/pattern/pat-struct-field-expr-has-type.rs stdout ----
diff of stderr:

4 LL |     match (S { f: 42 }) {
5    |           ------------- this expression has type `S`
6 LL |         S { f: Ok(_) } => {}
-    |                ^^^^^ expected `u8`, found enum `Result`
+    |                |
+    |                expected `u8`, found enum `Result`
+    |                expected `u8`, found enum `Result`
+    |                help: Either change the return type or use `unwrap`: `Ok(_).unwrap()`
9    = note: expected type `u8`
10               found enum `Result<_, _>`



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-struct-field-expr-has-type/pat-struct-field-expr-has-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/pat-struct-field-expr-has-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/pat-struct-field-expr-has-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-struct-field-expr-has-type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-struct-field-expr-has-type/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/pattern/pat-struct-field-expr-has-type.rs:7:16
   |
   |
LL |     match (S { f: 42 }) {
   |           ------------- this expression has type `S`
LL |         S { f: Ok(_) } => {} //~ ERROR mismatched types
   |                |
   |                expected `u8`, found enum `Result`
   |                expected `u8`, found enum `Result`
   |                help: Either change the return type or use `unwrap`: `Ok(_).unwrap()`
   = note: expected type `u8`
              found enum `Result<_, _>`

error: aborting due to previous error
---

20    |         ^^^^^  ---------- expected due to this
21    |         |
22    |         expected enum `Option`, found enum `Result`
+    |         help: Either change the return type or use `unwrap`: `Ok(0).unwrap()`
24    = note: expected enum `Option<u8>`
25               found enum `Result<_, _>`

31    |         ^^^^^  ---------- expected due to this
31    |         ^^^^^  ---------- expected due to this
32    |         |
33    |         expected enum `Option`, found enum `Result`
+    |         help: Either change the return type or use `unwrap`: `Ok(0).unwrap()`
35    = note: expected enum `Option<u8>`
36               found enum `Result<_, _>`

42    |         ^^^^^   ---- this expression has type `u8`
42    |         ^^^^^   ---- this expression has type `u8`
43    |         |
44    |         expected `u8`, found enum `Result`
+    |         help: Either change the return type or use `unwrap`: `Ok(0).unwrap()`
46    = note: expected type `u8`
47               found enum `Result<_, _>`



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-type-err-let-stmt/pat-type-err-let-stmt.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/pat-type-err-let-stmt.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/pat-type-err-let-stmt.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-type-err-let-stmt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-type-err-let-stmt/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/pattern/pat-type-err-let-stmt.rs:6:29
   |
   |
LL |     let Ok(0): Option<u8> = 42u8;
   |                ----------   ^^^^ expected enum `Option`, found `u8`
   |                expected due to this
   |
   = note: expected enum `Option<u8>`
              found type `u8`
              found type `u8`
help: try wrapping the expression in `Some`
   |
LL |     let Ok(0): Option<u8> = Some(42u8);

error[E0308]: mismatched types
  --> /checkout/src/test/ui/pattern/pat-type-err-let-stmt.rs:6:9
   |
   |
LL |     let Ok(0): Option<u8> = 42u8;
   |         ^^^^^  ---------- expected due to this
   |         expected enum `Option`, found enum `Result`
   |         expected enum `Option`, found enum `Result`
   |         help: Either change the return type or use `unwrap`: `Ok(0).unwrap()`
   = note: expected enum `Option<u8>`
              found enum `Result<_, _>`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/pattern/pat-type-err-let-stmt.rs:11:9
   |
LL |     let Ok(0): Option<u8>;
   |         ^^^^^  ---------- expected due to this
   |         expected enum `Option`, found enum `Result`
   |         expected enum `Option`, found enum `Result`
   |         help: Either change the return type or use `unwrap`: `Ok(0).unwrap()`
   = note: expected enum `Option<u8>`
              found enum `Result<_, _>`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/pattern/pat-type-err-let-stmt.rs:15:9
   |
LL |     let Ok(0) = 42u8; //~ ERROR mismatched types
   |         |
   |         expected `u8`, found enum `Result`
   |         expected `u8`, found enum `Result`
   |         help: Either change the return type or use `unwrap`: `Ok(0).unwrap()`
   = note: expected type `u8`
              found enum `Result<_, _>`

error: aborting due to 4 previous errors
---

5    |                      ^^^^^^   - this expression has type `Option<bool>`
6    |                      |
7    |                      expected enum `Option`, found enum `Result`
+    |                      help: Either change the return type or use `unwrap`: `Err(_).unwrap()`
9    = note: expected enum `Option<bool>`
10               found enum `Result<_, _>`



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2294-if-let-guard/typeck/typeck.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2294-if-let-guard/typeck.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2294-if-let-guard/typeck.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2294-if-let-guard/typeck" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2294-if-let-guard/typeck/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/rfc-2294-if-let-guard/typeck.rs:9:22
   |
   |
LL |         Ok(x) if let Err(_) = x => {},
   |                      |
   |                      expected enum `Option`, found enum `Result`
   |                      expected enum `Option`, found enum `Result`
   |                      help: Either change the return type or use `unwrap`: `Err(_).unwrap()`
   = note: expected enum `Option<bool>`
              found enum `Result<_, _>`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/rfc-2294-if-let-guard/typeck.rs:11:22
   |
LL |         Ok(x) if let 0 = x => {},
   |                      ^   - this expression has type `Option<bool>`
   |                      expected enum `Option`, found integer
   |
   = note: expected enum `Option<bool>`
              found type `{integer}`
