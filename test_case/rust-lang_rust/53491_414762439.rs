rust
let mut b = MaybeUninit::<bool>::uninitialized();
let bref = b.get_mut(); // insta-UB?
