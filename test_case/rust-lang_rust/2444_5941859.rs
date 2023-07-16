
use std;
import std::arc;

enum e<T: const> { e(arc::arc<T>) }

resource r(x : e<int>) {
}

fn foo() -> r {fail;}

fn main() {
    let x = foo();
}
