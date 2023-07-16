
error[E0271]: type mismatch resolving `<Foo2 as Bar2>::Ok == ()`
  --> f23.rs:16:20
   |
5  |     type Sibling: Bar2<Ok=Self::Ok>;
   |                        ----------- required by this bound in `Bar::Sibling`
...
16 |     type Sibling = Foo2;
   |                    ^^^^ type mismatch resolving `<Foo2 as Bar2>::Ok == ()`
...
20 |     type Ok = u32; // <- `u32` instead of `()`
   |               --- expected this to be `()`
