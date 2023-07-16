rust
#![feature(unboxed_closures)]
#![feature(min_type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]

use foo::bbb;

type Fun = impl Fn<()>;
static BBB: Fun = bbb;
type Out = <Fun as FnOnce<()>>::Output;
    

fn main() {
    println!("{}", std::any::type_name::<Fun>());
    println!("{}", std::any::type_name::<Out>());
    
    let bar: Out = BBB();
}


mod foo {
    use bar::Bar;

    pub fn bbb() -> Bar {
        Bar
    }

    mod bar {
        pub struct Bar;
    }
}
