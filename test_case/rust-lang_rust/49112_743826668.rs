rust
.map(|_| {
  if unsafe { js_sys::Math::random() < 0.5 } {
    Cell::Alive
  } else {
    Cell::Dead
  }
})
.collect();
