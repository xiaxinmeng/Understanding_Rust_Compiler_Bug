rust
mod inner {
    pub struct BadStruct {
        _field: (),
    }
}

fn main() {
    let _ = inner::BadStruct();
}
