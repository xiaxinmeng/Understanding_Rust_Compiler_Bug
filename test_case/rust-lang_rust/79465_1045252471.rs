
error[E0220]: associated type `A` not found for `T`
 --> out.rs:3:20
  |
3 | pub fn f1<T>(x: T::A) {}
  |                    ^ associated type `A` not found
