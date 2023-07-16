
pub unsafe fn cleanup() {
    let _guard = LOCK.lock();
    ARGC = 0;
    ARGV = ptr::null();
}
