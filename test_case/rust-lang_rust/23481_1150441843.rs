
error[E0277]: the trait bound `for<'a> <&'a () as Bar>::Xyz: Clone` is not satisfied
  --> <source>:10:6
   |
1  | pub trait Foo
   |           --- required by a bound in this
...
4  |     for<'a> <&'a Self as Bar>::Xyz: Clone {}
   |                                     ----- required by this bound in `Foo`
...
10 | impl Foo for () {}
   |      ^^^ the trait `for<'a> Clone` is not implemented for `<&'a () as Bar>::Xyz`
   |
   = help: the following implementations were found:
             <&T as Clone>
