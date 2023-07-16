 rust
mod some_module {
    enum _Enum { A, B, _Dummy }
    pub type Enum = self::_Enum;
    pub use self::_Enum::{A, B};
}
fn to_string(e: some_module::Enum) -> String {
    match e {
        some_module::A => 'A'.to_string(),
        some_module::B => 'B'.to_string(),
        _ => "_not_supported".to_string()
    }
}
