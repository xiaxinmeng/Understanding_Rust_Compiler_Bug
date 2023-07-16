rust
let x = &[(); usize::MAX];
x.array_windows::<N>(); // this now causes an integer overflow for any `N` because of my impl
x.array_windows::<0>(); // this has an unrepresentable size
