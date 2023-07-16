 rust
struct Foo { x: i32 }
let mut f = Foo { x: 0 };
drop(f);
f.x = 1;
