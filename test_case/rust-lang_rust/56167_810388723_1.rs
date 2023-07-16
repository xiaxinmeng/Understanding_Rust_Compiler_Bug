
let x = if let Some(x) = my_map.get(&key) {
    *x
} else {
    my_map.insert(key.clone(), 42);
    42
};
