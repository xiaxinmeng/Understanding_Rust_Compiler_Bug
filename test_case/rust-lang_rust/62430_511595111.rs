
$ time rustc +devel file7.rs
error[E0275]: overflow evaluating the requirement `MyStruct<'_, '_, A, B, F>: MyTrait`
  --> file7.rs:14:1
   |
14 | / impl<'a, 'b, A, B, F> MyTrait for MyStruct<'a, 'b, A, B, F> where
15 | |     F: MyOtherTrait<Self::Input, Self::Output>
16 | | {
17 | |     type Input = A;
18 | |     type Output = B;
19 | | }
   | |_^
   |
   = help: consider adding a `#![recursion_limit="8"]` attribute to your crate
   = note: required because of the requirements on the impl of `MyTrait` for `MyStruct<'_, '_, A, B, F>`
   = note: required because of the requirements on the impl of `MyTrait` for `MyStruct<'_, '_, A, B, F>`
   = note: required because of the requirements on the impl of `MyTrait` for `MyStruct<'_, '_, A, B, F>`
   = note: required because of the requirements on the impl of `MyTrait` for `MyStruct<'a, 'b, A, B, F>`

error[E0275]: overflow evaluating the requirement `MyStruct<'_, '_, A, B, F>: MyTrait`
  --> file7.rs:17:5
   |
17 |     type Input = A;
   |     ^^^^^^^^^^^^^^^
   |
   = help: consider adding a `#![recursion_limit="8"]` attribute to your crate
   = note: required because of the requirements on the impl of `MyTrait` for `MyStruct<'_, '_, A, B, F>`
   = note: required because of the requirements on the impl of `MyTrait` for `MyStruct<'_, '_, A, B, F>`
   = note: required because of the requirements on the impl of `MyTrait` for `MyStruct<'_, '_, A, B, F>`
   = note: required because of the requirements on the impl of `MyTrait` for `MyStruct<'a, 'b, A, B, F>`

error[E0275]: overflow evaluating the requirement `MyStruct<'_, '_, A, B, F>: MyTrait`
  --> file7.rs:18:5
   |
18 |     type Output = B;
   |     ^^^^^^^^^^^^^^^^
   |
   = help: consider adding a `#![recursion_limit="8"]` attribute to your crate
   = note: required because of the requirements on the impl of `MyTrait` for `MyStruct<'_, '_, A, B, F>`
   = note: required because of the requirements on the impl of `MyTrait` for `MyStruct<'_, '_, A, B, F>`
   = note: required because of the requirements on the impl of `MyTrait` for `MyStruct<'_, '_, A, B, F>`
   = note: required because of the requirements on the impl of `MyTrait` for `MyStruct<'a, 'b, A, B, F>`

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0275`.

real	0m0.128s
user	0m0.042s
sys	0m0.039s
