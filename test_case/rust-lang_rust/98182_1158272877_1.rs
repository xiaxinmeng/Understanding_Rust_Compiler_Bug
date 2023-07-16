rust
> #[macro_attr1] // expanded first
> #[doc = mac!()] // `mac!` is expanded fourth.
> #[macro_attr2] // expanded second
> #[derive(MacroDerive1, MacroDerive2)] // expanded third
> fn foo() {}
> 