rust
fn main() {
        let mut vec = Vec::new();
        {
            let s = String::from("hello world");
            vec.push(&s);
        }
}
