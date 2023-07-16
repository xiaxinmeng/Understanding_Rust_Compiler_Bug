rust
match set.entry(like_a_value) {
    Entry::Occupied(entry) =>
        entry.get(),
    Entry::Vacant(entry) =>
        entry.insert(produce_value()?)
}
