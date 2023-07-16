`
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
