
#[inline(never)]
pub fn do_bug() {
    THREAD_LOCAL_GLOBAL.with(set_state_func);
}
