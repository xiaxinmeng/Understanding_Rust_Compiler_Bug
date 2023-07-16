plain
---- [ui] tests/ui/associated-types/associated-types-in-ambiguous-context.rs stdout ----
diff of stderr:

31    |          ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
32 LL | type X = <OsString as Deref>::Target;
-      and 7 other candidates
+      and 4 other candidates
35 
36 error[E0223]: ambiguous associated type
---
To only update this specific test, also pass `--test-args associated-types/associated-types-in-ambiguous-context.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-types/associated-types-in-ambiguous-context.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-in-ambiguous-context" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-in-ambiguous-context/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0223]: ambiguous associated type
  --> fake-test-src-base/associated-types/associated-types-in-ambiguous-context.rs:6:36
   |
LL | fn get<T:Get,U:Get>(x: T, y: U) -> Get::Value {}
   |
   |
help: if there were a type named `Example` that implemented `Get`, you could use the fully-qualified path
   |
LL | fn get<T:Get,U:Get>(x: T, y: U) -> <Example as Get>::Value {}

error[E0223]: ambiguous associated type
  --> fake-test-src-base/associated-types/associated-types-in-ambiguous-context.rs:20:17
   |
   |
LL | trait Foo where Foo::Assoc: Bar {
   |                 ^^^^^^^^^^ help: use the fully-qualified path: `<Self as Foo>::Assoc`
error[E0223]: ambiguous associated type
  --> fake-test-src-base/associated-types/associated-types-in-ambiguous-context.rs:25:10
   |
LL | type X = std::ops::Deref::Target;
LL | type X = std::ops::Deref::Target;
   |          ^^^^^^^^^^^^^^^^^^^^^^^
   |
help: use the fully-qualified path
   |
LL | type X = <CString as Deref>::Target;
   |          ~~~~~~~~~~~~~~~~~~~~~~~~~~
LL | type X = <IoSlice<'_> as Deref>::Target;
   |          ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
LL | type X = <IoSliceMut<'_> as Deref>::Target;
   |          ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
LL | type X = <OsString as Deref>::Target;
     and 4 other candidates

error[E0223]: ambiguous associated type
  --> fake-test-src-base/associated-types/associated-types-in-ambiguous-context.rs:11:23
  --> fake-test-src-base/associated-types/associated-types-in-ambiguous-context.rs:11:23
   |
LL |     fn grab(&self) -> Grab::Value;
   |                       ^^^^^^^^^^^ help: use the fully-qualified path: `<Self as Grab>::Value`
error[E0223]: ambiguous associated type
  --> fake-test-src-base/associated-types/associated-types-in-ambiguous-context.rs:14:22
   |
   |
LL |     fn get(&self) -> Get::Value;
   |
   |
help: if there were a type named `Example` that implemented `Get`, you could use the fully-qualified path
   |
LL |     fn get(&self) -> <Example as Get>::Value;

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0223`.
