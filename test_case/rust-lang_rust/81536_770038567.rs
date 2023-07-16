plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0061]: this function takes 4 arguments but 3 arguments were supplied
  --> src/bin/cg_clif.rs:65:5
   |
65 |     print_time_passes_entry(callbacks.time_passes, "\ttotal", start.elapsed());
   |     |
   |     expected 4 arguments

error: aborting due to previous error
