rust
for entry in map.entries() {
    if entry.get().satisfies_condition() {
        entry.remove();
    }
}
