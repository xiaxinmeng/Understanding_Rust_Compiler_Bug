plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0308]: mismatched types
   --> src/driver/aot.rs:308:56
    |
308 |         tcx.sess.opts.cg.target_cpu.as_ref().unwrap_or(&tcx.sess.target.cpu).to_owned();
    |                                                        ^^^^^^^^^^^^^^^^^^^^ expected struct `std::string::String`, found enum `Cow`
    = note: expected reference `&std::string::String`
    = note: expected reference `&std::string::String`
               found reference `&Cow<'static, str>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_cranelift` due to previous error
Build completed unsuccessfully in 0:03:25
