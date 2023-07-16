rust
if vec.is_empty() {
  vec.push(0);
}
let last_element = vec.last().unwrap();
// vec is borrowed immutable so that further, immutable access
// to vec is still possible while last_element is used
