
error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`
  --> $DIR/bad-interconversion.rs:11:12
   |
LL | / fn option_to_result() -> Result<u64, String> {
LL | |     Some(3)?;
   | |            ^ use `.ok_or(...)?` to provide an error compatible with `Result<u64, String>`
LL | |
LL | |     Ok(10)
LL | | }
   | |_- this function returns a `Result`
   |
   = help: the trait `FromResidual<Option<Infallible>>` is not implemented for `Result<u64, String>`
   = note: required by `from_residual`
