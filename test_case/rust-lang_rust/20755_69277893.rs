 rust
#[inline(never)]
fn get_bool(slot: &mut Option<bool>, val: bool) -> &mut bool {
    if slot.is_none() {
        *slot = Some(x);
    }
    slot.as_mut().unwrap()
}
