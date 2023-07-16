rust
#![feature(specialization)]

use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::{Bound, Bound::Included, RangeBounds};

#[derive(Clone)]
pub struct MyRangeInclusive<Idx> {
    pub(crate) start: Idx,
    pub(crate) end: Idx,
    pub(crate) is_empty: Option<bool>,
}

impl<T> RangeBounds<T> for MyRangeInclusive<T> {
    fn start_bound(&self) -> Bound<&T> {
        Included(&self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        Included(&self.end)
    }
}

trait MyRangeInclusiveEquality: Sized {
    fn canonicalized_is_empty(range: &MyRangeInclusive<Self>) -> bool;
}

impl<T> MyRangeInclusiveEquality for T {
    #[inline]
    default fn canonicalized_is_empty(range: &MyRangeInclusive<Self>) -> bool {
        range.is_empty.unwrap_or_default()
    }
}

impl<T: PartialOrd> MyRangeInclusiveEquality for T {
    #[inline]
    fn canonicalized_is_empty(range: &MyRangeInclusive<Self>) -> bool {
        range.is_empty()
    }
}

// Make this one a `default fn`...
impl<Idx: PartialEq> PartialEq for MyRangeInclusive<Idx> {
    #[inline]
    default fn eq(&self, other: &Self) -> bool {
        self.start == other.start
            && self.end == other.end
            && MyRangeInclusiveEquality::canonicalized_is_empty(self)
                == MyRangeInclusiveEquality::canonicalized_is_empty(other)
    }
}

// And add a second version (with an identical implementation) for PartialOrd.
// The presence of this second impl specifically is what makes it fail the borrow
// checker as opposed to compiling and printing invalid text (though I don't know why.)
impl<Idx: PartialOrd> PartialEq for MyRangeInclusive<Idx> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
            && self.end == other.end
            && MyRangeInclusiveEquality::canonicalized_is_empty(self)
                == MyRangeInclusiveEquality::canonicalized_is_empty(other)
    }
}

impl<Idx: Eq> Eq for MyRangeInclusive<Idx> {}

impl<Idx: Hash> Hash for MyRangeInclusive<Idx> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.start.hash(state);
        self.end.hash(state);
        MyRangeInclusiveEquality::canonicalized_is_empty(self).hash(state);
    }
}

impl<Idx> MyRangeInclusive<Idx> {
    #[inline]
    pub const fn new(start: Idx, end: Idx) -> Self {
        Self {
            start,
            end,
            is_empty: None,
        }
    }

    #[inline]
    pub const fn start(&self) -> &Idx {
        &self.start
    }

    #[inline]
    pub const fn end(&self) -> &Idx {
        &self.end
    }

    #[inline]
    pub fn into_inner(self) -> (Idx, Idx) {
        (self.start, self.end)
    }
}

impl<Idx: fmt::Debug> fmt::Debug for MyRangeInclusive<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.start.fmt(fmt)?;
        write!(fmt, "..=")?;
        self.end.fmt(fmt)?;
        Ok(())
    }
}

impl<Idx: PartialOrd<Idx>> MyRangeInclusive<Idx> {
    pub fn contains<U>(&self, item: &U) -> bool
    where
        Idx: PartialOrd<U>,
        U: ?Sized + PartialOrd<Idx>,
    {
        <Self as RangeBounds<Idx>>::contains(self, item)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.is_empty.unwrap_or_else(|| !(self.start <= self.end))
    }

    #[inline]
    pub(crate) fn compute_is_empty(&mut self) {
        if self.is_empty.is_none() {
            self.is_empty = Some(!(self.start <= self.end));
        }
    }
}

use core::cell::RefCell;

#[derive(Debug)]
struct Evil<'a, 'b> {
    values: RefCell<Vec<&'a str>>,
    to_insert: &'b String,
}

impl<'a, 'b> PartialEq for Evil<'a, 'b> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<'a> PartialOrd for Evil<'a, 'a> {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        self.values.borrow_mut().push(self.to_insert);
        None
    }
}

fn main() {
    let e;
    let values;
    {
        let to_insert = String::from("Hello, world!");
        e = Evil {
            values: RefCell::new(Vec::new()),
            to_insert: &to_insert,
        };
        let range = MyRangeInclusive::new(&e, &e);
        let _ = range == range;
        values = e.values;
    }
    println!("{:?}", values.borrow());
}
