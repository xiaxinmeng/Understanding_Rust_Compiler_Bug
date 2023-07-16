
error[E0309]: the parameter type `A` may not live long enough
 --> <source>:3:37
  |
3 | struct MyStruct<'a, 'b, A, B, F>(F, &'a A, &'b B);
  |                         -           ^^^^^
  |                         |
  |                         help: consider adding an explicit lifetime bound `A: 'a`...
  |
note: ...so that the reference type `&'a A` does not outlive the data it points at
 --> <source>:3:37
  |
3 | struct MyStruct<'a, 'b, A, B, F>(F, &'a A, &'b B);
  |                                     ^^^^^

error[E0309]: the parameter type `B` may not live long enough
 --> <source>:3:44
  |
3 | struct MyStruct<'a, 'b, A, B, F>(F, &'a A, &'b B);
  |                            -               ^^^^^
  |                            |
  |                            help: consider adding an explicit lifetime bound `B: 'b`...
  |
note: ...so that the reference type `&'b B` does not outlive the data it points at
 --> <source>:3:44
  |
3 | struct MyStruct<'a, 'b, A, B, F>(F, &'a A, &'b B);
  |                                            ^^^^^

error[E0275]: overflow evaluating the requirement `<MyStruct<'_, '_, A, B, F> as MyTrait>::Input`
  --> <source>:12:1
   |
12 | / impl<'a, 'b, A, B, F> MyTrait for MyStruct<'a, 'b, A, B, F> where
13 | |     F: MyOtherTrait<Self::Input, Self::Output>
14 | | {
15 | |     type Input = A;
16 | |     type Output = B;
17 | | }
   | |_^
   |
   = help: consider adding a `#![recursion_limit="128"]` attribute to your crate
   = note: required because of the requirements on the impl of `MyTrait` for `MyStruct<'_, '_, A, B, F>`
   = note: required because of the requirements on the impl of `MyTrait` for `MyStruct<'_, '_, A, B, F>`
   = note: required because of the requirements on the impl of `MyTrait` for `MyStruct<'_, '_, A, B, F>`
   = note: required because of the requirements on the impl of `MyTrait` for `MyStruct<'_, '_, A, B, F>`
   = note: required because of the requirements on the impl of `MyTrait` for `MyStruct<'_, '_, A, B, F>`
   = note: required because of the requirements on the impl of `MyTrait` for `MyStruct<'_, '_, A, B, F>`
   = note: required because of the requirements on the impl of `MyTrait` for `MyStruct<'_, '_, A, B, F>`
   = note: required because of the requirements on the impl of `MyTrait` for `MyStruct<'_, '_, A, B, F>`
8<8<8<8<8<8<8<8<8<8<8<
   = note: required because of the requirements on the impl of `MyTrait` for `MyStruct<'_, '_, A, B, F>`
   = note: required because of the requirements on the impl of `MyTrait` for `MyStruct<'a, 'b, A, B, F>`

error: aborting due to 3 previous errors
