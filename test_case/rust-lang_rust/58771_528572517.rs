
error: lifetime may not live long enough
  --> src/lib.rs:24:9
   |
20 | impl<'slf, 'd: 'slf> AsSelf<'slf> for Foo<'d> {
   |      ----  -- lifetime `'d` defined here
   |      |
   |      lifetime `'slf` defined here
...
24 |         self
   |         ^^^^ returning this value requires that `'slf` must outlive `'d`
