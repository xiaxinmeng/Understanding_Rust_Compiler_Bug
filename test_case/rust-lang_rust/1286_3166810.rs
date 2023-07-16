
use std;
import std::str;
obj pcre(_re: int) {
  fn match(target: str) {
    _match(_re, target)
  }
}
fn _match(_re: int, target: str) {
  str::as_buf(target, { |_target|
    _re
  });
}
