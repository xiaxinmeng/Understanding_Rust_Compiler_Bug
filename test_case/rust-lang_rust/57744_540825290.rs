
error[E0277]: the size for values of type `dyn std::io::Write` cannot be known at compilation time
 --> src/lib.rs:4:13
  |
4 |     generic(w);
  |             ^ doesn't have a size known at compile-time
...
7 | fn generic<W: Write>(_w: &W) {
  |    ------- - required by this bound in `generic`
  |
  = help: the trait `std::marker::Sized` is not implemented for `dyn std::io::Write`
  = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
