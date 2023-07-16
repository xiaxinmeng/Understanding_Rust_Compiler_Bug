 Rust
let a: &mut [u32] = &mut [1,2];
let b: &mut [u32] = &mut [3,4];
let c;
a.get_mut({a=b; c=a; 1})
// equiv
let ix = {a=b; c=a; 1}
<&mut [u32]>::get_mut(a, ix) //~ ERROR use of moved value
