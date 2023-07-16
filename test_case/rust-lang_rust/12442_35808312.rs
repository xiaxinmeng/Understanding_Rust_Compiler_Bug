 rust
#[lang="ord"]
/// This trait is solely to provide access to {<, >, <=, >=} sugared operators,
///  but there are no guarantees about their semantics.  It has an intentionally ugly
/// name to discourage use as an accidental trait-bound on type parameters.
pub trait OrdOps {
    fn lt(&self, other: &Self) -> bool;
    #[inline]
    fn le(&self, other: &Self) -> bool { !other.lt(self) }
    #[inline]
    fn gt(&self, other: &Self) -> bool {  other.lt(self) }
    #[inline]
    fn ge(&self, other: &Self) -> bool { !self.lt(other) }

    // FIXME (#12068): Add min/max/clamp default methods
}

/// This trait provides concrete guarantees about ordering.
/// Note that f32/f64 does not impl this trait, but most other numerics do.
trait Ord : OrdOps {  }
