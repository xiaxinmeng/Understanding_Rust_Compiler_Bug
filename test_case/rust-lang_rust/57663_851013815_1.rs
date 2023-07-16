
error[E0271]: type mismatch resolving `<Foo2 as Bar2>::Ok == ()`
  --> src/lib.rs:16:5
   |
5  |     type Sibling: Bar2<Ok=Self::Ok>;
   |                        ----------- required by this bound in `Bar::Sibling`
...
16 |     type Sibling = Foo2;
   |     ^^^^^^^^^^^^^^^^^^^^ expected `()`, found `u32`
