 rust
match err {
    ApiError(_) => unimplemented!(),
    json::Error(_) => unimplemented!(),
    IoError(_) => unimplemented!(),
}
