rust
fn count_repeated_chars_in_list_of_groups__fold_first(list_of_groups: &[HashSet<char>]) -> usize {
    list_of_groups
        .iter()
        .cloned()
        .fold_first(|accumulator, next| accumulator.intersection(&next).copied().collect())
        .unwrap()
        .len()
}
