
macro_rules! print {
    //$crate::macros::alloc::
    ($($arg:tt)*) => ($crate::print($crate::alloc::format!($($arg)*).as_str()));
}
