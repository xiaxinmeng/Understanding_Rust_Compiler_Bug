rust
let token_iter = ...;
let borrowed_iter = token_iter.by_ref().filter(...);
// do something with borrowed_iter here
drop(borrowed_iter);
// token_iter becomes usable again
