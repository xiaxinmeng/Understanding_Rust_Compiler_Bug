 Rust
(*boxed).push({
    mem::replace(&mut boxed, Box::new(vec![])).len()
})
// equiv
let arg0 = mem::replace(&mut boxed, Box::new(vec![])).len();
Vec::push(&mut *boxed, arg0); // this can be surprising, I guess.
