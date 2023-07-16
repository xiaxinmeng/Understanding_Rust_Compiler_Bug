 java
// test re-export works across several crates

// file:foo.rs
// compile-flags:--lib
use std;
mod o {
  fn test() { std::io::println("foo::test"); }
}

// file:bar.rs
// compile-flags:--lib
use foo;
import foo::o::test;
export test;

// file:baz.rs
// compile-flags:--lib
use bar;
import tz = bar::test;
export tz;

// file:main.rs
// compile-flags:-L .
use baz;
fn main() {
  baz::tz();
}

// run:./main
