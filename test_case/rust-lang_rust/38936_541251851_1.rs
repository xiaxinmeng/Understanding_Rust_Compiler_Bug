
error[E0277]: the size for values of type `str` cannot be known at compilation time
  --> src/main.rs:23:35
   |
13 | fn train_length<T:Train>(text: &'static T) -> usize {
   |    ------------ -      - help: relax the implicit restriction:  `+ ?Sized`
   |                 |
   |                 `Sized` implicitly required in this bound in `train_length`
...
23 |     println!("{:?}", train_length("asdfas")); // compile error
   |                                   ^^^^^^^^ doesn't have a size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `str`
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
