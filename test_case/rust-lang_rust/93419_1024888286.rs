`
warning: type `Fg_Struct_s` should have an upper camel case name
 --> c.rs:8:12
  |
8 | pub struct Fg_Struct_s {
  |            ^^^^^^^^^^^ help: convert the identifier to upper camel case: `FgStructS`
  |
  = note: `#[warn(non_camel_case_types)]` on by default

warning: type `Fg_Struct` should have an upper camel case name
  --> c.rs:13:12
   |
13 | pub struct Fg_Struct(pub Fg_Struct_s);
   |            ^^^^^^^^^ help: convert the identifier to upper camel case: `FgStruct`

warning: `extern` block uses type `Fg_Struct`, which is not FFI-safe
  --> c.rs:4:25
   |
4  |     pub fn Fg_Init() -> *mut Fg_Struct;
   |                         ^^^^^^^^^^^^^^ not FFI-safe
   |
   = note: `#[warn(improper_ctypes)]` on by default
   = note: this struct contains only zero-sized fields
note: the type is defined here
  --> c.rs:13:1
   |
13 | pub struct Fg_Struct(pub Fg_Struct_s);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted
