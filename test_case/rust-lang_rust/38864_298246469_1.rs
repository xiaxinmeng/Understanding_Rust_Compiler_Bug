
rustc 1.19.0-nightly (afa1240e5 2017-04-29)
error[E0275]: overflow evaluating the requirement `<SomeType as SomeTrait>::AssocType`
 --> <anon>:9:6
  |
9 | impl SomeTrait for SomeType where Self::AssocType: SomeOtherTrait {
  |      ^^^^^^^^^
  |
  = note: required because of the requirements on the impl of `SomeTrait` for `SomeType`

error: aborting due to previous error
