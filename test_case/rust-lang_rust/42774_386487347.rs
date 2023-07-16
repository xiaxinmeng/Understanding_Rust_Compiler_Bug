rust
let foo = Vec::new_in(alloc);
foo.push(Box::new_in(123, alloc));
