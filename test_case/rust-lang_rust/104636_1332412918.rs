plain
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_hir_analysis v0.0.0 (/checkout/compiler/rustc_hir_analysis)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: field expressions cannot have generic arguments
    --> compiler/rustc_hir_analysis/src/astconv/mod.rs:1323:74
     |
1323 |                 .filter_map(|name| name.strip_prefix("NewTrait")?.parse::<u32>::().ok())


error: expected one of `(`, `)`, `,`, `.`, `?`, or an operator, found `::`
    --> compiler/rustc_hir_analysis/src/astconv/mod.rs:1323:79
     |
1323 |                 .filter_map(|name| name.strip_prefix("NewTrait")?.parse::<u32>::().ok())
     |                                                                               ^^ expected one of `(`, `)`, `,`, `.`, `?`, or an operator

error: expected one of `.`, `;`, `?`, `else`, or an operator, found `)`
    --> compiler/rustc_hir_analysis/src/astconv/mod.rs:1323:88
     |
1323 |                 .filter_map(|name| name.strip_prefix("NewTrait")?.parse::<u32>::().ok())
     |                                                                                        ^ expected one of `.`, `;`, `?`, `else`, or an operator
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error: could not compile `rustc_hir_analysis` due to 3 previous errors
