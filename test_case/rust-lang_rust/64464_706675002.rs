rust
trait NumberAsBytes {
    fn as_bytes(&self) -> &[u8];
}

// fn as_bytes(&self) -> &[u8] { self.as_ne_bytes() }
impl_num_as_bytes!(i8, u8, /* ... */, i64, u64);
