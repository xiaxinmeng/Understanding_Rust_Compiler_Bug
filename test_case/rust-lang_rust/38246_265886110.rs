
rustc 1.15.0-nightly (daf8c1dfc 2016-12-05)
error[E0491]: in type `&'d &'c str`, reference has a longer lifetime than the data it references
 --> <anon>:8:5
  |
8 |     sit: &'d &'c str,
  |     ^^^^^^^^^^^^^^^^
  |
note: the pointer is valid for the lifetime 'd as defined on the struct at 7:0
 --> <anon>:7:1
  |
7 |   struct Dolor<'c, 'd> {
  |  _^
8 | |     sit: &'d &'c str,
9 | | }
  | |_^
note: but the referenced data is only valid for the lifetime 'c as defined on the struct at 7:0
 --> <anon>:7:1
  |
7 |   struct Dolor<'c, 'd> {
  |  _^
8 | |     sit: &'d &'c str,
9 | | }
  | |_^

error[E0491]: in type `&'a Dolor<'a, 'b>`, reference has a longer lifetime than the data it references
  --> <anon>:12:5
   |
12 |       type Bar = &'a Dolor<
   |  _____^
13 | |         'a,
14 | |         'b
15 | |     >;
   | |______^
   |
   = note: the pointer is valid for the lifetime 'a as defined on unknown free region bounded by scope CodeExtent(15/DestructionScope(NodeId(33)))
   = note: but the referenced data is only valid for the lifetime 'b as defined on unknown free region bounded by scope CodeExtent(15/DestructionScope(NodeId(33)))
