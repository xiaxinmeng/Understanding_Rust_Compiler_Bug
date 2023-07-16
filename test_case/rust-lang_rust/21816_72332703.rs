 rust
trait Lattice : PartialOrd {
    fn max(&self, other: &Self) -> Self;
    fn min(&self, other: &Self) -> Self;
}
impl<T: Ord> Lattice for T { ... }
impl Lattice for f32 { ... }
impl Lattice for f64 { ... }
fn max<T: Lattice>(x: T, y: T) -> T { x.max(&y) }
fn min<T: Lattice>(x: T, y: T) -> T { x.min(&y) }
