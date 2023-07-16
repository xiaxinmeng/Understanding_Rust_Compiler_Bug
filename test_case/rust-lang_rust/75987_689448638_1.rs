rust
impl Foo for u32 { //~ ERROR not all trait items implemented, missing: `foo`
    default pub fn foo<T: Default>() -> T { T::default() }
    //~^ ERROR expected one of `async`, `const`, `extern`, `fn`, `pub`, `unsafe`, or `use`, found keyword `pub`
}
