rust
trait Tr { type Ty; }
struct S;
impl Tr for &S { type Ty = i32; }

fn main() {
    let f: fn(<&S as Tr>::Ty) = |i: i32| ();
}
