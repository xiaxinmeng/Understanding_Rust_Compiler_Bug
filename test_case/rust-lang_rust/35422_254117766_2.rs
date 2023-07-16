
macro_rules! int {
    ($x:expr) => ({
        asm!(concat!("int ", $x) :::: "intel");
    });
}
