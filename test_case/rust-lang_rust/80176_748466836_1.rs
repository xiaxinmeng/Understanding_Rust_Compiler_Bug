rust
static UNIT: &'static &'static () = &&();
static ZERO: &'static u8 = &0;

type Ty = Box<&'static u8>;
trait Bad {
    fn f<'a>(_: &'static &'static (), y: &'a Ty) -> &'static Ty;
}

impl Bad for () {
    // NOTE that this signature does _not_ match the trait definition
    // (the first argument has different lifetimes)
    fn f<'a>(_: &'static &'a (), y: &'a Ty) -> &'static Ty {
        y
    }
}

fn main() {
    let v = Box::new(ZERO);
    let s;
    {
        let r = &v;
        s = <()>::f(UNIT, r);
    }
    drop(v);
    let _x = Box::new(0usize);
    println!("{}", s);
}
