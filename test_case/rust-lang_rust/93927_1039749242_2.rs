
error[E0369]: binary operation `==` cannot be applied to type `MyType<T>`
  --> test.rs:13:9
   |
13 |     val == val
   |     --- ^^ --- MyType<T>
   |     |
   |     MyType<T>
   |
help: consider further restricting this bound
   |
12 | fn cond<T: PartialEq + std::cmp::Eq>(val: MyType<T>) -> bool {
   |                      ++++++++++++++
