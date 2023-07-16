rust
    r = RangeInclusive::new(127i8, 127);
    assert_eq!(r.next(), Some(127));
    assert_eq!(r.next(), None);
