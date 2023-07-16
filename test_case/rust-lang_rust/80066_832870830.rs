
error[E0275]: overflow evaluating the requirement `Test: Bar<Wrapper<&mut Test>>`
  --> f20.rs:20:5
   |
17 | fn assert<B: Foo>(b: B) {}
   |              --- required by this bound in `assert`
...
20 |     assert(Wrapper(&mut Test))
   |     ^^^^^^
   |
   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`f20`)
note: required because of the requirements on the impl of `Foo` for `Wrapper<&mut Test>`
  --> f20.rs:11:27
   |
11 | unsafe impl<S: Bar<Self>> Foo for Wrapper<&mut S> {
   |                           ^^^     ^^^^^^^^^^^^^^^
note: required because of the requirements on the impl of `Bar<Wrapper<&mut Test>>` for `Test`
  --> f20.rs:15:33
   |
15 | unsafe impl<B: Foo<Bar = Self>> Bar<B> for Test {}
   |                                 ^^^^^^     ^^^^
   = note: 127 redundant requirements hidden
   = note: required because of the requirements on the impl of `Foo` for `Wrapper<&mut Test>`
