
error[[E0277]](https://doc.rust-lang.org/stable/error_codes/E0277.html): the trait bound `for<'a> &'a <C as Container>::Item: From<&'a Wrapper>` is not satisfied
  --> src/main.rs:30:11
   |
30 | fn foo<C: Container>() {}
   |           ^^^^^^^^^ the trait `for<'a> From<&'a Wrapper>` is not implemented for `&'a <C as Container>::Item`
   |
   = note: required for `&'a Wrapper` to implement `for<'a> Into<&'a <C as Container>::Item>`
   = note: required for `&'a <C as Container>::Item` to implement `for<'a> TryFrom<&'a Wrapper>`
note: required by a bound in `Container`
