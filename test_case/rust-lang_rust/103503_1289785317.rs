plain
   Compiling hashbrown v0.12.3
   Compiling object v0.26.2
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling addr2line v0.16.0
error: use of deprecated constant `libc::MSG_CTRUNC`: recvmmsg call expects an unsigned type on musl/emscripten
   |
   |
46 |         ancillary.truncated = msg.msg_flags & libc::MSG_CTRUNC == libc::MSG_CTRUNC;
   |
   |
   = note: `-D deprecated` implied by `-D warnings`

error: use of deprecated constant `libc::MSG_CTRUNC`: recvmmsg call expects an unsigned type on musl/emscripten
   |
   |
46 |         ancillary.truncated = msg.msg_flags & libc::MSG_CTRUNC == libc::MSG_CTRUNC;


error: use of deprecated constant `libc::MSG_TRUNC`: recvmmsg call expects an unsigned type on musl/emscripten
   |
   |
48 |         let truncated = msg.msg_flags & libc::MSG_TRUNC == libc::MSG_TRUNC;


error: use of deprecated constant `libc::MSG_TRUNC`: recvmmsg call expects an unsigned type on musl/emscripten
   |
   |
48 |         let truncated = msg.msg_flags & libc::MSG_TRUNC == libc::MSG_TRUNC;


error: use of deprecated constant `libc::MSG_NOSIGNAL`: recvmmsg call expects an unsigned type on musl/emscripten
   |
   |
24 | use libc::MSG_NOSIGNAL;


error: use of deprecated constant `libc::MSG_NOSIGNAL`: recvmmsg call expects an unsigned type on musl/emscripten
    |
    |
507 |                 MSG_NOSIGNAL,


error: use of deprecated constant `libc::MSG_NOSIGNAL`: recvmmsg call expects an unsigned type on musl/emscripten
    |
    |
543 |                 MSG_NOSIGNAL,


error: use of deprecated constant `libc::MSG_PEEK`: recvmmsg call expects an unsigned type on musl/emscripten
    |
    |
962 |         self.recv_from_flags(buf, libc::MSG_PEEK)


error: use of deprecated constant `libc::MSG_PEEK`: recvmmsg call expects an unsigned type on musl/emscripten
   |
   |
13 | use libc::{c_int, c_void, size_t, sockaddr, socklen_t, MSG_PEEK};


error: use of deprecated constant `libc::MSG_PEEK`: recvmmsg call expects an unsigned type on musl/emscripten
    |
    |
255 |         self.recv_with_flags(buf, MSG_PEEK)


error: use of deprecated constant `libc::MSG_CMSG_CLOEXEC`: recvmmsg call expects an unsigned type on musl/emscripten
    |
    |
294 |         let n = cvt(unsafe { libc::recvmsg(self.as_raw_fd(), msg, libc::MSG_CMSG_CLOEXEC) })?;


error: use of deprecated constant `libc::MSG_PEEK`: recvmmsg call expects an unsigned type on musl/emscripten
    |
    |
299 |         self.recv_from_with_flags(buf, MSG_PEEK)


error: use of deprecated constant `libc::MSG_NOSIGNAL`: recvmmsg call expects an unsigned type on musl/emscripten
   |
   |
39 |         use libc::MSG_NOSIGNAL;


error: use of deprecated constant `libc::MSG_NOSIGNAL`: recvmmsg call expects an unsigned type on musl/emscripten
    |
    |
286 |             c::send(self.inner.as_raw(), buf.as_ptr() as *const c_void, len, MSG_NOSIGNAL)


error: use of deprecated constant `libc::MSG_NOSIGNAL`: recvmmsg call expects an unsigned type on musl/emscripten
    |
    |
548 |                 MSG_NOSIGNAL,


error: use of deprecated constant `libc::MSG_NOSIGNAL`: recvmmsg call expects an unsigned type on musl/emscripten
    |
    |
682 |             c::send(self.inner.as_raw(), buf.as_ptr() as *const c_void, len, MSG_NOSIGNAL)

error: could not compile `std` due to 16 previous errors
Build completed unsuccessfully in 0:00:19
