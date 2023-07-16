rust
use ::std::{fmt::Debug, iter::Step};

/// constraint alias
pub
trait Idx : Copy + Debug + Eq + Ord + Step {}
impl<T : Copy + Debug + Eq + Ord + Step> Idx for T {}

// type level enum
trait BoundType : Debug + Default + Copy + Eq { fn less<T : Idx> (lhs: T, rhs: T) -> bool; }
    impl BoundType for Exclusive {
        #[inline]
        fn less<T : Idx> (lhs: T, rhs: T) -> bool { lhs < rhs }
    }
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub
    struct Exclusive;

    impl BoundType for Inclusive {
        #[inline]
        fn less<T : Idx> (lhs: T, rhs: T) -> bool { lhs <= rhs }
    }
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub
    struct Inclusive;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub
struct RangeCopy<T : Idx, B : BoundType>
{
    pub
    start: T,

    pub
    end: T,

    _bound_type: B,
}

pub
type RangeExclusiveCopy<T> = RangeCopy<T, Exclusive>;

pub
type RangeInclusiveCopy<T> = RangeCopy<T, Inclusive>;

impl<T : Idx, B : BoundType> RangeCopy<T, B> {
    #[inline(always)]
    pub
    fn contains (self: Self, value: T) -> bool
    {
        self.start <= value && B::less(value, self.end)
    }
}

impl<T : Idx> From<Range<T>> for RangeExclusiveCopy<T> {
    #[inline]
    fn from (range: Range<T>) -> Self
    {
        let Range { start, end } = range;
        RangeCopy { start, end, _bound_type: BoundType::default() }
    }
}

impl<T : Idx> From<RangeExclusiveCopy<T>> for Range<T> {
    #[inline]
    fn from (range: RangeExclusiveCopy<T>) -> Self
    {
        let RangeCopy { start, end, .. } = range;
        Self { start, end }
    }
}

impl<T : Idx> IntoIterator for RangeExclusiveCopy<T> {
    type Item = T;
    type IntoIter = Range<T>;

    #[inline]
    fn into_iter (self: Self) -> Self::IntoIter
    {
         self.into()
    }
}

// etc.
