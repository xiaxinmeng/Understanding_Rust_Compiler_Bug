rust
    assert_eq!(1028, std::mem::size_of_val(&single()));
    assert_eq!(1028, std::mem::size_of_val(&single_with_noop()));
    assert_eq!(3080, std::mem::size_of_val(&joined()));
    assert_eq!(3080, std::mem::size_of_val(&joined_with_noop()));
