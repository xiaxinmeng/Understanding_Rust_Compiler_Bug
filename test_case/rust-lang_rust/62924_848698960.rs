rust
let mut seen_ids: BTreeSet<i64> = BTreeSet::new();
if seen_ids.len() > max_number_of_ids_to_track {
    let drop_oldest = seen_ids.iter().cloned().next();
    seen_ids.remove(&drop_oldest.unwrap());
}
