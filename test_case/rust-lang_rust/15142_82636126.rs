
src/xserver.rs:193:27: 193:64 error: mismatched types:
 expected `libc::types::common::c95::c_void`,
    found `libc::types::common::c95::c_void`
(expected enum `libc::types::common::c95::c_void`,
    found a different enum `libc::types::common::c95::c_void`) [E0308]
src/xserver.rs:193                     data: transmute::<[c_long; 5],c_void>(data),
