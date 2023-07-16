 rust
#![crate_type = "rlib"]
mod e {
    mod builder {
        struct Serializer;

        impl Serializer {
            fn serialize_int(self, v:int) {
                write!(self, "{}", v)
            }
        }
    }
}
