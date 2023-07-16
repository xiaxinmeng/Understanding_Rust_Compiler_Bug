rust
u.iter() // returns iterator over all Options in the Vec
   .filter(|_|true) // closure returns true for every option in the vec, so you get back the same iterator
   .map(|x| x.unwrap()) // program panics on the last Option, because it wasn't filtered out
   .collect()
