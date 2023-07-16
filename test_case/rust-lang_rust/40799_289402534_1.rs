
error[E0308]: mismatched types
 --> <anon>:6:13
  |
6 |             2
  |             ^ expected (), found integral variable
  |
  = note: expected type `()`
             found type `{integer}`

error[E0308]: match arms have incompatible types
 --> <anon>:2:5
  |
2 |       match 3 {
  |  _____^ starting here...
3 | |         4 => 1,
4 | |         3 => {
5 | |             println!("Yep it maches.");
6 | |             2
7 | |         }
8 | |         _ => 2
9 | |     }
  | |_____^ ...ending here: expected integral variable, found ()
  |
  = note: expected type `{integer}`
             found type `()`
note: match arm with an incompatible type
 --> <anon>:4:14
  |
4 |           3 => {
  |  ______________^ starting here...
5 | |             println!("Yep it maches.");
6 | |             2
7 | |         }
  | |_________^ ...ending here

error: aborting due to 2 previous errors
