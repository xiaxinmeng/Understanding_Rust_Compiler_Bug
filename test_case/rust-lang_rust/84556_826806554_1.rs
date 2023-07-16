
error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
  --> /home/r/src/rust/rustc.2/src/test/ui/rfc-2632-const-trait-impl/call-generic-method-nonconst-opt-out.rs:30:17
   |
LL |             Num(self.0.plus(rhs.0))
   |                 ^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
