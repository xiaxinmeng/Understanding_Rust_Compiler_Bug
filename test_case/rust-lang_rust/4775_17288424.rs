 rust
fn echo<T>(x:T) { io::println(fmt!("%?", x)); }

pub trait OpInt {
    fn call(&self, int, int) -> int;
}

impl<'self> OpInt for &'self fn(int,int) -> int {
    fn call(&self, a:int, b:int) -> int {
        echo("I dont wanna die!");
        (*self)(a,b)
    }
}

fn squarei(x:int, op:&OpInt) -> int { op.call(x, x) }

fn muli(x:int, y:int) -> int {
    echo("You will never get here.");
    x * y
}


fn main() {
    echo("Entered main");
    let f: &fn(int, int) -> int = |x, y| muli(x, y);
    let r = squarei(3, &f as &OpInt);
    echo(r);
}
