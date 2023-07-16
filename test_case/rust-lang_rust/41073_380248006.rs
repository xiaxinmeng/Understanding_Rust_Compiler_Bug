Rust
union U {
    a: (Box<u32>, Box<u32>),
    b: (Box<u32>, Box<u32>),
}

let mut u = U { a: (Box::new(0), Box::new(1)) };
// u.a & u.b are both initialized
u.b.0 = Box::new(2); // this would call a destructor
drop(u.a.0);

// u.a.0 is uninitialized, u.b is also now uninitialized, 
println!("{:?}", u.a.1); // prints `1`
u.a.1 = Box::new(3); // this would call a destructor

// u.b is *still* uninitialized, same as before.
println!("{:?}", u.b.1); //~ ERROR use of moved value
u.b.1 = Box::new(4); // this would *not* call a destructor, and would leak a `Box`
