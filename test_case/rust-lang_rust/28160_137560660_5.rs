 Rust
let boxed: Box<&[u8]> = get();
boxed.get({drop(boxed); 4})
// equiv
let boxed: Box<&[u8]> = get();
let ix = { drop(boxed); 4 };
<[u8]>::get(&**boxed) //~ ERROR use of moved value
