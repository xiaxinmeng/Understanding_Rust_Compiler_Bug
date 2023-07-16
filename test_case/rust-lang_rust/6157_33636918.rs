
use std::io;
use std::fmt::Default;

fn echo<T: Default>(x: T) { io::println(format!("{}", x)); }

pub trait OpInt<'a> { fn call<'a>(&'a self, int, int) -> int; }

impl<'a> OpInt<'a> for 'a |int, int| -> int {
    fn call(&self, a:int, b:int) -> int {
        echo("I dont wanna die!");
        (*self)(a, b)
    }
}

fn squarei<'a>(x: int, op: &'a OpInt) -> int { op.call(x, x) }

fn muli(x:int, y:int) -> int {
    echo("You will never get here.");
    x * y
}


fn main() {
    echo("Entered main");
    let f = |x,y| muli(x,y);
    {
        let g = &f;
        let h = g as &OpInt;
        let r = squarei(3, h);
        echo(r);
    }
}
