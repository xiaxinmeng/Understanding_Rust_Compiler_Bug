
error[E0502]: cannot borrow `self.bytes` as mutable because it is also borrowed as immutable
  --> src/lib.rs:25:13
   |
20 |             if let Some(p) = Packet::new(&self.bytes[..]) {
   |                                           ---------- immutable borrow occurs here
...
25 |             self.bytes.truncate(0);
   |             ^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
   |
note: immutable borrowed value must be valid for the lifetime 'a as defined on the method body at 18:13...
  --> src/lib.rs:18:13
   |
18 |     fn poll<'a>(&'a mut self) -> Packet<'a> {
   |             ^^

error: aborting due to previous error
