
error[E0599]: no method named `pin_recv` found for type parameter `T` in the current scope
  --> src/lib.rs:17:18
   |
4  |     fn pin_recv(self: Pin<&mut Self>);
   |        --------       -------------- the method might not be found because of this arbitrary self type
   |        |
   |        the method is available for `Pin<&mut T>` here
...
17 |         self.num.pin_recv()
   |                  ^^^^^^^^ method not found in `T`
   |
help: consider wrapping the receiver expression with the appropriate type
   |
17 |         Pin::new(&mut self.num).pin_recv()
   |         +++++++++++++         +
