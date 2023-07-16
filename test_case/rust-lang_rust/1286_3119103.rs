 java
use std;
import std::str;
obj pcre(_re: int) {
  fn match(target: str) {
    str::as_buf(target, { |_target|
        _re
    });
  }
}
