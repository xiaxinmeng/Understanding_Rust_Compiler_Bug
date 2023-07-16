rust
// Given a `#[proc_macro_attribute]` `foo`
use attrs::foo;
// and a custom derive `#[proc_macro_derive(Trait, attributes(foo))]`,
use derives::Trait;

#[derive(Trait)] #[foo] struct A; // (1)
#[foo] #[derive(Trait)] struct B; // (2)
