rust
fn main() {
    let opt = Some(vec![2]);
    match opt {
        Some(ref inner) => println!("{:?}", inner),
        None => {},
    }
    println!("{:?}", opt); // no problem; the match only borrowed the contents
}
