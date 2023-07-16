 rust
pub struct UTF8Encoding;
pub const UTF_8: &'static UTF8Encoding = &UTF8Encoding;
pub trait Encoding {}
impl Encoding for UTF8Encoding {}
pub fn f() -> &'static Encoding { UTF_8 as &'static Encoding }
