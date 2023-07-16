rust
// force-host
// no-prefer-dynamic
#![crate_type = "proc-macro"]
extern crate proc_macro;
struct PanicOnDrop;

impl Drop for PanicOnDrop {
    fn drop(&mut self) { panic!("panic on drop!"); }
}

#[proc_macro_derive(Panic)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _p_on_d = PanicOnDrop;
    panic!("panic during panic-during-expand's derive")
}
