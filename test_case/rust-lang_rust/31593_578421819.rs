
error[E0053]: method `badoowop` has an incompatible type for trait
 --> src/main.rs:9:26
  |
3 |     fn badoowop(self) -> Self::Shaboom;
  |                          ------------- type in trait
...
9 |     fn badoowop(self) -> Self {
  |                          ^^^^ expected associated type, found type parameter
  |
  = note: expected type `fn(T) -> <T as Doop>::Shaboom`
             found type `fn(T) -> T`
  = note: you might be missing a type parameter or trait bound
