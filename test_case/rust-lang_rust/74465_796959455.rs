rust
// `value` is `Mutex<T>`.
let mut value = Lazy::new(|| value.lock().expect("Poisoned"))
match other_value {
  0 => *value += 1,
  2 => *value -= 1,
  _ => (),
}
