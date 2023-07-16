
error: lifetime may not live long enough
  --> src/lib.rs:9:9
   |
7  |   impl<'a> Parser<'a> {
   |        -- lifetime `'a` defined here
8  |       fn parse(&mut self) -> Option<Packet<'a>> {
   |                - let's call the lifetime of this reference `'1`
9  | /         Some(Packet {
10 | |             buf: self.buf,
11 | |         })
   | |__________^ returning this value requires that `'1` must outlive `'a`
