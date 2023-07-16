rust
fn check(opt: Option<u32>) -> u32 {
    opt.expect("This option should always be Some by the time this function is called.")
}
