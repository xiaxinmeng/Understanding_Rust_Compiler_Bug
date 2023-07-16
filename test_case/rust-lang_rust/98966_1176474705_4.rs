rust
let foo = mem::uninitialized();
// initialize undef fields
foo.x = 1;
foo.y = 2;
// use foo
// ...
