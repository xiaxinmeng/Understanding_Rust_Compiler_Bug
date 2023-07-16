
error[[E0277]](https://doc.rust-lang.org/stable/error-index.html#E0277): the trait bound `Option<&str>: From<&String>` is not satisfied
  --> src/main.rs:19:13
   |
19 |     print_a(&String::from("123"));
   |     ------- ^^^^^^^^^^^^^^^^^^^^
   |     |       |
   |     |       the trait `From<&String>` is not implemented for `Option<&str>`
   |     |       help: consider dereferencing here: `&*String::from("123")`
   |     required by a bound introduced by this call
   |
   = note: required because of the requirements on the impl of `Into<Option<&str>>` for `&String`
note: required by a bound in `print_a`
  --> src/main.rs:1:24
   |
1  | fn print_a<'a>(s: impl Into<Option<&'a str>>) {
   |                        ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `print_a`
