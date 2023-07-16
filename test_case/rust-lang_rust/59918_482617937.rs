rs
pub enum Enum {
    Variant { field: String },
}

pub fn frob(arg: &Enum) {
    match arg {
        Enum::Variant { ref field } => (),
    }
}
