rust
pub trait Bar { }

pub trait Quux<T> { type Assoc; }
pub fn demo(_: impl Quux<(), Assoc=<() as Quux<impl Bar>>::Assoc>) { }

impl<T> Quux<T> for () { type Assoc = u32; }

fn main() { }
