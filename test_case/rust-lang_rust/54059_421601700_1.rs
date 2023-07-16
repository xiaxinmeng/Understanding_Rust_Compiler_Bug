
            $( #[$attr] )*
            #[proc_macro_derive($func)]
            pub fn $func(input: proc_macro_tokenstream!()) -> proc_macro_tokenstream!() {
                unsafe { puts("hello!\n\0" as *const str as *const u8); }
                panic!()
            }
        )+
    };
}

proc_macro_expr_impl! {
    pub fn base2_impl(input: &str) -> String {
        panic!()
    }
}

#[link(name="c")]
extern "C" {
    fn puts(inp: *const u8);
}
