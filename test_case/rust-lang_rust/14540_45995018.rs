 rust
enum Value {
  A = 20,
  B = 21,
}

// Option<Result<Value, IoErrorKind>>
match maybe_result {
  // matches on {22, _}
  None => a,
  // matches on {18, 0}
  Some(Err(ShortWrite(0))) => b,
  // matches on {x, _} if 20 <= x && x < 22
  Some(Ok(_)) => c,
}
