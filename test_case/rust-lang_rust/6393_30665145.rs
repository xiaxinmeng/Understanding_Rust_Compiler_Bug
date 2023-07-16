 rust
use std::util;

enum List<T> {
    Cons(T, ~List<T>),
    Nil
}

fn find_mut<'a,T>(prev: &'a mut ~List<T>, pred: |&T| -> bool) -> Option<&'a mut ~List<T>> {
    match prev {
        &~Cons(ref x, _) if pred(x) => {}, // NB: can't return Some(prev) here
        &~Cons(_, ref mut rs) => return find_mut(rs, pred),
        &~Nil => return None
    };
    return Some(prev)
}
