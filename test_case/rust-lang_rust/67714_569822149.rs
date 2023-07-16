
fn main() {
    // SAFETY: because () is a ZST, we can coerce a &() to a &[(); N] array.
    let arr: &[(); isize::max_value() as usize + 1] = unsafe { &*(&() as *const _ as *const _) };
}
