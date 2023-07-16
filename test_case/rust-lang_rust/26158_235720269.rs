
#![feature(slice_patterns)]
fn main() {
    let x: &[u32] = &[];
    let (a, b) = match x { &[[a, ref b..]..] => (a, b) };
    println!("{:08x}", a);
    for x in b.iter().take(30) {
        println!("{:08x}", x);
    }
}
