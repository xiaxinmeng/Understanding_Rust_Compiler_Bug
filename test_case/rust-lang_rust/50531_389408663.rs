plain
[00:06:08]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:06:12]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:07:45]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:08:02]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:08:08] error: unused import: `ty::fold::TypeVisitor`
[00:08:08]   --> librustc/ty/util.rs:20:5
[00:08:08]    |
[00:08:08] 20 | use ty::fold::TypeVisitor;
[00:08:08]    |
[00:08:08]    = note: `-D unused-imports` implied by `-D warnings`
[00:08:08] 
[00:08:34] error: aborting due to previous error
