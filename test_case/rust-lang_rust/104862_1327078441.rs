plain
........................................................................................ 1496/13907
..........i............................................................................. 1584/13907
........................................................................................ 1672/13907
........................................................................................ 1760/13907
...................................F...........F.............i............ii............ 1848/13907
........................................................................................ 2024/13907
..............................................i......................................... 2112/13907
........................................................................................ 2200/13907
........................................................................................ 2288/13907
---
---- [ui] src/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-registers.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-registers.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-registers" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-registers/auxiliary" "--target" "thumbv8m.main-none-eabi" "--crate-type" "lib"
stdout: none
--- stderr -------------------------------
error: requires `panic_bounds_check` lang_item
   |
LL |     non_secure_function(a, b, c, d)
   |     ^^^^^^^^^^^^^^^^^^^


error: aborting due to previous error
------------------------------------------


---- [ui] src/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-stack.rs stdout ----
diff of stderr:

- error: <unknown>:0:0: in function test i32 (i32, i32, i32, i32, i32): call to non-secure function would require passing arguments on stack
+ error: requires `panic_bounds_check` lang_item
+    |
+    |
+ LL |     non_secure_function(a, b, c, d, e)
2 
3 error: aborting due to previous error
4 

---
To only update this specific test, also pass `--test-args cmse-nonsecure/cmse-nonsecure-call/params-on-stack.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-stack.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-stack" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-stack/auxiliary" "--target" "thumbv8m.main-none-eabi" "--crate-type" "lib"
stdout: none
--- stderr -------------------------------
error: requires `panic_bounds_check` lang_item
   |
LL |     non_secure_function(a, b, c, d, e)
   |     ^^^^^^^^^^^^^^^^^^^

---
---- [ui] src/test/ui/mir/remove-zsts-query-cycle.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/remove-zsts-query-cycle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/remove-zsts-query-cycle" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/remove-zsts-query-cycle/auxiliary" "--edition=2018" "--crate-type=lib" "-Zinline-mir=yes"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when optimizing MIR for `listen::{closure#0}`
   |
   |
LL |   pub async fn listen() -> Result<(), std::io::Error> {
   |  _____________________________________________________^
LL | |     let f = do_async();
LL | |     std::mem::forget(f);
LL | |     Ok(())
LL | | }
   |
   |
   = note: ...which requires computing layout of `impl core::future::future::Future<Output = ()>`...
   = note: ...which requires computing layout of `impl Future<Output = ()>`...
note: ...which requires optimizing MIR for `do_async::{closure#0}`...
   |
   |
LL |   pub async fn do_async() {
   |  _________________________^
LL | |     listen().await.unwrap()
LL | | }
   | |_^
   = note: ...which requires computing layout of `impl core::future::future::Future<Output = core::result::Result<(), std::io::error::Error>>`...
   = note: ...which requires computing layout of `impl Future<Output = core::result::Result<(), std::io::error::Error>>`...
   = note: ...which again requires optimizing MIR for `listen::{closure#0}`, completing the cycle
error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/polymorphization/predicates.rs stdout ----
diff of stderr:

11    |    ^^^    - generic parameter `T` is unused
12 
13 error: item has unused generic parameters
-   --> $DIR/predicates.rs:45:19
-    |
- LL | impl<'a, I, T: 'a, E> Iterator for Foo<'a, I, E>
-    |          -         - generic parameter `E` is unused
-    |          |
-    |          generic parameter `I` is unused
- ...
- LL |         self.find(|_| true)
- 
- 
- error: item has unused generic parameters
26    |
26    |
27 LL | fn quux<A, B, C: Default>() -> usize
34    |
34    |
35 LL | fn foobar<F, G>() -> usize
36    |    ^^^^^^ - generic parameter `F` is unused
+ 
+ error: item has unused generic parameters
+    |
+    |
+ LL | impl<'a, I, T: 'a, E> Iterator for Foo<'a, I, E>
+    |          -         - generic parameter `E` is unused
+    |          |
+    |          generic parameter `I` is unused
+ ...
+ LL |         self.find(|_| true)
37 
37 
38 error: item has unused generic parameters


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/polymorphization/predicates/predicates.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/polymorphization/predicates/predicates.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args polymorphization/predicates.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/polymorphization/predicates.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/polymorphization/predicates" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/polymorphization/predicates/auxiliary" "-Copt-level=0" "-Zpolymorphize=on"
stdout: none
--- stderr -------------------------------
error: item has unused generic parameters
   |
   |
LL | fn foo<I, T>(_: I)
   |    ^^^    - generic parameter `T` is unused

error: item has unused generic parameters
   |
   |
LL | fn baz<I, T>(_: I)
   |    ^^^    - generic parameter `T` is unused

error: item has unused generic parameters
   |
   |
LL | fn quux<A, B, C: Default>() -> usize
   |    ^^^^ -  - generic parameter `B` is unused
   |         |
   |         generic parameter `A` is unused

error: item has unused generic parameters
   |
   |
LL | fn foobar<F, G>() -> usize
   |    ^^^^^^ - generic parameter `F` is unused

error: item has unused generic parameters
   |
   |
LL | impl<'a, I, T: 'a, E> Iterator for Foo<'a, I, E>
   |          -         - generic parameter `E` is unused
   |          |
   |          generic parameter `I` is unused
...
LL |         self.find(|_| true)


error: item has unused generic parameters
   |
   |
LL | fn bar<I>() {
   |    ^^^ - generic parameter `I` is unused

note: the above error was encountered while instantiating `fn foo::<std::slice::Iter<'_, u32>, T>`
   |
LL |     foo(x.iter());
   |     ^^^^^^^^^^^^^

