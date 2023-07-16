
rustc 1.18.0-nightly (bbdaad0dc 2017-04-14)
error[E0277]: the trait bound `u8: Trait<bar::Type>` is not satisfied
  --> <anon>:15:5
   |
15 |     foo::<u8, Type>();
   |     ^^^^^^^^^^^^^^^ the trait `Trait<bar::Type>` is not implemented for `u8`
   |
   = help: the following implementations were found:
             <u8 as Trait<foo::Type>>
   = note: required by `foo`
