
#[repr(C)]
struct X {
    x : [i32],
}

pub extern "C" fn use_x(x : X) {
}
