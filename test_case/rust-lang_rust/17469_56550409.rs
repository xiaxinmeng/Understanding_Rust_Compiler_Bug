 rust
// compare:
opt.into_result()
opt.into_result(())

// compare:
opt.into_result().map_err(|_| MyError)
opt.into_result(MyError)
