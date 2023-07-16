
error[E0046]: not all trait items implemented, missing: `bar`, `bay`
  --> file.rs:27:1
   |
15 |       fn bar();
   |       --------- `bar` from trait
...
16 | ┌     fn    bay<'lifetime,
   | ╎           ^^^ showing off an error
17 | ╎                                      TypeParameterA
18 | ╎
19 | ╎ ┌             >(  a   : usize,
   | ╎ ╎                 ----------- error
20 | ╎ ╎                 b: u8  );
   | └╴┼╴╴╴╴^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ some error
   |   └╴╴╴╴╴╴╴╴╴╴╴╴╴---------------- overlapped errors to show off how it could look like

...    
27 |     impl X<usize> for A {
   |     ^ missing `bar`, `bay` in implementation
