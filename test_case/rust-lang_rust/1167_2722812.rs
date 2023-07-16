
mod c_vec {
    import ptr;

    type t<T> = {
        base: *T;
        len: uint;
    };

    unsafe create(base: *T, len: uint) -> t<T> {
        ret { base: base, len: len };
    }

    fn get<T>(v: t<T>, idx: uint) -> T unsafe {
        assert idx < v.len;
        let ptr = ptr::offset(v.base, idx)
        ret *ptr;
    }

    fn set<T>(v: t<T>, idx: uint, value: T) unsafe {
        assert idx < v.len;
        let ptr = ptr::offset(v.base, idx)
        *ptr = value;
    }
}
