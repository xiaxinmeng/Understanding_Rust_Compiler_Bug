 rust
let mut x = Rc::new(75u);
let y = x.make_unique();

println!("{}", x);

error: cannot borrow `x` as immutable because it is also borrowed as mutable
