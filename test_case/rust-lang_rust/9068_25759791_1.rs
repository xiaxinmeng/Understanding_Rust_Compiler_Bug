
if !table.contains_key(&key) {
    table.insert(key, ~[1]);
} else {
    table.find_mut(&key).unwrap().push(1);
}
