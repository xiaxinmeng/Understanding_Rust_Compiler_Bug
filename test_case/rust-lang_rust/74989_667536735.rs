rust
pub struct Predicate<const P: bool>;

pub trait Satisfied {}

impl Satisfied for Predicate<true> {}

// Guarantees that if `x` is a `BoundedUsize<A, B>` value, then `x.0 >= A && x.0 <= B && A <= B`
pub struct BoundedUsize<const A: usize, const B: usize>(usize)
where Predicate<{ A <= B }>: Satisfied;

impl<T, const N: usize, const A: usize, const B: usize> Index<BoundedUsize<A, B>> for [T; N]
where Predicate<{ A <= B && B < N }>: Satisfied
{
    type Output = T;

    fn index(&self, index: BoundedUsize<A, B>) -> &T {
        unsafe { self.get_unchecked(index.0) }
    }
}
