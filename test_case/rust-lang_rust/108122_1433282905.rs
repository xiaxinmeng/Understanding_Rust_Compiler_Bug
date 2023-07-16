rust
let origin: *const P2 = ::std::ptr::null();
// ...
unsafe { &((*origin).0) as *const _ as _ }
