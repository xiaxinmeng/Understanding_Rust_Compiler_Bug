plain

---- [ui] src/test/ui/consts/async-block.rs#without_feature stdout ----
diff of stderr:

- error[E0658]: `async` blocks are not allowed in constants
-   --> $DIR/async-block.rs:12:47
+ error: fatal error triggered by #[rustc_error]
3    |
3    |
- LL | const _: i32 = { core::mem::ManuallyDrop::new(async { 0 }); 4 };
-    |
-    = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
-    = help: add `#![feature(const_async_blocks)]` to the crate attributes to enable
+ LL | fn main() {}
+ LL | fn main() {}
+    | ^^^^^^^^^
9 
- error[E0658]: `async` blocks are not allowed in statics
-   --> $DIR/async-block.rs:15:51
-    |
- LL | static _FUT: &(dyn Future<Output = ()> + Sync) = &async {};
-    |
-    = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
-    = help: add `#![feature(const_async_blocks)]` to the crate attributes to enable
+ error: aborting due to previous error
---
22 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/async-block.without_feature/async-block.without_feature.stderr
To only update this specific test, also pass `--test-args consts/async-block.rs`


error in revision `without_feature`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/async-block.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "without_feature" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/async-block.without_feature" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/async-block.without_feature/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error: fatal error triggered by #[rustc_error]
   |
   |
LL | fn main() {} //[with_feature]~ fatal error triggered by #[rustc_error]

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/impl-trait/in-trait/default-body-with-rpit.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/in-trait/default-body-with-rpit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-trait/default-body-with-rpit" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-trait/default-body-with-rpit/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
error: concrete type differs from previous defining opaque type use
   |
LL |         ""
LL |         ""
   |         ^^ expected `impl Debug`, got `&'static str`
note: previous use here
  --> /checkout/src/test/ui/impl-trait/in-trait/default-body-with-rpit.rs:10:39
   |
   |
LL |       async fn baz(&self) -> impl Debug {
LL | |         ""
LL | |     }
   | |_____^


error[E0720]: cannot resolve opaque type
  --> /checkout/src/test/ui/impl-trait/in-trait/default-body-with-rpit.rs:10:28
   |
LL |     async fn baz(&self) -> impl Debug {
   |                            ^^^^^^^^^^ cannot resolve opaque type
   |
   = note: these returned values have a concrete "never" type
   = help: this error will resolve once the item's body returns a concrete type
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0720`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/impl-trait/issues/issue-78722.rs stdout ----
diff of stderr:

- error[E0658]: `async` blocks are not allowed in constants
-   --> $DIR/issue-78722.rs:13:20
-    |
- LL |         let f: F = async { 1 };
-    |
-    = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
-    = help: add `#![feature(const_async_blocks)]` to the crate attributes to enable
- 
- 
- error[E0493]: destructor of `F` cannot be evaluated at compile-time
-   --> $DIR/issue-78722.rs:13:13
-    |
- LL |         let f: F = async { 1 };
-    |             ^ the destructor for this type cannot be evaluated in constants
- LL |     }],
-    |     - value is dropped here
- 
- 
19 error[E0271]: expected `impl Future<Output = ()>` to be a future that resolves to `u8`, but it resolves to `()`
21    |

22 LL |         fn concrete_use() -> F {
23    |                              ^ expected `()`, found `u8`
---
To only update this specific test, also pass `--test-args impl-trait/issues/issue-78722.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/issue-78722.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-78722" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-78722/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0271]: expected `impl Future<Output = ()>` to be a future that resolves to `u8`, but it resolves to `()`
   |
LL |         fn concrete_use() -> F {
   |                              ^ expected `()`, found `u8`

---
---- [ui] src/test/ui/regions/issue-72051-member-region-hang.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/issue-72051-member-region-hang.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-72051-member-region-hang" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-72051-member-region-hang/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | pub async fn query<'a>(_: &(), _: &(), _: (&(dyn std::any::Any + 'a),) ) {}
   |                    |
   |                    |
   |                    hidden type `impl Future<Output = ()>` captures the lifetime `'a` as defined here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/self/self_lifetime-async.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/self_lifetime-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_lifetime-async" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_lifetime-async/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = &'a ()>` captures lifetime that does not appear in bounds
   |
   |
LL |     async fn bar<'a>(self: &Alias, arg: &'a ()) -> &() { arg }
   |
   |
   = note: hidden type `impl Future<Output = &'a ()>` captures lifetime '_#17r
error: aborting due to previous error

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
