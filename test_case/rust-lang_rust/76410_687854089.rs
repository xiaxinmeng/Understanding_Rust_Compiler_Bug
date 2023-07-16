rust
fn f1<F: FnMut(u32) -> u32, T: impl Iterator<Item=u32>>(a: u32) -> impl FnOnce(F) -> T {
    move |f: F| {
        (0..a).map(f)
    }
}

fn main() {
    let iter = f1(10)(|b| b * 2);
    for x in iter {
        println!("{:}", x);
    }
}
