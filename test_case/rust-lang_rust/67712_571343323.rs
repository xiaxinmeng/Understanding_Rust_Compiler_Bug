rust
#![feature(slice_patterns)]

#[derive(Clone, Debug)]
struct D(&'static str);

impl Drop for D {
    fn drop(&mut self) { println!("Dropping D({})", self.0); }
}

fn main() {
    let a = [D("a0"),D("a1"),D("a2"),D("a3"),D("a4"),D("a5"),D("a6"),D("a7")];
    println!("before matches");
    match a {
        [_, _, s2_5 @ .., _, _] => { println!("sub2_5 {:?}", s2_5); }
    }
    println!("between matches");
    match a {
        [_, _, _, _, _, _, s6_7 @ ..] => { println!("sub6_7 {:?}", s6_7); }
    }
    println!("after matches");
}
