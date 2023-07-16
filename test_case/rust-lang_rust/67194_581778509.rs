rust
pub struct RangeInclusive<Idx> {
    pub(crate) start: Idx,
    pub(crate) end: Idx,
    pub(crate) is_empty: Option<bool>,
    // This field is:
    //  - `None` when next() or next_back() was never called
    //  - `Some(false)` when `start <= end` assuming no overflow
    //  - `Some(true)` otherwise
    // The field cannot be a simple `bool` because the `..=` constructor can
    // accept non-PartialOrd types, also we want the constructor to be const.
}
