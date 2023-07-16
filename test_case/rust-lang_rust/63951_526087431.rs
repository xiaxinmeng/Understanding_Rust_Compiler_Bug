rust
extern {
    static foo: *const libc::c_void;
}
mod bar {
    extern {
        static foo: *const libc::c_void;
    }
}
