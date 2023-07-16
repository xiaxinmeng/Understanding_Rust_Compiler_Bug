Rust
extern crate proc_macro;

macro_rules! proc_macro_tokenstream {
    () => {
        ::proc_macro::TokenStream
    };
}

macro_rules! proc_macro_expr_impl {
    ($(
        $( #[$attr:meta] )*
        pub fn $func:ident($input:ident: &str) -> String $body:block
    )+) => {
        $(
            // Parses an input that looks like:                                                                                                                                                                                                            
            //                                                                                                                                                                                                                                             
            // 