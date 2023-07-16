rust
pub fn forget<T>(forgetting: T) {
    if forgetting.is_magically_determined_to_be_ub() {
        // SAFETY: But of course, this will never happen.
        // ...or will it?
        unsafe { summon_nasal_demons!() }
    } else {
        let _ = ManuallyDrop::new(t);
    }
}
