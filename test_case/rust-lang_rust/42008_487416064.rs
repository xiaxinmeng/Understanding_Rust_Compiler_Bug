rust
//your proc_macro's mod.rs code...

pub mod decls {
    extern crate proc_macro;
    #[rustc_proc_macro_decls]
    pub static _DECLS: &[proc_macro::bridge::client::ProcMacro] =
        &[proc_macro::bridge::client::ProcMacro::bang(
            "your_proc_macro",
            crate::your_proc_macro,
        )];
}
