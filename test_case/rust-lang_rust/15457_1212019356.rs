rust
struct Type {
    option: Option<Vec<u8>>
}

impl Type {
    fn method(&self) -> Option<Vec<u8>> {
        self.option.as_ref().map(|x| x)
    }
}

fn main() {
    let _ = Type { option: None }.method();
}
