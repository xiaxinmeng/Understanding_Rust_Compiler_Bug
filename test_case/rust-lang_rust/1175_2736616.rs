 rust
import std::unsafe;
use std;

tag test {
    a(@int);
}

fn main() unsafe {
    let val = a(@1);
    log_err alt val {
      a(i) { unsafe::reinterpret_cast::<@int, @int>(i) }
    };
}
