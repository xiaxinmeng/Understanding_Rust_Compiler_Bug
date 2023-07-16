rust
  match mutex.lock() {
      Ok(ref data) => Some(data.0),
      _ => None,
  }.map(|data| {
      data + 1
  })
  