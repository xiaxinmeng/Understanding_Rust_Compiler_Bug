rust
// Only breaks at opt-level=2 or higher.
#![crate_type="lib"]

pub fn check_overlap(ext: &i32) -> usize {
    let src_usize = &1 as *const _ as usize;
    let dst_usize = ext as *const _ as usize;
    let mut diff = src_usize.wrapping_sub(dst_usize);
    if src_usize > dst_usize {
        diff = diff.wrapping_neg();
    };

    diff
}
