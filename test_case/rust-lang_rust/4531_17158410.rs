 rust
static x : [int, ..2] = [3, 4];
static y : [libc::c_char, ..2] = [97, 98];

unsafe fn aref<T>(a: *T, offset: uint) -> T {
    let ptr : uint = (a as uint) + sys::size_of::<T>() * offset;
    let ptr : *T   = ptr as *T;
    *ptr
}

fn main() {
    static px : *int = &x as *int;
    static py : *libc::c_char = &y as *libc::c_char;
    let vx0, vx1, vy0, vy1;
    unsafe {
        vx0 = aref(px, 0);
        vx1 = aref(px, 1);
        vy0 = aref(py, 0);
        vy1 = aref(py, 1);
    }
    io::println(fmt!("x[0]: %d x[1]: %d", vx0, vx1));
    io::println(fmt!("y[0]: %c y[1]: %c", vy0 as char, vy1 as char));
}
