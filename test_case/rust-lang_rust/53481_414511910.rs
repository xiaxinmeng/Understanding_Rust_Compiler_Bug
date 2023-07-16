rust
#[macro_use]
extern crate serde_derive;

mod m1 {
    use m2::Deserialize;

    #[derive(Deserialize)] //~ ERROR cannot determine resolution for the derive macro `Deserialize`
    struct A {}
}

mod m2 {
    pub type Deserialize = u8;

    #[derive(Deserialize)]
    #[serde]
    struct B {}
}
