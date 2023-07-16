rust
    let mut it = (2..20).take(3);
    assert_eq!(it.try_for_each(Err), Err(2));
    assert_eq!(it.try_for_each(Err), Err(3));
    assert_eq!(it.try_for_each(Err), Err(4));
    assert_eq!(it.try_for_each(Err), Ok(()));

    let mut it = (2..20).take(3).rev();
    assert_eq!(it.try_for_each(Err), Err(4));
    assert_eq!(it.try_for_each(Err), Err(3));
    assert_eq!(it.try_for_each(Err), Err(2));
    assert_eq!(it.try_for_each(Err), Ok(()));
