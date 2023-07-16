rust
error[E0053]: method `add` has an incompatible type for trait
 --> <anon>:8:5
  |
8 |     fn add(&self, rhs: &Self) -> Self {loop{}}
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Fixed13`, found &Fixed13
  |
  = note: expected type `fn(Fixed13, Fixed13) -> Fixed13`
             found type `fn(&Fixed13, &Fixed13) -> Fixed13`
