

error[E0308]: mismatched types
 --> file.rs:7:15
  |
4 | fn send_email<R>(req: &mut Request) -> Result<Response<R>, String>
  |               - this type parameter
...
7 |   Ok(Response(42))
  |               ^^ expected type parameter `R`, found integer
  |
  = note: expected type parameter `R`
                       found type `{integer}`
  = help: type parameters must be constrained to match other types
  = note: for more information on traits as parameters, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
  = note: for more information on `impl Trait`, visit <URL>
help: because type parameter `R` is only used in the return, you can instead use `impl Trait`:
  |
4 | fn send_email(req: &mut Request) -> Result<Response<impl std::fmt::Display>, String>
  |             --                                      ^^^^^^^^^^^^^^^^^^^^^^
