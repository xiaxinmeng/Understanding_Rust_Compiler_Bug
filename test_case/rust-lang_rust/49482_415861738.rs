Rust
#[macro_use]
extern crate binary_macros;

pub trait KvStorage
{
    fn get(&self);
}

impl<K> KvStorage for Box<K>
where
    K: KvStorage + ?Sized,
{
    fn get(&self) {
        (**self).get()
    }
}
