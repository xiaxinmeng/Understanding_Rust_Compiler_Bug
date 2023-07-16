rust
    fn check13() {
        macro m() {}
        {
            gen_inner!();
            #[rustc_transparent_macro]
            macro gen_invoc() { m!() } //~ ERROR `m` is ambiguous
            gen_invoc!();
        }
    }
