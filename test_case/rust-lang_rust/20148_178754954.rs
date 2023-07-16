
mod private_module {
    pub trait Trait { }
    impl Trait for () { }
    impl Trait for ((),) { }
}


pub fn function<K:private_module::Trait>() {
}

pub struct Struct<K:private_module::Trait> {
    phantom: std::marker::PhantomData<K>,
}

fn main() { }
