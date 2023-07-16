plain
    Checking cranelift-native v0.95.1
    Checking cranelift-frontend v0.95.1
    Checking cranelift-object v0.95.1
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0599]: no method named `require` found for reference `&LanguageItems` in the current scope
    |
969 |       let def_id = fx
    |  __________________-
970 | |         .tcx
970 | |         .tcx
971 | |         .lang_items()
972 | |         .require(lang_item)
    | |         -^^^^^^^ method not found in `&LanguageItems`
    | 

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_codegen_cranelift` (lib) due to previous error
