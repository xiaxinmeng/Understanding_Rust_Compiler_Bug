rust
#[macro_use]
extern crate serde; //1.0.117
macro_rules! formats {
    {
        $($name:ident,)*
    } => {
        $(
            #[derive(Deserialize)]
            pub struct $name;
        )*
    }
}
formats! { Foo, Bar, }
