rust
let alignment_groups = optimizing.group_by(|a, b| {
    field_align(&fields[a as usize]) == field_align(&fields[b as usize])
});
for group in alignment_groups {
    group.shuffle(&mut rng);
}
