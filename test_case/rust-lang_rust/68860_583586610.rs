rust
pub fn foo<D>(b: Vec<u8>) where D: Deserialize<'static> {
    let _ = serde_json::from_slice::<D>(b.as_ref());
}
