
macro_rules! int {
    ($num:expr) => ({
        asm!(concat!("int ", stringify!($num)) :::: "intel");
    });
}
