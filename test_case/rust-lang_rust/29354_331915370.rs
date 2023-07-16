rust
struct CStr(u8);

impl !Sized for CStr {}

mem::size_of::<CStr>(); // compile error
mem::size_of::<&CStr>() == mem::size_of::<usize>();
