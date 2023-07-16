 Rust
struct S<'a, 'b: 'a>(Option<&'a &'b ()>);

impl<'a, 'b> S<'a, 'b> {
    fn xform(&self, a: &'b mut u32) -> &'a mut u32 {
        a
    }
}

fn return_dangling_pointer<'a>(s: S<'a, 'a>) -> &'a mut u32 {
    s.xform(&mut 3)
}

fn main() {
    let s = S(None);
    let a = return_dangling_pointer(s);
    println!("{}", a);
}
