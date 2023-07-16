plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:7ea263296c702710d7fac3c6d5bccdb2895b4e79)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
........................................................................................ 616/14447
.......................................F................................................ 704/14447
........................................................................................ 792/14447
........................................................................................ 880/14447
...........................F.i..F....................................................... 968/14447
........................................................................................ 1144/14447
........................................................................................ 1232/14447
........................................................................................ 1320/14447
........................................................................................ 1408/14447
---
To only update this specific test, also pass `--test-args async-await/dont-suggest-missing-await.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/dont-suggest-missing-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/dont-suggest-missing-await" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/dont-suggest-missing-await/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/async-await/dont-suggest-missing-await.rs:14:18
   |
LL |         take_u32(x)
   |         -------- ^ expected `u32`, found future
---
   |                   ^
note: function defined here
  --> fake-test-src-base/async-await/dont-suggest-missing-await.rs:5:4
   |
LL | fn take_u32(x: u32) {}
   |    ^^^^^^^^ ------
help: consider `await`ing on the `Future`
   |
LL |         take_u32(x.await)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
---
To only update this specific test, also pass `--test-args async-await/suggest-missing-await-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/suggest-missing-await-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await-closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await-closure/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/async-await/suggest-missing-await-closure.rs:16:18
   |
LL |         take_u32(x)
   |         -------- ^ expected `u32`, found future
---
   |                   ^
note: function defined here
  --> fake-test-src-base/async-await/suggest-missing-await-closure.rs:6:4
   |
LL | fn take_u32(_x: u32) {}
   |    ^^^^^^^^ -------
help: consider `await`ing on the `Future`
   |
LL |         take_u32(x.await)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
---
32    |
33 LL |     dummy()
-    |     ^^^^^^^
+    |            ^
35 help: consider `await`ing on the `Future`
36    |
37 LL |     dummy().await

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await/suggest-missing-await.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/suggest-missing-await.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/suggest-missing-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/async-await/suggest-missing-await.rs:12:14
   |
LL |     take_u32(x)
   |     -------- ^ expected `u32`, found future
---
   |               ^
note: function defined here
  --> fake-test-src-base/async-await/suggest-missing-await.rs:3:4
   |
LL | fn take_u32(_x: u32) {}
   |    ^^^^^^^^ -------
help: consider `await`ing on the `Future`
   |
LL |     take_u32(x.await)

error[E0308]: mismatched types
  --> fake-test-src-base/async-await/suggest-missing-await.rs:22:5
   |
---
  --> fake-test-src-base/async-await/suggest-missing-await.rs:22:12
   |
LL |     dummy()
   |            ^
help: consider `await`ing on the `Future`
   |
LL |     dummy().await
help: consider using a semicolon here
   |
LL |     dummy();
   |            +
   |            +

error[E0308]: `if` and `else` have incompatible types
  --> fake-test-src-base/async-await/suggest-missing-await.rs:35:9
LL |       let _x = if true {
   |  ______________-
LL | |         dummy()
   | |         ------- expected because of this
   | |         ------- expected because of this
LL | |         //~^ HELP consider `await`ing on the `Future`
LL | |     } else {
LL | |         dummy().await
   | |         ^^^^^^^^^^^^^ expected future, found `()`
LL | |         //~^ ERROR `if` and `else` have incompatible types [E0308]
LL | |     };
   | |_____- `if` and `else` have incompatible types
   = note: expected opaque type `impl Future<Output = ()>`
                found unit type `()`
                found unit type `()`
help: consider `await`ing on the `Future`
   |
LL |         dummy().await

error[E0308]: `match` arms have incompatible types
  --> fake-test-src-base/async-await/suggest-missing-await.rs:45:14
   |
   |
LL |       let _x = match 0usize {
   |  ______________-
LL | |         0 => dummy(), //~ HELP consider `await`ing on the `Future`
   | |              ------- this is found to be of type `impl Future<Output = ()>`
LL | |         1 => dummy(),
   | |              ------- this is found to be of type `impl Future<Output = ()>`
LL | |         2 => dummy().await,
   | |              ^^^^^^^^^^^^^ expected future, found `()`
LL | |         //~^ `match` arms have incompatible types [E0308]
LL | |     };
   |
   = note: expected opaque type `impl Future<Output = ()>`
                found unit type `()`
                found unit type `()`
help: consider `await`ing on the `Future`
   |
LL ~         0 => dummy().await, //~ HELP consider `await`ing on the `Future`
LL ~         1 => dummy().await,

error[E0308]: mismatched types
  --> fake-test-src-base/async-await/suggest-missing-await.rs:53:9
   |
   |
LL |     let _x = match dummy() { //~ HELP consider `await`ing on the `Future`
   |                    ------- this expression has type `impl Future<Output = ()>`
LL |         () => {} //~ ERROR mismatched types [E0308]
   |
   = note: expected opaque type `impl Future<Output = ()>`
                found unit type `()`
                found unit type `()`
help: consider `await`ing on the `Future`
   |
LL |     let _x = match dummy().await { //~ HELP consider `await`ing on the `Future`

error[E0308]: mismatched types
  --> fake-test-src-base/async-await/suggest-missing-await.rs:67:9
   |
---
   |         ^^^^^ expected future, found `Result<_, _>`
   |
   = note: expected opaque type `impl Future<Output = Result<(), ()>>`
                     found enum `Result<_, _>`
help: consider `await`ing on the `Future`
   |
LL |     match dummy_result().await {

error[E0308]: mismatched types
  --> fake-test-src-base/async-await/suggest-missing-await.rs:69:9
   |
---
   |         ^^^^^^ expected future, found `Result<_, _>`
   |
   = note: expected opaque type `impl Future<Output = Result<(), ()>>`
                     found enum `Result<_, _>`
help: consider `await`ing on the `Future`
   |
LL |     match dummy_result().await {

error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0308`.
---
9 note: calling an async function returns a future
-   --> $DIR/issue-102605.rs:13:20
+   --> $DIR/issue-102605.rs:13:25
11    |
12 LL |     convert_result(foo())
+    |                         ^
14 note: function defined here
15   --> $DIR/issue-102605.rs:7:4
16    |
---
To only update this specific test, also pass `--test-args impl-trait/issue-102605.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/impl-trait/issue-102605.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issue-102605" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issue-102605/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/impl-trait/issue-102605.rs:13:20
   |
   |
LL |     convert_result(foo())
   |     -------------- ^^^^^ expected `Result<(), _>`, found future
   |     arguments to this function are incorrect
   |
note: calling an async function returns a future
  --> fake-test-src-base/impl-trait/issue-102605.rs:13:25
  --> fake-test-src-base/impl-trait/issue-102605.rs:13:25
   |
LL |     convert_result(foo())
note: function defined here
  --> fake-test-src-base/impl-trait/issue-102605.rs:7:4
   |
   |
LL | fn convert_result<T, E>(r: Result<T, E>) -> Option<T> {
   |    ^^^^^^^^^^^^^^       ---------------
help: consider `await`ing on the `Future`
   |
LL |     convert_result(foo().await)
help: try wrapping the expression in `Err`
   |
   |
LL |     convert_result(Err(foo()))

error[E0277]: `main` has invalid return type `Option<()>`
  --> fake-test-src-base/impl-trait/issue-102605.rs:11:14
   |
   |
LL | fn main() -> Option<()> {
   |              ^^^^^^^^^^ `main` can only return types that implement `Termination`
   |
   = help: consider using `()`, or a `Result`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] tests/ui/suggestions/if-then-neeing-semi.rs stdout ----
diff of stderr:

16    | |_____- `if` and `else` have incompatible types
18 note: calling an async function returns a future
-   --> $DIR/if-then-neeing-semi.rs:28:9
+   --> $DIR/if-then-neeing-semi.rs:28:22
20    |
20    |
21 LL |         async_dummy()
-    |         ^^^^^^^^^^^^^
+    |                      ^
23 help: consider `await`ing on the `Future`
24    |
25 LL |         async_dummy().await

48    | |_____- `if` and `else` have incompatible types
50 note: calling an async function returns a future
-   --> $DIR/if-then-neeing-semi.rs:41:9
+   --> $DIR/if-then-neeing-semi.rs:41:23
52    |
52    |
53 LL |         async_dummy2()
-    |         ^^^^^^^^^^^^^^
+    |                       ^
55 help: consider `await`ing on the `Future`
56    |
57 LL |         async_dummy2().await

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/if-then-neeing-semi/if-then-neeing-semi.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/if-then-neeing-semi.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/suggestions/if-then-neeing-semi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/if-then-neeing-semi" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/if-then-neeing-semi/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0308]: `if` and `else` have incompatible types
  --> fake-test-src-base/suggestions/if-then-neeing-semi.rs:28:9
LL |       let _ = if true {
   |  _____________-
   |  _____________-
LL | |         //~^ NOTE `if` and `else` have incompatible types
LL | |         async_dummy(); //~ NOTE expected because of this
   | |         -------------- expected because of this
LL | |         //~^ HELP consider removing this semicolon
LL | |     } else {
LL | |         async_dummy() //~ ERROR `if` and `else` have incompatible types
...  |
...  |
LL | |         //~| HELP consider `await`ing on the `Future`
LL | |     };
   | |_____- `if` and `else` have incompatible types
note: calling an async function returns a future
  --> fake-test-src-base/suggestions/if-then-neeing-semi.rs:28:22
   |
   |
LL |         async_dummy() //~ ERROR `if` and `else` have incompatible types
   |                      ^
help: consider `await`ing on the `Future`
   |
LL |         async_dummy().await //~ ERROR `if` and `else` have incompatible types
help: consider removing this semicolon
   |
   |
LL -         async_dummy(); //~ NOTE expected because of this
LL +         async_dummy() //~ NOTE expected because of this


error[E0308]: `if` and `else` have incompatible types
  --> fake-test-src-base/suggestions/if-then-neeing-semi.rs:41:9
LL |       let _ = if true {
   |  _____________-
   |  _____________-
LL | |         //~^ NOTE `if` and `else` have incompatible types
LL | |         async_dummy(); //~ NOTE expected because of this
   | |         -------------- expected because of this
LL | |         //~^ HELP consider removing this semicolon
LL | |     } else {
LL | |         async_dummy2() //~ ERROR `if` and `else` have incompatible types
...  |
...  |
LL | |         //~| HELP consider `await`ing on the `Future`
LL | |     };
   | |_____- `if` and `else` have incompatible types
note: calling an async function returns a future
  --> fake-test-src-base/suggestions/if-then-neeing-semi.rs:41:23
   |
   |
LL |         async_dummy2() //~ ERROR `if` and `else` have incompatible types
   |                       ^
help: consider `await`ing on the `Future`
   |
LL |         async_dummy2().await //~ ERROR `if` and `else` have incompatible types
   |                       ++++++
help: consider removing this semicolon and boxing the expressions
   |
LL ~         Box::new(async_dummy()) //~ NOTE expected because of this
LL |         //~^ HELP consider removing this semicolon
LL |     } else {
LL ~         Box::new(async_dummy2()) //~ ERROR `if` and `else` have incompatible types


error[E0308]: `if` and `else` have incompatible types
  --> fake-test-src-base/suggestions/if-then-neeing-semi.rs:54:9
LL |       let _ = if true {
   |  _____________-
   |  _____________-
LL | |         //~^ NOTE `if` and `else` have incompatible types
LL | |         async_dummy() //~ NOTE expected because of this
   | |         ------------- expected because of this
LL | |         //~| HELP consider `await`ing on both `Future`s
LL | |     } else {
LL | |         async_dummy2() //~ ERROR `if` and `else` have incompatible types
   | |         ^^^^^^^^^^^^^^ expected future, found a different future
LL | |         //~^ NOTE expected future, found a different future
LL | |         //~| NOTE distinct uses of `impl Trait` result in different opaque types
LL | |     };
   | |_____- `if` and `else` have incompatible types
   = note: distinct uses of `impl Trait` result in different opaque types
   = note: distinct uses of `impl Trait` result in different opaque types
help: consider `await`ing on both `Future`s
   |
LL ~         async_dummy().await //~ NOTE expected because of this
LL |         //~| HELP consider `await`ing on both `Future`s
LL |     } else {
LL ~         async_dummy2().await //~ ERROR `if` and `else` have incompatible types


error[E0308]: `if` and `else` have incompatible types
  --> fake-test-src-base/suggestions/if-then-neeing-semi.rs:13:9
LL |       let _ = if true {
   |  _____________-
   |  _____________-
LL | |         //~^ NOTE `if` and `else` have incompatible types
LL | |         dummy(); //~ NOTE expected because of this
   | |         |      |
   | |         |      help: consider removing this semicolon
   | |         expected because of this
   | |         expected because of this
LL | |         //~^ HELP consider removing this semicolon
LL | |     } else {
LL | |         dummy() //~ ERROR `if` and `else` have incompatible types
   | |         ^^^^^^^ expected `()`, found `i32`
LL | |         //~^ NOTE expected `()`, found `i32`
LL | |     };
   | |_____- `if` and `else` have incompatible types
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
---
18 note: calling an async function returns a future
-   --> $DIR/match-prev-arm-needing-semi.rs:26:18
+   --> $DIR/match-prev-arm-needing-semi.rs:26:31
20    |
21 LL |         false => async_dummy(),
+    |                               ^
+    |                               ^
23 help: consider `await`ing on the `Future`
24    |
25 LL |         false => async_dummy().await,
48    | |_____- `match` arms have incompatible types
49    |
50 note: calling an async function returns a future
-   --> $DIR/match-prev-arm-needing-semi.rs:39:18
-   --> $DIR/match-prev-arm-needing-semi.rs:39:18
+   --> $DIR/match-prev-arm-needing-semi.rs:39:32
52    |
53 LL |         false => async_dummy2(),
+    |                                ^
+    |                                ^
55 help: consider `await`ing on the `Future`
56    |
57 LL |         false => async_dummy2().await,

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-prev-arm-needing-semi/match-prev-arm-needing-semi.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/match-prev-arm-needing-semi.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/suggestions/match-prev-arm-needing-semi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-prev-arm-needing-semi" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-prev-arm-needing-semi/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/suggestions/match-prev-arm-needing-semi.rs:26:18
   |
   |
LL |       let _ = match true { //~ NOTE `match` arms have incompatible types
LL | |         true => {
LL | |         true => {
LL | |             async_dummy(); //~ NOTE this is found to be
   | |             -------------- this is found to be of type `()`
LL | |             //~^ HELP consider removing this semicolon
LL | |         }
LL | |         false => async_dummy(), //~ ERROR `match` arms have incompatible types
...  |
...  |
LL | |         //~| HELP consider `await`ing on the `Future`
LL | |     };
   |
note: calling an async function returns a future
  --> fake-test-src-base/suggestions/match-prev-arm-needing-semi.rs:26:31
   |
   |
LL |         false => async_dummy(), //~ ERROR `match` arms have incompatible types
   |                               ^
help: consider `await`ing on the `Future`
   |
LL |         false => async_dummy().await, //~ ERROR `match` arms have incompatible types
help: consider removing this semicolon
   |
   |
LL -             async_dummy(); //~ NOTE this is found to be
LL +             async_dummy() //~ NOTE this is found to be

error[E0308]: `match` arms have incompatible types
  --> fake-test-src-base/suggestions/match-prev-arm-needing-semi.rs:39:18
   |
   |
LL |       let _ = match true { //~ NOTE `match` arms have incompatible types
LL | |         true => {
LL | |         true => {
LL | |             async_dummy(); //~ NOTE this is found to be
   | |             -------------- this is found to be of type `()`
LL | |             //~^ HELP consider removing this semicolon
LL | |         }
LL | |         false => async_dummy2(), //~ ERROR `match` arms have incompatible types
...  |
...  |
LL | |         //~| HELP consider `await`ing on the `Future`
LL | |     };
   |
note: calling an async function returns a future
  --> fake-test-src-base/suggestions/match-prev-arm-needing-semi.rs:39:32
   |
   |
LL |         false => async_dummy2(), //~ ERROR `match` arms have incompatible types
   |                                ^
help: consider `await`ing on the `Future`
   |
LL |         false => async_dummy2().await, //~ ERROR `match` arms have incompatible types
   |                                ++++++
help: consider removing this semicolon and boxing the expressions
   |
LL ~             Box::new(async_dummy()) //~ NOTE this is found to be
LL |             //~^ HELP consider removing this semicolon
LL |         }
LL ~         false => Box::new(async_dummy2()), //~ ERROR `match` arms have incompatible types

error[E0308]: `match` arms have incompatible types
  --> fake-test-src-base/suggestions/match-prev-arm-needing-semi.rs:50:18
   |
   |
LL |       let _ = match true { //~ NOTE `match` arms have incompatible types
   |  _____________-
LL | |         true => async_dummy(), //~ NOTE this is found to be
   | |                 ------------- this is found to be of type `impl Future<Output = ()>`
LL | |         //~| HELP consider `await`ing on both `Future`s
LL | |         false => async_dummy2(), //~ ERROR `match` arms have incompatible types
   | |                  ^^^^^^^^^^^^^^ expected future, found a different future
LL | |         //~^ NOTE expected future, found a different future
LL | |         //~| NOTE distinct uses of `impl Trait` result in different opaque types
LL | |     };
   |
   = note: distinct uses of `impl Trait` result in different opaque types
   = note: distinct uses of `impl Trait` result in different opaque types
help: consider `await`ing on both `Future`s
   |
LL ~         true => async_dummy().await, //~ NOTE this is found to be
LL |         //~| HELP consider `await`ing on both `Future`s
LL ~         false => async_dummy2().await, //~ ERROR `match` arms have incompatible types

error[E0308]: `match` arms have incompatible types
  --> fake-test-src-base/suggestions/match-prev-arm-needing-semi.rs:11:18
   |
   |
LL |       let _ = match true { //~ NOTE `match` arms have incompatible types
LL | |         true => {
LL | |         true => {
LL | |             dummy(); //~ NOTE this is found to be
   | |             |      |
   | |             |      help: consider removing this semicolon
   | |             this is found to be of type `()`
   | |             this is found to be of type `()`
LL | |             //~^ HELP consider removing this semicolon
LL | |         }
LL | |         false => dummy(), //~ ERROR `match` arms have incompatible types
   | |                  ^^^^^^^ expected `()`, found `i32`
LL | |         //~^ NOTE expected `()`, found `i32`
LL | |     };

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.
---
25    |
26 LL |     hello()
-    |     ^^^^^^^
+    |            ^
28 help: consider `await`ing on the `Future`
29    |
30 LL |     hello().await

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-90027-async-fn-return-suggestion/issue-90027-async-fn-return-suggestion.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args typeck/issue-90027-async-fn-return-suggestion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/typeck/issue-90027-async-fn-return-suggestion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-90027-async-fn-return-suggestion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-90027-async-fn-return-suggestion/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/typeck/issue-90027-async-fn-return-suggestion.rs:4:5
   |
   |
LL | async fn hello() { //~ HELP try adding a return type
   |                  - help: try adding a return type: `-> i32`
   |     ^ expected `()`, found integer

error[E0308]: mismatched types
  --> fake-test-src-base/typeck/issue-90027-async-fn-return-suggestion.rs:9:5
  --> fake-test-src-base/typeck/issue-90027-async-fn-return-suggestion.rs:9:5
   |
LL | async fn world() -> () {
   |                     -- expected `()` because of return type
   |     ^ expected `()`, found integer

error[E0308]: mismatched types
  --> fake-test-src-base/typeck/issue-90027-async-fn-return-suggestion.rs:14:5
---
  --> fake-test-src-base/typeck/issue-90027-async-fn-return-suggestion.rs:14:12
   |
LL |     hello()
   |            ^
help: consider `await`ing on the `Future`
   |
LL |     hello().await
help: consider using a semicolon here
   |
LL |     hello();
   |            +
