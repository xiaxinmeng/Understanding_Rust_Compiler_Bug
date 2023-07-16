rust
#[derive(Debug)]
struct Wrapper<T: ?Sized>(T);

fn main () {
    let mut w = Wrapper("A");
    let p: *const _ = &w as *const _;
    println!("{:?}", unsafe { &*p});
    drop(w);
    println!("{:?}", unsafe { &*p});
}
