
unsafe fn x() {}

unsafe fn z() {
    fn y() {
        unsafe { x() }
    }
    y()
}

fn main() {
    unsafe { z() }
}
