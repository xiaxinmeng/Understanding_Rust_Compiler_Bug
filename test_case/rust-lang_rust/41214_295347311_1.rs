rust
error[E0072]: recursive type `ListNode` has infinite size
 --> file2.rs:1:1
  |
1 |   struct
  |  _^ starting here...
2 | | ListNode
3 | | {
... |
9 | | }
  | |_^ ...ending here: recursive type has infinite size
