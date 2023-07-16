
assign.rs:32:19: 32:20 error: `*b` does not live long enough
assign.rs:32     let bb: & B = b;
                               ^
assign.rs:30:23: 34:2 note: reference must be valid for the anonymous lifetime #1 defined on the block at 30:22...
assign.rs:30 fn make_make_a() -> A {
assign.rs:31     let b: Box<B> = box B {i:1};
assign.rs:32     let bb: & B = b;
assign.rs:33     make_a(bb)
assign.rs:34 }
assign.rs:30:23: 34:2 note: ...but borrowed value is only valid for the block at 30:22
assign.rs:30 fn make_make_a() -> A {
assign.rs:31     let b: Box<B> = box B {i:1};
assign.rs:32     let bb: & B = b;
assign.rs:33     make_a(bb)
assign.rs:34 }
