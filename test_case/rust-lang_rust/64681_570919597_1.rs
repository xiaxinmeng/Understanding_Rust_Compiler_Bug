
error[E0308]: mismatched types
  --> src/lib.rs:16:27
   |
11 | pub fn add_1_foo<T>(y: T) -> Foo
   |                  - this type parameter
...
16 |     let foo_1 = Foo::from(1);
   |                           ^ expected type parameter `T`, found integer
   |
   = note: expected type parameter `T`
                        found type `{integer}`
   = help: type parameters must be constrained to match other types
   = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
