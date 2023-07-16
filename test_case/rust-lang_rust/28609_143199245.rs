 Rust
use std::ops::Deref;

struct S<'a, 'b: 'a>(Option<&'a &'b ()>, &'b u32);

impl<'a, 'b> Deref for S<'a, 'b> {
    type Target = &'a u32;
    fn deref(&self) -> &&'a u32 {
        &self.1
    }
}

fn return_dangling_pointer<'a>(s: S<'a, 'a>) -> &'a u32 {
    let four = 4;
    let mut s = s;
    s.1 = &four;
    &s // or &*s
}

fn main() {
    let temp = &42;
    let ptr = return_dangling_pointer(S(None,&temp));
    println!("{}", ptr);
}
