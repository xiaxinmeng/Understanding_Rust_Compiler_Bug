 rust
let x = Rc::new(1);
let y = x.downgrade();

// x is still valid, so should print 1
println!("{}", y);

// remove the strong reference
drop(x);

// no more strong references, should print <invalid reference>
println!("{}", y); 
