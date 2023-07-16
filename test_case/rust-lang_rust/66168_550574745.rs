rust
fn test<'r, F, Fut>(_f: F) 
  where
    Fut: Future<Output=()> + 'r,
    F: FnOnce(Foo<'r>) -> Fut,
