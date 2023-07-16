rust
#![no_std]
use core::arch::wasm32::unreachable;
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    unsafe { unreachable() }
}
#[no_mangle]
fn main() {
    let b = c();
    if b != 8000000000000000000u128 {
        unsafe { unreachable() }
    }
}
pub fn c() -> u128 {
    let d = [0, 0, 32, 59, 157, 181, 5, 111, 0, 0, 0, 0, 0, 0, 0, 0];
    match e(&mut &d[..]) {
        Ok(f) => f,
        _ => unsafe { unreachable() },
    }
}
#[inline(never)]
fn e(g: &&[u8]) -> Result<u128, ()> {
    let mut buf = [0; 16];
    h(g, &mut buf).ok_or_else(|| ())?;
    Ok(u128::from_le_bytes(buf))
}
fn h(g: &&[u8], i: &mut [u8]) -> Option<()> {
    if i.len() != g.len() {
        return None;
    }
    i.copy_from_slice(g);
    Some(())
}
