
rustc 1.18.0 (03fc9d622 2017-06-06)
error[E0275]: overflow evaluating the requirement `<[Foo<'a>] as std::borrow::ToOwned>::Owned`
  --> <anon>:12:20
   |
12 | pub struct Foo<'a>(Cow<'a, [Foo<'a>]>);
   |                    ^^^^^^^^^^^^^^^^^^^
   |
   = note: required because it appears within the type `Foo<'a>`
   = note: required because of the requirements on the impl of `std::borrow::ToOwned` for `[Foo<'a>]`
   = note: required because it appears within the type `Foo<'a>`
   = note: required because of the requirements on the impl of `std::borrow::ToOwned` for `[Foo<'a>]`
   = note: required by `Cow`

error: aborting due to previous error
