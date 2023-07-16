plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0658]: use of unstable library feature 'iter_intersperse': recently added
   --> compiler/rustc_passes/src/intrinsicck.rs:414:69
    |
414 | ...                   features.iter().map(|f| f.as_str()).intersperse(", ").collect::<String>(),
    |
    = note: see issue #79524 <https://github.com/rust-lang/rust/issues/79524> for more information
    = help: add `#![feature(iter_intersperse)]` to the crate attributes to enable

