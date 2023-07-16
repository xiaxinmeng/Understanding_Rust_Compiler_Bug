
error[E0599]: no method named `align_to` found for type `rustc_target::abi::Size` in the current scope
   --> librustc_codegen_llvm/mir/operand.rs:301:59
    |
301 |                 let b_offset = a_scalar.value.size(bx.cx).align_to(b_scalar.value.align(bx.cx).abi);
    |                                                           ^^^^^^^^

error[E0615]: attempted to take value of method `abi` on type `rustc_target::abi::Align`
   --> librustc_codegen_llvm/mir/operand.rs:301:96
    |
301 |                 let b_offset = a_scalar.value.size(bx.cx).align_to(b_scalar.value.align(bx.cx).abi);
    |                                                                                                ^^^
    |
    = help: maybe a `()` to call it is missing?

error: aborting due to 2 previous errors

Some errors occurred: E0599, E0615.
For more information about an error, try `rustc --explain E0599`.

