
error: internal compiler error: broken MIR in DefId(0:3 ~ inference[4f8b]::main) ((_1.0: u32)): can't project out of PlaceTy { ty: main::T, variant_index: None }
  --> /home/ubuntu/rust5/src/test/ui/type-alias-impl-trait/inference.rs:6:10
   |
LL |     let (a, b): (u32, u32) = foo;
   |          ^
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:855:31

error: internal compiler error: broken MIR in DefId(0:3 ~ inference[4f8b]::main) ((_1.1: u32)): can't project out of PlaceTy { ty: main::T, variant_index: None }
  --> /home/ubuntu/rust5/src/test/ui/type-alias-impl-trait/inference.rs:6:13
   |
LL |     let (a, b): (u32, u32) = foo;
   |             ^
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:855:31
