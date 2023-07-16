
macro_rules! test {
    ($msg : expr) => {{
        extern "C" fn wrap<T>() {
            println!("{}", $msg);
        }
        wrap::<()>
    }}
}
