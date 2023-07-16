rust
#![feature(type_alias_impl_trait)]
#![feature(auto_traits)]

auto trait Auto {}

fn b() -> Box<dyn Auto> { Box::new(foo::c()) }

mod foo {
    type Type = impl Sized;
    
    pub fn c() -> Type {}
}
