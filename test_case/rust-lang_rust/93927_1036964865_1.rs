
error[[E0369]](https://doc.rust-lang.org/stable/error-index.html#E0369): binary operation `==` cannot be applied to type `MyType<T>`
  [--> src/main.rs:13:9
](https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=a69190abd7a2b258c588d8e994e69871#)   |
13 |     val == val
   |     --- ^^ --- MyType<T>
   |     |
   |     MyType<T>
   |
help: consider further restricting this bound
   |
12 | fn cond<T: PartialEq + std::cmp::PartialEq>(val: MyType<T>) -> bool {
   |                      +++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
