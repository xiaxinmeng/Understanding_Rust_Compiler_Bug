rust
#[rustc_args_required_const(0, 1)]
fn x(a: u8, b:u8) {} // won't error as 0 refers to `a`, and 1 refers to `b`

#[rustc_args_required_const(0, 1)]
fn y(a: u8) {} // will error as 1 refers to nonexistent parameter, i.e. parameter 1 doesn't exist (only parameter 0 exists)
