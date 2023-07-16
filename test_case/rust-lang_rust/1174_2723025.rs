
use std;

native "cdecl" mod libc = "" {
    fn printf(s: *u8, a: int); /* A tenuous definition, indeed.  Herp, derp. */
}

fn printf(s: *u8, a: int) {
    libc::printf(s, a);
}

fn main() {
    let b = std::str::bytes("%d\n");
    let b8 = unsafe { std::vec::unsafe::to_ptr(b) };
    printf(b8, 4);
    let a = bind printf(b8, 5);
    a(); /* core dump */
}
