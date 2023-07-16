rust
fn test<'a>() -> impl MyFn<&'a A, Output=impl Iterator + 'a>
