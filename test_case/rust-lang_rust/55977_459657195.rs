rust
let mut count = 0;
let counter = std::iter::from_fn(move || {
    count += 1;

    if count < 6 {
        Some(count)
    } else {
        None
    }
});
assert_eq!(counter.collect::<Vec<_>>(), &[1, 2, 3, 4, 5]);
