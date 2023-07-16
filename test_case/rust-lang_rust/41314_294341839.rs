
error[E0026]: variant `X::Y` does not have a field named `data`
 --> <anon>:7:16
  |
7 |         X::Y { data } => ()
  |                ^^^^ variant `X::Y` does not have field `data`

error[E0027]: pattern does not mention field `0`
 --> <anon>:7:9
  |
7 |         X::Y { data } => ()
  |         ^^^^^^^^^^^^^ missing field `0`
note[E????]: trying to match a tuple variant with a struct variant pattern
