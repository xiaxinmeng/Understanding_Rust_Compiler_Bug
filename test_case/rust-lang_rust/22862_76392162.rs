 rust
use libc;
use std::mem;

#[no_mangle]
pub extern fn rw_file_raw(path: *const libc::c_char) -> libc::c_int {
    extern {
        fn open(path: *const libc::c_char, flags: libc::c_int, mode: libc::mode_t) -> libc::c_int;
    }

    unsafe {
        let x = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
        let fd = open(path,
                        libc::O_RDWR | libc::O_TRUNC | libc::O_CREAT,
                        libc::S_IRUSR | libc::S_IWUSR);

        if fd < 0 {
            return fd;
        }

        let txt = "HelloWorld";
        libc::write(fd, mem::transmute(txt.as_ptr()), txt.len() as libc::size_t);
        libc::write(fd, mem::transmute(x.as_ptr()), x.len() as libc::size_t);
        libc::close(fd);
    }
    return 0;
}


#[no_mangle]
pub extern fn rw_file_raw_vararg(path: *const libc::c_char) -> libc::c_int {
    extern {
        fn open(path: *const libc::c_char, flags: libc::c_int, ...) -> libc::c_int;
    }

    unsafe {
        let x = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
        let fd = open(path,
                      libc::O_RDWR | libc::O_TRUNC | libc::O_CREAT,
                      (libc::S_IRUSR | libc::S_IWUSR) as libc::c_uint);

        if fd < 0 {
            return fd;
        }

        let txt = "HelloWorld";
        libc::write(fd, mem::transmute(txt.as_ptr()), txt.len() as libc::size_t);
        libc::write(fd, mem::transmute(x.as_ptr()), x.len() as libc::size_t);
        libc::close(fd);
    }
    return 0;
}
