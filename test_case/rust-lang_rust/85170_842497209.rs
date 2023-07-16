rust
fn use_it<'a, 'b: 'a, I: IntoIterator<Item = &'a mut (dyn std::fmt::Debug + 'b)>>(iter: I) {
    iter.into_iter().for_each(drop);
}
