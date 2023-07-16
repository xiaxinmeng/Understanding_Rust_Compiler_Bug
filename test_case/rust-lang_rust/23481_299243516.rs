


rustc 1.17.0 (56124baa9 2017-04-24)
error[E0277]: the trait bound `for<'a> <&'a () as Bar>::Xyz: std::clone::Clone` is not satisfied
  --> <anon>:10:6
   |
10 | impl Foo for () {}
   |      ^^^ the trait `for<'a> std::clone::Clone` is not implemented for `<&'a () as Bar>::Xyz`
   |
   = help: the following implementations were found:
             <&'a T as std::clone::Clone>
   = note: required by `Foo`

error: aborting due to previous error

Compilation failed.
