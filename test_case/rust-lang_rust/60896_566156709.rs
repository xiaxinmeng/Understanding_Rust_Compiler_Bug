rust
match map.entry(key) {
    Entry::Occupied(entry) =>
        entry.get(),
    Entry::Vacant(entry) =>
        entry.insert(produce_value()?)
}
