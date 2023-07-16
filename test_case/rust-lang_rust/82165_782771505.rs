plain
.................................................................................................... 100/11471
.......................................i........ii.................................................. 200/11471
.................................................................................................... 300/11471
.................................................................................................... 400/11471
...........................................................................................F......F. 500/11471
F................................................................................................... 600/11471
..............i......................FF.................................................ii.......... 700/11471
.................................................................................................... 900/11471
.................................................................................................... 1000/11471
.................................................................................................... 1100/11471
......i............................................................................................. 1200/11471
---
.................................................................................................... 9200/11471
..................F................................................................................. 9300/11471
.................................................................................................... 9400/11471
.............................i......i............................................................... 9500/11471
....................................................................iiiiiii..iiiiii.i............... 9600/11471
.................................................................................................... 9800/11471
.................................................................................................... 9900/11471
.................................................................................................... 10000/11471
.................................................................................................... 10100/11471
---

---- [ui] ui/async-await/dont-suggest-missing-await.rs stdout ----
diff of stderr:

7 LL |         take_u32(x)
8    |                  ^ expected `u32`, found opaque type
9    |
-    = note: while checking the return type of this `async fn`
+    = note: while checking the return type of the `async fn`
11    = note:     expected type `u32`
12            found opaque type `impl Future`
13 help: consider `await`ing on the `Future`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/dont-suggest-missing-await/dont-suggest-missing-await.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/dont-suggest-missing-await/dont-suggest-missing-await.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/dont-suggest-missing-await.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/dont-suggest-missing-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/dont-suggest-missing-await" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/dont-suggest-missing-await/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/dont-suggest-missing-await.rs:14:18
   |
LL | async fn make_u32() -> u32 {
   |                        --- checked the `Output` of this `async fn`, found opaque type
LL |         take_u32(x)
LL |         take_u32(x)
   |                  ^ expected `u32`, found opaque type
   |
   = note: while checking the return type of the `async fn`
   = note:     expected type `u32`
           found opaque type `impl Future`
help: consider `await`ing on the `Future`
   |
LL |         take_u32(x.await)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
---
diff of stderr:

13   --> $DIR/generator-desc.rs:12:16
14    |
15 LL | async fn one() {}
-    |                - the `Output` of this `async fn`'s expected opaque type
+    |                - checked the `Output` of this `async fn`, expected opaque type
17 LL | async fn two() {}
-    |                - the `Output` of this `async fn`'s found opaque type
+    |                - checked the `Output` of this `async fn`, found opaque type
19 ...
20 LL |     fun(one(), two());
21    |                ^^^^^ expected opaque type, found a different opaque type
22    |
22    |
+    = note: while checking the return type of the `async fn`
+    = note: while checking the return type of the `async fn`
23    = note: expected opaque type `impl Future` (opaque type at <$DIR/generator-desc.rs:5:16>)
24               found opaque type `impl Future` (opaque type at <$DIR/generator-desc.rs:6:16>)
25    = help: consider `await`ing on both `Future`s

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generator-desc/generator-desc.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generator-desc/generator-desc.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/generator-desc.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/generator-desc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generator-desc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generator-desc/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/generator-desc.rs:10:25
   |
LL |     fun(async {}, async {});
   |               --        ^^ expected `async` block, found a different `async` block
   |               |
   |               the expected `async` block
   |
   = note: expected `async` block `[static generator@/checkout/src/test/ui/async-await/generator-desc.rs:10:15: 10:17]`
              found `async` block `[static generator@/checkout/src/test/ui/async-await/generator-desc.rs:10:25: 10:27]`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/generator-desc.rs:12:16
   |
   |
LL | async fn one() {}
   |                - checked the `Output` of this `async fn`, expected opaque type
LL | async fn two() {}
   |                - checked the `Output` of this `async fn`, found opaque type
...
LL |     fun(one(), two());
   |                ^^^^^ expected opaque type, found a different opaque type
   |
   = note: while checking the return type of the `async fn`
   = note: while checking the return type of the `async fn`
   = note: expected opaque type `impl Future` (opaque type at </checkout/src/test/ui/async-await/generator-desc.rs:5:16>)
              found opaque type `impl Future` (opaque type at </checkout/src/test/ui/async-await/generator-desc.rs:6:16>)
   = help: consider `await`ing on both `Future`s
   = note: distinct uses of `impl Trait` result in different opaque types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/generator-desc.rs:14:26
   |
   |
LL |     fun((async || {})(), (async || {})());
   |                   --     ^^^^^^^^^^^^^^^ expected `async` closure body, found a different `async` closure body
   |                   |
   |                   the expected `async` closure body
  ::: /checkout/library/core/src/future/mod.rs:61:43
   |
   |
LL | pub const fn from_generator<T>(gen: T) -> impl Future<Output = T::Return>
   |                                           |
   |                                           the expected opaque type
   |                                           the found opaque type
   |
   |
   = note: expected opaque type `impl Future` (`async` closure body)
              found opaque type `impl Future` (`async` closure body)
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.


------------------------------------------


---- [ui] ui/async-await/issue-61076.rs stdout ----
diff of stderr:

66 LL |         Tuple(_) => {}
67    |         ^^^^^^^^ expected opaque type, found struct `Tuple`
68    |
-    = note: while checking the return type of this `async fn`
+    = note: while checking the return type of the `async fn`
70    = note: expected opaque type `impl Future`
71                    found struct `Tuple`
72 help: consider `await`ing on the `Future`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61076/issue-61076.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61076/issue-61076.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issue-61076.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-61076.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61076" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61076/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the `?` operator can only be applied to values that implement `Try`
   |
   |
LL |     foo()?; //~ ERROR the `?` operator can only be applied to values that implement `Try`
   |     ^^^^^^ the `?` operator cannot be applied to type `impl Future`
   |
   = help: the trait `Try` is not implemented for `impl Future`
   = note: required by `into_result`
help: consider `await`ing on the `Future`
   |
LL |     foo().await?; //~ ERROR the `?` operator can only be applied to values that implement `Try`


error[E0277]: the `?` operator can only be applied to values that implement `Try`
   |
   |
LL |     t?; //~ ERROR the `?` operator can only be applied to values that implement `Try`
   |     ^^ the `?` operator cannot be applied to type `T`
   |
   = help: the trait `Try` is not implemented for `T`
   = note: required by `into_result`
help: consider `await`ing on the `Future`
   |
LL |     t.await?; //~ ERROR the `?` operator can only be applied to values that implement `Try`


error[E0609]: no field `0` on type `impl Future`
   |
   |
LL |     let _: i32 = tuple().0; //~ ERROR no field `0`
   |                          ^ field not available in `impl Future`, but it is available in its `Output`
   |
help: consider `await`ing on the `Future` and access the field of its `Output`
   |
LL |     let _: i32 = tuple().await.0; //~ ERROR no field `0`


error[E0609]: no field `a` on type `impl Future`
   |
   |
LL |     let _: i32 = struct_().a; //~ ERROR no field `a`
   |                            ^ field not available in `impl Future`, but it is available in its `Output`
   |
help: consider `await`ing on the `Future` and access the field of its `Output`
   |
LL |     let _: i32 = struct_().await.a; //~ ERROR no field `a`


error[E0599]: no method named `method` found for opaque type `impl Future` in the current scope
   |
   |
LL |     struct_().method(); //~ ERROR no method named
   |               ^^^^^^ method not found in `impl Future`
   |
help: consider `await`ing on the `Future` and calling the method on its `Output`
   |
LL |     struct_().await.method(); //~ ERROR no method named

error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/issue-61076.rs:92:9
   |
   |
LL | async fn tuple() -> Tuple {
   |                     ----- checked the `Output` of this `async fn`, expected opaque type
...
LL |         Tuple(_) => {} //~ ERROR mismatched types
   |         ^^^^^^^^ expected opaque type, found struct `Tuple`
   |
   = note: while checking the return type of the `async fn`
   = note: expected opaque type `impl Future`
                   found struct `Tuple`
help: consider `await`ing on the `Future`
   |
LL |     match tuple().await { //~ HELP consider `await`ing on the `Future`

error: aborting due to 6 previous errors

Some errors have detailed explanations: E0277, E0308, E0599, E0609.
---

---- [ui] ui/async-await/suggest-missing-await-closure.rs stdout ----
diff of stderr:

7 LL |         take_u32(x)
8    |                  ^ expected `u32`, found opaque type
9    |
-    = note: while checking the return type of this `async fn`
+    = note: while checking the return type of the `async fn`
11    = note:     expected type `u32`
12            found opaque type `impl Future`
13 help: consider `await`ing on the `Future`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await-closure/suggest-missing-await-closure.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await-closure/suggest-missing-await-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/suggest-missing-await-closure.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/suggest-missing-await-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await-closure" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await-closure/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/suggest-missing-await-closure.rs:16:18
   |
LL | async fn make_u32() -> u32 {
   |                        --- checked the `Output` of this `async fn`, found opaque type
LL |         take_u32(x)
LL |         take_u32(x)
   |                  ^ expected `u32`, found opaque type
   |
   = note: while checking the return type of the `async fn`
   = note:     expected type `u32`
           found opaque type `impl Future`
help: consider `await`ing on the `Future`
   |
LL |         take_u32(x.await)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.

------------------------------------------


---- [ui] ui/async-await/suggest-missing-await.rs stdout ----
diff of stderr:

7 LL |     take_u32(x)
8    |              ^ expected `u32`, found opaque type
9    |
-    = note: while checking the return type of this `async fn`
+    = note: while checking the return type of the `async fn`
11    = note:     expected type `u32`
12            found opaque type `impl Future`
13 help: consider `await`ing on the `Future`
24 LL |     dummy()
24 LL |     dummy()
25    |     ^^^^^^^ expected `()`, found opaque type
26    |
-    = note: while checking the return type of this `async fn`
+    = note: while checking the return type of the `async fn`
28    = note: expected unit type `()`
29             found opaque type `impl Future`
30 help: consider `await`ing on the `Future`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await/suggest-missing-await.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await/suggest-missing-await.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/suggest-missing-await.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/suggest-missing-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/suggest-missing-await.rs:12:14
   |
LL | async fn make_u32() -> u32 {
   |                        --- checked the `Output` of this `async fn`, found opaque type
LL |     take_u32(x)
LL |     take_u32(x)
   |              ^ expected `u32`, found opaque type
   |
   = note: while checking the return type of the `async fn`
   = note:     expected type `u32`
           found opaque type `impl Future`
help: consider `await`ing on the `Future`
   |
LL |     take_u32(x.await)

error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/suggest-missing-await.rs:22:5
   |
   |
LL | async fn dummy() {}
   |                  - checked the `Output` of this `async fn`, found opaque type
LL |     dummy()
LL |     dummy()
   |     ^^^^^^^ expected `()`, found opaque type
   |
   = note: while checking the return type of the `async fn`
   = note: expected unit type `()`
            found opaque type `impl Future`
help: consider `await`ing on the `Future`
   |
LL |     dummy().await
help: try adding a semicolon
   |
LL |     dummy();
   |            ^
---

---- [ui] ui/parser/fn-header-semantic-fail.rs stdout ----
diff of stderr:

192    |                        checked the `Output` of this `async fn`, found opaque type
193    |                        expected `()`, found opaque type
194    |
-    = note: while checking the return type of this `async fn`
+    = note: while checking the return type of the `async fn`
196    = note: expected fn pointer `fn()`
197               found fn pointer `fn() -> impl Future`


208    |                                                checked the `Output` of this `async fn`, found opaque type
209    |                                                expected `()`, found opaque type
210    |
-    = note: while checking the return type of this `async fn`
+    = note: while checking the return type of the `async fn`
212    = note: expected fn pointer `unsafe extern "C" fn()`
213               found fn pointer `unsafe extern "C" fn() -> impl Future`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/fn-header-semantic-fail/fn-header-semantic-fail.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/fn-header-semantic-fail/fn-header-semantic-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/fn-header-semantic-fail.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/fn-header-semantic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/fn-header-semantic-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/fn-header-semantic-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: functions cannot be both `const` and `async`
   |
   |
LL |     const async unsafe extern "C" fn ff5() {} // OK.
   |     ^^^^^-^^^^^------------------------------
   |     |     |
   |     |     `async` because of this
   |     `const` because of this

error[E0706]: functions in traits cannot be declared `async`
   |
   |
LL |         async fn ft1(); //~ ERROR functions in traits cannot be declared `async`
   |         |
   |         |
   |         `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:19:9
   |
   |
LL |         const fn ft3(); //~ ERROR functions in traits cannot be declared const

error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:21:9
   |
   |
LL |         const async unsafe extern "C" fn ft5();


error[E0706]: functions in traits cannot be declared `async`
   |
   |
LL |         const async unsafe extern "C" fn ft5();
   |               |
   |               |
   |               `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait

error: functions cannot be both `const` and `async`
   |
   |
LL |         const async unsafe extern "C" fn ft5();
   |         ^^^^^-^^^^^----------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this

error[E0706]: functions in traits cannot be declared `async`
   |
   |
LL |         async fn ft1() {} //~ ERROR functions in traits cannot be declared `async`
   |         |
   |         |
   |         `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:32:9
   |
   |
LL |         const fn ft3() {} //~ ERROR functions in traits cannot be declared const

error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:34:9
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}


error[E0706]: functions in traits cannot be declared `async`
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}
   |               |
   |               |
   |               `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait

error: functions cannot be both `const` and `async`
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}
   |         ^^^^^-^^^^^------------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this

error: functions cannot be both `const` and `async`
   |
   |
LL |         const async unsafe extern "C" fn fi5() {}
   |         ^^^^^-^^^^^------------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this

error: functions in `extern` blocks cannot have qualifiers
   |
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
LL |         async fn fe1(); //~ ERROR functions in `extern` blocks cannot have qualifiers
   |
help: remove the qualifiers
   |
   |
LL |         fn fe1(); //~ ERROR functions in `extern` blocks cannot have qualifiers


error: functions in `extern` blocks cannot have qualifiers
   |
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
LL |         async fn fe1(); //~ ERROR functions in `extern` blocks cannot have qualifiers
LL |         unsafe fn fe2(); //~ ERROR functions in `extern` blocks cannot have qualifiers
   |
help: remove the qualifiers
   |
   |
LL |         fn fe2(); //~ ERROR functions in `extern` blocks cannot have qualifiers


error: functions in `extern` blocks cannot have qualifiers
   |
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
...
LL |         const fn fe3(); //~ ERROR functions in `extern` blocks cannot have qualifiers
   |
help: remove the qualifiers
   |
   |
LL |         fn fe3(); //~ ERROR functions in `extern` blocks cannot have qualifiers


error: functions in `extern` blocks cannot have qualifiers
   |
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
...
LL |         extern "C" fn fe4(); //~ ERROR functions in `extern` blocks cannot have qualifiers
   |
help: remove the qualifiers
   |
   |
LL |         fn fe4(); //~ ERROR functions in `extern` blocks cannot have qualifiers


error: functions in `extern` blocks cannot have qualifiers
   |
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
...
LL |         const async unsafe extern "C" fn fe5(); //~ ERROR functions in `extern` blocks
   |
help: remove the qualifiers
   |
   |
LL |         fn fe5(); //~ ERROR functions in `extern` blocks


error: functions cannot be both `const` and `async`
   |
   |
LL |         const async unsafe extern "C" fn fe5(); //~ ERROR functions in `extern` blocks
   |         ^^^^^-^^^^^----------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this
error[E0053]: method `ft1` has an incompatible type for trait
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:29:24
   |
   |
LL |         async fn ft1(); //~ ERROR functions in traits cannot be declared `async`
   |                       - type in trait
...
LL |         async fn ft1() {} //~ ERROR functions in traits cannot be declared `async`
   |                        |
   |                        |
   |                        checked the `Output` of this `async fn`, found opaque type
   |                        expected `()`, found opaque type
   |
   = note: while checking the return type of the `async fn`
   = note: expected fn pointer `fn()`
              found fn pointer `fn() -> impl Future`
error[E0053]: method `ft5` has an incompatible type for trait
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:34:48
   |
   |
LL |         const async unsafe extern "C" fn ft5();
   |                                               - type in trait
...
LL |         const async unsafe extern "C" fn ft5() {}
   |                                                |
   |                                                |
   |                                                checked the `Output` of this `async fn`, found opaque type
   |                                                expected `()`, found opaque type
   |
   = note: while checking the return type of the `async fn`
   = note: expected fn pointer `unsafe extern "C" fn()`
              found fn pointer `unsafe extern "C" fn() -> impl Future`
error: aborting due to 20 previous errors

Some errors have detailed explanations: E0053, E0379, E0706.
For more information about an error, try `rustc --explain E0053`.
For more information about an error, try `rustc --explain E0053`.

------------------------------------------


---- [ui] ui/resolve/issue-70736-async-fn-no-body-def-collector.rs stdout ----
diff of stderr:

56    |                          checked the `Output` of this `async fn`, found opaque type
57    |                          expected `()`, found opaque type
58    |
-    = note: while checking the return type of this `async fn`
+    = note: while checking the return type of the `async fn`
60    = note: expected fn pointer `fn()`
61               found fn pointer `fn() -> impl Future`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-70736-async-fn-no-body-def-collector/issue-70736-async-fn-no-body-def-collector.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-70736-async-fn-no-body-def-collector/issue-70736-async-fn-no-body-def-collector.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/issue-70736-async-fn-no-body-def-collector.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/issue-70736-async-fn-no-body-def-collector.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-70736-async-fn-no-body-def-collector" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-70736-async-fn-no-body-def-collector/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: free function without a body
   |
   |
LL | async fn free(); //~ ERROR without a body
   |                |
   |                |
   |                help: provide a definition for the function: `{ <body> }`

error: associated function in `impl` without body
   |
   |
LL |     async fn inherent(); //~ ERROR without body
   |                        |
   |                        |
   |                        help: provide a definition for the function: `{ <body> }`

error[E0706]: functions in traits cannot be declared `async`
   |
   |
LL |     async fn associated();
   |     |
   |     |
   |     `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait

error: associated function in `impl` without body
   |
   |
LL |     async fn associated(); //~ ERROR without body
   |                          |
   |                          |
   |                          help: provide a definition for the function: `{ <body> }`

error[E0706]: functions in traits cannot be declared `async`
   |
   |
LL |     async fn associated(); //~ ERROR without body
   |     |
   |     |
   |     `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait

error[E0053]: method `associated` has an incompatible type for trait
   |
   |
LL |     async fn associated();
   |                          - type in trait
...
LL |     async fn associated(); //~ ERROR without body
   |                          |
   |                          |
   |                          checked the `Output` of this `async fn`, found opaque type
   |                          expected `()`, found opaque type
   |
   = note: while checking the return type of the `async fn`
   = note: expected fn pointer `fn()`
              found fn pointer `fn() -> impl Future`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0053, E0706.
For more information about an error, try `rustc --explain E0053`.
For more information about an error, try `rustc --explain E0053`.

------------------------------------------


---- [ui] ui/suggestions/match-prev-arm-needing-semi.rs stdout ----
diff of stderr:

18 LL | |     };
19    | |_____- `match` arms have incompatible types
20    |
-    = note: while checking the return type of this `async fn`
+    = note: while checking the return type of the `async fn`
22    = note:     expected type `()`
23            found opaque type `impl Future`
24 help: consider `await`ing on the `Future`
53 LL | |     };
53 LL | |     };
54    | |_____- `match` arms have incompatible types
55    |
-    = note: while checking the return type of this `async fn`
+    = note: while checking the return type of the `async fn`
57    = note:     expected type `()`
58            found opaque type `impl Future`
59 help: consider `await`ing on the `Future`
86 LL | |     };
86 LL | |     };
87    | |_____- `match` arms have incompatible types
88    |
-    = note: while checking the return type of this `async fn`
+    = note: while checking the return type of the `async fn`
90    = note:     expected type `impl Future` (opaque type at <$DIR/match-prev-arm-needing-semi.rs:16:24>)
91            found opaque type `impl Future` (opaque type at <$DIR/match-prev-arm-needing-semi.rs:17:25>)
92    = note: distinct uses of `impl Trait` result in different opaque types

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-prev-arm-needing-semi/match-prev-arm-needing-semi.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-prev-arm-needing-semi/match-prev-arm-needing-semi.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/match-prev-arm-needing-semi.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/match-prev-arm-needing-semi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-prev-arm-needing-semi" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-prev-arm-needing-semi/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: `match` arms have incompatible types
   |
   |
LL |   async fn async_dummy() {} //~ NOTE checked the `Output` of this `async fn`, found opaque type
   |                          - checked the `Output` of this `async fn`, found opaque type
...
LL |       let _ = match true { //~ NOTE `match` arms have incompatible types
LL | |         true => {
LL | |         true => {
LL | |             async_dummy(); //~ NOTE this is found to be
   | |             -------------- this is found to be of type `()`
LL | |             //~^ HELP consider removing this semicolon
LL | |         }
LL | |         false => async_dummy(), //~ ERROR `match` arms have incompatible types
   | |                  ^^^^^^^^^^^^^ expected `()`, found opaque type
...  |
LL | |         //~| HELP consider `await`ing on the `Future`
LL | |     };
   | |_____- `match` arms have incompatible types
   |
   = note: while checking the return type of the `async fn`
   = note:     expected type `()`
           found opaque type `impl Future`
help: consider `await`ing on the `Future`
   |
LL |         false => async_dummy().await, //~ ERROR `match` arms have incompatible types
   |                               ^^^^^^
help: consider removing this semicolon and boxing the expressions
   |
LL |             Box::new(async_dummy()) //~ NOTE this is found to be
LL |             //~^ HELP consider removing this semicolon
LL |         }
LL |         false => Box::new(async_dummy()), //~ ERROR `match` arms have incompatible types


error[E0308]: `match` arms have incompatible types
   |
   |
LL |   async fn async_dummy2() {} //~ NOTE checked the `Output` of this `async fn`, found opaque type
   |                           - checked the `Output` of this `async fn`, found opaque type
...
LL |       let _ = match true { //~ NOTE `match` arms have incompatible types
LL | |         true => {
LL | |         true => {
LL | |             async_dummy(); //~ NOTE this is found to be
   | |             -------------- this is found to be of type `()`
LL | |             //~^ HELP consider removing this semicolon
LL | |         }
LL | |         false => async_dummy2(), //~ ERROR `match` arms have incompatible types
   | |                  ^^^^^^^^^^^^^^ expected `()`, found opaque type
...  |
LL | |         //~| HELP consider `await`ing on the `Future`
LL | |     };
   | |_____- `match` arms have incompatible types
   |
   = note: while checking the return type of the `async fn`
   = note:     expected type `()`
           found opaque type `impl Future`
help: consider `await`ing on the `Future`
   |
LL |         false => async_dummy2().await, //~ ERROR `match` arms have incompatible types
   |                                ^^^^^^
help: consider removing this semicolon and boxing the expressions
   |
LL |             Box::new(async_dummy()) //~ NOTE this is found to be
LL |             //~^ HELP consider removing this semicolon
LL |         }
LL |         false => Box::new(async_dummy2()), //~ ERROR `match` arms have incompatible types


error[E0308]: `match` arms have incompatible types
   |
   |
LL |   async fn async_dummy2() {} //~ NOTE checked the `Output` of this `async fn`, found opaque type
   |                           - checked the `Output` of this `async fn`, found opaque type
...
LL |       let _ = match true { //~ NOTE `match` arms have incompatible types
   |  _____________-
LL | |         true => async_dummy(), //~ NOTE this is found to be
   | |                 ------------- this is found to be of type `impl Future`
LL | |         //~| HELP consider `await`ing on both `Future`s
LL | |         false => async_dummy2(), //~ ERROR `match` arms have incompatible types
   | |                  ^^^^^^^^^^^^^^ expected opaque type, found a different opaque type
...  |
LL | |         //~| NOTE while checking the return type of this `async fn`
LL | |     };
   | |_____- `match` arms have incompatible types
   |
   = note: while checking the return type of the `async fn`
   = note:     expected type `impl Future` (opaque type at </checkout/src/test/ui/suggestions/match-prev-arm-needing-semi.rs:16:24>)
           found opaque type `impl Future` (opaque type at </checkout/src/test/ui/suggestions/match-prev-arm-needing-semi.rs:17:25>)
   = note: distinct uses of `impl Trait` result in different opaque types
help: consider `await`ing on both `Future`s
   |
LL |         true => async_dummy().await, //~ NOTE this is found to be
LL |         //~| HELP consider `await`ing on both `Future`s
LL |         false => async_dummy2().await, //~ ERROR `match` arms have incompatible types


---
test result: FAILED. 11370 passed; 8 failed; 93 ignored; 0 measured; 0 filtered out; finished in 133.92s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:15
