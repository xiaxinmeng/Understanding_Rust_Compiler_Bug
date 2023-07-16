
warning: attribute should be applied to a function
 --> test10.rs:2:5
  |
2 |       #[cold]
  |       ^^^^^^^
3 | /     'lbl:
4 | |     for i in 1..10 {
5 | |         break 'lbl;
6 | |     }
  | |_____- not a function
  |
  = note: `#[warn(unused_attributes)]` on by default
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
