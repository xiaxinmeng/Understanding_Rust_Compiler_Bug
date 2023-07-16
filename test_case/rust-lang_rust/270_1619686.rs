
use std;
import std::uint;

fn foo() {
  for each (uint i in uint::range(0u, 10u)) { 
    if (i > 5u) {
      ret;
    }
  }
  fail "Shouldn't have reached this point";
}

fn main() {
  foo();
  log_err "Okay!";
}
