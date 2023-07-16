
error[E0271]: type mismatch resolving `<Foo2 as Bar2>::Ok == ()`
  --> file8.rs:13:5
   |
6  |     type Ok;
   |          -- associated type defined here
...
12 | impl Bar for Foo {
   | ---------------- in this `impl` item
13 |     type Ok = ();
   |     ^^^^^^^^^^^^^ expected u32, found ()
   |
   = note: expected type `u32`
              found type `()`
