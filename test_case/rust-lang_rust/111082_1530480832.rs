plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0308]: mismatched types
   --> src/base.rs:339:21
    |
338 |                 match msg {
    |                       --- this expression has type `&std::boxed::Box<rustc_middle::mir::AssertKind<rustc_middle::mir::Operand<'_>>>`
339 |                     AssertKind::BoundsCheck { ref len, ref index } => {
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<AssertKind<Operand<'_>>>`, found `AssertKind<_>`
    |
    = note: expected struct `std::boxed::Box<rustc_middle::mir::AssertKind<rustc_middle::mir::Operand<'_>>>`
                 found enum `rustc_middle::mir::AssertKind<_>`
error[E0308]: mismatched types
   --> src/base.rs:351:21
    |
338 |                 match msg {
338 |                 match msg {
    |                       --- this expression has type `&std::boxed::Box<rustc_middle::mir::AssertKind<rustc_middle::mir::Operand<'_>>>`
...
351 |                     AssertKind::MisalignedPointerDereference { ref required, ref found } => {
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<AssertKind<Operand<'_>>>`, found `AssertKind<_>`
    |
    = note: expected struct `std::boxed::Box<rustc_middle::mir::AssertKind<rustc_middle::mir::Operand<'_>>>`
                 found enum `rustc_middle::mir::AssertKind<_>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_cranelift` (lib) due to 2 previous errors
Build completed unsuccessfully in 0:01:38
