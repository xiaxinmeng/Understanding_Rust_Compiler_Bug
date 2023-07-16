plain
   Compiling regex v1.5.4
   Compiling block-buffer v0.9.0
   Compiling digest v0.9.0
   Compiling measureme v10.0.0
error[E0277]: the trait bound `Box<std::io::Error>: std::error::Error` is not satisfied
   |
   |
29 |     s.write_all(file_magic).map_err(Box::new)?;
   |                                              ^ the trait `std::error::Error` is not implemented for `Box<std::io::Error>`
   |
   = note: required because of the requirements on the impl of `From<Box<std::io::Error>>` for `Box<dyn std::error::Error + Send + Sync>`
   = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, Box<std::io::Error>>>` for `Result<(), Box<dyn std::error::Error + Send + Sync>>`

error[E0277]: the trait bound `Box<std::io::Error>: std::error::Error` is not satisfied
   |
31 |         .map_err(Box::new)?;
31 |         .map_err(Box::new)?;
   |                           ^ the trait `std::error::Error` is not implemented for `Box<std::io::Error>`
   |
   = note: required because of the requirements on the impl of `From<Box<std::io::Error>>` for `Box<dyn std::error::Error + Send + Sync>`
   = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, Box<std::io::Error>>>` for `Result<(), Box<dyn std::error::Error + Send + Sync>>`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `measureme` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
