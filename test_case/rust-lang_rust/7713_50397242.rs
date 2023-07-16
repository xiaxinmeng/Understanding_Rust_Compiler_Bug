 rust
fn main() {
    let v = {
        let s = "foo bar".to_string();
        s.as_slice().words().collect::<Vec<&str>>()
    };
    println!("{}", v.to_string());
}
