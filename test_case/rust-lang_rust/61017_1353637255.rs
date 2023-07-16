
  |
2 |         let _ = if true {
  |  _______________-
3 | |           (1,
  | |  _________-
4 | | |         2)
  | | |__________- expected because of this
5 | |       } else {
6 | | /         ("",
7 | | |         2)
  | | |__________^ expected integer, found `&str`
8 | |       };
  | |_______- `if` and `else` have incompatible types
  |
  = note: expected tuple `({integer}, {integer})`
             found tuple `(&str, {integer})`
