 rust
#![feature(core_intrinsics)]
struct ConnWrap(Conn);
impl ::std::ops::Deref for ConnWrap {
    type Target=Conn;
    fn deref(&self) -> &Conn { &self.0 }
}

struct Conn;
impl Drop for  Conn {
    fn drop(&mut self) { unsafe { ::std::intrinsics::abort() } }
}

fn main() {
    let conn = &*match Some(ConnWrap(Conn)) {
        Some(val) => val,
        None => return,
    };
    return
}
