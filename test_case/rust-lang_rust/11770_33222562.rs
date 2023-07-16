 rust
fn main() {
    let mut xs = [1, 2, 3];
    let mut ys = [4, 5, 6];
    xs.mut_iter().chain(ys.mut_iter()).reverse_();
    println!("{:?} {:?}", xs, ys);
}
