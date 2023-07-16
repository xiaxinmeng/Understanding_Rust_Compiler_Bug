
error[E0599]: no method named `resume` found for type parameter `T` in the current scope
   --> src/main.rs:13:30
    |
13  |         match self.generator.resume() {
    |                              ^^^^^^ method not found in `T`
    |
help: consider wrapping the receiver expression with the appropriate type
    |
13  |         match Pin::new(&mut self.generator).resume() {
    |               +++++++++++++               +
