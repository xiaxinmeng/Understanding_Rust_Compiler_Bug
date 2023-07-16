plain

---- [ui] src/test/ui/impl-trait/issues/issue-99348-impl-compatibility.rs stdout ----
diff of stderr:

- error[E0271]: type mismatch resolving `<Concrete as Bar>::Other == Concrete`
+ error[E0308]: mismatched types
3    |
4 LL | type Tait = impl Sized;

-    |             ---------- the found opaque type
-    |             ---------- the found opaque type
+    |             ---------- the expected opaque type
6 ...
7 LL |     type Item = Concrete;
-    |                 ^^^^^^^^ type mismatch resolving `<Concrete as Bar>::Other == Concrete`
+    |                 ^^^^^^^^ types differ
- note: expected this to be `Concrete`
-   --> $DIR/issue-99348-impl-compatibility.rs:13:18
-    |
-    |
- LL |     type Other = Tait;
-    = note:   expected struct `Concrete`
-            found opaque type `Tait`
- note: required by a bound in `Foo::Item`
-   --> $DIR/issue-99348-impl-compatibility.rs:17:20
-   --> $DIR/issue-99348-impl-compatibility.rs:17:20
-    |
- LL |     type Item: Bar<Other = Self>;
-    |                    ^^^^^^^^^^^^ required by this bound in `Foo::Item`
+    = note: expected opaque type `Tait`
+                    found struct `Concrete`
23 error: aborting due to previous error
24 

- For more information about this error, try `rustc --explain E0271`.
---
To only update this specific test, also pass `--test-args impl-trait/issues/issue-99348-impl-compatibility.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/issue-99348-impl-compatibility.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-99348-impl-compatibility" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-99348-impl-compatibility/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/impl-trait/issues/issue-99348-impl-compatibility.rs:8:17
   |
LL | type Tait = impl Sized;
   |             ---------- the expected opaque type
---
221    |
- LL |         async fn ft1() {}
-    |                        ^
-    |                        |
-    |                        checked the `Output` of this `async fn`, found opaque type
-    |
-    = note: while checking the return type of the `async fn`
- note: type in trait
-   --> $DIR/fn-header-semantic-fail.rs:17:23
-   --> $DIR/fn-header-semantic-fail.rs:17:23
-    |
232 LL |         async fn ft1();
-    |                       ^
-    = note: expected fn pointer `fn()`
-               found fn pointer `fn() -> impl Future<Output = ()>`
+    |         --------------- definition of `ft1` from trait
+ ...
+ LL |         async fn ft1() {}
+    |         ^^^^^^^^^^^^^^ impl has extra requirement `(): Future`
- error[E0053]: method `ft5` has an incompatible type for trait
-   --> $DIR/fn-header-semantic-fail.rs:34:48
+ error[E0276]: impl has stricter requirements than trait
+   --> $DIR/fn-header-semantic-fail.rs:34:9
+   --> $DIR/fn-header-semantic-fail.rs:34:9
239    |
- LL |         const async unsafe extern "C" fn ft5() {}
-    |                                                |
-    |                                                |
-    |                                                checked the `Output` of this `async fn`, found opaque type
-    |
-    = note: while checking the return type of the `async fn`
- note: type in trait
-   --> $DIR/fn-header-semantic-fail.rs:21:47
-   --> $DIR/fn-header-semantic-fail.rs:21:47
-    |
250 LL |         const async unsafe extern "C" fn ft5();
-    = note: expected fn pointer `unsafe extern "C" fn()`
-    = note: expected fn pointer `unsafe extern "C" fn()`
-               found fn pointer `unsafe extern "C" fn() -> impl Future<Output = ()>`
+    |         --------------------------------------- definition of `ft5` from trait
+ ...
+ LL |         const async unsafe extern "C" fn ft5() {}
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl has extra requirement `(): Future`
254 
255 error[E0391]: cycle detected when computing type of `main::<impl at $DIR/fn-header-semantic-fail.rs:28:5: 28:17>::ft5::{opaque#0}`
256   --> $DIR/fn-header-semantic-fail.rs:34:48
326 
327 error: aborting due to 23 previous errors
328 
- Some errors have detailed explanations: E0053, E0379, E0391, E0706.
---
To only update this specific test, also pass `--test-args parser/fn-header-semantic-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/fn-header-semantic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/fn-header-semantic-fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/fn-header-semantic-fail/auxiliary"
stdout: none
--- stderr -------------------------------
error: functions cannot be both `const` and `async`
   |
   |
LL |     const async unsafe extern "C" fn ff5() {}
   |     ^^^^^-^^^^^------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |     |     `async` because of this
   |     `const` because of this
error[E0706]: functions in traits cannot be declared `async`
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:17:9
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
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:21:9
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
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:29:9
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
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:34:9
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

error[E0391]: cycle detected when computing type of `main::ff5::{opaque#0}`
   |
   |
LL |     const async unsafe extern "C" fn ff5() {}
   |
   |
note: ...which requires borrow-checking `main::ff5`...
   |
   |
LL |     const async unsafe extern "C" fn ff5() {}
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `main::ff5`...
   |
   |
LL |     const async unsafe extern "C" fn ff5() {}
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const checking `main::ff5`...
   |
   |
LL |     const async unsafe extern "C" fn ff5() {}
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires computing whether `impl core::future::future::Future<Output = ()>` is freeze...
   = note: ...which requires evaluating trait selection obligation `impl core::future::future::Future<Output = ()>: core::marker::Freeze`...
   = note: ...which again requires computing type of `main::ff5::{opaque#0}`, completing the cycle
note: cycle used when checking item types in top-level module
   |
LL | / #![feature(const_extern_fn)]
LL | |
LL | | fn main() {
LL | | fn main() {
LL | |     async fn ff1() {} // OK.
LL | |     }
LL | | }
   | |_^


error[E0276]: impl has stricter requirements than trait
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:29:9
   |
LL |         async fn ft1(); //~ ERROR functions in traits cannot be declared `async`
   |         --------------- definition of `ft1` from trait
...
LL |         async fn ft1() {} //~ ERROR functions in traits cannot be declared `async`
   |         ^^^^^^^^^^^^^^ impl has extra requirement `(): Future`
error[E0276]: impl has stricter requirements than trait
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:34:9
   |
   |
LL |         const async unsafe extern "C" fn ft5();
   |         --------------------------------------- definition of `ft5` from trait
...
LL |         const async unsafe extern "C" fn ft5() {}
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl has extra requirement `(): Future`

error[E0391]: cycle detected when computing type of `main::<impl at /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:28:5: 28:17>::ft5::{opaque#0}`
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}
   |
   |
note: ...which requires borrow-checking `main::<impl at /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:28:5: 28:17>::ft5`...
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `main::<impl at /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:28:5: 28:17>::ft5`...
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const checking `main::<impl at /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:28:5: 28:17>::ft5`...
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires computing whether `impl core::future::future::Future<Output = ()>` is freeze...
   = note: ...which requires evaluating trait selection obligation `impl core::future::future::Future<Output = ()>: core::marker::Freeze`...
   = note: ...which again requires computing type of `main::<impl at /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:28:5: 28:17>::ft5::{opaque#0}`, completing the cycle
note: cycle used when checking item types in top-level module
   |
LL | / #![feature(const_extern_fn)]
LL | |
LL | | fn main() {
LL | | fn main() {
LL | |     async fn ff1() {} // OK.
LL | |     }
LL | | }
   | |_^


error[E0391]: cycle detected when computing type of `main::<impl at /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:42:5: 42:11>::fi5::{opaque#0}`
   |
   |
LL |         const async unsafe extern "C" fn fi5() {}
   |
   |
note: ...which requires borrow-checking `main::<impl at /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:42:5: 42:11>::fi5`...
   |
   |
LL |         const async unsafe extern "C" fn fi5() {}
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `main::<impl at /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:42:5: 42:11>::fi5`...
   |
   |
LL |         const async unsafe extern "C" fn fi5() {}
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const checking `main::<impl at /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:42:5: 42:11>::fi5`...
   |
   |
LL |         const async unsafe extern "C" fn fi5() {}
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires computing whether `impl core::future::future::Future<Output = ()>` is freeze...
   = note: ...which requires evaluating trait selection obligation `impl core::future::future::Future<Output = ()>: core::marker::Freeze`...
   = note: ...which again requires computing type of `main::<impl at /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:42:5: 42:11>::fi5::{opaque#0}`, completing the cycle
note: cycle used when checking item types in top-level module
   |
LL | / #![feature(const_extern_fn)]
LL | |
LL | | fn main() {
LL | | fn main() {
LL | |     async fn ff1() {} // OK.
LL | |     }
LL | | }
   | |_^

---

---- [ui] src/test/ui/resolve/issue-70736-async-fn-no-body-def-collector.rs stdout ----
diff of stderr:

44    = note: `async` trait functions are not currently supported
45    = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
- error[E0053]: method `associated` has an incompatible type for trait
-   --> $DIR/issue-70736-async-fn-no-body-def-collector.rs:15:26
+ error[E0276]: impl has stricter requirements than trait
+   --> $DIR/issue-70736-async-fn-no-body-def-collector.rs:15:5
+   --> $DIR/issue-70736-async-fn-no-body-def-collector.rs:15:5
49    |
50 LL |     async fn associated();
-    |                          ^
-    |                          |
-    |                          checked the `Output` of this `async fn`, found opaque type
-    |
-    = note: while checking the return type of the `async fn`
- note: type in trait
-   --> $DIR/issue-70736-async-fn-no-body-def-collector.rs:11:26
-   --> $DIR/issue-70736-async-fn-no-body-def-collector.rs:11:26
-    |
+    |     ---------------------- definition of `associated` from trait
+ ...
60 LL |     async fn associated();
-    |                          ^
-    = note: expected fn pointer `fn()`
-               found fn pointer `fn() -> impl Future<Output = ()>`
+    |     ^^^^^^^^^^^^^^^^^^^^^^ impl has extra requirement `(): Future`
65 error: aborting due to 6 previous errors
66 

- Some errors have detailed explanations: E0053, E0706.
---
To only update this specific test, also pass `--test-args resolve/issue-70736-async-fn-no-body-def-collector.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/issue-70736-async-fn-no-body-def-collector.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-70736-async-fn-no-body-def-collector" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-70736-async-fn-no-body-def-collector/auxiliary"
stdout: none
--- stderr -------------------------------
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
  --> /checkout/src/test/ui/resolve/issue-70736-async-fn-no-body-def-collector.rs:11:5
   |
LL |     async fn associated();
LL |     async fn associated();
   |     -----^^^^^^^^^^^^^^^^^
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
  --> /checkout/src/test/ui/resolve/issue-70736-async-fn-no-body-def-collector.rs:15:5
   |
   |
LL |     async fn associated(); //~ ERROR without body
   |     |
   |     |
   |     `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
error[E0276]: impl has stricter requirements than trait
  --> /checkout/src/test/ui/resolve/issue-70736-async-fn-no-body-def-collector.rs:15:5
   |
LL |     async fn associated();
LL |     async fn associated();
   |     ---------------------- definition of `associated` from trait
...
LL |     async fn associated(); //~ ERROR without body
   |     ^^^^^^^^^^^^^^^^^^^^^^ impl has extra requirement `(): Future`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0276, E0706.
For more information about an error, try `rustc --explain E0276`.
For more information about an error, try `rustc --explain E0276`.
------------------------------------------


---- [ui] src/test/ui/type-alias-impl-trait/issue-57961.rs stdout ----
diff of stderr:

- error[E0271]: expected `std::vec::IntoIter<u32>` to be an iterator that yields `X`, but it yields `u32`
+ error[E0308]: mismatched types
3    |
3    |
4 LL | type X = impl Sized;
5    |          ---------- the expected opaque type
6 ...
7 LL |     type Bar = std::vec::IntoIter<u32>;
-    |                ^^^^^^^^^^^^^^^^^^^^^^^ expected opaque type, found `u32`
---

- note: required by a bound in `Foo::Bar`
-   --> $DIR/issue-57961.rs:6:24
-    |
- LL |     type Bar: Iterator<Item = X>;
-    |                        ^^^^^^^^ required by this bound in `Foo::Bar`
18 error: aborting due to previous error
19 

- For more information about this error, try `rustc --explain E0271`.
---
To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-57961.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-57961.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57961" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57961/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/type-alias-impl-trait/issue-57961.rs:10:16
   |
   |
LL | type X = impl Sized;
...
LL |     type Bar = std::vec::IntoIter<u32>;
   |                ^^^^^^^^^^^^^^^^^^^^^^^ types differ
   |
