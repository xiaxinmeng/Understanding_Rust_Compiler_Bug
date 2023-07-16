rust
unsafe fn clone_raw<T>(ptr: *const T) -> Rc< T > {
    let tmp = Rc::from_raw(ptr);
    let result = tmp.clone();
    Rc::into_raw(tmp);
    result
}
