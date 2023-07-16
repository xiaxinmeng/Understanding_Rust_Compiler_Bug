
rustc 1.19.0-nightly (6684d176c 2017-06-03)
warning: variable `baz` is assigned to, but never used
  --> <anon>:8:17
   |
8  |             let mut baz = 1;
   |                 ^^^^^^^
...
16 |     foo![1, 2,];
   |     ------------ in this macro invocation
   |
   = note: #[warn(unused_variables)] on by default

warning: value assigned to `baz` is never read
  --> <anon>:10:24
   |
4  | |         $baz += 1;
   | |____________^
...
10 |               foo!(@bar [baz]);
   |  ________________________^
...
16 |       foo![1, 2,];
   |       ------------ in this macro invocation
   |
   = note: #[warn(unused_assignments)] on by default
