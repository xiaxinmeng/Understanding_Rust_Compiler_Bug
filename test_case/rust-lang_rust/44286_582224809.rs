rust
match map.entry(key) {
    Entry::Occupied(entry) => 
        entry.into_mut().do_thing_which_can_fail().map_err(|_| entry.replace_key()),
    Entry::Vacant(entry) => ...,
}
