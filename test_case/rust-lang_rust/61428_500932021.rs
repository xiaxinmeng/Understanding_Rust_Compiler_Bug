rust
let pthread: RawPthread = ...;
libc::foo(mem::transmute(pthread));
