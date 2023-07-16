
error[E0308]: mismatched types
  --> file12.rs:15:19
   |
9  | impl<A> Default for Config<A>
   |      - this type parameter
...
15 |             addr: "127.0.0.1:1883",
   |                   ^^^^^^^^^^^^^^^^ expected type parameter `A`, found `&str`
   |
   = note: expected type parameter `A`
                   found reference `&'static str`
   = help: type parameters must be constrained to match other types
   = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
