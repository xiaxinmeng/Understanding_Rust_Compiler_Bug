
pub fn foo<T>(lists: &[&[T]]) {
    let e: &[T] = &[];
    let (first, rest) = lists.split_first().unwrap_or((&e, &[&[]]));
}

pub fn foo2<T>(lists: &[&[T]]) {
    let (first, rest) = lists.split_first().unwrap_or((&(&[] as _), &[&[]]));
}
