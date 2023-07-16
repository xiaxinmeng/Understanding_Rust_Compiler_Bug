rust
match map.raw_entry_mut().from_hash(hash, |k| is_same(k)) {
    RawMutEntry::Occupied(entry) =>
        entry.get(),
    RawMutEntry::Vacant(entry) =>
        entry.insert(produce_key()?, produce_value()?).0
}
