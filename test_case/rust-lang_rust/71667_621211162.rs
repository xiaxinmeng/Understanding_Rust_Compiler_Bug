rust
use std::intrinsics::transmute;

#[repr(transparent)]
struct A(u8);

fn main() {
    assert_eq!(unsafe { transmute::<_, u8>(A(7)) }, 7);
}
