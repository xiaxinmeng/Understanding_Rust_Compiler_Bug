
error[E0599]: no method named `resume` found for type parameter `T` in the current scope
  --> src/main.rs:13:30
   |
13 |         match self.generator.resume() {
   |                              ^^^^^^ method not found in `T`
   |
   = help: items from traits can only be used if the type parameter is bounded by the trait
