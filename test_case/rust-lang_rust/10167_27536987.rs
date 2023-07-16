 rust
struct Foo<'self> { x: Option<&'self int> }

let mut x = 1;
let mut y = Foo { x: None };
y.x = Some(&x); // `y` is frozen, but `x` isn't.
