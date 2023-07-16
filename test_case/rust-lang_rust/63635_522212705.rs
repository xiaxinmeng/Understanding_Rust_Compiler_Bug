rust
    assert_eq!(<Vec<i32>>::new().as_ptr(), <&[i32]>::default().as_ptr());
    assert_eq!(<Box<[i32]>>::default().as_ptr(), (&[]).as_ptr());
