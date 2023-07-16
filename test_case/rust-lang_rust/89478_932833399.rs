plain
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error: argument never used
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:2276:73
     |
2276 |                     err.note(&format!("1 redundant requirement hidden", count));
     |                                       --------------------------------  ^^^^^ argument never used
     |                                       formatting specifier missing

    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: could not compile `rustc_trait_selection` due to previous error
