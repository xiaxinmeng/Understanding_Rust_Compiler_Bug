
error: lifetime may not live long enough
  --> src/lib.rs:24:9
   |
20 | impl<'slf, 'd: 'slf> AsSelf<'slf> for Foo<'d> {
   |      ----  -- lifetime `'d` defined here
   |      |
   |      lifetime `'slf` defined here
...
24 |         self
   |         ^^^^ associated function was supposed to return data with lifetime `'d` but it is returning data with lifetime `'slf`
   |
   = help: consider adding the following bound: `'slf: 'd`
   = note: requirement occurs because of the type `Foo<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `Foo<'a>` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
