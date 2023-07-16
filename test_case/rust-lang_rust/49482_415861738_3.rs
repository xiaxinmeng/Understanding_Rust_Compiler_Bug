                                                                                                                                                                                                                                         
            $( #[$attr] )*
            #[proc_macro_derive($func)]
            pub fn $func(input: proc_macro_tokenstream!()) -> proc_macro_tokenstream!() {
                panic!()
            }
        )+
    };
}



proc_macro_expr_impl! {
    pub fn base2_impl(input: &str) -> String {
        panic!()
    }
    pub fn base4_impl(input: &str) -> String {
        panic!()
    }
    pub fn base8_impl(input: &str) -> String {
        panic!()
    }
    pub fn base16_impl(input: &str) -> String {
        panic!()
    }
    pub fn base32hex_impl(input: &str) -> String {
        panic!()
    }
    pub fn base32_impl(input: &str) -> String {
        panic!()
    }
    pub fn base64_impl(input: &str) -> String {
        panic!()
    }
    pub fn base64url_impl(input: &str) -> String {
        panic!()
    }


    pub fn base2_nopad_impl(input: &str) -> String {
        panic!()
    }
    pub fn base4_nopad_impl(input: &str) -> String {
        panic!()
    }
    pub fn base8_nopad_impl(input: &str) -> String {
        panic!()
    }
    pub fn base16_nopad_impl(input: &str) -> String {
        panic!()
    }
    pub fn base32hex_nopad_impl(input: &str) -> String {
        panic!()
    }
    pub fn base32_nopad_impl(input: &str) -> String {
        panic!()
    }
    pub fn base64_nopad_impl(input: &str) -> String {
        panic!()
    }
    pub fn base64url_nopad_impl(input: &str) -> String {
        panic!()
    }
}
