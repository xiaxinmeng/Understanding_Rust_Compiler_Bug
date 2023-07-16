
error[E0308]: mismatched types
 --> <anon>:2:5
  |
2 |     assert_eq!(10u64, 10usize);
  |     ------------------^^^^^^^- in this macro call
  |                       |
  |                       expected u64, found usize
  |
  = note: this error originates in a macro outside of the current crate
