
* It might be a good idea to reset the debug location in debuginfo::create_function_debug_context() because trans::base::new_fn_ctxt_w_id() already creates some LLVM instructions which might be assigned some random, call-site source positions otherwise.
* trans::base::alloca_maybe_zeroed() changes the position of the IR builder, possibly putting it back into the prelude section of the function. Clear source positions here also removed some unwanted source position assignments in IR.

Other than that I have not found out why we still get different results than clang. The generated IR for a small test program looked very similar, but in the clang program the arguments already had the correct value in the first line of the function, while in the Rust program the correct values where loaded after the first line...

Maybe some instruction appearing later in the IR gets assigned a wrong source position before at the beginning of the function body.
