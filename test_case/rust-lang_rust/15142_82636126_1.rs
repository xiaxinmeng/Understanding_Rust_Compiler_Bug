
src/xserver.rs:196:74: 196:100 error: mismatched types:
 expected `*mut libc::types::common::c95::c_void`,
    found `*mut libc::types::common::c95::c_void`
(expected enum `libc::types::common::c95::c_void`,
    found a different enum `libc::types::common::c95::c_void`) [E0308]
src/xserver.rs:196                 XSendEvent(self.display, wind, 0 /*false*/, NoEventMask, (mess_ptr as *mut c_void));
