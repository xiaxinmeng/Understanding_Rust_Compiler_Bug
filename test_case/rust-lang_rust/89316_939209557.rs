plain
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
error[E0658]: use of unstable library feature 'iter_intersperse': recently added
    --> compiler/rustc_ast_pretty/src/pprust/state.rs:2274:45
     |
2274 |                 for abi in abis.into_iter().intersperse(&Symbol::intern(", ")) {
     |
     = note: see issue #79524 <https://github.com/rust-lang/rust/issues/79524> for more information
     = help: add `#![feature(iter_intersperse)]` to the crate attributes to enable

