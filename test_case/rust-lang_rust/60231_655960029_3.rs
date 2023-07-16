rust
static HI: [u8; 2] = [b'h', b'i'];

#[export_name = "main"]
pub unsafe fn main() {
    let x = 5u8;
    foo(&x as *const u8);
    foo(HI.as_ptr());
}
