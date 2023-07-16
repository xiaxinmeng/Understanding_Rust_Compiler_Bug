rust
enum DefaultEnum {
    A,
}

impl Default for DefaultEnum {
    fn default() -> Self {
        Default::default()
    }
}

fn main() {
    let default_enum: DefaultEnum = Default::default();
}
