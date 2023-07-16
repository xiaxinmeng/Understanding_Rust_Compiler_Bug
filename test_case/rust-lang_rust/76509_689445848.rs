rust
    assert_eq!(one_ref.inner.strong_count(), 2);
    assert_eq!(one_ref.inner.weak_count(), 1);
