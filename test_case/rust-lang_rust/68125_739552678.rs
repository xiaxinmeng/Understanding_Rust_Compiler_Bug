rust
fn count_repeated_chars_in_list_of_groups(list_of_groups: &[HashSet<char>]) -> usize {
    let mut iter = list_of_groups.iter();
    let first = iter.next().unwrap().clone();
    iter.fold(first, |accumulator, next| {
        accumulator.intersection(&next).copied().collect()
    })
    .len()
}
