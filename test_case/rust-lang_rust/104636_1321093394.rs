plain
     |
1316 |                 let regular_traits_vec = regular_traits
     |                 ^^^

error: expected `,`, found `;`
    --> compiler/rustc_hir_analysis/src/astconv/mod.rs:1319:41
1319 |                     .collect::<Vec<_>>();
     |                                         ^ expected `,`

error: expected expression, found `let` statement
error: expected expression, found `let` statement
    --> compiler/rustc_hir_analysis/src/astconv/mod.rs:1320:17
     |
1320 |                 let sug_trait_name = if regular_traits_vec.contains(&"NewTrait".to_string()) {


error: expected one of `,`, `.`, `?`, or an operator, found `;`
    --> compiler/rustc_hir_analysis/src/astconv/mod.rs:1328:22
1328 |                     };
1328 |                     };
     |                      ^ expected one of `,`, `.`, `?`, or an operator
error: format argument must be a string literal
    --> compiler/rustc_hir_analysis/src/astconv/mod.rs:1316:17
     |
1316 | /                 let regular_traits_vec = regular_traits
1316 | /                 let regular_traits_vec = regular_traits
1317 | |                     .iter()
1318 | |                     .map(|t| t.trait_ref().print_only_trait_path().to_string())
1319 | |                     .collect::<Vec<_>>();
     |
help: you might be missing a string literal to format with
     |
     |
1316 |                 "{} {} {} {} {}", let regular_traits_vec = regular_traits

    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
