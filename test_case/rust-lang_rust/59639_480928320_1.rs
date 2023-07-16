rust
#![crate_type = "lib"]

pub fn lines<'a>(left: &'a str) {
    iter(left.lines());
}

fn iter<I, T>(left: I)
where
    I: Clone + Iterator<Item = T> + DoubleEndedIterator,
    T: PartialEq,
{
    let _left_count = left.clone().count();
}
