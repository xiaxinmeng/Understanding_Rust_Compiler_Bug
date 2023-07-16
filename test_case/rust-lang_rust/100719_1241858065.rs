plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0133]: call to unsafe function is unsafe and requires unsafe block
   --> library/core/src/ptr/const_ptr.rs:573:20
    |
573 |         let this = intrinsics::ptr_mask(self.cast::<()>(), mask);
    |
    = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe block
error[E0133]: call to unsafe function is unsafe and requires unsafe block
   --> library/core/src/ptr/mut_ptr.rs:589:20
    |
589 |         let this = intrinsics::ptr_mask(self.cast::<()>(), mask) as *mut ();
    |
    = note: consult the function's documentation for information on how to avoid undefined behavior

For more information about this error, try `rustc --explain E0133`.
