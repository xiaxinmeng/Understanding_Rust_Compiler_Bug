rust
loop {
    if let Occupied(o) = map.raw_entry_mut().search_bucket(rand(), || true) {
        o.remove();
        break;
    }
}
