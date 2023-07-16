
error[E0277]: `?` couldn't convert the error to `&str`
 --> src/main.rs:2:41
  |
1 | fn foo(x: Option<i32>) -> Result<(), &'static str> {
  |                           ------------------------ expected `&str` because of this
2 |     let _ = x.ok_or_else(|| Err("nope"))?;
  |                                         ^ the trait `From<Result<_, &str>>` is not implemented for `&str`
  |
  = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
  = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, Result<_, &str>>>` for `Result<(), &str>`
  = note: required by `from_residual`
