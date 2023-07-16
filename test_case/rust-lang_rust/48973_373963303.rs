rust
let x = &mut Foo;
let y = &mut Foo;

let _ = bar(x); // x is reborrowed
println!("{:?}", x);

let _ = x + y; // x is consumed
// println!("{:?}", x); // "use of moved value: `x`"
