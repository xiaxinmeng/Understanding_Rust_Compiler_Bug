rust

pub trait Helper<G, H>: FnOnce(G) -> H {}
impl<T, G, H> Helper<G, H> for T where T: FnOnce(G) -> H {}

pub fn fun<T, U, F>()
where
    T: Trait,
    U: Trait,
    F: for<'a> Helper<T::Gat<'a>, U::Gat<'a>>,
{}
