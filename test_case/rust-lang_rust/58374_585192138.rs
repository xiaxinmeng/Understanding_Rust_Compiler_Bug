rust
struct A {
    a: u32,
    b: !,
}

#[no_partial_initialisation]
struct B {
    a: u32,
    b: !,
}


fn main() {
    println!("{}", std::mem::size_of::<A>()); // 4
    loop { let _ = A { a: 23, b: break }; }
    
    println!("{}", std::mem::size_of::<A>()); // 0
    loop { let _ = B { a: 23, b: break }; }   // compile error
}
