
let buf: [u8] = [];
vec::reserve(buf, len + 1);
vec::as_buf(buf) {|buf| sys::memcpy(buf, sbuf, len) };
vec::unsafe::set_len(buf, len);
buf += [0u8];
let s = unsafe::reinterpret_cast(buf);
unsafe::leak(buf);
ret s;
