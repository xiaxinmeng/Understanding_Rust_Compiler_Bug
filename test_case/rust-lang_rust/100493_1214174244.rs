rust
let mut x = &mut whatever;
let y = &x.foo;
drop(y);
x.mutate();
