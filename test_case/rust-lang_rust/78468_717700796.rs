
// proc-macro crate
pub fn my_proc_macro() { ... }

// main crate
/// Here are [intra-doc links](my_function) for my_proc_macro
pub use proc_macro_crate::my_proc_macro;
