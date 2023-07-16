
error[E0599]: no method named `poll_next` found for type parameter `T` in the current scope
  --> src/lib.rs:14:23
   |
14 |         self.upstream.poll_next()
   |                       ^^^^^^^^^ method not found in `T`
   |
   = help: items from traits can only be used if the type parameter is bounded by the trait
