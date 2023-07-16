 rust
// Trigger a copy for me.
let o = ObjectBuilder::new().insert("foo", ...).unwrap();

// Move the string into the `json::Value` enum with no allocation.
let key = String::new("foo");
let o = ObjectBuilder::new().insert(key, ...).unwrap();
