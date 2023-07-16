 rust
extern "fastcall" fn inner_call(a: u32, b: u8, c: u16) -> u32 {
    a + b as u32 + c as u32
}

#[link(name="outer")]
extern "fastcall"
{
    fn outer_call(a: u32, b: u8, c: u16) -> u32;
}

fn main() {
    let r1 = inner_call(1,2,3);
    let r2 = unsafe { outer_call(1,2,3) };
    println!("fcall: {} and {}", r1, r2);
}
