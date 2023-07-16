
error: internal compiler error: compiler/rustc_hir_typeck/src/writeback.rs:697:17: writeback: `Delta<dyn Trait>` has inference variables
  --> fake-test-src-base/offset-of/offset-of-dst-field.rs:44:5
   |
LL |     offset_of!(Delta<dyn Trait>, z); //~ ERROR the size for values of type
   |
   = note: this error: internal compiler error originates in the macro `offset_of` (in Nightly builds, run with -Z macro-backtrace for more info)
