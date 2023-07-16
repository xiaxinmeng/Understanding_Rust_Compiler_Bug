
#[inline]
#[cfg_attr(test, assert_instr(cmpxchg16b, success = Ordering::SeqCst, failure = Ordering::SeqCst))]
#[target_feature(enable = "cmpxchg16b")]
pub unsafe fn cmpxchg16b(
    ...
