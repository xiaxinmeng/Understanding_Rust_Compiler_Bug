plain

---- [ui] src/test/ui/transmute/main.rs stdout ----
diff of stderr:

1 error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
-   --> $DIR/main.rs:13:5
- LL |     transmute(x)
-    |     ^^^^^^^^^
-    |
-    |
-    = note: `<C as TypeConstructor<'_>>::T` does not have a fixed size
- 
- error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
11    |
12 LL |     let x: u8 = transmute(10u16);


---

error: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/transmute/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmute/main" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmute/main/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
   |
   |
LL |     let x: u8 = transmute(10u16); //~ ERROR cannot transmute between types of different sizes
   |
   |
   = note: source type: `u16` (16 bits)
   = note: target type: `u8` (8 bits)

error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
   |
   |
LL |     let x: u8 = transmute("test"); //~ ERROR cannot transmute between types of different sizes
   |
   |
   = note: source type: `&str` (128 bits)
   = note: target type: `u8` (8 bits)

error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
   |
   |
LL |     let x: Foo = transmute(10); //~ ERROR cannot transmute between types of different sizes
   |
   |
   = note: source type: `i32` (32 bits)
   = note: target type: `Foo` (0 bits)
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0512`.
------------------------------------------
