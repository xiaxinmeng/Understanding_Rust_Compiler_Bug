 rust
pub struct Range<A>;

impl<A> Clone for Range<A> where A: Clone {
    fn clone(&self) -> Range<A> {
        panic!()
    }
}

pub struct RangeInclusive<A> {
    range: Range<A>,
}

impl<A> Clone for RangeInclusive<A> where Range<A>: Clone {
    fn clone(&self) -> RangeInclusive<A> {
        panic!()
    }
}

fn main() {}
