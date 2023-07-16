 rust
fn main() {
    for a in "ababa".splitn(2, 'b') {
        println!("{}", a);
    }
    for a in [1, 2, 1, 2, 1].splitn(2, |x| *x == 2) {
        println!("{:?}", a);
    }
}
