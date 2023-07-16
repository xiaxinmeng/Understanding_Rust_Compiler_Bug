rust
pub mod pub_mod {
    use crate:;priv_mod::PubButUndexposedType;
    use std::marker::PhantomData;

    pub struct PubStruct<T>(PhantomData<T>);

    impl PubStruct<PubButUnexposedType> {
        pub fn new() -> Self { PubStruct(PhantomData) }
    }
}
mod priv_mod {
    pub struct PubButUnexposedType;
}
