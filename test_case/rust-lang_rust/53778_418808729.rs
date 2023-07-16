rust
    fn check1() {
        macro m() {} // candidate A
        {
            #[rustc_transparent_macro]
            macro gen_gen_inner_invoc() {
                gen_inner!(); // generates candidate B
                m!(); //~ ERROR `m` is ambiguous
            }
            gen_gen_inner_invoc!();
        }
    }
