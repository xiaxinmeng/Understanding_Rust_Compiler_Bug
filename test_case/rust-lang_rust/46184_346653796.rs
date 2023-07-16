
> cargo +nightly-2017-11-21 build
   Compiling net v0.1.0 (file:///home/philipp/Documents/net)
    Finished dev [unoptimized + debuginfo] target(s) in 0.90 secs

> cargo +nightly-2017-11-22 build
   Compiling net v0.1.0 (file:///home/philipp/Documents/net)
error[E0495]: cannot infer an appropriate lifetime for autoref due to conflicting requirements
   --> src/tcp.rs:215:27
    |
215 |         self.packet_queue.values()
    |                           ^^^^^^
    |
note: first, the lifetime cannot outlive the lifetime 'a as defined on the method body at 214:5...
   --> src/tcp.rs:214:5
    |
214 | /     pub fn packets<'a>(&'a mut self) -> impl Iterator<Item = &TcpPacket<Box<[u8]>>> {
215 | |         self.packet_queue.values()
216 | |     }
    | |_____^
note: ...so that reference does not outlive borrowed content
   --> src/tcp.rs:215:9
    |
215 |         self.packet_queue.values()
    |         ^^^^^^^^^^^^^^^^^
    = note: but, the lifetime must be valid for the static lifetime...
note: ...so that return value is valid for the call
   --> src/tcp.rs:214:41
    |
214 |     pub fn packets<'a>(&'a mut self) -> impl Iterator<Item = &TcpPacket<Box<[u8]>>> {
    |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
