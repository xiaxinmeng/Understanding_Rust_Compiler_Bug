 rust
struct Foo { x: i32 }
impl Drop for Foo { fn drop(&mut self) {} }
let mut f = Foo { x: 0 };
drop(f);
f.x = 1;
