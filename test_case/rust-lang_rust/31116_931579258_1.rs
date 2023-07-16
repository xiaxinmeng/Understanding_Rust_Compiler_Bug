rust
fn check(opt: Option<u32>) -> u32 {
    match opt {
        Some(val) => return val,
        None => panic!("This option should always be Some by the time this function is called.")
    }
}
