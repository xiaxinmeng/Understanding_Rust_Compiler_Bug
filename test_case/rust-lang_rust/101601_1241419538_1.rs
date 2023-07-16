
error: lifetime may not live long enough
  --> src/lib.rs:15:9
   |
10 | impl<'a, 'b, T> Type<'a, 'b, T>
   |      --  -- lifetime `'b` defined here
   |      |
   |      lifetime `'a` defined here
...
15 |         Self::bar(param);
   |         ^^^^^^^^^ requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
