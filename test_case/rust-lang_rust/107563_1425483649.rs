rust
error[E0252]: the name `fmt` is defined multiple times
  --> library/core/src/ffi/mod.rs:14:5
   |
12 | use crate::fmt;
   |     ---------- previous import of the module `fmt` here
13 | use crate::marker::PhantomData;
14 | use crate::num::*;
   |     ^^^^^^^^^^^^^
   |     |
   |     `fmt` reimported here
   |     you can use `as` to change the binding name of the import
   |
   = note: `fmt` must be defined only once in the type namespace of this module

error[E0659]: `fmt` is ambiguous
   --> library/core/src/ffi/mod.rs:221:6
    |
221 | impl fmt::Debug for c_void {
    |      ^^^ ambiguous name
    |
    = note: ambiguous because of a conflict between a name from a glob import and a macro-expanded name in the same module during import or macro resolution
