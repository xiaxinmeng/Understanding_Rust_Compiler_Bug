 rust
    let mut foo = Foo;
    assert_eq!(foo.len(), 0);      // ok
    assert_eq!(foo.get(0), None);  // ok
    // compile error, but generated documentation lists get_mut as an available function
    assert_eq!(foo.get_mut(0), None); 
