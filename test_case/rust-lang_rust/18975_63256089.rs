
extern crate libc;
use libc::HANDLE;

#[link(name = "user32")]
extern {
    pub fn GetDC(hdc: HANDLE) -> HANDLE;
}

fn main() {
    let hdc = unsafe { GetDC(std::mem::transmute(0i64)) };
    println!("{}", hdc);
}
