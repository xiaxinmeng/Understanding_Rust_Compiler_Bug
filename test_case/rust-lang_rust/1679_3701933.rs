
alt check foo {
    some(some(bar)) {
        // adds a predicate foo | some(*)
    }
    some(none) {
        // also adds a predicate foo | some(*)
    }
    none { ... }
}
