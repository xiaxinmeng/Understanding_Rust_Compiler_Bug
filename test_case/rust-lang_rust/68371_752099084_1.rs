rust
let mut state = init;
iter.map(move |x| {
   state = f(state, x);
   state
}))
