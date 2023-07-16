rust
error[E0277]: the trait bound `std::option::NoneError: std::error::Error` is not satisfied
   --> src/main.rs:189:25
    |
189 |       let title: String = piece
    |  _________________________^
190 | |         .file_stem()?
    | |_____________________^ the trait `std::error::Error` is not implemented for `std::option::NoneError`
    |
    = note: required because of the requirements on the impl of `std::convert::From<std::option::NoneError>` for `SiteError`
    = note: required by `std::convert::From::from`
