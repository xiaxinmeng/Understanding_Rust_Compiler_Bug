
error[E0282]: type annotations needed
  --> /home/miswan/repo/trace32.rs/crates/core/ice-example/src/lib.rs:15:14
   |
15 |         self.g();
   |              ^
   |
help: try using a fully qualified path to specify the expected types
   |
15 |         <Y as X<T>>::g(self);
   |         +++++++++++++++    ~

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0282`.
