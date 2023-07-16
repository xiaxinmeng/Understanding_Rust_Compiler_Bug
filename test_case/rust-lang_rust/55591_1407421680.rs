
error[E0277]: the trait bound `Vec<i32>: IntoProducer` is not satisfied
  --> src/main.rs:26:13
   |
26 |     dostuff(Vec::<i32>::new());
   |     ------- ^^^^^^^^^^^^^^^^^ the trait `IntoProducer` is not implemented for `Vec<i32>`
   |     |
   |     required by a bound introduced by this call
   |
note: required for `Vec<i32>` to implement `IntoProducer`
  --> src/main.rs:5:19
   |
5  | impl<P: Producer> IntoProducer for P {
   |         --------  ^^^^^^^^^^^^     ^
   |         |
   |         unsatisfied trait bound introduced here
note: required by a bound in `dostuff`
  --> src/main.rs:19:9
   |
17 | fn dostuff<IP, P>(arg: IP)
   |    ------- required by a bound in this
18 | where
19 |     IP: IntoProducer<Output = P>,
   |         ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `dostuff`
help: consider borrowing here
   |
26 |     dostuff(&Vec::<i32>::new());
   |             +

For more information about this error, try `rustc --explain E0277`.
