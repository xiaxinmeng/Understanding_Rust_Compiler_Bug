plain
---- [ui] ui/unsized/unsized3-rpass.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized/unsized3-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized3-rpass/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized3-rpass/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/unsized/unsized3-rpass.rs:90:81
   |
LL |         let x: &Qux = &*ptr::from_raw_parts::<Qux>((&*data as *const _).cast(), ptr::metadata(obj));
   |                                                                                 ^^^^^^^^^^^^^^^^^^ expected struct `Qux`, found trait object `dyn Tr`
   |
   = note: expected struct `DynMetadata<Qux<'_>>`
              found struct `DynMetadata<dyn Tr>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.

