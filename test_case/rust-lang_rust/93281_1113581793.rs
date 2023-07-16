rust
#[derive(Debug)]
struct S(u64);

fn main() {
    let u: Vec<_> = vec![1, 2].iter().map(|x| (x, S(*x))).collect();
    for (x, &y) in &u {
        println!("{} {:?}", x, y);
    }
}
