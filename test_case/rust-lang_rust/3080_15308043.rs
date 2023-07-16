 rust
unsafe fn f(x:*int) -> int {
    let p = x as int;
    let q = p + 8; // danger: assumes sizeof(int*) in a vec is 8 !
    let y = q as *int;
    *y
}

trait x { unsafe fn uwith(&self, x:*int) -> int; }
impl x for () {
    unsafe fn uwith(&self, x:*int) -> int {  3+f(x) }
}

impl x for @() {
    unsafe fn uwith(&self, x:*int) -> int { 13+f(x) }
}

impl x for @int {
    unsafe fn uwith(&self, x:*int) -> int { 23+f(x) }
}

fn main() {
    let obju = ();
    let objo = @();
    let obji = @3;
    let vec = [0,7,1000];
    io::print(fmt!("%?\n", obju.uwith(&vec[0])));
    io::print(fmt!("%?\n", objo.uwith(&vec[0])));
    io::print(fmt!("%?\n", obji.uwith(&vec[0])));
}
