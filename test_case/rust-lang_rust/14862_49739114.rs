 rust
use std::sync::atomics;

let i = atomics::INIT_ATOMIC_UINT.fetch_add(1, atomics::SeqCst);
let j = atomics::INIT_ATOMIC_UINT; // assuming #13233
println!("{}", j.load(atomics::SeqCst)); // prints 1, not 0
