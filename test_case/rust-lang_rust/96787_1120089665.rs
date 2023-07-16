
    make_accessors! {
            struct A {
                    #[quux]
                    a: u32 : get Q,
                    #[foo]
                    b: u32,
                    #[bar]
                    c: u32 : get R,
            }
    }
