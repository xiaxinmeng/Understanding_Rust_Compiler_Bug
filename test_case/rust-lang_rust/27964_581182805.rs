
error[E0277]: the size for values of type `dyn Reader` cannot be known at compilation time
 --> file.rs:8:15
  |
5 | fn check_vec<R:Reader>(r:&R){} //R needs to be Reader+?Sized for some reason, but new rust programmers wont know that.
  |    --------- -       - help: consider relaxing the implicit `Sized` restriction: `+  ?Sized`
  |              |
  |              required by this bound in `check_vec`
...
8 |     check_vec(&*reader_for());
  |               ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `dyn Reader`
  = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
