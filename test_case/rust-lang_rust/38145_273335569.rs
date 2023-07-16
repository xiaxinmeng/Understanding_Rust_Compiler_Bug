rust
extern crate serde;

struct Foo<N> {
    x: N,
}

pub struct Sd<V>(V);

impl<N> serde::ser::Serialize for Sd<Foo<N>>
{
    fn serialize<S: serde::ser::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> 
    {
        S::Ok()
    }
}
