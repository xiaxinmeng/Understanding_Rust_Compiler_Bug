
error[E0308]: method not compatible with trait
  --> <anon>:8:5
   |
8  |       fn bar(n: &'a i32) -> Foo<'a> {
   |  _____^ starting here...
9  | |         Foo(n)
10 | |     }
   | |_____^ ...ending here: lifetime mismatch
   |
   = note: expected type `fn(&i32) -> Foo<'a>`
   = note:    found type `fn(&'a i32) -> Foo<'a>`
   = note: the anonymous lifetime #1 defined on unknown free region bounded by scope CodeExtent(15/CallSiteScope { fn_id: NodeId(27), body_id: NodeId(43) })...
   = note: ...does not necessarily outlive the lifetime 'a as defined on unknown free region bounded by scope CodeExtent(15/CallSiteScope { fn_id: NodeId(27), body_id: NodeId(43) })
help: consider using an explicit lifetime parameter as shown: fn bar(n: &'a i32) -> Foo<'a>
  --> <anon>:8:5
   |
8  |       fn bar(n: &'a i32) -> Foo<'a> {
   |  _____^ starting here...
9  | |         Foo(n)
10 | |     }
   | |_____^ ...ending here
