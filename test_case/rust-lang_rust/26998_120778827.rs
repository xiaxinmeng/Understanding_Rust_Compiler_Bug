 rust
match foo {
  (Some(x), _) |
  (_, Some(x)) if cond(x) => { ... } 
}
