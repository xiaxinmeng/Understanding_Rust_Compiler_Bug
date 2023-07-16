
unsafe fn uninitialized<T> () -> T {
    panic!("this used to be allowed but isn't anymore because ???")
}

unsafe fn zeroed<T> () -> T {
    panic!("this used to be allowed but isn't anymore because ???")
}
