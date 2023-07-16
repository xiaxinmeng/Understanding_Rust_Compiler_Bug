 rust
#[allow(dead_code)]
#[repr(u32)]
enum SomeEnum {
    A = 0x0,
    B = 0x0,
}

#[allow(dead_code)]
fn a(b: SomeEnum) {
    match b {
        SomeEnum::A => (),
        SomeEnum::B => (),
    }
}

fn main() {}
