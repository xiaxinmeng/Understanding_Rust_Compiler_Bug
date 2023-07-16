
/// Current way
// key1, key2: String;
let mut t = TreeMap::new()
t.insert(key1, 34);
t.insert(key2, 45);
let v = t.find(&"test".to_string()); // Ohh, allocating?!

/// Closures 
// key1, key2: String;
let mut t = TreeMap::new()
t.insert(key1, 34);
t.insert(key2, 45);
let v = t.find_with(|k| "test".cmp(&k.as_slice()); // Ohh, and that for every key to request?

/// Proposed 
//  key1, key2: CustomType<Str>;
let mut t = TreeMap::new()
t.insert(key1, 34);
t.insert(key2, 45);
let v = t.find_equiv(&"test");
