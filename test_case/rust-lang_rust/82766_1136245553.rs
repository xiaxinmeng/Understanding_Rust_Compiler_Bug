rust
let mut occupied = false;
let val : &mut Value = my_map.get_or_insert_with(key, ||{ occupied = true; Value::new() });
