
error[E0277]: `?` couldn't convert the error to `Report<GatewayError>`
  --> hartex-gateway\src\main.rs:36:34
   |
36 |     dotenv::dotenv().map_report()?;
   |                                  ^ the trait `From<Report<hartex_core::dotenv::Error>>` is not implemented for `Report<GatewayError>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the following other types implement trait `From<T>`:
             Report<Box<(dyn std::error::Error + 'a)>>
             Report<E>
   = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, Report<hartex_core::dotenv::Error>>>` for `Result<(), Report<GatewayError>>`
