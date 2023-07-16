
use std;
import std::io;

fn foo(x: float) -> str {
    log(error, x);
    assert x < 0.0;
    ret "";
}

fn main() {
    let f = -0.550153;
    log(error, f);
    assert f < 0.0;
    let s = foo(f);
    io::println(s);
    //io::println("before " + s + " after");
}
