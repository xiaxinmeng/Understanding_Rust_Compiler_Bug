
#![no_std]

#![feature(thread_local)]

pub struct BB;

#[thread_local]
static mut KEY: Key = Key {
    inner: BB,
    dtor_running: false,
};

pub unsafe fn set() -> Option<&'static BB> {
    if KEY.dtor_running {
        return None
    }
    Some(&KEY.inner)
}

pub struct Key {
    inner: BB,
    dtor_running: bool,
}
