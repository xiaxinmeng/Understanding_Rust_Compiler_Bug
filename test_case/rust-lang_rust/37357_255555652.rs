 rust
macro_rules! eno {
    ($expr:expr) => (match $expr {
        $crate::std::result::Result::Ok(val) => (),
        $crate::std::result::Result::Err(_) => (),
    })
}
eno!(Ok(()));
