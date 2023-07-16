 rust
fn main() {
    let mut x = box 5i;
    add_one(&mut x);
    println!("{}", x);
}

fn add_one(num: &mut Box<int>) {
    **num += 1;
}
