rust
let vec = vec![0, -1, 2, 3];

assert_eq!(
    vec.iter().copied().flat_map(|x| usize::try_from(x).ok()).collect::<Vec<_>>(),
    &[0, 2, 3],
);

assert_eq!(
    vec.iter().copied().map_while(|x| usize::try_from(x).ok()).collect::<Vec<_>>(),
    &[0],
);
