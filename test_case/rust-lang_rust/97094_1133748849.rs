Rust
#[cfg_attr(all(), allow(unknown_lints, while_true))]
mod bar {
    #[allow(noex_inside_module)] // Warns for the unknown lint name here due to the allow above not working
    fn _foo() { while true {} } // Doesn't warn for the `while true` here due to the allow working
}
