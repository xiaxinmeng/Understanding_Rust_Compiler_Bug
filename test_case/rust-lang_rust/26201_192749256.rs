
macro_rules! test {
    ($msg : expr) => {{
        extern "C" fn wrap() {
            println!("{}", $msg);
        }
        wrap
    }}
}
