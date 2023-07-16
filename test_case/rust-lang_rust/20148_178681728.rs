
trait Trait { }
impl Trait for () { }
impl Trait for ((),) { }

pub fn function<K:Trait>() {
}

pub struct Struct<K:Trait> {
    phantom: std::marker::PhantomData<K>,
}

fn main() { }
