 rust
#![crate_type="lib"]
#[inline(never)]
fn wrap<T>(f: T) -> Option<T> {
    Some(f)
}
pub fn get_i32(slot: &mut Option<i32>, def: i32) -> &mut i32 {
    if slot.is_none() {
        *slot = wrap(def);
        let x = box 32i;
    }
    slot.as_mut().unwrap()
}
