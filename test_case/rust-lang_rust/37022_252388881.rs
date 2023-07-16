 rust
pub struct Decoder {
    stack: Vec<Json>,
    str: Option<String>,
}

impl Decoder {
    fn read_str_slice(&mut self) -> DecodeResult<&str> {
        match self.pop() {
            Json::String(v) => {
                self.str = Some(v);
                Ok(self.str.as_ref().unwrap())
            }
            // ...
        }
    }
}
