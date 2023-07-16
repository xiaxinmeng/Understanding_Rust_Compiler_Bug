 rust
let mut boxed = Box::new(vec![...]);
(*boxed).push({
    mem::replace(&mut boxed, Box::new(vec![])).len()
})
