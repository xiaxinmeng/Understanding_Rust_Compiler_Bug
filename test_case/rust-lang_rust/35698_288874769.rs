rust
error[E0408]: variable `y` is not bound in all patterns
 --> <anon>:5:19
  |
5 |         Some(y) | None => {}
  |              -    ^^^^ pattern doesn't bind `y`
  |              |
  |              variable not in all patterns
