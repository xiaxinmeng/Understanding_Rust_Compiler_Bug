text
   |
10 |     use crate::user::PubStruct;
   |                      ^^^^^^^^^ private struct import
   |
note: the struct import `PubStruct` is defined here...
  --> src/lib.rs:6:9
   |
6  |     use crate::definer::PubStruct;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...and refers to the struct `PubStruct` which is defined here
  --> src/lib.rs:2:5
   |
2  |     pub struct PubStruct;
   |     ^^^^^^^^^^^^^^^^^^^^^ consider importing it directly

