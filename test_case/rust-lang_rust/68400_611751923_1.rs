rust
let a = A;
a.test::<true>();
// we could infer from `true` that we require a method
// `test<const _: bool>()` which means that `N` must be `7`.
