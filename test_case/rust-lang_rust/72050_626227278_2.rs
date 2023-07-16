
error[E0308]: mismatched types
 --> src/main.rs:3:10
  |
3 |     for (&s, _) in &tuples {
  |          ^^        ------- this expression has type `&({integer}, {integer})`
  |          |
  |          this is a borrow of a tuple, but you used a borrow of an inner field
  |          help: you can borrow the tuple instead: `&(s, _)`
  |
  = note:   expected type `&({integer}, _)`
          found tuple `(&{integer}, _)`
