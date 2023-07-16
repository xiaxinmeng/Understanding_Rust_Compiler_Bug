sh
$ RUSTC_LOG=rustc_codegen_ssa::back::link=info rustc-custom -v -Z cf-protection=full -o empty-custom-rust empty.rs

 INFO rustc_codegen_ssa::back::link preparing Executable to "empty-custom-rust"
 INFO rustc_codegen_ssa::back::link "cc" "-m64" "empty-custom-rust.empty.ea9ce8e5-cgu.0.rcgu.o" "empty-custom-rust.empty.ea9ce8e5-cgu.1.rcgu.o" "empty-custom-rust.empty.ea9ce8e5-cgu.2.rcgu.o" "empty-custom-rust.empty.ea9ce8e5-cgu.3.rcgu.o" "empty-custom-rust.empty.ea9ce8e5-cgu.4.rcgu.o" "empty-custom-rust.empty.ea9ce8e5-cgu.5.rcgu.o" "empty-custom-rust.empty.ea9ce8e5-cgu.6.rcgu.o" "empty-custom-rust.30bkn99o4ddymxvy.rcgu.o" "-Wl,--as-needed" "-L" [... some rlib files]
