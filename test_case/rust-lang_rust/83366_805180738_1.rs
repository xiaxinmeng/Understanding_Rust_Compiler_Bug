rust
    #[macro_attr1]
    #[doc = mac!()]
    #[macro_attr2]
    #[derive(MacroDerive1, MacroDerive2)]
    struct S;

    bar!();
    