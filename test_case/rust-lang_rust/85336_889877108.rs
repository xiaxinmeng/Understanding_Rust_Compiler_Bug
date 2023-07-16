rust
struct Ex;

impl Ex {
    const F: &'static u8 = unsafe { std::mem::transmute::<_,_>(0xbeefbeef as *const u8) };
}

fn main() {
    let f = Ex::F;
}
