rust
{
  let pinned = foo;
  // use `Pin::new_unchecked(&mut pinned)`
}  // `pinned` implicitly dropped here
