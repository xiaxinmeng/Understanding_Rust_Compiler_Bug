rust
#[doc(hidden)]
pub mod decls {
    extern crate proc_macro;
    #[rustc_proc_macro_decls]
    pub static _DECLS: &[proc_macro::bridge::client::ProcMacro] =
        &[proc_macro::bridge::client::ProcMacro::custom_derive("MyDerive",
                                                               &[],
                                                               crate::my_derive),
          proc_macro::bridge::client::ProcMacro::attr("my_attr",
                                                      crate::my_attr),
          proc_macro::bridge::client::ProcMacro::bang("my_macro",
                                                      crate::my_macro)];
}
