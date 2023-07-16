
match opt_value.entry() {
    VacantEntry(entry) => entry.insert(x),
    OccupiedEntry(entry) => { assert_eq!(entry.get(), x); }
}
