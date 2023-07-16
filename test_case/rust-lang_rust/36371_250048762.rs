
note: missing trait item definition
  --> missing-impls.rs:10:5
   |
10 |     fn bar();
   |     ^^^^^^^^^ `bar` from trait
note: missing trait item definition
  --> missing-impls.rs:11:5
   |
11 |     fn    bay<
12 |        'lifetime,    TypeParameterA
13 |            >(  a   : usize,
14 |                b: u8  );
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `bay` from trait
