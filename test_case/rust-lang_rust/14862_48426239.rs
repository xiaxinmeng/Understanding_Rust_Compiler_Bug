 rust
static mut FOO: uint = 3;
static mut LOCK: NativeMutex = NATIVE_MUTEX_INIT;

fn write() {
    unsafe {
        let _l = LOCK.lock();
        FOO = 4;
    }
}


fn read() -> uint {
    unsafe {
        let _l = LOCK.lock();
        FOO
    }
}
