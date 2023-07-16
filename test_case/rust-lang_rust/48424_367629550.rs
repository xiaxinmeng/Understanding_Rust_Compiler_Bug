rust
fn main() {
    let v = vec!(true, false, true);
    let w = v.into_iter().all(|x|x); // why |x|x ?
    println!("{:?}", w);
}
