rust
#![feature(type_alias_impl_trait)]
type Iter<'t, D: 'static + Copy> = impl Iterator<Item = D>;
fn iter_discrims<D: 'static + Copy>(vec: &Vec<D>) -> Iter<'_, D> { vec[..].iter().copied() }
