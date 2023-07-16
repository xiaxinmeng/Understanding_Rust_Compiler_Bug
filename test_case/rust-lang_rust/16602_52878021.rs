 rust
#[deriving(Show)]
enum Foo {
    Bar(uint, uint),
    Baz(&'static uint, &'static uint)
}

static num: uint = 100;

fn main () {
    let mut b = Baz(&num, &num);
    b = Bar(f(&b), g(&b));
    println!("main: {}", b);
}

static fnum: uint = 1;

fn f (b: &Foo) -> uint {
    println!("f   : {}", b);
    fnum
}

static gnum: uint = 2;

fn g (b: &Foo) -> uint {
    println!("g   : {}", b);
    gnum
}
