rust
let last_element = match vec.last() {
  Some(last) => last,
  None => vec.push(0),
};
// vec is borrowed mutably so that further access
// to vec is impossible until last_element is no longer used
