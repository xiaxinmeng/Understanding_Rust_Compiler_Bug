rs 
#![feature(min_type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]

type Out = impl core::any::Any;
fn main() {
    println!("{}", core::any::type_name::<Out>());
    let _v: Out = foo::bbb();
}

mod foo {
    use bar::Bar;
    pub fn bbb() -> Bar { Bar }
    mod bar {
        pub struct Bar;
    }
}
