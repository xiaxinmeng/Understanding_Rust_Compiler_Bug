
error[[E0277]](https://doc.rust-lang.org/nightly/error-index.html#E0277): the trait bound `String: Bar` is not satisfied
 --> src/main.rs:8:5
  |
8 |     foo(String::new(), String::new());
  |     ^^^ the trait `Bar` is not implemented for `String`
  |
note: required by a bound in `foo`
 --> src/main.rs:3:11
  |
3 | fn foo<T: Bar>(t: T, s: String) {}
  |           ^^^ required by this bound in `foo`

error[[E0277]](https://doc.rust-lang.org/nightly/error-index.html#E0277): the trait bound `String: Bar` is not satisfied
  --> src/main.rs:11:9
   |
11 |     bar(String::new(), 1u32);
   |     --- ^^^^^^^^^^^^^ the trait `Bar` is not implemented for `String`
   |     |
   |     required by a bound introduced by this call
   |
note: required by a bound in `bar`
  --> src/main.rs:5:11
   |
5  | fn bar<T: Bar>(t: T, u: u32) {}
   |           ^^^ required by this bound in `bar`
