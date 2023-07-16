rust
type Ty = Box<&'static u8>;
trait Bad<'a> {
    fn f<'b>(x: &'static &'a Ty, y: &'b Ty) -> &'static Ty;
    fn g<'b>(y: &'b Ty) -> &'static Ty;
}

impl Bad<'static> for () {
    // NOTE that this signature does _not_ match the trait definition
    // (the first argument has different lifetimes)
    fn f<'b>(_: &'static &'b Ty, y: &'b Ty) -> &'static Ty {
        Box::leak::<'static>(Box::new(y))
    }
    fn g<'b>(y: &'b Ty) -> &'static Ty {
        let z: &_ = Box::leak(Box::new(Box::new(&0)));
        let z: &_ = Box::leak(Box::new(z));
        <Self as Bad<'static>>::f(z, y)
    }
}
fn h<'b>(y: &'b Ty) -> &'static Ty {
    <() as Bad<'static>>::g(y)
}

fn main() {
    let v = Box::new(&0);
    let s;
    {
        let r = &v;
        s = h(r);
    }
    drop(v);
    let _x = Box::new(0usize);
    println!("{}", s);
}
