rust
use crate::vm;

pub struct RootDeserializer<Der> {
    _der: std::marker::PhantomData<Der>,
}

impl<'de, Der: serde::Deserializer<'de>> RootDeserializer<Der> {
    pub fn new(der: Der, max_calls: usize) -> Self {
        Self {
            _der: todo!()
        }
    }
}

impl<'de, Der> serde::Deserializer<'de> for RootDeserializer<Der>
where
    Der: serde::Deserializer<'de>
{
    type Error = Der::Error;
}
