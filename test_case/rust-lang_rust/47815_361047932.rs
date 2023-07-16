rust
fn identity(x: _) -> _ { x }

// Probably not? the two _'s needn't be the same types?
// so you would get
// :: forall a b. a -> b
// instead of
// :: forall a. a -> a
// and thus it would not compile?
