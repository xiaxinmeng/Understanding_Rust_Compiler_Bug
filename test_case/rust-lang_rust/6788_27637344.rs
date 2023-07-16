 rust
#[link(name = "min", vers = "0.1", uuid = "430a7e67-8e6a-4041-be05-643fabff74cf")];
#[feature(macro_rules)];
#[crate_type = "lib"];

use std::{libc, result, ptr, vec, str};
#[nolink]
#[link_args = "-framework OpenCL"]
#[cfg(target_os = "macos")]
extern { }

#[nolink]
#[link_args = "-lOpenCL"]
#[cfg(target_os = "linux")]
extern { }

#[nolink]
extern {
    fn clGetPlatformInfo(platform: *libc::c_void, name: u32, value_size: libc::size_t, value: *mut libc::c_void, value_size_ret: *mut libc::size_t) -> i32;
}

macro_rules! cl_call(($name:ident : $($arg:expr),+) => ({
    let error = unsafe { $name($($arg),+) };

    if (error != 0) {
        return result::Err(error);
    }
}))

macro_rules! cl_call_unknown_length(
    ($name:ident, $n:ty, $in_:ty, $out:ty <- $($arg:expr),*) => ({
            let mut n:$n = 0;

            cl_call!($name: $($arg),*, 0, ptr::mut_null(), ptr::to_mut_unsafe_ptr(&mut n));

            let mut result:~[$out] = vec::with_capacity(n as uint);

            cl_call!($name: $($arg),*, n, vec::raw::to_mut_ptr(result) as *mut $in_, ptr::mut_null());

            unsafe { vec::raw::set_len(&mut result, n as uint) };

            result
}))

#[fixed_stack_segment]
pub fn get_info(id: *libc::c_void, name: u32) -> result::Result<~str, i32> {
    return result::Ok(str::from_utf8_owned(
            cl_call_unknown_length!(clGetPlatformInfo,
                                    libc::size_t,
                                    libc::c_void,
                                    u8 <- id, name)));
}
