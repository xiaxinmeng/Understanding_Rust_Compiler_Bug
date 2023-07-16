 rust
for 10.times {
    let x: uint = std::rand::random();
    assert_eq!(fmt!("%p", x as *uint), fmt!("%x", x));
}
