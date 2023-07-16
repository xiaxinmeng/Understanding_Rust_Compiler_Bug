 rust
fn find_mut<'a,T>(prev: &'a mut ~List<T>, pred: |&T| -> bool) -> Option<&'a mut ~List<T>> {
    match prev {
        &~Cons(ref x, _) if pred(x) => return Some(prev),
        &~Cons(_, ref mut rs) => return find_mut(rs, pred),
        &~Nil => return None
    }
}
