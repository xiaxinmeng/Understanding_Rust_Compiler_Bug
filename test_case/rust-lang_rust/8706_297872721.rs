
warning: unused variable: `s`
 --> test.rs:2:9
  |
2 |     let s = "ZͨA͑ͦ͒͋ͤ͑̚L̄͑͋Ĝͨͥ̿͒̽̈́Oͥ͛ͭ!̏"; while true { break; }
  |         ^
  |
  = note: #[warn(unused_variables)] on by default

warning: denote infinite loops with loop { ... }
 --> test.rs:2:45
  |
2 |     let s = "ZͨA͑ͦ͒͋ͤ͑̚L̄͑͋Ĝͨͥ̿͒̽̈́Oͥ͛ͭ!̏"; while true { break; }
  |                                             ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(while_true)] on by default
