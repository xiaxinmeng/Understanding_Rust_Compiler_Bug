rust
let x: &&dyn Any = &&1; // This doesn't work
let x: &&dyn Any = &{&1}; // This works
