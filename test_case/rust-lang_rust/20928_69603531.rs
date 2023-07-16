 rust
let mut name = [0 as u8; 17];
let res = prctl(PrctlOption::PR_GET_NAME as c_int, name.as_mut_ptr() as c_ulong, 0, 0, 0);
(res, ffi::c_str_to_bytes(name.as_ptr()).to_vec())
