
#![feature(const_transmute)]

static mut ST:St = unsafe { std::mem::transmute([0u8;std::mem::size_of::<St>()]) };

struct St {
    x:f64,
    y:[f64;5],
}
