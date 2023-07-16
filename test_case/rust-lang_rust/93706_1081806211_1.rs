rust
// Return type is !Send !Sync even if the type underpinning `x` is
const fn foo(x: &dyn Trait) -> &dyn Trait { x }
// Return type Send and/or Sync if the type of `x` is
const fn foo(x: impl Trait) -> impl Trait { x }
