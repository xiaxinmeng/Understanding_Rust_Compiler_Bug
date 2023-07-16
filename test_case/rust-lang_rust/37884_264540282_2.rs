
error[E0308]: method not compatible with trait
 --> <anon>:5:5
  |
5 |     fn next(&'a mut self) -> Option<Self::Item> {
  |     ^ lifetime mismatch
  |
  = note: expected type `fn(&mut RepeatMut<'a, T>) -> std::option::Option<&mut T>`
  = note:    found type `fn(&'a mut RepeatMut<'a, T>) -> std::option::Option<&mut T>`
  = note: the anonymous lifetime #1 defined on unknown free region bounded by scope CodeExtent(7/CallSiteScope { fn_id: NodeId(28), body_id: NodeId(43) })...
  = note: ...does not necessarily outlive the lifetime 'a as defined on unknown free region bounded by scope CodeExtent(7/CallSiteScope { fn_id: NodeId(28), body_id: NodeId(43) })
