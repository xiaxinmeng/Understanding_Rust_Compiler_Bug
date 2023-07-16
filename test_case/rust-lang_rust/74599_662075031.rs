rust
#![feature(rustc_attrs)]

#[rustc_args_required_const(0)]
pub fn print_raw(_: *const u8) {}

fn interpret() {
    match true {
        true => {
            [()].iter().filter(|_| true);
        }
        false => {
            print_raw("".as_ptr());
        }
        _ => {}
    };
}
