
error[E0464]: multiple matching crates for `serde_derive`
  --> $DIR/used_underscore_binding_macro.rs:4:1
   |
LL | extern crate serde_derive;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `serde_derive`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8823e1c401e486b.so
           crate `serde_derive`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserde_derive-4b65394b709f9e2e.so
