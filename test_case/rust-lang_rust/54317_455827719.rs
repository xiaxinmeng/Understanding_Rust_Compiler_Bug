
#[macro_export]
macro_rules! dbg {
    ($expr:expr) => (dbg!("{:#?}", $expr));
    ($fmt:expr, $expr:expr$(, $opt:expr)*) => {
        match $expr {
            expr => {
                eprintln!(concat!("[{}:{}] {} = ", $fmt), file!(), line!(), stringify!($expr), &expr$(, $opt)*);
                expr
            }
        }
    }
}
