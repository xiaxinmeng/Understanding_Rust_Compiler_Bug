
fn option_from_iter_new<A, V: FromIterator<A>, I: IntoIterator<Item=Option<A>>>(iter: I) -> Option<V> {
    let iter = iter.into_iter();
    let mut v = Vec::with_capacity(
        if iter.size_hint().1.is_some() { iter.size_hint().0 } else { 0 }
    );

    for elem in iter.into_iter() {
        match elem {
            Some(e) => v.push(e),
            None => return None
        }
    }

    Some(v.into_iter().collect())
}
