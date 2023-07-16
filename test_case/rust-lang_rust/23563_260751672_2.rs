 bash
$ rustc a.rs --crate-type=lib
$ rustc b.rs --crate-type=lib --extern a=liba.rlib
error[E0119]: conflicting implementations of trait `a::LolFrom<&[_]>` for type `LocalType<_>`:
  --> b.rs:9:1
   |
9  | impl<'a, T> LolFrom<&'a [T]> for LocalType<T> {
   | ^
   |
   = note: conflicting implementation in crate `a`

error: aborting due to previous error
