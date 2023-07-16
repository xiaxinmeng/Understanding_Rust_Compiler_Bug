 Rust
let y = Vec::new();
(***boxed).get({boxed=&&&y; 42})
// equiv
let y = Vec::new();
let ix = {boxed=&&&y; 42};
<[u8]>::get(&<Vec<u8> as Deref>::deref(&***boxed), ix)
