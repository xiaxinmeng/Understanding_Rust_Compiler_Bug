rust
fn test<F, Fut>(_f: F) 
  where
    Fut: Future<Output=()>,
    F: for<'r> FnOnce(Foo<'r>) -> Fut +'r,
