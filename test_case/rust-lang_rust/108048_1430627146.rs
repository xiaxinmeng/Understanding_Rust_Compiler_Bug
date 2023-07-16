
pub unsafe fn demo_std_iter(x: &mut Iter<'_, u32>) -> u32 {
    *x.next().unwrap_unchecked()
}
