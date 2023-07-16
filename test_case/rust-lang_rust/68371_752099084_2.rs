rust
let mut state = init;
iter.map(move |x| {
   let new_state = f(&state, x);
   std::mem::replace(&mut state, new_state)
})
