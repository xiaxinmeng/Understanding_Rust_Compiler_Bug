rust
trait Deserializer {
    type Error;
}

trait Deserialize {
    fn deserialize<D>(_: D) -> D::Error
    where D: Deserializer;
}

macro_rules! impl_deserialize {
    ($name:ident) => {
        impl Deserialize for $name {
            fn deserialize<D>(_: D) -> D::Error
            where D: Deserializer
            {
                loop {}
            }
        }
    }
}

macro_rules! formats {
    {
        $($name:ident,)*
    } => {
        $(
            pub struct $name;

            impl_deserialize!($name);
        )*
    }
}
formats! { Foo, Bar, }
