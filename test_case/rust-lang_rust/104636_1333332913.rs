plain
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0433]: failed to resolve: could not find `cow` in `std`
    --> compiler/rustc_hir_analysis/src/astconv/mod.rs:1325:38
     |
1325 |                 .map_or(|| std::cow::Cow::from("NewTrait", |max| std::cow::Cow::from(format!("NewTrait{}", max + 1))));
     |                                      ^^^ not found in `std::cow`
help: consider importing this enum
     |
8    | use std::borrow::Cow;
     |
     |
help: if you import `Cow`, refer to it directly
     |
1325 -                 .map_or(|| std::cow::Cow::from("NewTrait", |max| std::cow::Cow::from(format!("NewTrait{}", max + 1))));
1325 +                 .map_or(|| Cow::from("NewTrait", |max| std::cow::Cow::from(format!("NewTrait{}", max + 1))));

error[E0433]: failed to resolve: could not find `cow` in `std`
    --> compiler/rustc_hir_analysis/src/astconv/mod.rs:1325:76
     |
     |
1325 |                 .map_or(|| std::cow::Cow::from("NewTrait", |max| std::cow::Cow::from(format!("NewTrait{}", max + 1))));
     |                                                                            ^^^ not found in `std::cow`
help: consider importing this enum
     |
8    | use std::borrow::Cow;
     |
     |
help: if you import `Cow`, refer to it directly
     |
1325 -                 .map_or(|| std::cow::Cow::from("NewTrait", |max| std::cow::Cow::from(format!("NewTrait{}", max + 1))));
1325 +                 .map_or(|| std::cow::Cow::from("NewTrait", |max| Cow::from(format!("NewTrait{}", max + 1))));

    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> compiler/rustc_hir_analysis/src/astconv/mod.rs:1325:18
     |
1325 |                 .map_or(|| std::cow::Cow::from("NewTrait", |max| std::cow::Cow::from(format!("NewTrait{}", max + 1))));
     |
note: associated function defined here
    --> /checkout/library/core/src/option.rs:981:18
     |
     |
981  |     pub const fn map_or<U, F>(self, default: U, f: F) -> U
help: provide the argument
     |
     |
1325 |                 .map_or(|| std::cow::Cow::from("NewTrait", |max| std::cow::Cow::from(format!("NewTrait{}", max + 1))), /* value */);

Some errors have detailed explanations: E0061, E0433.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `rustc_hir_analysis` due to 3 previous errors
