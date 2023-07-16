rust
error[E0308]: mismatched types
 --> <source>:8:43
  |
8 |     fn y(&self, y: f64) -> Self { P{y, .. self.clone() } }
  |                                           ^^^^^^^^^^^^ expected struct `P`, found `&P<T>`
  |
  = note: expected struct `P<T>`
          found reference `&P<T>`
