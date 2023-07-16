 Rust
use std::ops::Shl;

struct S<'a, 'b: 'a>(Option<&'a &'b ()>);

impl<'a, 'b> Shl<&'b u32> for S<'a, 'b> {
    type Output = &'a u32;
    fn shl(self, t: &'b u32) -> &'a u32 { t }
}

fn return_dangling_pointer<'a>(s: S<'a, 'a>) -> &'a u32 {
    let s = s;
    s << &mut 3 /* avoid promotion */
}

fn main() {
    let a = return_dangling_pointer(S(None));
    println!("{}", a);
}
