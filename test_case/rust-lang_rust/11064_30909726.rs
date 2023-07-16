 rust
pub trait LessThanEqualComparator<T> {
    fn less_eq(&self, a: &T, b: &T) -> bool;
}

pub struct SortForward;
impl<T: Ord> LessThanEqualComparator<T> for SortForward {
    fn less_eq(&self, a: &T, b: &T) -> bool { *a <= *b }
}

pub struct SortReverse;
impl<T: Ord> LessThanEqualComparator<T> for SortReverse {
    fn less_eq(&self, a: &T, b: &T) -> bool { *b <= *a }
}

impl<'a, T> LessThanEqualComparator<T> for 'a |&T, &T| -> bool {
    fn less_eq(&self, a: &T, b: &T) -> bool {
        (*self)(a, b)
    }
}

[...]

    fn sort<Sort: LessThanEqualComparator>(self, less_eq: Sort);
