plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0405]: cannot find trait `Hasher` in this scope
  --> src/driver/aot.rs:25:23
   |
25 |     fn hash_stable<H: Hasher>(&self, _: &mut HCX, _: &mut StableHasher<H>) {
   |
help: consider importing one of these items
   |
4  | use core::hash::Hasher;
