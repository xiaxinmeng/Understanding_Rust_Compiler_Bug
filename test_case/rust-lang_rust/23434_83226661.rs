 rust
#![feature(core)]
use std::cmp::Ordering;
use std::cmp::Ord;
use std::num::Int;
use std::ops::Index;
use Bound::*;

pub struct Stuff;
// Supporting both generic range and usize indices.
impl<R: Range<usize> + !usize> Index<R> for Stuff { ... }
impl Index<usize> for Stuff { ... }

pub enum Bound<Idx> {
    Inclusive(Idx),
    Exclusive(Idx),
    Unbounded,
}
pub trait Range<Idx> {
    fn into_bounds(self) -> (Bound<Idx>, Bound<Idx>);
    fn bounds(&self) -> (Bound<&Idx>, Bound<&Idx>);
}

pub struct ExclusiveRange<Idx> {
    pub start: Idx,
    pub end: Idx,
}

pub struct LeftExclusiveRange<Idx> {
    pub start: Idx,
    pub end: Idx,
}

pub struct RightExclusiveRange<Idx> {
    pub start: Idx,
    pub end: Idx,
}

pub struct InclusiveRange<Idx> {
    pub start: Idx,
    pub end: Idx,
}

pub struct ExclusiveRangeTo<Idx> {
    pub end: Idx,
}

pub struct InclusiveRangeTo<Idx> {
    pub end: Idx,
}

pub struct ExclusiveRangeFrom<Idx> {
    pub start: Idx,
}

pub struct InclusiveRangeFrom<Idx> {
    pub start: Idx,
}
pub struct RangeFull;

impl<Idx> Range<Idx> for ExclusiveRange<Idx> {
    fn into_bounds(self) -> (Bound<Idx>, Bound<Idx>) {
        (Bound::Exclusive(self.start), Bound::Exclusive(self.end))
    }
    fn bounds(&self) -> (Bound<&Idx>, Bound<&Idx>) {
        (Bound::Exclusive(&self.start), Bound::Exclusive(&self.end))
    }
}

impl<Idx> Range<Idx> for LeftExclusiveRange<Idx> {
    fn into_bounds(self) -> (Bound<Idx>, Bound<Idx>) {
        (Bound::Exclusive(self.start), Bound::Inclusive(self.end))
    }
    fn bounds(&self) -> (Bound<&Idx>, Bound<&Idx>) {
        (Bound::Exclusive(&self.start), Bound::Inclusive(&self.end))
    }
}

impl<Idx> Range<Idx> for RightExclusiveRange<Idx> {
    fn into_bounds(self) -> (Bound<Idx>, Bound<Idx>) {
        (Bound::Inclusive(self.start), Bound::Exclusive(self.end))
    }
    fn bounds(&self) -> (Bound<&Idx>, Bound<&Idx>) {
        (Bound::Inclusive(&self.start), Bound::Exclusive(&self.end))
    }
}

impl<Idx> Range<Idx> for InclusiveRange<Idx> {
    fn into_bounds(self) -> (Bound<Idx>, Bound<Idx>) {
        (Bound::Inclusive(self.start), Bound::Inclusive(self.end))
    }
    fn bounds(&self) -> (Bound<&Idx>, Bound<&Idx>) {
        (Bound::Inclusive(&self.start), Bound::Inclusive(&self.end))
    }
}

impl<Idx> Range<Idx> for ExclusiveRangeFrom<Idx> {
    fn into_bounds(self) -> (Bound<Idx>, Bound<Idx>) {
        (Bound::Exclusive(self.start), Bound::Unbounded)
    }
    fn bounds(&self) -> (Bound<&Idx>, Bound<&Idx>) {
        (Bound::Exclusive(&self.start), Bound::Unbounded)
    }
}

impl<Idx> Range<Idx> for InclusiveRangeFrom<Idx> {
    fn into_bounds(self) -> (Bound<Idx>, Bound<Idx>) {
        (Bound::Inclusive(self.start), Bound::Unbounded)
    }
    fn bounds(&self) -> (Bound<&Idx>, Bound<&Idx>) {
        (Bound::Inclusive(&self.start), Bound::Unbounded)
    }
}

impl<Idx> Range<Idx> for ExclusiveRangeTo<Idx> {
    fn into_bounds(self) -> (Bound<Idx>, Bound<Idx>) {
        (Bound::Unbounded, Bound::Exclusive(self.end))
    }
    fn bounds(&self) -> (Bound<&Idx>, Bound<&Idx>) {
        (Bound::Unbounded, Bound::Exclusive(&self.end))
    }
}

impl<Idx> Range<Idx> for InclusiveRangeTo<Idx> {
    fn into_bounds(self) -> (Bound<Idx>, Bound<Idx>) {
        (Bound::Unbounded, Bound::Inclusive(self.end))
    }
    fn bounds(&self) -> (Bound<&Idx>, Bound<&Idx>) {
        (Bound::Unbounded, Bound::Inclusive(&self.end))
    }
}

impl<Idx> Range<Idx> for RangeFull {
    fn into_bounds(self) -> (Bound<Idx>, Bound<Idx>) {
        (Bound::Unbounded, Bound::Unbounded)
    }
    fn bounds(&self) -> (Bound<&Idx>, Bound<&Idx>) {
        (Bound::Unbounded, Bound::Unbounded)
    }
}
