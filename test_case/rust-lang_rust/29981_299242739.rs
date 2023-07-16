


rustc 1.17.0 (56124baa9 2017-04-24)

error[E0277]: the trait bound `<<Self as Trait>::Associated as std::ops::Index<usize>>::Output: std::marker::Sized` is not satisfied
 --> <anon>:3:1
  |
3 |   trait Trait: Index<usize>
  |  _^ starting here...
4 | | where <Self as Index<usize>>::Output: Sized {
5 | |     type Associated: Trait;
6 | | }
  | |_^ ...ending here: the trait `std::marker::Sized` is not implemented for `<<Self as Trait>::Associated as std::ops::Index<usize>>::Output`
  |
  = help: consider adding a `where <<Self as Trait>::Associated as std::ops::Index<usize>>::Output: std::marker::Sized` bound
  = note: required by `Trait`

error: aborting due to previous error

