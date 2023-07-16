 rust
#![feature(test)]
#![allow(warnings)]
extern crate test;

use std::sync::Mutex;

#[bench]
fn pthreads(b: &mut test::Bencher) {
    let mut buf = [0u8; 1024];
    enum pthread_mutex_t {}
    enum pthread_attr_t {}
    extern {
        fn pthread_mutex_init(lock: *mut pthread_mutex_t,
                              attr: *mut pthread_attr_t) -> i32;
        fn pthread_mutex_lock(lock: *mut pthread_mutex_t) -> i32;
        fn pthread_mutex_unlock(lock: *mut pthread_mutex_t) -> i32;
    }

    unsafe {
        let ptr = buf.as_ptr() as *mut pthread_mutex_t;
        assert_eq!(pthread_mutex_init(ptr, 0 as *mut _), 0);
        b.iter(|| {
            assert_eq!(pthread_mutex_lock(ptr), 0);
            assert_eq!(pthread_mutex_unlock(ptr), 0);
        });
    }
}

#[bench]
fn libstd(b: &mut test::Bencher) {
    let m = Mutex::new(());
    b.iter(|| drop(m.lock()));
}
