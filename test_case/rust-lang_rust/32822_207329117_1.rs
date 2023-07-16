 rust
fn main() {
    let v = vec!["abc".to_string(), "def".to_string()];

    let s: &str = "def";
    println!("{:?}", v.binary_search_by(|x| x.as_str().cmp(s)));
}
