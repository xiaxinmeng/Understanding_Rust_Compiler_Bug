plain
---- [ui] src/test/ui/infinite/infinite-autoderef.rs stdout ----
diff of stderr:

25    |
26    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`infinite_autoderef`)
- error[E0055]: reached the recursion limit while auto-dereferencing `Foo`
-   --> $DIR/infinite-autoderef.rs:24:5
-    |
-    |
- LL |     Foo.foo;
-    |     ^^^ deref recursion limit reached
-    |
-    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`infinite_autoderef`)
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
36 error[E0609]: no field `foo` on type `Foo`
37   --> $DIR/infinite-autoderef.rs:24:9
38    |


47    |
48    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`infinite_autoderef`)
- error[E0055]: reached the recursion limit while auto-dereferencing `Foo`
-   --> $DIR/infinite-autoderef.rs:25:5
-    |
- LL |     Foo.bar();
- LL |     Foo.bar();
-    |     ^^^ deref recursion limit reached
-    |
-    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`infinite_autoderef`)
- 
58 error[E0599]: no method named `bar` found for struct `Foo` in the current scope
60    |

64 LL |     Foo.bar();
65    |         ^^^ method not found in `Foo`
---
To only update this specific test, also pass `--test-args infinite/infinite-autoderef.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/infinite/infinite-autoderef.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-autoderef" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zdeduplicate-diagnostics=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-autoderef/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/infinite/infinite-autoderef.rs:19:13
   |
LL |         x = Box::new(x);
LL |         x = Box::new(x);
   |             ^^^^^^^^^^^ cyclic type of infinite size
help: consider unboxing the value
   |
   |
LL |         x = *Box::new(x);

error[E0055]: reached the recursion limit while auto-dereferencing `Foo`
  --> /checkout/src/test/ui/infinite/infinite-autoderef.rs:24:5
   |
   |
LL |     Foo.foo;
   |     ^^^^^^^ deref recursion limit reached
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`infinite_autoderef`)
error[E0055]: reached the recursion limit while auto-dereferencing `Foo`
  --> /checkout/src/test/ui/infinite/infinite-autoderef.rs:24:9
   |
LL |     Foo.foo;
LL |     Foo.foo;
   |         ^^^ deref recursion limit reached
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`infinite_autoderef`)
error[E0609]: no field `foo` on type `Foo`
  --> /checkout/src/test/ui/infinite/infinite-autoderef.rs:24:9
   |
LL |     Foo.foo;
---
   |
LL |     Foo.bar();
   |         ^^^ deref recursion limit reached
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`infinite_autoderef`)

error[E0599]: no method named `bar` found for struct `Foo` in the current scope
   |
LL | struct Foo;
LL | struct Foo;
   | ---------- method `bar` not found for this struct
LL |     Foo.bar();
   |         ^^^ method not found in `Foo`

error: aborting due to 6 previous errors
