rust
catch_unwind(|| {
   call_c(rust_callback);
});

extern { fn call_c(*const fn()) }

extern "C" fn rust_callback() {
   panic!();
}
