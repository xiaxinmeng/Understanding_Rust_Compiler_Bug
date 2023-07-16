 rust
for entry in map.entries() {
    if let Entry::Occupied(o) = entry {
        return o.remove();
    } else { unreachable!() }
}
