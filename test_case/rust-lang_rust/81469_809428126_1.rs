rust
pub(crate) unsafe fn android_set_abort_message(payload: *mut &mut dyn BoxMeUp) {
    let mut v = Vec::<u8>::new();
    if v.try_reserve(1).is_err() {
        return;
    }
}
