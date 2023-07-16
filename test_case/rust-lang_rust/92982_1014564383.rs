rust
let len1 = vec![1, 2, 3]
    .into_iter()
    .filter(|&x| x < 3)
    .collect_with(Vec::with_capacity(2))
    .len();

// Instead of
let mut vec = Vec::with_capacity(2);
let len2 = vec![1, 2, 3]
    .into_iter()
    .filter(|&x| x < 3)
    .collect_into(&mut vec)
    .len();
