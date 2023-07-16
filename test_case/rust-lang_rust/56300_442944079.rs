
error[E0277]: the trait bound `&builder::Builder<'_, 'll, 'tcx>: rustc_target::abi::HasDataLayout` is not satisfied
   --> librustc_codegen_llvm/mir/operand.rs:301:47
    |
301 |                 let b_offset = a_scalar.value.size(bx).align_to(b_scalar.value.align(bx).abi);
    |                                               ^^^^ the trait `rustc_target::abi::HasDataLayout` is not implemented for `&builder::Builder<'_, 'll, 'tcx>`

error[E0599]: no method named `align_to` found for type `rustc_target::abi::Size` in the current scope
   --> librustc_codegen_llvm/mir/operand.rs:301:56
    |
301 |                 let b_offset = a_scalar.value.size(bx).align_to(b_scalar.value.align(bx).abi);
    |                                                        ^^^^^^^^

error[E0277]: the trait bound `&builder::Builder<'_, 'll, 'tcx>: rustc_target::abi::HasDataLayout` is not satisfied
   --> librustc_codegen_llvm/mir/operand.rs:301:80
    |
301 |                 let b_offset = a_scalar.value.size(bx).align_to(b_scalar.value.align(bx).abi);
    |                                                                                ^^^^^ the trait `rustc_target::abi::HasDataLayout` is not implemented for `&builder::Builder<'_, 'll, 'tcx>`

error[E0615]: attempted to take value of method `abi` on type `rustc_target::abi::Align`
   --> librustc_codegen_llvm/mir/operand.rs:301:90
    |
301 |                 let b_offset = a_scalar.value.size(bx).align_to(b_scalar.value.align(bx).abi);
    |                                                                                          ^^^
    |
    = help: maybe a `()` to call it is missing?

error: aborting due to 4 previous errors

Some errors occurred: E0277, E0599, E0615.
For more information about an error, try `rustc --explain E0277`.
