rust
fn use_it<'a, I: IntoIterator<Item = &'a mut (dyn std::fmt::Debug + 'a)>>(iter: I) {
    iter.into_iter().for_each(drop);
}
