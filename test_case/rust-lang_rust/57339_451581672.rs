rust
let x = Foo { ... };
pin_mut!(x);
let x2 = x.as_mut();
x2.set(Foo { ... });
// At this point, x2 has been destroyed, but the old x borrow becomes usable again
