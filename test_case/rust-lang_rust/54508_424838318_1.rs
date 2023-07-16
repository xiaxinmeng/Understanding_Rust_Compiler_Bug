rust
impl PartialEq for fn<*>(*) -> * {
     fn eq(&self: other: &Self) -> bool { self as usize == other as usize }
}
