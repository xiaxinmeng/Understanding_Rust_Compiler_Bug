
use std::convert::identity;

pub fn is_sorted_and_has_duplicates_by<'a, T, S: PartialEq + PartialOrd>(
    data: &'a [T],
    key: impl Fn(&'a T) -> S,
) -> Option<&T> {
    unimplemented!()
}

pub fn is_sorted_and_has_duplicates<T: PartialEq + PartialOrd>(data: &[T]) -> Option<&T> {
    is_sorted_and_has_duplicates_by(data, identity)
}
