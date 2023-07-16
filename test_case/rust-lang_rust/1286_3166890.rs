
use std;
import std::str;
obj pcre(_re: int) {
  fn match(target: str) {
    let re = _re;
    str::as_buf(target, { |_target|
        re
    });
  }
}
