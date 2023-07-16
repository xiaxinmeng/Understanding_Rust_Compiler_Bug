 rust
let mut map : HashMap<u8, &u8> = HashMap::new();
let tmp = Box::new(2);
map.insert(43, &*tmp);
