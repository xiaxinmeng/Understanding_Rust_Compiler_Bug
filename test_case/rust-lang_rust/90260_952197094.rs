plain
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0425]: cannot find value `filename` in this scope
    --> compiler/rustc_codegen_ssa/src/back/link.rs:2378:49
     |
2378 |         let symbol_name = get_dylib_symbol_name(filename, &sess.target)

error[E0425]: cannot find value `filename` in this scope
    --> compiler/rustc_codegen_ssa/src/back/link.rs:2382:13
     |
