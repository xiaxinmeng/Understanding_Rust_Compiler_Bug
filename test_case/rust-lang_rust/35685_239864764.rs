
error: match arms have incompatible types [--explain E0308]
 --> <anon>:2:5
2 |>     match Some(()) {
  |>     ^ expected bool, found ()
note: expected type `_`
note:    found type `()`
note: match arm with an incompatible type
 --> <anon>:3:21
3 |>         Some(()) => { }
  |>                     ^^^
