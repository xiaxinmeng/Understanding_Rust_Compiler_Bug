rust
trait Add<RHS = Self> {
    type Output;
    fn add(self, RHS) -> Self::Output;
}

struct S;
impl Add for S {
    type Output = ();
    fn add(self, _: S) -> () { unimplemented!() }
}

fn main() {
    fn f<T: Add>() {}
    f::<S>();
}
