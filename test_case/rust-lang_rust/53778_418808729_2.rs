rust
    fn check5() {
        macro m() {}
        {
            #[rustc_transparent_macro]
            macro gen_inner_invoc() {
                macro m() {}
                m!(); // OK
            }
            gen_inner_invoc!();
        }
    }
