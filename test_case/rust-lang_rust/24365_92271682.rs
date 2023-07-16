 rust
pub enum Attribute {
    Code {attr_name_idx: u16},
}

fn main() {
    let x = Attribute::Code {
        attr_name_idx: 42,
    };
    let z = (&x).attr_name_idx; // no ice
    let y = x.attr_name_idx; // ice
}
