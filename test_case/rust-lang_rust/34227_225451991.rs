 rust
rooted!(let data = RootedValue::new(global.get_cx(), init.data));

// Expands to:
let data = RootedValueContainer::new(global.get_cx(), init.data);
let data = RootedValue::new(&mut data);
