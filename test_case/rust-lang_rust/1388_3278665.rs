
use std;
import std::io;

fn foo(num: float) -> str {
    assert num < 0.0;
    ret "";
}

fn main() {
    let f = -0.550153;
    log(error, f);
    assert f < 0.0;
    let s = foo(f);
    io::println(s);
    io::println("before " + s + " after");
}
